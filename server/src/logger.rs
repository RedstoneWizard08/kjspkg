use tracing_subscriber::fmt;

pub fn init_logger() {
    fmt()
        .pretty()
        .compact()
        .with_ansi(true)
        .with_level(true)
        .with_target(true)
        .with_file(false)
        .with_line_number(false)
        .without_time()
        .init();
}
