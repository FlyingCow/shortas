use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

use crate::adapters::{aws::settings::AWS, kafka::settings::Kafka, geo_ip::settings::GeoIP, moka::settings::Moka, uaparser::settings::UAParser};
// use crate::adapters::geo_ip::settings::GeoIP;
// use crate::adapters::moka::settings::Moka;
// use crate::adapters::uaparser::settings::UAParser;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Server {
    pub threads: usize,
    pub listen_os_signals: bool,
    pub exit: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub aws: AWS,
    pub kafka: Kafka,
    pub moka: Moka,
    pub uaparser: UAParser,
    pub geo_ip: GeoIP,
}
const DEV_RUN_MODE: &'static str = "development";

impl Settings {
    pub fn new(run_mode: Option<&str>, path: Option<&str>) -> Result<Self, ConfigError> {

        let run_mode = run_mode.unwrap_or(DEV_RUN_MODE);

        let path = path.expect("No configuration folder specified.");

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(&format!("{}/default", path)))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("{}/{}", path, run_mode))
                    .required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name(&format!("{}/local", path)).required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            //.set_override("database.url", "postgres://")?
            .build()?;

            
            // // Now that we're done, let's access our configuration
            // println!("debug: {:?}", s.get_bool("debug"));
            // println!("database: {:?}", s.get::<String>("database.url"));

            // // You can deserialize (and thus freeze) the entire configuration as

            s.try_deserialize()
    }
}