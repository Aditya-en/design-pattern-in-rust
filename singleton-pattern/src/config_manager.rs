use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::Once;
#[derive(Clone)]
struct ConfigManager {
    configs: HashMap<String, String>,
}
impl ConfigManager {
    fn new() -> Self{
        let config: HashMap<String, String> = HashMap::new();
        Self {configs: config}
    }
    fn add(&mut self, key:String, value:String) {
        self.configs.insert(key, value);
    }
    fn get(&self, key:&str) -> Option<&String> {
        self.configs.get(key)
    }
    fn get_all(&self)-> HashMap<String, String>{
        self.configs.clone()
    }
    fn remove(&mut self, key: &str) {
        self.configs.remove(key);
    }
    fn update(&mut self, key: String, value: String) {
        self.configs.insert(key, value);
    }
}

struct ConfigManagerSingleton;
impl ConfigManagerSingleton {
    fn get_instance() -> Arc<Mutex<ConfigManager>> {
        static mut SINGLETON: Option<Arc<Mutex<ConfigManager>>> = None;
        static init:Once = Once::new();
        unsafe { 
                init.call_once(|| {
                let config_manager = ConfigManager::new();
                SINGLETON = Some(Arc::new(Mutex::new(config_manager)));
            });
        
            SINGLETON.clone().unwrap()
        }
    }
}

fn main() {
    let config_manager1 = ConfigManagerSingleton::get_instance();
    let config_manager2 = ConfigManagerSingleton::get_instance();
    
    config_manager1.lock().unwrap().add("username".to_string(), "Aditya".to_string());
    config_manager2.lock().unwrap().add("email".to_string(), "example@email.com".to_string());
    
    println!("HashMap {:?}", config_manager1.lock().unwrap().get_all());

    // verify both managers are the same
    println!("instances are the same {:?}", Arc::ptr_eq(&config_manager1, &config_manager2));

}
