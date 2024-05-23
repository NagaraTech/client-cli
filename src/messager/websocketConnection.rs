use clap::builder::Str;
use tokio::net::TcpStream;
use tokio::sync::OnceCell;
use tokio_tungstenite::{connect_async, MaybeTlsStream, tungstenite::protocol::WebSocket, WebSocketStream};
use url::Url;
use lazy_static::lazy_static;

use tokio::sync::Mutex;

static WS_CONN: OnceCell<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>> > = OnceCell::const_new();

// async fn init_ws_conn() -> Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>> {
//     let ws_url = "ws://127.0.0.1:23333/vlc23333";
//
// }
pub async fn get_ws_conn(ws_url:String) -> WebSocketStream<MaybeTlsStream<TcpStream>>{
    let url = Url::parse(&ws_url).expect("Failed to parse WS_URL");

    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to WebSocket");

    ws_stream
}