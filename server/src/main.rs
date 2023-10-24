use axum::extract::ws::{ Message, WebSocket, WebSocketUpgrade };
use axum::extract::{ self, State, Query, Extension };
use axum::response::{ self, IntoResponse, };
use axum::http::{ Method, StatusCode, };
use axum::routing::{ get, post, patch };
use axum::Router;
use tower_http::cors::{ CorsLayer, Any };
use futures::sink::SinkExt;
use futures::stream::{ SplitSink, SplitStream, StreamExt };
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt };
use serde::{ Deserialize, Serialize };
use serde_json;
use std::sync::{ Arc, Mutex, MutexGuard };
use std::net::TcpListener;
use sqlx::{ FromRow, Pool };
use sqlx::mysql::{ MySql, MySqlPoolOptions, };

enum ClientEvent {
    Skip,
    Leave, 
    Connect,
    ConnectCancel,
}

#[derive(Clone)]
enum SendEvent {
    ServerEvent(String),
    SkipEvent(usize),
}

#[derive(Serialize, Clone)]
#[serde(tag = "type", content = "data")]
enum ServerEvent { // maybe these should be serialized before sending to all users so we're just sending a string?
    Message { user_idx: usize, content: String, },
    Join(usize),
    Skip(usize),
    Leave(usize),
    ConnectRequest /*{ contact_fields: Arc<[ContactField]> }*/,
    ConnectSuccess(Arc<[UserContacts]>),
    ConnectFailure,
}

#[derive(Debug)]
struct ChatApp { 
    rooms: Mutex<Vec<Room>>
}

#[derive(Debug)]
struct Room {
    tx: broadcast::Sender<SendEvent>,
    users: Vec<String>,
    connection: Option<Vec<UserContacts>>,
}

#[derive(Deserialize)]
struct User {
    x500: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserContacts {
    x500: String, 
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    instagram: Option<String>,
    snapchat: Option<String>,
    discord: Option<String>,
}

#[derive(Deserialize, Serialize)]
enum ContactField {
    FirstName,
    LastName,
    PhoneNumber,
    Instagram,
    Snapchat, 
    Discord,
}

#[derive(Deserialize)]
struct UserContactUpdate {
    x500: String, 
    field: String,
    value: Option<String>,
}

#[derive(Deserialize)]
struct UserConnection {
    x500: String, 
    partner_x500: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    instagram: Option<String>,
    snapchat: Option<String>,
    discord: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
struct Contacts {
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    instagram: Option<String>,
    snapchat: Option<String>,
    discord: Option<String>,
}

struct SqlInsert(String);

struct OwnedReceiverTaskState {
    receiver: SplitStream<WebSocket>, 
    app: Arc<ChatApp>, 
    x500: String, 
    contacts: UserContacts,
    skipped_users: Vec<String>,
}

impl ClientEvent {

    fn from_u8(u8: u8) -> Option<Self> {
        match u8 {
            0 => Some(ClientEvent::Skip),
            1 => Some(ClientEvent::Leave),
            2 => Some(ClientEvent::Connect),
            3 => Some(ClientEvent::ConnectCancel),
            _ => None,
        }
    }

}

impl ServerEvent {

    // errors unhandled
    fn send(&self, tx: &broadcast::Sender<SendEvent>) {
        match serde_json::to_string(self) {
            Ok(json) => { let _ = tx.send(SendEvent::ServerEvent(json)); },
            Err(error) => (),
        }
    }

}

impl Room {

    fn new(tx: broadcast::Sender<SendEvent>) -> Self {
        Room { tx, users: Vec::new(), connection: None, }
    }

    fn remove_user(&mut self, x500: &str) {
        
        if let Some(idx) = self.users.iter().position(|x| *x == x500) {
            self.users.remove(idx);
        }

    }

    fn skip_users(&self, skipped_users: &mut Vec<String>)  {
        
        for x500 in &self.users {
            skipped_users.push(x500.to_owned());
        }
        
    }

}

impl SqlInsert {
    
    fn new(table: &str, columns: Option<&str>) -> Self {
        
        let mut sql: String = format!("INSERT INTO {table} ");
        
        if let Some(columns) = columns {
            sql.push('(');
            sql.push_str(columns);
            sql.push(')');
        }

        sql.push_str(" VALUES (");

        SqlInsert(sql)

    }

    fn open(mut self, initial: &str) -> Self {

        self.0.push('\"');
        self.0.push_str(initial);
        self.0.push('\"');

        self

    }

    fn value(mut self, value: &str) -> Self {
        
        self.0.push(',');
        self.0.push('\"');
        self.0.push_str(value);
        self.0.push('\"');

        self

    }

    fn value_optional(mut self, value: Option<&str>) -> Self {

        self.0.push(',');
        
        match value {
            Some(value) => {
                self.0.push('\"');
                self.0.push_str(&value);
                self.0.push('\"');
            }
            None => self.0.push_str("NULL"),
        }

        self

    }

    fn close(mut self) -> String {

        self.0.push(')');
        self.0

    }

}

const MAX_ROOM_SIZE: usize = 2;

#[tokio::main]
async fn main() {
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "chatu=trace".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool: Pool<MySql> = MySqlPoolOptions::new()
        .connect("mysql://cheemie:ex_pw@localhost:3306/chatu")
        .await
        .unwrap();

    let app: Arc<ChatApp> = Arc::new(ChatApp { rooms: Mutex::new(Vec::new()) });

    let cors: CorsLayer = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PATCH])
        .allow_headers(Any)
        .allow_origin(Any);

    let router: Router = Router::new()
        .route("/join", get(websocket_handler))
        .route("/get_contacts", get(get_contacts))
        .route("/new_contacts", post(new_contacts))
        .route("/edit_contact", patch(edit_contact))
        .route("/get_connections", get(get_connections))
        .route("/new_connection", get(new_connection))
        .with_state(app)
        .layer(cors)
        .layer(Extension(pool));

    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(router.into_make_service())
        .await
        .unwrap();

}

async fn get_connections(Query(User { x500, }): Query<User>, Extension(pool): Extension<Pool<MySql>>) -> Result<response::Json<Vec<Contacts>>, StatusCode> {

    let sql: &str = &format!("SELECT * FROM connections WHERE x500 = \"{x500}\"");
    
    let connections: Vec<Contacts> = sqlx::query_as(sql)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(response::Json(connections))

}

// maybe use the app state room contacts to build connections instead of json
async fn new_connection(Extension(pool): Extension<Pool<MySql>>, extract::Json(connection): extract::Json<UserConnection>) -> Result<(), StatusCode> {

    let table: &str = "connections";

    let columns: &str = "x500,partner_x500,first_name,last_name,phone_number,instagram,snapchat,discord";

    let sql = &SqlInsert::new(table, Some(columns))
        .open(&connection.x500)
        .value(&connection.partner_x500)
        .value_optional(connection.first_name.as_deref())
        .value_optional(connection.last_name.as_deref())
        .value_optional(connection.phone_number.as_deref())
        .value_optional(connection.instagram.as_deref())
        .value_optional(connection.snapchat.as_deref())
        .value_optional(connection.discord.as_deref())
        .close();

    sqlx::query(sql)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(())

}

async fn get_contacts(Query(User { x500, }): Query<User>, Extension(pool): Extension<Pool<MySql>>) -> Result<response::Json<Contacts>, StatusCode> {

    let sql: &str = &format!("SELECT * FROM contacts WHERE x500 = \"{x500}\"");

    let contacts: Contacts = sqlx::query_as(sql)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(response::Json(contacts))

}

async fn new_contacts(Extension(pool): Extension<Pool<MySql>>, extract::Json(contacts): extract::Json<UserContacts>) -> Result<(), StatusCode> {

    let table: &str = "contacts";

    let columns: &str = "x500,first_name,last_name,phone_number,instagram,snapchat,discord";

    let sql = &SqlInsert::new(table, Some(columns))
        .open(&contacts.x500)
        .value_optional(contacts.first_name.as_deref())
        .value_optional(contacts.last_name.as_deref())
        .value_optional(contacts.phone_number.as_deref())
        .value_optional(contacts.instagram.as_deref())
        .value_optional(contacts.snapchat.as_deref())
        .value_optional(contacts.discord.as_deref())
        .close();

    let _ = sqlx::query(&sql)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(())

}

async fn edit_contact(Query(UserContactUpdate { x500, field, value }): Query<UserContactUpdate>, Extension(pool): Extension<Pool<MySql>>) -> Result<(), StatusCode> {

    match field.as_str() {
        "first_name" | "last_name" | "phone_number" | "instagram" | "snapchat" | "discord" => Ok(()),
        _ => Err(StatusCode::NOT_ACCEPTABLE),
    }?;

    let sql: &str = &match value {
        Some(value) => format!("
            UPDATE contacts 
            SET {field} = \"{value}\" 
            WHERE x500 = \"{x500}\"
        "),
        None => format!("
            UPDATE contacts 
            SET {field} = NULL  
            WHERE x500 = \"{x500}\"
        "), 
    };

    sqlx::query(sql)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(())

}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(contacts): Query<UserContacts>, 
    State(state): State<Arc<ChatApp>>,
) -> impl IntoResponse {

    let x500: String = contacts.x500.to_owned();

    ws.on_upgrade(|socket| websocket(socket, state, x500, contacts))

}

async fn websocket(stream: WebSocket, mut app: Arc<ChatApp>, mut x500: String, mut contacts: UserContacts) {

    let (mut sender, mut receiver) = stream.split();

    let mut skipped_users: Vec<String> = Vec::new();

    'join_loop: loop {

        let (tx, mut rx, room_idx, user_idx) = find_room(&app.rooms, &skipped_users, &x500);

        if sender.send(Message::Text(room_idx.to_string())).await.is_err() {
            return;
        }
    
        let _ = tx.send(SendEvent::ServerEvent(serde_json::to_string::<ServerEvent>(&ServerEvent::Join(user_idx)).unwrap()));
    
        let mut send_task: JoinHandle<SplitSink<WebSocket, Message>> = tokio::spawn(async move {

            while let Ok(event) = rx.recv().await {

                // probably want to serialize this inside the revc task instead

                match event {
                    SendEvent::SkipEvent(idx) => {
                        if user_idx == idx {
                            break;
                        }

                        continue;
                    }
                    SendEvent::ServerEvent(json) => {
                        if sender.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                }

            }

            return sender;

        });
            
        let mut recv_task: JoinHandle<(OwnedReceiverTaskState, bool)> = tokio::spawn(async move {

            let mut skipped: bool = false;
 
            while let Some(Ok(msg)) = receiver.next().await {

                match msg {
                    Message::Text(content) => {
                        ServerEvent::Message { user_idx, content }.send(&tx);
                    }
                    Message::Binary(bytes) => {

                        let mut bytes = bytes.iter();

                        if let Some(event) = bytes.next().and_then(|u8| ClientEvent::from_u8(*u8)) {

                            match event {
                                ClientEvent::Skip => {

                                    if tx.send(SendEvent::SkipEvent(user_idx)).is_err() {
                                        break;
                                    }

                                    skipped = true;

                                    break;

                                }
                                ClientEvent::Leave => {
                                    break;
                                },
                                ClientEvent::Connect => {
                                    // once a user is connected, they will be locked from connecting until they cancel

                                    let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                                    let room: &mut Room = rooms.get_mut(room_idx).unwrap();

                                    let first_connection: bool = room.connection.is_none();

                                    if first_connection {
                                        room.connection = Some(Vec::new());
                                    }

                                    let user_is_unconnected: bool = first_connection || !room.connection.as_ref().unwrap().iter().any(|contacts| contacts.x500 == x500);

                                    if user_is_unconnected {

                                        // maybe move this into a UserContacts impl function
                                        //try and see if there's a way to avoid copying data until a connection is made

                                        room.connection.as_mut().unwrap().push(UserContacts { 
                                            x500: x500.to_owned(),
                                            first_name: bytes.next().filter(|byte| **byte == 1).and_then(|_| contacts.first_name.to_owned()),
                                            last_name: bytes.next().filter(|byte| **byte == 1).and_then(|_| contacts.last_name.to_owned()),
                                            phone_number: bytes.next().filter(|byte| **byte == 1).and_then(|_| contacts.phone_number.to_owned()),
                                            instagram: bytes.next().filter(|byte| **byte == 1).and_then(|_| contacts.instagram.to_owned()), 
                                            snapchat: bytes.next().filter(|byte| **byte == 1).and_then(|_| contacts.snapchat.to_owned()),
                                            discord: bytes.next().filter(|byte| **byte == 1).and_then(|_| contacts.discord.to_owned()),
                                        });

                                    }

                                    let all_users_connected: bool = !first_connection || room.connection.as_mut().unwrap().len() == room.users.len();

                                    if all_users_connected {

                                        let contacts: Vec<UserContacts> = room.connection.take().unwrap();

                                        ServerEvent::ConnectSuccess(contacts.into()).send(&tx);

                                        room.connection = Some(Vec::new());
                                        
                                        continue;
                                        
                                        // storing new connections should be done either in the send task or as part of a seperate post request made by the client after receiving the successful connection 

                                    }

                                    ServerEvent::ConnectRequest.send(&tx);
                                    
                                },
                                ClientEvent::ConnectCancel => {
                                    //can be used by the user who sent the connection to cancel a connection

                                    let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                                    let room: &mut Room = rooms.get_mut(room_idx).unwrap();

                                    if room.connection.is_some() {
                                        room.connection.as_mut().unwrap().clear();
                                    }

                                    ServerEvent::ConnectFailure.send(&tx);

                                }
                            };
                        }

                    }
                    _ => (),
                }
            
            }

            {
                let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                let room: &mut Room = rooms.get_mut(room_idx).unwrap();
                
                room.remove_user(&x500);

                if skipped {
                    room.skip_users(&mut skipped_users);
                }

                if room.connection.is_some() {
                    room.connection.as_mut().unwrap().clear();
                }

                ServerEvent::Leave(user_idx).send(&tx);
            }
    
            (OwnedReceiverTaskState { receiver, app, x500, contacts, skipped_users, }, skipped)
    
        });
    
        tokio::select! {
            
            _ = (&mut send_task) => {
                //we will run into issues here
                println!("⚠️ Send Task Ended Too Early");
                break 'join_loop recv_task.abort()
            },
            result = (&mut recv_task) => match result {
                Err(_) => {
                    println!("⚠️ Error in Recv Task");
                    break 'join_loop send_task.abort()
                },
                Ok((state, skipped)) => match skipped {
                    false => {
                        break 'join_loop send_task.abort()
                    },
                    true => match send_task.await {
                        Err(_) => break 'join_loop,
                        Ok(_sender) => {
                            sender = _sender;
                            receiver = state.receiver;
                            app = state.app;
                            x500 = state.x500;
                            contacts = state.contacts;
                            skipped_users = state.skipped_users;
                        }
                    }
                }
            }
        };
            
    }

    // if tx.receiver_count() == 1 {
    //     state.rooms.lock().unwrap().remove(i);
    // }

}

fn find_room(rooms: &Mutex<Vec<Room>>, skipped_users: &[String], x500: &str) -> (broadcast::Sender<SendEvent>, broadcast::Receiver<SendEvent>, usize, usize) {

    let mut rooms: MutexGuard<'_, Vec<Room>> = rooms.lock().unwrap();

    println!("{:#?}", rooms);

    for optimal_user_count in (0..MAX_ROOM_SIZE).rev() {

        'room_loop: for (room_idx, Room { tx, users, .. }) in rooms.iter_mut().enumerate() {

            println!("{room_idx}: {}, {}, {}", tx.receiver_count(), users.len(), optimal_user_count);
    
            if tx.receiver_count() != optimal_user_count {
                continue;
            }
    
            for skipped_x500 in skipped_users {
                if users.contains(skipped_x500) {
                    continue 'room_loop;
                }
            } 

            let user_idx: usize = users.len();
    
            users.push(x500.to_owned());
    
            return (tx.clone(), tx.subscribe(), room_idx, user_idx);
    
        }

    }

    let (tx, rx) = broadcast::channel::<SendEvent>(2);

    let room_idx: usize = rooms.len();

    let mut room: Room = Room::new(tx.clone());

    room.users.push(x500.to_owned());

    rooms.push(room);

    let user_idx: usize = 0;

    return (tx, rx, room_idx, user_idx);

}
