pub mod account;
pub mod podcast;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ApiData<T: serde::Serialize> {
    pub code: u16,
    pub data: T,
    pub message: String,
}

impl<T: serde::Serialize + Default> Default for ApiData<T> {
    fn default() -> Self {
        Self {
            code: 400,
            data: Default::default(),
            message: Default::default(),
        }
    }
}
