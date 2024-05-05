use std::collections::VecDeque;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;
use Commands::Command;
use TcpSockets::*;

#[tokio::main]
async fn main() {
    let _task_list: VecDeque<Command> = VecDeque::new();
    let task_list = Arc::new(Mutex::new(_task_list));

    let address = Address::new(7877, "127.0.0.1".to_string());

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
                        Command::GetTopImages(sentence, top) => {
                            let command: Command = Command::FromSentence(sentence);
                            send_command(&command, 7878).await.unwrap();
                        },
                        //TODO add top x number
                        Command::FromSentenceRes(semantic_vector) => {
                            //send request to db
                            let mut command = Command::CreateVec(semantic_vector);
                            send_command(&command, 7879).await.unwrap();

                            //get top x requests
                            for i in 0..10 {
                                command = Command::Get(i);
                                send_command(&command, 7879).await.unwrap();
                            }
                        },
                        // may not work (Anton)
                        _command @ Command::StartIndexingFile(_) => {
                            //let mut command = Command::StartIndexingFile(path);
                            send_command(&_command, 7880).await.unwrap();
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

    start_listening(listener, task_list, 512).await;
    println!("Crash");

}
