use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tokio::io::AsyncWriteExt;
use Commands::*;
pub struct Address {
    ip: String,
    port: u16
}

impl Address {
    pub fn new(port: u16, ip: String) -> Self {
        Address {
            ip,
            port
        }
    }

    pub async fn listener(&self) -> Result<TcpListener, Box<dyn Error>> {
        match TcpListener::bind(format!("{}:{}", self.ip, self.port)).await {
            Ok(listener) => Ok(listener),
            Err(e) => {
                Err(Box::new(e))
            }
        }
    }

    pub async fn stream(&self) -> TcpStream {
        TcpStream::connect(format!("{}:{}", self.ip, self.port)).await.unwrap()
    }
}

pub async fn send_command(command: &Command, port: u16) -> Result<(), String> {
    let mut stream = match TcpStream::connect(format!("127.0.0.1:{}", port)).await {
        Ok(_stream) => {_stream},
        Err(_) => { return Err(format!("{} is not active", port)) },
    };

    let command = command.as_bytes();

    stream.writable().await.unwrap();
    println!("Writable");
    stream.write_all(&command).await.unwrap();
    println!("Command has been send");
    Ok(())
}
