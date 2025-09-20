extern crate derive_more;

use std::error::Error;
use std::fmt;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use derive_more::Display;
use serde::Deserialize;
use strum_macros;

pub struct Lamp {
    name: String,
    ip: String, 
    stream: Option<TcpStream>,
}

impl Lamp { 
    pub fn new(name: String, ip: String) -> Self {
        Self { name, ip, stream: None }
    }

    pub fn connect(&mut self) -> std::io::Result<()> {
        self.stream = Some(TcpStream::connect(&self.ip)?);
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub lamp_ip: String,
    pub mqtt_addr: String,
}

impl Config {
    pub fn read_file(path: String) -> Result<Self,Box<dyn Error>> {
        let cont = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&cont)?)
    }
}

#[derive(strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Effect {
    Sudden,
    Smooth,
}

/*
impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",match self {
            Effect::Sudden => "sudden",
            Effect::Smooth => "smooth",
        })
    }
}
*/

#[derive(strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
//#[derive(Display)]
//#[display(fmt = r#"{"id":1,"method":"{}","params":"{}"}"#, )]
pub enum Command {
    GetProp(Vec<String>),
    SetCtAbx(u8,Effect,usize),
    SetRgb(u32,Effect,usize),
    SetHsv(u8,u8,Effect,usize),
    SetBright(u8,Effect,usize),
    SetPower(bool,Effect,usize,Option<usize>),
}

impl Command {
    pub fn to_command(&self) -> String {
        //let param_part = match self {
        //    GetProp(ps) => ps.to_string(), // unwrap Vec from GetProp
        //};
        format!(r#"{{"id":1,"method":"{}","params":{:?}}}"#,self,self)
    }
}

/*
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out_str: String = match self {
            command::GetProp(params) => 
        }
    }
}
*/
