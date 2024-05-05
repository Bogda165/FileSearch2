use std::sync::Arc;
use Embeddings::*;
use serde_json::Value;
use std::collections::VecDeque;
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::Mutex;
use AzureApi::{MyRequest, MyResponse};
use TcpSockets::*;
use Commands::*;

#[tokio::main]
async fn main() {
    // load embeddings

    let mut _embeddings = Embedding::new();
    _embeddings.get_embeddings("/Users/bogdankoval/Downloads/glove.6B/glove.6B.50d.txt");

    let _task_list: VecDeque<Command> = VecDeque::new();
    let task_list = Arc::new(Mutex::new(_task_list));

    // start server
    let address = Address::new(7878, "127.0.0.1".to_string());

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
                        Command::TryAdd(path) => {
                            let command = Command::IsExist(path);
                            send_command(&command, 7879).await.unwrap();
                        },
                        Command::FromSentence(sentence) => {
                            //TODO modify, correctly transform sentence
                            let request_em = RequestType::Caption(sentence);
                            let semantic_vector = _embeddings.main_vector(request_em);
                            println!("Semantic vector: {:?}", semantic_vector);
                            println!("Task: send to main service");
                            //send to main
                            let command: Command = Command::FromSentenceRes(semantic_vector);
                            send_command(&command, 7877).await.unwrap();
                        },
                        Command::AddToDb(path) => {
                            //TODO: modify this. Do not fully work.
                            println!("Adding, then sending");
                            // create a request to azure
                            let mut request = MyRequest::new("4d7bd39a70c249eebd19f5b8d62f5d7b", vec!["tags", "caption"]);
                            request.set_img(&*path).unwrap();
                            let response = request.send_request().await.unwrap();
                            let response_copy = response.json::<Value>().await.unwrap();
                            let mut response_struct: Result<MyResponse, Infallible> = MyResponse::try_from(response_copy.clone());
                            let request_em = RequestType::Caption(response_struct.unwrap().caption);

                            //create  semantic vector
                            let semantic_vector = _embeddings.main_vector(request_em);
                            println!("Semantic vector: {:?}", semantic_vector);

                            //sending to db
                            println!("Sending request to db");
                            let command = Command::Add(path, semantic_vector);

                            send_command(&command, 7879).await.unwrap();
                            println!("command has been send");
                            println!("Task: send to db service");
                        }
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
