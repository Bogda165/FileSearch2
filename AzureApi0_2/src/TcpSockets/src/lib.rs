use std::collections::VecDeque;
use std::error::Error;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
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

pub async fn start_listening(listener: TcpListener, task_list: Arc<Mutex<VecDeque<Command>>>, buf_len: i32) {
    //TODO send a const varialbe in the function
    let mut buffer = [0; 2048];

    loop {
        println!("wating for data");
        let (stream, _) =  listener.accept().await.unwrap();
        stream.readable().await.unwrap();
        println!("Connection started");
        match stream.try_read(&mut buffer) {
            Ok(0) => {
                println!("There is no data to read!!!");
                break;
            },
            Ok(buffer_size) => {

                let command: Command = Command::from_utf8(&buffer, buffer_size);

                let mut task_list_clone = task_list.clone();
                let mut task_list_clone = task_list_clone.lock().await;
                task_list_clone.push_back(command);
                println!("Added");
            },
            Err(_) => {
                println!("Error while reading!!!!");
                break;
            }
        }
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
