use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RoutePresenter {
    pub switch: String,
    pub link: String,
    pub dest: String,
}

