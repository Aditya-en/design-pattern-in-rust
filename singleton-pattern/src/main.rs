use std::sync::{Arc, Mutex};
use std::sync::Once;

struct Logger {
    messages: Vec<String>,
}

impl Logger {
    fn log(&mut self, message: String) {
        self.messages.push(message);
    }

    fn show_logs(&self) {
        for (i, msg) in self.messages.iter().enumerate() {
            println!("{}: {}", i + 1, msg);
        }
    }
}

struct LoggerSingleton;

impl LoggerSingleton {
    fn get_instance() -> Arc<Mutex<Logger>> {
        static mut SINGLETON: Option<Arc<Mutex<Logger>>> = None;
        static INIT: Once = Once::new();

        unsafe {
            INIT.call_once(|| {
                let logger = Logger {
                    messages: Vec::new(),
                };
                SINGLETON = Some(Arc::new(Mutex::new(logger)));
            });

            SINGLETON.clone().unwrap()
        }
    }
}

fn main() {
    let logger1 = LoggerSingleton::get_instance();
    let logger2 = LoggerSingleton::get_instance();

    logger1.lock().unwrap().log("First message".to_string());
    logger1.lock().unwrap().log("Second message".to_string());

    logger2.lock().unwrap().show_logs();

    println!(
        "Logger instances are the same: {}",
        Arc::ptr_eq(&logger1, &logger2)
    );
}

