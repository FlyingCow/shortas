#[derive(Debug, Clone)]
pub struct Route {
    pub switch: String,
    pub link: String,
    pub dest: Option<String>,
}

impl Route {
    pub fn new(switch: String, link: String, dest: Option<String>) -> Self {
        Route { switch, link, dest }
    }
}