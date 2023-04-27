## koops
An eBPF kprobe that attaches to oops_exit function and counts the number of kernel oopses happend after the attachment.

To simulate an oops, build and install the kernel module located in `oops_mod` directory.
### Run
1. Download and extract the binary from releases
1. Run it on the prefered port: `sudo ./koops -p 3031`

### Build and run from source 
1. Install rust stable toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
1. Install rust nightly toolchain with the rust-src component: `rustup toolchain install nightly --component rust-src`
1. Add x86_64-unknown-linux-musl target: `rustup target add x86_64-unknown-linux-musl`
1. Install bpf-linker: `cargo install bpf-linker`
1. Build eBPF: `cargo xtask build-ebpf --release`
1. Build binary: `cargo build --release --target=x86_64-unknown-linux-musl`
1. Run with info log-level: `sudo ./target/x86_64-unknown-linux-musl/release/koops`

### Simulate an oops
```bash
cd oops_mod
make
sudo insmod oops_mod.ko
```
