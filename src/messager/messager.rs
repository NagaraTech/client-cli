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
use tokio_tungstenite::{WebSocketStream, tungstenite::protocol};
use futures::{SinkExt, StreamExt};
use generic_array::GenericArray;
use generic_array::typenum::U32;


pub async fn account_send_message(
    from: String,
    to: String,
    message: String,
    // node: String,
) {
    let config = config::Config::load().unwrap();
    let key_store = keystore::Keystore::load(config.KeypairPath).unwrap();
    let signature = key_store.sign(&message);


    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

    let message_id_time = format!("{}{}", message.clone(), now);

    let mut hasher = Blake2s256::new();
    hasher.update(message_id_time);
    let message_id = hasher.finalize();

    let chat_message = chat_message::ChatMessage {
        id: message_id.to_vec(),
        version: 0,
        public_key: from.as_bytes().to_vec(),
        data: message.as_bytes().to_vec(),
        signature: Vec::from(signature), // This seems incorrect in the original code, adjust as needed
        from: from.as_bytes().to_vec(),
        to: to.as_bytes().to_vec(),
        r#type: 0,
        timestamp: now as u64,
    };
    let mut chat_buffer = Vec::new();
    chat_message.encode(&mut chat_buffer).unwrap();


    let mut hasher2 = Blake2s256::new();
    hasher2.update(&chat_buffer);
    let hash_id = hasher2.finalize().to_vec();

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
    
    let message_created = protos::ZMessage{
        id: hash_id,
        version: 0,
        r#type: 0,
        action: i32::from(protos::ZAction::ZTypeWrite),
        identity: i32::from(protos::ZIdentity::UTypeCli),
        public_key: from.as_bytes().to_vec(),
        data: chat_buffer,
        signature: Vec::from(signature),
        from: from.as_bytes().to_vec(),
        to: to.as_bytes().to_vec(),
    };




    let mut buffer = Vec::new();
    message_created.encode(&mut buffer).unwrap();
    let conn = get_ws_conn().await;
    let mut locked_conn = conn.lock().await;
    locked_conn.send(protocol::Message::Binary(buffer)).await.expect("send fail");
}


pub async fn account_read_message() {
    let conn = get_ws_conn().await;
    let mut locked_conn = conn.lock().await; // Asynchronously lock the mutex
    let (mut write, mut read) = (&mut *locked_conn).split();
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