use std::collections::VecDeque;
use std::convert::Infallible;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use Commands::Command;
use TcpSockets::{Address, send_command, start_listening};


#[tokio::main]
async fn main() {
    let task_list: VecDeque<Command> = VecDeque::new();
    let task_list = Arc::new(Mutex::new(task_list));

    let address = Address::new(6665, "127.0.0.1".to_string());

    tokio::spawn( {
        async move {
            //work
            sleep(Duration::new(5, 0)).await;
/*
            let command = Command::GetTopImages("women".to_string(), 10);
            send_command(&command, 7877).await.expect("TODO: panic message");
            */

            // do not still work
            let command = Command::StartIndexingFile("/Users/bogdankoval/Downloads/".to_string());
            send_command(&command, 7877).await.unwrap();
        }
    });

    tokio::spawn({
        //TODO catch panics
        let mut task_list = task_list.clone();
        async move{
            loop {
                let mut task_list_clone = task_list.lock().await;
                if !task_list_clone.is_empty() {
                    let command = task_list_clone.pop_front().unwrap();
                    drop(task_list_clone);
                    match command {
                        Command::WriteImage(path) => {
                            println!("Path: {}", path);
                        }
                        _ => {
                            println!("Not the right service");
                        }
                    }
                    //println!("Doing command {:?}", command);
                } else {
                    //println!("There are no tasks aviable at the moment");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    });

    let listener = address.listener().await.unwrap();

    start_listening(listener, task_list, 512).await;
    println!("Crash");
}
