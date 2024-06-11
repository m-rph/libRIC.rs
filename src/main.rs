use libric::logging::LambdaLogger;

pub fn main() {
    log::set_logger(&LambdaLogger).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    // Your application code here
    log::info!("This is an info message");
    log::warn!("This is a warning message {}", 1);
}