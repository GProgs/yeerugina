use std::net::TcpStream;

pub struct Lamp {
    name: String,
    ip: String, 
    stream: Option<TcpStream>,
}

impl Lamp {
    
    pub fn new(name: String, ip: String) -> Self {
        Self { name, ip, stream: None }
    }

    fn connect(&mut self) -> std::io::Result<()> {
        self.stream = Some(TcpStream::connect(&self.ip)?);
        Ok(())
    }
}
