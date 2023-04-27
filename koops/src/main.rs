use log::info;
use tokio::signal;

mod api;
mod ebpf;
mod shared_map;

use shared_map::SharedMap;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let ebpf = ebpf::init();
    let shared_map = SharedMap::new(&ebpf);

    tokio::spawn(async move { api::server::serve(shared_map.clone(), 3031).await });

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");
    Ok(())
}
