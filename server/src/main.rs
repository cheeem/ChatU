use axum::extract::ws::{ Message, WebSocket, WebSocketUpgrade };
use axum::extract::{ State, Query };
use axum::response::{ Html, IntoResponse };
use axum::routing::get;
use axum::Router;
use futures::sink::SinkExt;
use futures::stream::{ SplitSink, SplitStream, StreamExt };
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt };
use serde::Deserialize;
use std::collections::{ HashMap, HashSet };
use std::sync::{ Arc, Mutex, MutexGuard };
use std::sync::atomic::{ Ordering, AtomicBool };
use std::net::TcpListener;

#[derive(Debug)]
struct ChatApp {
    rooms: Mutex<Vec<Room>>,
}

enum ClientEvent {
    Skip,
    Leave, 
    Connect,
}

enum ServerEvent {
    Join,
    Leave,
    ConnectRequest,
    ConnectSuccess,
    ConnectFailure,
}

impl Room {

    fn new(tx: broadcast::Sender<Option<String>>, user_id: String) -> Self {
        Room { tx: tx.clone(), users: vec![user_id.to_owned()] }
    }

    fn remove_user(&mut self, user_id: &str) -> &Self {
        
        if let Some(idx) = self.users.iter().position(|x| *x == user_id) {
            self.users.remove(idx);
        }

        self

    }

    fn skip_users(&self, skipped_users: &mut Vec<String>) -> &Self  {
        
        for user_id in &self.users {
            skipped_users.push(user_id.to_owned());
        }

        self
        
    }

}

#[derive(Debug)]
struct Room {
    tx: broadcast::Sender<Option<String>>,
    users: Vec<String>,
}

#[derive(Deserialize)]
struct JoiningUser {
    id: String,
}

const MAX_ROOM_SIZE: usize = 2;

#[tokio::main]
async fn main() {
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "chatu=trace".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app: Arc<ChatApp> = Arc::new(ChatApp { rooms: Mutex::new(Vec::new()) });

    let router: Router = Router::new()
        .route("/join", get(websocket_handler))
        .with_state(app);

    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(router.into_make_service())
        .await
        .unwrap();

}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(JoiningUser { id, }): Query<JoiningUser>, 
    State(state): State<Arc<ChatApp>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state, id))
}

fn find_room(rooms: &Mutex<Vec<Room>>, skipped_users: &[String], user_id: &str) -> (broadcast::Sender<Option<String>>, broadcast::Receiver<Option<String>>, usize) {

    let mut rooms: MutexGuard<'_, Vec<Room>> = rooms.lock().unwrap();

    println!("{:#?}", rooms);

    for optimal_user_count in (0..MAX_ROOM_SIZE).rev() { //test this

        'room_loop: for (room_idx, Room { tx, users }) in rooms.iter_mut().enumerate() {

            println!("{room_idx}: {}, {}, {}", tx.receiver_count(), users.len(), optimal_user_count);
    
            if tx.receiver_count() != optimal_user_count {
                continue;
            }
    
            for skipped_user_id in skipped_users {
                if users.contains(skipped_user_id) {
                    continue 'room_loop;
                }
            } 
    
            users.push(user_id.to_owned());
    
            return (tx.clone(), tx.subscribe(), room_idx);
    
        }

    }

    let (tx, rx) = broadcast::channel::<Option<String>>(2);

    let room_idx: usize = rooms.len();

    rooms.push(Room::new(tx.clone(), user_id.to_owned()));

    return (tx, rx, room_idx);

}

async fn websocket(stream: WebSocket, app: Arc<ChatApp>, user_id: String) {

    let (mut sender, mut receiver) = stream.split();

    let mut skipped_users: Vec<String> = Vec::new();

    'join_loop: loop {

        let (tx, mut rx, room_idx) = find_room(&app.rooms, &skipped_users, &user_id);
    
        let msg: String = format!("{user_id} joined room {room_idx}.");
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
                        
            while let Some(Ok(msg)) = receiver.next().await {

                match msg {
                    Message::Text(msg) => {
        
                        let _ = tx_chat.send(Some(msg));

                    }
                    Message::Binary(bytes) => {

                        println!("{:#?}", bytes);

                        let mut iter = bytes.iter();

                        match iter.next() {
                            Some(0) => {
                                //skip
                                skipped = true;
                                let _ = tx.send(None);
                                break;
                            }
                            Some(1) => {
                                //leave
                                break;
                            },
                            Some(2) => {
                                //connect
                            },
                            _ => (),
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
                room.remove_user(&user_id);

                break 'join_loop;  
            }
            result = (&mut recv_task) => match result {
                Err(_) => {

                    send_task.abort();

                    let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                    let room: &mut Room = rooms.get_mut(room_idx).unwrap();
                    room.remove_user(&user_id);

                    break 'join_loop;

                },
                Ok((_receiver, skipped)) => match skipped {
                    false => {

                        send_task.abort();

                        let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                        let room: &mut Room = rooms.get_mut(room_idx).unwrap();
                        room.remove_user(&user_id);

                        break 'join_loop;

                    },
                    true => match send_task.await {
                        Err(_) => {

                            let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                            let room: &mut Room = rooms.get_mut(room_idx).unwrap();
                            room.remove_user(&user_id);

                            break 'join_loop;

                        }
                        Ok(_sender) => {

                            let rooms: &mut [Room] = &mut *app.rooms.lock().unwrap();
                            let room: &mut Room = rooms.get_mut(room_idx).unwrap();
                            room.remove_user(&user_id).skip_users(&mut skipped_users);

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
