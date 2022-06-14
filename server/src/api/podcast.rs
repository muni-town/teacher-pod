use crate::{
    error::{ApiResult, Error},
    models::podcast::{BestPodcasts, Podcast},
    Routers,
};
use salvo::prelude::*;

use super::JsonApi;

#[fn_handler]
async fn get_podcast(res: &mut Response) -> ApiResult {
    let info = Podcast::fetch_by_id("4d3fe717742d4963a85562e9f84d8c79").await;
    if info.is_none() {
        return Err(Error::DataNotFound);
    }
    let info = info.unwrap();
    res.success(info);
    Ok(())
}

#[fn_handler]
async fn recommend_podcasts(res: &mut Response) -> ApiResult {
    let info = BestPodcasts::get_recommend().await.unwrap_or_default();
    res.success(info);
    Ok(())
}

pub struct PodcstApi;
impl Routers for PodcstApi {
    fn build() -> Vec<salvo::Router> {
        vec![
            Router::with_path("podcasts").get(recommend_podcasts),
            Router::with_path("podcasts/<id>").get(get_podcast),
        ]
    }
}
