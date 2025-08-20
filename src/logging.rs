use tracing::{error, info, warn};

pub fn main() {
    // this tells the compiler to output the logs
    tracing_subscriber::fmt::init();

    info!("An info log");
    error!("An error log");
    warn!("A warn log");
}
