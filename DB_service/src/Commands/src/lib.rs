use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Command {
    TryAdd(String),
    Add (String, Vec<f32>),
    FromSentence (String),
    IsExist (String),
    AddToDb (String),
    CreateVec(Vec<f32>),
    Drop(),
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