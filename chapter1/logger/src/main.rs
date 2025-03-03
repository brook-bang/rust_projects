use log::{debug, error, info, warn};

fn main() {
    // unsafe {
    //     std::env::set_var("RUST_LOG", "debug");
    // }
    env_logger::init();
    error!("Error message");
    warn!("Warning message");
    info!("Information message");
    debug!("Debugging message");
}
// 在终端中执行   $env:RUST_LOG="debug"; cargo run
