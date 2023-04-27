use std::sync::{Arc, Mutex};

use aya::{
    maps::{self, MapRefMut},
    Bpf,
};

pub struct SharedMap {
    pub oops_count: maps::Array<MapRefMut, u64>,
}
impl SharedMap {
    pub fn new(ebpf: &Bpf) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            oops_count: maps::Array::try_from(
                ebpf.map_mut("OOPS_COUNT")
                    .expect("unable to borrow OOPS_COUNT mutably"),
            )
            .expect("failed to create a map from OOPS_COUNT"),
        }))
    }

    pub fn get_oops_count(&self) -> Option<u64> {
        match self.oops_count.get(&0, 0) {
            Ok(count) => Some(count),
            Err(_) => None,
        }
    }
}
