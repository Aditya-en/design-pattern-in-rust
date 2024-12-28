#[derive(Debug, Clone)]
struct TLSCert {
    key: String,
    cert: String,
}
type ms = u32;

// #[derive(Debug, Clone)]
struct Server {
    host: String,
    port: u16,
    hot_reload: bool,
    tls: Option<TLSCert>,
    timeout: ms,
}

impl Server {
    fn new(host: String, port: u16, hot_reload: bool, tls:Option<TLSCert>, timeout:ms) -> Self {
        Self { host: host, port: port, hot_reload: hot_reload, tls: tls, timeout: timeout}
    }
}

struct ServerBuilder {
    host: String,
    port: u16,
    timeout: Option<ms>,
    hot_reload: Option<bool>,
    tls: Option<TLSCert>,
}

impl ServerBuilder {
    fn new(host: String, port: u16) -> Self {
        Self {host: host, port: port, hot_reload: None, tls: None, timeout: None} 
    }
    fn tls(&mut self, cert:TLSCert) -> &mut Self {
        self.tls = Some(cert);
        self
    }
    fn hot_reload(&mut self, hot_reload: bool) -> &mut Self{
        self.hot_reload = Some(hot_reload);
        self
    }
    fn timeout(&mut self, timeout: ms) -> &mut Self{
        self.timeout = Some(timeout);
        self
    }
    fn build(&self) -> Server {
        Server {
            host: self.host.clone(), 
            port: self.port, 
            timeout: self.timeout.unwrap_or(1000), 
            hot_reload: self.hot_reload.unwrap_or(false), 
            tls: self.tls.clone(),
        }
    }
}

fn main() {
    let host = String::from("localhost");
    let port = 3000;
    let timeout = 5000;
    let hot_reload = true;
    let cert = TLSCert{ key: "Some key".to_string(), cert: "Some cert".to_string()};

    let _server = ServerBuilder::new(host.clone(), port.clone()).build();

    let _server_tls = ServerBuilder::new(host.clone(), port.clone())
        .tls(cert.clone())
        .build();

    let _server_full = ServerBuilder::new(host, port)
        .tls(cert)
        .timeout(timeout)
        .hot_reload(hot_reload)
        .build();
}
