#[derive(Debug, Clone)]
pub struct RouteEntity {
    pub switch: String,
    pub link: String,
    pub dest: Option<String>,
}

impl RouteEntity {
    pub fn new(switch: String, link: String, dest: Option<String>) -> RouteEntity {
        RouteEntity { switch, link, dest }
    }
}
