// colorise log output
fn log_info(message: &str) {
    println!("\x1b[1;32m{}\x1b[0m", message);
}

fn log_error(message: &str) {
    println!("\x1b[1;31m{}\x1b[0m", message)
}

fn log_warning(message: &str) {
    println!("\x1b[1;33m{}\x1b[0m", message)
}

fn log_debug(message: &str) {
    println!("\x1b[1;34m{}\x1b[0m", message)
}

fn log_trace(message: &str) {
    println!("\x1b[1;35m{}\x1b[0m", message)
}

pub fn logger(level: &str, message: &str) {
    match level {
        "info" => log_info(message),
        "error" => log_error(message),
        "warning" => log_warning(message),
        "debug" => log_debug(message),
        "trace" => log_trace(message),
        _ => log_info(message),
    }
}
