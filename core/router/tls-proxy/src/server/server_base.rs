use crate::log::*;

use crate::Server;
impl<'a> Server<'a> {
    pub async fn run(&self) {
        info!("Starting...");
        println!("Running...");
    }
}
