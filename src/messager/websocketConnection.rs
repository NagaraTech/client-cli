use tokio::net::TcpStream;
use tokio::sync::OnceCell;
use tokio_tungstenite::{connect_async, MaybeTlsStream, tungstenite::protocol::WebSocket, WebSocketStream};
use url::Url;
use lazy_static::lazy_static;

use tokio::sync::Mutex;

static WS_CONN: OnceCell<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>> > = OnceCell::const_new();

pub async fn init_ws_conn(ws_url:String)  {
    // let ws_url = "ws://31.220.78.83:23333/vlc23333";
    let url = Url::parse(&ws_url).expect("Failed to parse WS_URL");

    let (ws_stream, _) = connect_async(url)
        .await
        .expect("Failed to connect to WebSocket");

    WS_CONN.set(Mutex::new(ws_stream)).expect("Failed to init ws");
}

pub async fn get_ws_conn() -> &'static Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>> {
    WS_CONN.get().expect("connection not available")
}