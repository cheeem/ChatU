use axum::extract::ws::{ Message, WebSocket, WebSocketUpgrade };
use axum::extract::{ State, Query };
use axum::response::{ Html, IntoResponse };
use axum::routing::get;
use axum::Router;
use futures::{ sink::SinkExt, stream::StreamExt };
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt };
use serde::Deserialize;
use std::collections::{ HashMap, HashSet };
use std::sync::{ Arc, Mutex, MutexGuard };
use std::net::TcpListener;

// Our shared state
struct AppState {
    rooms: Mutex<Vec<broadcast::Sender<String>>>,
}

#[derive(Deserialize)]
struct JoiningUser {
    id: String,
}

#[tokio::main]
async fn main() {
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "chatu=trace".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let rooms: Mutex<Vec<broadcast::Sender<String>>> = Mutex::new(Vec::new());

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

fn find_room(rooms: &Mutex<Vec<broadcast::Sender<String>>>) -> (broadcast::Sender<String>, broadcast::Receiver<String>) {

    let mut rooms: MutexGuard<'_, Vec<broadcast::Sender<String>>> = rooms.lock().unwrap();

    for tx in rooms.iter() {
        println!("{}", tx.receiver_count());
        if tx.receiver_count() < 2 {
            return (tx.clone(), tx.subscribe());
        }
    }

    let (tx, rx) = broadcast::channel(2);

    rooms.push(tx.clone());

    return (tx, rx);

}

async fn websocket(stream: WebSocket, state: Arc<AppState>, user_id: String) {

    let (mut sender, mut receiver) = stream.split();

    let (tx, mut rx) = find_room(&state.rooms);
    
    //let tx: &broadcast::Sender<String> = state.rooms.lock().unwrap().get(i).unwrap();

    let msg: String = format!("{user_id} joined.");
    tracing::debug!("{msg}");
    let _ = tx.send(msg);

    let mut send_task: JoinHandle<()> = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx_chat: broadcast::Sender<String> = tx.clone();
    let name: String = user_id.clone();

    let mut recv_task: JoinHandle<()> = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let _ = tx_chat.send(format!("{name}: {text}"));
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
    
    // clean up

    let msg: String = format!("{user_id} left.");
    tracing::debug!("{msg}");
    let _ = tx.send(msg);

    // if tx.receiver_count() == 1 {
    //     state.rooms.lock().unwrap().remove(i);
    // }

}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../chat.html"))
}
