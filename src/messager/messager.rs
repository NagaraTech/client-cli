use std::collections::HashMap;
use blake2::{Blake2s256, Digest};
use bytes::Bytes;
use std::time::{SystemTime, UNIX_EPOCH};
use prost::Message;
use crate::messager::z_messaage;
use crate::messager::chat_message;
use crate::messager::protos;
use crate::messager::websocketConnection::get_ws_conn;
use crate::config::config;
use crate::keystore::keystore;
use tokio_tungstenite::{WebSocketStream, tungstenite::protocol, MaybeTlsStream, connect_async};
use futures::{SinkExt, StreamExt};
use std::fs;
use tokio::net::TcpStream;
use url::Url;
use rand::{Rng, RngCore};
use rand::thread_rng;
use std::process;
use ed25519_dalek::PublicKey;

mod msg {
    include!(concat!(env!("OUT_DIR"), "/msg.rs"));
}

pub struct AppState {
    pub(crate) socket: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pub(crate) to: String,
}

impl AppState {
    pub async fn init_state(url: String, to: String) -> Self {
        let url = Url::parse(&url).expect("Failed to parse WS_URL");
        let (ws_stream, _) = connect_async(url)
            .await
            .expect("Failed to connect to WebSocket");

        AppState {
            socket: Option::from(ws_stream),
            to,
        }
    }

    pub fn buildMsg(&self, msg: &str) -> Vec<u8> {
        let config = config::Config::load().unwrap();
        let key_store = keystore::Keystore::load(config.KeypairPath).unwrap();
        let public_key = key_store.public_key();


        let to_bytes = hex::decode(self.to.clone()).unwrap();
        let from_bytes = hex::decode(public_key).unwrap();



        let mut id = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut id);

        let mut id2 = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut id2);

        let mut v = HashMap::new();
        v.insert("clock".to_string(), 1);

        let cc = msg::Clock {
            values: v
        };


        let ci = msg::ClockInfo {
            clock: Some(cc),
            node_id: id.to_vec(),
            clock_hash: vec![],
            message_id: id2.to_vec(),
            count: 2,
            create_at: 1243,
        };

        let chat = msg::ZChat {
            message_data: msg.parse().unwrap(),
            clock: Some(ci),
        };

        let mut chat_buf = Vec::new();
        chat.encode(&mut chat_buf).expect("Failed to encode chat");

        let signature = key_store.sign(&chat_buf);

        let z_msg = msg::ZMessage {
            id: id2.to_vec(),
            from: from_bytes.clone(),
            public_key: from_bytes,
            data: chat_buf,
            to: to_bytes,
            r#type: msg::ZType::Zchat as i32,
            version: 0,
            signature: signature.to_vec(),
        };

        let mut z_msg_buf = Vec::new();
        z_msg.encode(&mut z_msg_buf).expect("Failed to encode z_msg");

        z_msg_buf
    }


    pub fn randomMsg() -> Vec<u8> {
        let config = config::Config::load().unwrap();
        let key_store = keystore::Keystore::load(config.KeypairPath).unwrap();
        let public_key = key_store.public_key();

        let to_bytes = public_key.clone().encode_to_vec();
        let from_bytes = public_key.clone().encode_to_vec();
        let mut id = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut id);

        let mut id2 = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut id2);

        let mut id3 = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut id3);

        let mut v = HashMap::new();
        v.insert(hex::encode(id), 1);

        let cc = msg::Clock {
            values: v
        };


        let ci = msg::ClockInfo {
            clock: Some(cc),
            node_id: id.to_vec(),
            clock_hash: vec![],
            message_id: id2.to_vec(),
            count: 2,
            create_at: 1243,
        };


        let pid = process::id();
        let r = format!("hello hetu! {}", pid);
        let chat = msg::ZChat {
            message_data: r,
            clock: Some(ci),
        };

        let mut chat_buf = Vec::new();
        chat.encode(&mut chat_buf).expect("Failed to encode chat");

        let signature = key_store.sign(&chat_buf);

        let z_msg = msg::ZMessage {
            id: id2.to_vec(),
            from: from_bytes.clone(),
            public_key: from_bytes,
            data: chat_buf,
            to: to_bytes,
            r#type: msg::ZType::Zchat as i32,
            version: 0,
            signature: signature.to_vec(),
        };

        let mut z_msg_buf = Vec::new();
        z_msg.encode(&mut z_msg_buf).expect("Failed to encode z_msg");

        z_msg_buf
    }


    pub async fn send_message(&mut self, data: Vec<u8>) {
        if let Some(socket) = &mut self.socket {
            socket.send(protocol::Message::Binary(data)).await.expect("send fail");
            println!("Message sent");
        } else {
            println!("No active WebSocket connection.");
        }
    }
}


pub async fn account_send_message(
    from: String,
    to: String,
    message: String,
    // node: String,
) {
    let config = config::Config::load().unwrap();
    let key_store = keystore::Keystore::load(config.KeypairPath).unwrap();

    // let signature = key_store.sign(&message);


    // let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
    //
    // let message_id_time = format!("{}{}", message.clone(), now);
    //
    // let mut hasher = Blake2s256::new();
    // hasher.update(message_id_time);
    // let message_id = hasher.finalize();
    //
    // let chat_message = chat_message::ChatMessage {
    //     id: message_id.to_vec(),
    //     version: 0,
    //     public_key: from.as_bytes().to_vec(),
    //     data: message.as_bytes().to_vec(),
    //     signature: Vec::from(signature), // This seems incorrect in the original code, adjust as needed
    //     from: from.as_bytes().to_vec(),
    //     to: to.as_bytes().to_vec(),
    //     r#type: 0,
    //     timestamp: now as u64,
    // };
    // let mut chat_buffer = Vec::new();
    // chat_message.encode(&mut chat_buffer).unwrap();


    // let mut hasher2 = Blake2s256::new();
    // hasher2.update(&chat_buffer);
    // let hash_id = hasher2.finalize().to_vec();

    // let message_created = z_messaage::ZMessage {
    //     id: hash_id,
    //     version: 0,
    //     r#type: 0,
    //     public_key: from.as_bytes().to_vec(),
    //     data: chat_buffer,
    //     signature: Vec::from(signature),
    //     from: from.as_bytes().to_vec(),
    //     to: node.as_bytes().to_vec(),
    // };


    // let message_created = protos::ZMessage{
    //     id: hash_id,
    //     version: 0,
    //     r#type: 0,
    //     action: i32::from(protos::ZAction::ZTypeWrite),
    //     identity: i32::from(protos::ZIdentity::UTypeCli),
    //     public_key: from.as_bytes().to_vec(),
    //     data: chat_buffer,
    //     signature: Vec::from(signature),
    //     from: from.as_bytes().to_vec(),
    //     to: to.as_bytes().to_vec(),
    // };


    let mut my_map: HashMap<String, u64> = HashMap::new();
    let cc = protos::Clock {
        values: my_map,
    };
    let ci = protos::ClockInfo {
        clock: Option::from(cc),
        id: Vec::from("1"),
        message_id: Vec::from("222"),
        count: 1,
        create_at: 123,
    };

    let chat = protos::ZChat {
        message_data: message,
        clock: Option::from(ci),
    };

    let mut chat_buffer = Vec::new();
    chat.encode(&mut chat_buffer).unwrap();
    let mut hasher2 = Blake2s256::new();
    hasher2.update(&chat_buffer);
    let hash_id = hasher2.finalize().to_vec();

    let message_created = protos::ZMessage {
        id: vec![],
        version: 0,
        r#type: i32::from(protos::ZType::Zchat),
        action: i32::from(protos::ZAction::ZTypeWrite),
        identity: i32::from(protos::ZIdentity::UTypeCli),
        public_key: vec![],
        data: chat_buffer,
        signature: vec![],
        from: vec![],
        to: decode_hex(to).unwrap(),
    };

    let mut buffer = Vec::new();
    message_created.encode(&mut buffer).unwrap();
    println!("z_message:{:?}", buffer);


    let mut conn = get_ws_conn("".to_string()).await;

    conn.send(protocol::Message::Binary(buffer)).await.expect("send fail");
}


pub async fn account_read_message(ws_url: String) {
    let conn = get_ws_conn(ws_url).await;
    let (mut write, mut read) = conn.split();
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    println!("Received: {}", msg);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

pub fn save_state(state: String) {
    fs::write("/tmp/app_state.txt", state).expect("Unable to write to file");
}

pub fn load_state() -> String {
    fs::read_to_string("/tmp/app_state.txt").expect("Unable to read file")
}


fn decode_hex(hex_str: String) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex_str)
}

fn string_to_u8_array(message: &str) -> Vec<u8> {
    // Get the current time
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    // Create the string by concatenating message and milliseconds
    let full_message = format!("{}{}", message, now);

    // Convert the string to a vector of bytes
    full_message.into_bytes()
}