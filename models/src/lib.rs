pub mod account;
pub mod podcast;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ApiData<T: serde::Serialize> {
    pub code: u16,
    pub data: T,
    pub message: String,
}