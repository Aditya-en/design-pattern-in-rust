use std::rc::Rc;
use std::cell::RefCell;

trait Observer {
    fn update(&mut self, msg: String);
}

struct DisplayDevice {
    id: String,
    msgs: Vec<String>,
}
impl DisplayDevice {
    fn new(id: String) -> Self {
        let v: Vec<String> = vec![];

        Self{ id: id, msgs: v}
    }
}

impl Observer for DisplayDevice {
    fn update(&mut self, msg: String) {
        println!("Weather update for device {} is {}", self.id, msg);
        self.msgs.push(msg);
    }
}

struct WeatherApp {
    id: String,
    history: Vec<String>,
}
impl WeatherApp {
    fn new(id: String) -> Self {
        let v: Vec<String> = vec![];
        Self {id: id, history: v}
    }
}
impl Observer for WeatherApp {
    fn update(&mut self, msg: String) {
        println!("WeatherApp with id {} got update {}", self.id, msg);
        self.history.push(msg);
    }
}

trait Subject {
    fn subscribe(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn unsubscribe(&mut self, index: usize);
    fn notify(&self, msg: String);
}

struct WeatherStation {
    observers : Vec<Rc<RefCell<dyn Observer>>>,
}
impl WeatherStation {
    fn new() -> Self {
        Self {observers: Vec::new()}
    }
}

impl Subject for WeatherStation {
    fn subscribe(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }
    fn unsubscribe(&mut self, index: usize) {
        if index < self.observers.len() {
            self.observers.remove(index);
        }
    }
    fn notify(&self, msg: String) {
        for observer in self.observers.iter() {
            observer.borrow_mut().update(msg.clone());
        }
    }
}
fn main() {
    let observer1 = Rc::new(RefCell::new(WeatherApp::new("1234".to_string())));
    let observer2 = Rc::new(RefCell::new(WeatherApp::new("5678".to_string())));
    let observer3 = Rc::new(RefCell::new(DisplayDevice::new("9101".to_string())));
    let observer4 = Rc::new(RefCell::new(DisplayDevice::new("2131".to_string())));

    let mut weather_station = WeatherStation::new();
    weather_station.subscribe(observer1);
    weather_station.subscribe(observer3);

    weather_station.notify("storm incoming, take shelter".to_string());
    
    weather_station.subscribe(observer2);
    weather_station.subscribe(observer4);

    weather_station.notify("massive tsunami about to strike, Brace for impact".to_string());
    weather_station.unsubscribe(0);
    weather_station.unsubscribe(3);
    
    weather_station.notify("nuclear winter on the way, be prepared".to_string());

}
