use std::fmt;

struct Connection {
    ip_addr: IpAddr,
}

struct IpAddr(u8, u8, u8, u8);

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.0, self.1, self.2, self.3)
    }
}

impl Connection {
    fn new(ip: IpAddr) -> Connection {
        println!("Connected to {}", ip);
        Connection { ip_addr: ip }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("Disconnected from {}", self.ip_addr);
    }
}

fn main() {
    let _ = Connection::new(IpAddr(127,0,0,1));
}

