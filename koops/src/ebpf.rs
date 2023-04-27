use aya::{include_bytes_aligned, programs::KProbe, Bpf};
use aya_log::BpfLogger;
use log::warn;

pub fn init() -> Bpf {
    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/koops"
    ))
    .expect("error while loding ebpf bytecode");
    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/koops"
    ))
    .expect("error while loding ebpf bytecode");
    if let Err(e) = BpfLogger::init(&mut bpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", e);
    }
    let program: &mut KProbe = bpf
        .program_mut("koops")
        .unwrap()
        .try_into()
        .expect("error while getting the program");
    program
        .load()
        .expect("error while loading the ebpf program");
    program
        .attach("oops_exit", 0)
        .expect("error while attaching the ebpf program");

    bpf
}
