## koops
An eBPF kprobe that attaches to oops_exit function and counts the number of kernel oopses happend after the attachment.

To simulate an oops, build and install the kernel module located in `oops_mod` directory.
### Build and run locally
1. Install rust stable toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
1. Install rust nightly toolchain with the rust-src component: `rustup toolchain install nightly --component rust-src`
1. Install bpf-linker: `cargo install bpf-linker`
1. Build eBPF: `cargo xtask build-ebpf`
1. Run: `RUST_LOG=info cargo xtask run`

### Simulate an oops
```bash
cd oops_mod
make
sudo insmod oops_mod.ko
```