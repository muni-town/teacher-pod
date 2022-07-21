use salvo::prelude::*;
use tp_models::data::SearchInfo;

use crate::error::{ApiResult, Error};
use crate::models::data::SearchInfoQuery;
use crate::Routers;

use super::JsonApi;

#[fn_handler]
async fn search_episode(req: &mut Request, res: &mut Response) -> ApiResult {
    let query = req.param::<String>("query").unwrap_or_default();
    let info = SearchInfo::search_episode(&query).await;
    if info.count == 0 {
        return Err(Error::DataNotFound);
    }
    res.success(info);
    Ok(())
}

pub struct SearchApi;
impl Routers for SearchApi {
    fn build() -> Vec<salvo::Router> {
        vec![Router::with_path("search/<query>").get(search_episode)]
    }
}
