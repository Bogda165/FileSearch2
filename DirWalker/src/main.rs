use std::collections::VecDeque;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use Commands::Command;
use file_system::dir_walker::DirWalker;
use TcpSockets::{Address, send_command, start_listening};

pub async fn add_to_map(path: &str) {
    //send to storage service
    let command = Command::AddToStorage(path.to_string());
    println!("Hello how are you?");
    let hui = send_command(&command, 60000).await;
    println!("Hui иннициализировіан {:?}", thread::current().id());
    match hui{
        Ok(_) => {println!("Я кста живой  pizda");}
        Err(_) => {println!("Если ві єто читатет мі сидими над єтой паршей 5 часов pisun: Anron");}
        _ => {
            println!("hello howare you boobs");
        }
    };

}

#[tokio::main]
async fn main() {
    let task_list: VecDeque<Command> = VecDeque::new();
    let task_list = Arc::new(Mutex::new(task_list));

    let address = Address::new(7880, "127.0.0.1".to_string());

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
                        //add argumetns to command image/files
                        Command::StartIndexingFile(path) => {
                            //start indexing directory
                            let dir_walker = DirWalker::new(path.as_str()).unwrap();
                            dir_walker.walk(|_path| {
                                let _path = _path.to_owned();
                                let res = async move {
                                    let _path = _path.clone();
                                    add_to_map(&*_path).await;
                                };
                                println!("Падаем zopa");
                                res
                            }).await;
                            println!("Hellow");
                        },
                        _ => {
                            println!("Not the right service");
                        }
                    }
                    //println!("Doing command {:?}", command);
                } else {
                    println!("There are no tasks aviable at the moment");
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
            }
        }
    });


    let listener = address.listener().await.unwrap();

    start_listening(listener, task_list, 2048).await;
    println!("Crash");

}
