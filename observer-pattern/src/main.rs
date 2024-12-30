use std::rc::Rc;
use std::cell::RefCell;

trait Observer {
    fn update(&mut self, msg: String);
}

struct EmailSubscriber {
    email: String,
    msgs: Vec<String>,
}
impl EmailSubscriber {
    fn new(email:String) -> Self {
        Self { email: email, msgs: vec![]}
    }
}
impl Observer for EmailSubscriber {
    fn update(&mut self, msg: String) {
        self.msgs.push(msg.clone());
        println!("Email to {} with message {}",self.email, msg);
    }
}

struct SMSSubscriber {
    phone_number: String,
    msgs: Vec<String>,
}
impl SMSSubscriber {
    fn new(number: String) -> Self {
        let v:Vec<String> = Vec::new(); 
        Self {phone_number: number, msgs: v}
    }
}
impl Observer for SMSSubscriber {
    fn update(&mut self, msg:String) {
        self.msgs.push(msg.clone());
        println!("Sms to {} with message {}", self.phone_number, msg);
    }
}

trait Subject {
    fn subscribe(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn unsubscribe(&mut self, index: usize);
    fn notify(&self, msg:String);
}

struct NewsAgency {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}
impl NewsAgency {
    fn new() -> Self{
        Self{ observers: Vec::new()}
    }
}
impl Subject for NewsAgency {
    fn subscribe(&mut self, observer:Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }
    fn unsubscribe(&mut self, index: usize) {
        if index < self.observers.len() {
            self.observers.remove(index);
        }
    }
    fn notify(&self, msg: String) {
        for obsvr in &self.observers {
            obsvr.borrow_mut().update(msg.clone());
        }
    }
}

fn main() {
    let observer1 = Rc::new(RefCell::new(EmailSubscriber::new("example@example.com".to_string())));
    let observer2 = Rc::new(RefCell::new(SMSSubscriber::new("123123123".to_string())));
    let observer3 = Rc::new(RefCell::new(SMSSubscriber::new("234234234".to_string())));
    let observer4 = Rc::new(RefCell::new(SMSSubscriber::new("345345345".to_string())));

    let mut news = NewsAgency::new();

    news.subscribe(observer1);
    news.subscribe(observer2);

    news.notify("Earthquake nearby: stay safe".to_string());
    news.subscribe(observer3);
    news.subscribe(observer4);
    news.notify("world war 3 initiates as china becomes the global leader".to_string());

    news.unsubscribe(0);
    news.unsubscribe(2);

    news.notify("millions die due to global pendemic, stay safe".to_string());

}
