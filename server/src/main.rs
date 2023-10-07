use axum::extract::ws::{ Message, WebSocket, WebSocketUpgrade };
use axum::extract::{ self, State, Query, Extension, };
use axum::response::{ self, IntoResponse, };
use axum::http::StatusCode;
use axum::routing::{ get, post, put };
use axum::Router;
use futures::sink::SinkExt;
use futures::stream::{ SplitSink, SplitStream, StreamExt };
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt };
use serde::{ Deserialize, Serialize };
//use std::collections::{ HashMap, HashSet };
use std::sync::{ Arc, Mutex, MutexGuard };
use std::net::TcpListener;
use sqlx::{ FromRow, Pool };
use sqlx::mysql::{ MySql, MySqlPoolOptions, };

enum ClientEvent {
    Skip,
    Leave, 
    Connect,
    ConnectDecline,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "content")]
enum ServerEvent {
    Join,
    Leave,
    ConnectRequest,
    ConnectSuccess,
    ConnectFailure,
}

#[derive(Debug)]
struct ChatApp { 
    rooms: Mutex<Vec<Room>>,
    db: Pool<MySql>
}

#[derive(Debug)]
struct Room {
    tx: broadcast::Sender<Option<String>>,
    users: Vec<String>,
}

#[derive(Deserialize)]
struct User {
    x500: String,
}

#[derive(Deserialize)]
struct UserContacts {
    x500: String, 
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    instagram: Option<String>,
    snapchat: Option<String>,
    discord: Option<String>,
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

// returned to the user on login or after they enter contacts
// returned to all users connected with them on connections page
#[derive(Debug, FromRow, Serialize)]
struct Contacts {
    first_name: Option<String>,
    last_name: Option<String>,
    phone_number: Option<String>,
    instagram: Option<String>,
    snapchat: Option<String>,
    discord: Option<String>,
}

impl ClientEvent {

    fn from_u8(u8: u8) -> Option<Self> {
        match u8 {
            0 => Some(ClientEvent::Skip),
            1 => Some(ClientEvent::Leave),
            2 => Some(ClientEvent::Connect),
            _ => None,
        }
    }

}

impl Room {

    fn new(tx: broadcast::Sender<Option<String>>, x500: String) -> Self {
        Room { tx: tx.clone(), users: vec![x500.to_owned()] }
    }

    fn remove_user(&mut self, x500: &str) -> &Self {
        
        if let Some(idx) = self.users.iter().position(|x| *x == x500) {
            self.users.remove(idx);
        }

        self

    }

    fn skip_users(&self, skipped_users: &mut Vec<String>) -> &Self  {
        
        for x500 in &self.users {
            skipped_users.push(x500.to_owned());
        }

        self
        
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
        .connect("mysql://cheemie:ex_pw@localhost:3306/chatu").await.unwrap();

    let app: Arc<ChatApp> = Arc::new(ChatApp { rooms: Mutex::new(Vec::new()), db: pool });

    let router: Router = Router::new()
        .route("/join", get(websocket_handler))
        .route("/get_contacts", get(get_contacts))
        .route("/new_contacts", post(new_contacts))
        .route("/edit_contact", put(edit_contact))
        .route("/get_connections", get(get_connections))
        .route("/new_connection", get(new_connection))
        .with_state(app);

    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(router.into_make_service())
        .await
        .unwrap();

}

async fn get_connections(Query(User { x500, }): Query<User>, State(app): State<Arc<ChatApp>>) -> Result<response::Json<Vec<Contacts>>, StatusCode> {

    let sql: &str = "SELECT * FROM connections WHERE x500 = \"$1\"";
    
    let connections: Vec<Contacts> = sqlx::query_as(sql)
        .bind(x500)
        .fetch_all(&app.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(response::Json(connections))

}

// maybe use the app state room contacts to build connections instead of json
async fn new_connection(State(app): State<Arc<ChatApp>>, extract::Json(connection): extract::Json<UserConnection>) -> Result<(), StatusCode> {

    let sql: &str = "
        INSERT INTO connections (
            x500,
            partner_x500,
            first_name, 
            last_name,
            phone_number,
            instagram,
            snapchat, 
            discord 
        ) VALUES (
            \"$1\",
            \"$2\",
            \"$3\",
            \"$4\",
            \"$5\",
            \"$6\",
            \"$7\",
            \"$8\"
        )
    ";

    sqlx::query(sql)
        .bind(connection.x500)
        .bind(connection.partner_x500)
        .bind(connection.first_name)
        .bind(connection.last_name)
        .bind(connection.phone_number)
        .bind(connection.instagram)
        .bind(connection.snapchat)
        .bind(connection.discord)
        .execute(&app.db)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(())

}

async fn get_contacts(Query(User { x500, }): Query<User>, State(app): State<Arc<ChatApp>>) -> Result<response::Json<Contacts>, StatusCode> {

    let sql: &str = "SELECT * FROM contacts WHERE x500 = \"$1\"";

    let contacts: Contacts = sqlx::query_as(sql)
        .bind(x500)
        .fetch_optional(&app.db)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(response::Json(contacts))

}

async fn new_contacts(State(app): State<Arc<ChatApp>>, extract::Json(contacts): extract::Json<UserContacts>) -> &'static str {

    let sql: &str = "
        INSERT INTO contacts (
            x500,
            first_name, 
            last_name,
            phone_number,
            instagram,
            snapchat, 
            discord 
        ) VALUES (
            \"$1\",
            \"$2\",
            \"$3\",
            \"$4\",
            \"$5\",
            \"$6\",
            \"$7\"
        )
    ";

    let _ = sqlx::query(sql)
        .bind(contacts.x500)
        .bind(contacts.first_name)
        .bind(contacts.last_name)
        .bind(contacts.phone_number)
        .bind(contacts.instagram)
        .bind(contacts.snapchat)
        .bind(contacts.discord)
        .execute(&app.db)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY);

    "hai"

    //Ok(())

}

async fn edit_contact(Query(contact_update): Query<UserContactUpdate>, State(app): State<Arc<ChatApp>>) -> Result<(), StatusCode> {

    match contact_update.field.as_str() {
        "x500" | "first_name" | "last_name" | "phone_number" | "instagram" | "snapchat" | "discord" => Ok(()),
        _ => Err(StatusCode::NOT_ACCEPTABLE),
    }?;

    let contact_deleted: bool = contact_update.value.is_none();

    if contact_deleted {

        let sql: &str = "
            UPDATE contacts 
            SET $1 = \"$2\" 
            WHERE x500 = NULL 
        ";

        sqlx::query(sql)
            .bind(contact_update.x500)
            .bind(contact_update.field)
            .execute(&app.db)
            .await
            .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

        return Ok(());

    }

    let sql: &str = "
        UPDATE contacts 
        SET $1 = \"$2\" 
        WHERE x500 = \"$3\" 
    ";

    sqlx::query(sql)
        .bind(contact_update.x500)
        .bind(contact_update.field)
        .bind(contact_update.value)
        .execute(&app.db)
        .await
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    Ok(())

}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(user_contacts): Query<UserContacts>, 
    State(state): State<Arc<ChatApp>>,
) -> impl IntoResponse {

    let x500: String = user_contacts.x500;

    let contacts: Contacts = Contacts {
        first_name: user_contacts.first_name,
        last_name: user_contacts.last_name,
        phone_number: user_contacts.phone_number,
        instagram: user_contacts.instagram,
        snapchat: user_contacts.snapchat,
        discord: user_contacts.discord,
    };

    ws.on_upgrade(|socket| websocket(socket, state, x500, /*contacts*/))

}

async fn websocket(stream: WebSocket, app: Arc<ChatApp>, x500: String, /*contacts: Contacts*/) {

    let (mut sender, mut receiver) = stream.split();

    let mut skipped_users: Vec<String> = Vec::new();

    'join_loop: loop {

        let (tx, mut rx, room_idx) = find_room(&app.rooms, &skipped_users, &x500);
    
        let msg: String = format!("{x500} joined room {room_idx}.");
        tracing::debug!("{msg}");
        let _ = tx.send(Some(msg));
    
        let mut send_task: JoinHandle<SplitSink<WebSocket, Message>> = tokio::spawn(async move {
            
            while let Ok(Some(msg)) = rx.recv().await {

                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }

            }

            return sender;

        });
    
        let tx_chat: broadcast::Sender<Option<String>> = tx.clone();
            
        let mut recv_task: JoinHandle<(SplitStream<WebSocket>, bool)> = tokio::spawn(async move {

            let mut skipped: bool = false;
            //let name = contacts.first_name.as_ref().map(|str| str.as_str()).unwrap_or("");
                        
            while let Some(Ok(msg)) = receiver.next().await {

                match msg {
                    Message::Text(msg) => {
        
                        let _ = tx_chat.send(Some(msg));

                    }
                    Message::Binary(bytes) => {

                        if let Some(event) = bytes.get(0).and_then(|u8| ClientEvent::from_u8(*u8)) {
                            match event {
                                ClientEvent::Skip => {
                                    skipped = true;
                                    let _ = tx.send(None);
                                    break;
                                }
                                ClientEvent::Leave => {
                                    break;
                                },
                                ClientEvent::Connect => {
                                    //sqlx::query("SELECT * FROM contacts").fetch_all(&app.db);
                                    
                                    //let name: &String = &(contacts.first_name.unwrap_or("".to_owned())).clone();
                                    //println!("{name}");
                                },
                                ClientEvent::ConnectDecline => {

                                }
                            };
                        }

                    }
                    _ => (),
                }
            
            }
    
            (receiver, skipped)
    
        });
    
        tokio::select! {
            _ = (&mut send_task) => {

                recv_task.abort();

                let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                let room: &mut Room = rooms.get_mut(room_idx).unwrap();
                room.remove_user(&x500);

                break 'join_loop;  
            }
            result = (&mut recv_task) => match result {
                Err(_) => {

                    send_task.abort();

                    let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                    let room: &mut Room = rooms.get_mut(room_idx).unwrap();
                    room.remove_user(&x500);

                    break 'join_loop;

                },
                Ok((_receiver, skipped)) => match skipped {
                    false => {

                        send_task.abort();

                        let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                        let room: &mut Room = rooms.get_mut(room_idx).unwrap();
                        room.remove_user(&x500);

                        break 'join_loop;

                    },
                    true => match send_task.await {
                        Err(_) => {

                            let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                            let room: &mut Room = rooms.get_mut(room_idx).unwrap();
                            room.remove_user(&x500);

                            break 'join_loop;

                        }
                        Ok(_sender) => {

                            let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                            let room: &mut Room = rooms.get_mut(room_idx).unwrap();
                            room.remove_user(&x500).skip_users(&mut skipped_users);

                            receiver = _receiver;
                            sender = _sender;

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

fn find_room(rooms: &Mutex<Vec<Room>>, skipped_users: &[String], x500: &str) -> (broadcast::Sender<Option<String>>, broadcast::Receiver<Option<String>>, usize) {

    let mut rooms: MutexGuard<'_, Vec<Room>> = rooms.lock().unwrap();

    //println!("{:#?}", rooms);

    for optimal_user_count in (0..MAX_ROOM_SIZE).rev() { //test this

        'room_loop: for (room_idx, Room { tx, users }) in rooms.iter_mut().enumerate() {

            //println!("{room_idx}: {}, {}, {}", tx.receiver_count(), users.len(), optimal_user_count);
    
            if tx.receiver_count() != optimal_user_count {
                continue;
            }
    
            for skipped_x500 in skipped_users {
                if users.contains(skipped_x500) {
                    continue 'room_loop;
                }
            } 
    
            users.push(x500.to_owned());
    
            return (tx.clone(), tx.subscribe(), room_idx);
    
        }

    }

    let (tx, rx) = broadcast::channel::<Option<String>>(2);

    let room_idx: usize = rooms.len();

    rooms.push(Room::new(tx.clone(), x500.to_owned()));

    return (tx, rx, room_idx);

}
