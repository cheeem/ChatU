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
use std::net::TcpListener;

#[derive(Debug)]
struct AppState {
    rooms: Mutex<Vec<Room>>,
}

#[derive(Debug)]
struct Room {
    tx: broadcast::Sender<String>,
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

    let rooms: Mutex<Vec<Room>> = Mutex::new(Vec::new());

    let app_state: Arc<AppState> = Arc::new(AppState { rooms });

    let app: Router = Router::new()
        .route("/", get(index))
        .route("/websocket", get(websocket_handler))
        .with_state(app_state);

    let listener: TcpListener = TcpListener::bind("127.0.0.1:3000").unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(JoiningUser { id, }): Query<JoiningUser>, 
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state, id))
}

fn find_room(rooms: &Mutex<Vec<Room>>, skipped_users: &[String], user_id: &str) -> (broadcast::Sender<String>, broadcast::Receiver<String>, usize) {

    let mut rooms: MutexGuard<'_, Vec<Room>> = rooms.lock().unwrap();

    'room_loop: for (room_idx, Room { tx, users }) in rooms.iter().enumerate() {

        if tx.receiver_count() >= MAX_ROOM_SIZE {
            continue;
        }

        for skipped_user_id in skipped_users {
            if users.contains(skipped_user_id) {
                continue 'room_loop;
            }
        } 

        println!("{:#?}", rooms);

        return (tx.clone(), tx.subscribe(), room_idx);

    }

    let (tx, rx) = broadcast::channel(2);

    let room_idx: usize = rooms.len();

    rooms.push(Room { 
        tx: tx.clone(), 
        users: vec![user_id.into()],
    });

    println!("{:#?}", rooms);

    return (tx, rx, room_idx);

}

async fn websocket(stream: WebSocket, state: Arc<AppState>, user_id: String) {

    let (mut sender, mut receiver) = stream.split();

    let mut skipped_users: Vec<String> = Vec::new();

    loop {

        let state = state.clone();

        let (tx, mut rx, room_idx) = find_room(&state.rooms, &skipped_users, &user_id);
    
        let msg: String = format!("{user_id} joined.");
        tracing::debug!("{msg}");
        let _ = tx.send(msg);
    
        let mut send_task: JoinHandle<SplitSink<WebSocket, Message>> = tokio::spawn(async move {
            
            while let Ok(msg) = rx.recv().await {

                if &msg == "__skip" {
                    return sender;
                }

                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }

            }

            return sender;

        });
    
        let tx_chat: broadcast::Sender<String> = tx.clone();
    
        let mut recv_task: JoinHandle<(SplitStream<WebSocket>, bool)> = tokio::spawn(async move {
            
            let mut skipped: bool = false;
            
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
       
                if text == "__skip" {
                    skipped = true;
                    let _ = tx_chat.send(text);
                    break;
                }

                let _ = tx_chat.send(text);
            
            }
    
            (receiver, skipped)
    
        });
    
        tokio::select! {
            _ = (&mut send_task) => {
                recv_task.abort();
                return;
            }
            result = (&mut recv_task) => match result {
                Err(_) => return,
                Ok((_receiver, skipped)) => match skipped {
                    false => return,
                    true => match send_task.await {
                        Err(_) => return,
                        Ok(_sender) => {

                            receiver = _receiver;
                            sender = _sender;

                            let rooms: &mut [Room] = &mut*state.rooms.lock().unwrap();
                            let room: &mut Room = rooms.get_mut(room_idx).unwrap();

                            for user_id in &room.users {
                                skipped_users.push(user_id.to_owned());
                            }

                            room.users.remove(room.users.iter().position(|x| x == &user_id).unwrap())

                        }
                    }
                }
            }
        };
        
        // clean up
    
        let msg: String = format!("{user_id} left.");
        tracing::debug!("{msg}");
        let _ = tx.send(msg);

    }

    // if tx.receiver_count() == 1 {
    //     state.rooms.lock().unwrap().remove(i);
    // }

}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../chat.html"))
}
