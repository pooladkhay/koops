use log::debug;
use std::sync::{Arc, Mutex};
use warp::{http, Filter};

use crate::{api::prometheus, shared_map::SharedMap};

pub async fn serve(map: Arc<Mutex<SharedMap>>, server_port: u16) {
    let metrics_route = warp::get()
        .and(warp::path("metrics"))
        .and(warp::any().map(move || map.clone()))
        .and_then(prometheus_metrics);

    warp::serve(metrics_route)
        .run(([0, 0, 0, 0], server_port))
        .await;
}

async fn prometheus_metrics(
    map: Arc<Mutex<SharedMap>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match prometheus::generate_mertics(map) {
        Ok(metrics_buffer) => Ok(warp::reply::with_status(
            metrics_buffer,
            http::StatusCode::OK,
        )),
        Err(err) => {
            debug!("error while generating prometheus metrics: {}", err);
            Ok(warp::reply::with_status(
                "Internal Server Error".to_owned(),
                http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
