use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Command {
    TryAdd(String),
    Add(String, Vec<f32>),
    FromSentence(String),
    FromSentenceRes(Vec<f32>),
    IsExist(String),
    AddToDb(String),
    GetTopImages(String, i32),
    CreateVec(Vec<f32>),
    Drop(),
    Get(i32),
    StartIndexingFile(String),
}

impl Command {
    pub fn as_bytes(&self) -> Vec<u8> {
        let command = to_string(self).unwrap();
        command.as_bytes().to_vec()
    }

    pub fn from_utf8(buffer: &[u8], buffer_size: usize) -> Self {
        println!("{}", String::from_utf8(buffer[..buffer_size].to_vec()).unwrap());
        let command: Command = serde_json::from_slice(&buffer[..buffer_size].to_vec()).unwrap();
        command
    }
}