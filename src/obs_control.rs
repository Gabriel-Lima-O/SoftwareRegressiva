use serde::{Deserialize, Serialize};
use serde_json::json;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    op: u16,
    d: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    op: u16,
    d: Option<serde_json::Value>,
}

pub struct OBSControl {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl OBSControl {
    pub fn new(obs_address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (mut socket, _) = connect(obs_address)?;

        // Authenticate if necessary (assuming no authentication for simplicity)
        let auth_message = json!({
            "op": 0, // Authentication
            "d": {
                "password": "" // Enter your OBS WebSocket password here if needed
            }
        });
        socket.write_message(Message::Text(auth_message.to_string()))?;

        Ok(Self { socket })
    }

    pub fn switch_scene(&mut self, scene_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let scene_message = json!({
            "op": 6, // Scene Switch
            "d": {
                "sceneName": scene_name
            }
        });
        self.socket.write_message(Message::Text(scene_message.to_string()))?;
        Ok(())
    }

    pub fn close(mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.socket.close(None)?;
        Ok(())
    }
}
