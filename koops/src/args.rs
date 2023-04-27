use clap::Parser;

/// An eBPF kprobe that attaches to oops_exit function and counts the number of kernel oopses happend after the attachment.
/// Metrics are served in prometheus format on :[server_port]/metrics.
#[derive(Debug, Parser)]
pub struct Args {
    #[clap(short, long, default_value = "3031")]
    /// Port to serve prometheus metrics on (i.e. HTTP Server Port)
    port: String,
}

impl Args {
    pub fn parse_server_port(&self) -> u16 {
        let port = self
            .port
            .trim()
            .parse::<u16>()
            .expect("port must be a positive integer");

        port
    }
}
