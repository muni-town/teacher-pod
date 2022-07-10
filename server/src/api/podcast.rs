use crate::{
    error::{ApiResult, Error},
    models::podcast::{BestPodcastsQuery, PodcastQuery},
    Routers,
};
use salvo::prelude::*;
use tp_models::podcast::{BestPodcasts, Podcast};

use super::JsonApi;

#[fn_handler]
async fn get_podcast(req: &mut Request, res: &mut Response) -> ApiResult {
    let pid = req.param::<String>("id").unwrap_or_default();
    let info = Podcast::fetch_by_id(&pid).await;
    if info.is_none() {
        return Err(Error::DataNotFound);
    }
    let info = info.unwrap();
    res.success(info);
    Ok(())
}

#[fn_handler]
async fn search_podcasts(req: &mut Request, res: &mut Response) -> ApiResult {
    todo!()
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
