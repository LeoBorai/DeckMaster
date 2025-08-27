mod modules;
mod router;
mod services;
mod utils;

use std::sync::Arc;

use tower_service::Service;
use worker::{Context, Env, Request, Response, Result, console_error, event};

use crate::router::make_router;
use crate::services::Services;
use crate::utils::worker::{to_axum_request, to_worker_response};

const KV_STORAGE_NAMESPACE: &str = "__deckmaster_mtg_datasets";

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let Ok(kv) = env.kv(KV_STORAGE_NAMESPACE) else {
        console_error!("Failed to retrieve KV Store");
        return Response::error("Internal Server Error", 500);
    };

    let services = Services::new(&kv).await.map_err(|err| {
        console_error!("Failed to instantiate Services. {err}");
        worker::Error::from(err.to_string())
    })?;

    let services = Arc::new(services);

    let Ok(mut router) = make_router(services).await else {
        eprintln!("Failed to create router");
        return Response::error("Internal Server Error", 500);
    };

    let req = to_axum_request(req).await.unwrap();
    let res = router.call(req).await.unwrap();
    Ok(to_worker_response(res).await.unwrap())
}
