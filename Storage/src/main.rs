use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
//use tokio::sync::Mutex;
use arc_str::arc_str::ArcStr;
use Commands::Command;
use TcpSockets::{Address, start_listening};

#[tokio::main]
async fn main() {
    let storage: Arc<std::sync::Mutex<HashMap<ArcStr, HashSet<ArcStr>>>> = Arc::new(std::sync::Mutex::new(HashMap::new()));

    let task_list: VecDeque<Command> = VecDeque::new();
    let task_list = Arc::new(tokio::sync::Mutex::new(task_list));

    let address = Address::new(60000, "127.0.0.1".to_string());

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
                        Command::AddToStorage(path) => {
                            let path = path.to_owned().replace("\\", "/");
                            let map_for_closure = storage.clone();
                            async move {
                                let path_string = Path::new(&path);
                                let filename = path_string.file_name().unwrap().to_str().unwrap().to_string().replace("\\", "/");
                                let filename_arc: Arc<str> = Arc::from(filename.clone());
                                let path_arc: Arc<str> = Arc::from(path);

                                let mut map = map_for_closure.lock().unwrap();
                                if !map.contains_key(&ArcStr(filename_arc.clone())) {
                                    let mut v = HashSet::new();
                                    v.insert(ArcStr(path_arc.clone()));
                                    map.insert(ArcStr(filename_arc.clone()), v);
                                } else {
                                    map.get_mut(&ArcStr(filename_arc.clone())).unwrap().insert(ArcStr(path_arc.clone()));
                                }
                            }.await;
                            //println!("Length {}", storage.keys().len());

                        },
                        Command::Save() => {

                        },
                        Command::FACK() => {
                            println!("Storage size: {}", "pisun");
                        }
                        _ => {
                            println!("Not the right service");
                        }
                    }
                    //println!("Doing command {:?}", command);
                } else {
                    //println!("There are no tasks aviable at the moment");
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }
        }
    });


    let listener = address.listener().await.unwrap();

    start_listening(listener, task_list, 2048).await;
    println!("Crash");

}
