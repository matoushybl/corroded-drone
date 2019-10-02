mod tello;

#[macro_use]
extern crate log;

fn main() {
    // enable logging
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "auto");

    env_logger::init_from_env(env);
    error!("Hello, world!");
}
