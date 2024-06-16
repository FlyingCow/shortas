#[derive(Default, Clone, Debug)]
pub enum ActiveStatus {
    #[default]
    Active,
    Blocked,
}

pub const SKIP_TRACKING: &'static str = "tracking"; 

#[derive(Default, Clone, Debug)]
pub struct UserSettings {
    pub user_id: String,
    pub user_email: String,
    pub api_key: Option<String>,
    pub active_status: ActiveStatus,
    pub debug: bool,
    pub overflow: bool,
    pub skip: Vec<String>,
    pub allowed_request_params: Vec<String>,
    pub allowed_destination_params: Vec<String>,
}

impl UserSettings {
    pub fn new(
        user_id: String,
        user_email: String,
        api_key: Option<String>,
        active_status: ActiveStatus,
        debug: bool,
        overflow: bool,
        skip: Vec<String>,
        allowed_request_params: Vec<String>,
        allowed_destination_params: Vec<String>,
    ) -> Self {
        Self {
            user_id,
            user_email,
            api_key,
            active_status,
            debug,
            overflow,
            skip,
            allowed_request_params,
            allowed_destination_params
        }
    }
}
