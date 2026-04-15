use serde_derive::Deserialize;

#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Dynamo {
    pub routes_table: String,
}
