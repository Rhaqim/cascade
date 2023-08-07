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

pub struct Logger {
    pub scope: String,
}

pub trait CliLog {
    fn info(&self, message: &str);
    fn error(&self, message: &str);
    fn warning(&self, message: &str);
    fn debug(&self, message: &str);
    fn trace(&self, message: &str);
}

impl CliLog for Logger {
    fn info(&self, message: &str) {
        let msg = self.scope.clone() + ": " + message;
        logger("info", &msg);
    }

    fn error(&self, message: &str) {
        let msg = self.scope.clone() + ": " + message;
        logger("error", &msg);
    }

    fn warning(&self, message: &str) {
        let msg = self.scope.clone() + ": " + message;
        logger("warning", &msg);
    }

    fn debug(&self, message: &str) {
        let msg = self.scope.clone() + ": " + message;
        logger("debug", &msg);
    }

    fn trace(&self, message: &str) {
        let msg = self.scope.clone() + ": " + message;
        logger("trace", &msg);
    }
}
