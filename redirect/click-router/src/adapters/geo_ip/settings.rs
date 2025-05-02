use serde_derive::Deserialize;
#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct GeoIP {
    pub mmdb: String,
}
