#![no_std]
#![no_main]

use aya_bpf::{
    macros::{kprobe, map},
    maps::Array,
    programs::ProbeContext,
};
use aya_log_ebpf::info;

#[map(name = "OOPS_COUNT")]
pub static mut OOPS_COUNT: Array<u64> = Array::with_max_entries(1, 0);

#[kprobe(name = "koops")]
pub fn koops(ctx: ProbeContext) -> u32 {
    match try_koops(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_koops(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function oops_exit called");

    match unsafe { OOPS_COUNT.get_ptr_mut(0) } {
        Some(oops_count) => unsafe { *oops_count += 1 },
        None => {}
    }

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
