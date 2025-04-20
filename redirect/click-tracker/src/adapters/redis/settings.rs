use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Redis {
    pub initial_nodes: Vec<String>,
    pub password: String, 
}