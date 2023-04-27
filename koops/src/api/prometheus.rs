use std::{
    fmt::Write,
    sync::{Arc, Mutex},
};

use anyhow::anyhow;

use crate::shared_map::SharedMap;

pub fn generate_mertics(shared_map: Arc<Mutex<SharedMap>>) -> Result<String, anyhow::Error> {
    match shared_map.try_lock() {
        Ok(prom_metrics) => match prom_metrics.get_oops_count() {
            Some(oops_count) => {
                let mut metrics_buffer = String::new();
                metrics_buffer.write_str(
                    "# HELP kernel_oops_total Total number of kernel oops occured since this program started.\n",
                )?;
                metrics_buffer.write_str("# TYPE kernel_oops_total counter\n")?;
                metrics_buffer.write_str(format!("kernel_oops_total {}\n", oops_count).as_str())?;
                metrics_buffer.write_str("# EOF\n")?;

                Ok(metrics_buffer)
            }
            None => Err(anyhow!("error trying to read from shared map")),
        },
        Err(_) => Err(anyhow!("error trying to acuire shared map's lock")),
    }
}
