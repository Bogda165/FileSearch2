use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;
use db::database::{Database, Save};
use tokio::sync::Mutex;
use db::image::Image;
use db::semantic_vector::SemanticVec;
use TcpSockets::*;
use Commands::*;

#[tokio::main]
async fn main() {
    let mut db = Database::new().unwrap();

    let task_list: VecDeque<Command> = VecDeque::new();
    let task_list = Arc::new(Mutex::new(task_list));

    let mut semantic_vector_array: Vec<(String, u32, f32)> = vec![];
    // start server
    let address = Address::new(7879, "127.0.0.1".to_string());

    tokio::spawn({
        let mut task_list = task_list.clone();
        async move{
            loop {
                let mut task_list_clone = task_list.lock().await;
                if !task_list_clone.is_empty() {
                    let command = task_list_clone.pop_front().unwrap();
                    std::mem::drop(task_list_clone);
                    match command {
                        Command::Add(path, semantic_vectors) => {
                            //TODO: DO I NEED TITLE?????.
                            println!("Adding to database");

                            let mut image = Image::new(path, "hello".to_string());
                            let semantic_vector = SemanticVec::from_vec(semantic_vectors);

                            image.set_semantic_vector(semantic_vector);

                            if let Some(connection) = &mut db.connection {
                                match image.save(connection) {
                                    Ok(_) => { println!("Image saved successfully") },
                                    Err(_) => { println!("Error while adding an image!!!") }
                                }
                            } else {
                                println!("Error while opening database");
                                continue;
                            }
                        },
                        Command::FromSentence(sentence) => {

                        },
                        Command::IsExist(path ) => {
                            match db.exists_image_by_path(path.as_str()) {
                                Ok(exist) => {
                                    if !exist {
                                        let command = Command::AddToDb(path);
                                        //TODO if error is returned write that serivce is not active
                                        send_command(&command, 7878).await.unwrap();
                                    }else {
                                        println!("Image already exist in db");
                                    }
                                }
                                Err(_) => {
                                    //TODO error
                                    println!("Error while looking for image!!!");
                                }
                            }
                        }
                        Command::CreateVec(semantic_vector) => {
                            semantic_vector_array = db.create_vector(semantic_vector).await;
                        },
                        Command::Drop() => {
                            semantic_vector_array.clear();
                        },
                        Command::Get(index) => {
                            if(index < semantic_vector_array.len() as i32) {
                                //TODO! send to gui(main app still)
                                let command = Command::WriteImage(semantic_vector_array[index as usize].0.clone());
                                send_command(&command, 7876).await.unwrap();
                                println!("{:?}", semantic_vector_array[index as usize]);
                            }else {
                                println!("Out of range of semantic_vector_array");
                                //TODO create panic
                            }
                        },
                        _ => {
                            println!("Not the right service");
                        },
                    }
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
