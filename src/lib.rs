use std::error::Error;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;

#[derive(Debug)]
pub enum Errors {
    ConnectionError(String, u32)
}

impl std::error::Error for Errors {}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Errors::ConnectionError(host, port) => write!(f, "Unable to connect to {}:{}", host, port)
        }
    }
}

pub struct Client {
    host: String,
    port: u32,
    connection: TcpStream,
}

impl Client {
    pub fn new(host: String, port: u32, connection: TcpStream) -> Self {
        Client{ host, port, connection }
    }

    pub fn read(&self) -> Vec<u8> {
        let mut response: Vec<u8> = vec![];
        let mut buf = BufReader::new(client.connection);
        loop {
            match client.connection.read(&mut response) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                },
                Err(e) => panic!("{}", e)
            }
        }
        return response;
    }
}

pub fn connect(host: &str, port: u32) -> Result<Client, Errors> {
    let mut connection = match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(stream) => stream,
        Err(error) => {
            return Err(Errors::ConnectionError(String::from(host), port));
        }
    };
    let client = Client::new(String::from(host), port, connection);
    return Ok(client);
}

pub fn ping(client: &mut Client) {
    //client.connection.write(b"*1\r\n$4\r\nPING\r\n").unwrap();
    write(client, b"*1\r\n$4\r\nPING\r\n".to_vec());
    let resp = read(client);
    println!("{}", from_utf8(&resp).unwrap());
}

fn write(client: &mut Client, buf: Vec<u8>) -> usize{
    let size = buf.len();
    let mut sent: usize = 0;
    while sent < size {
        match client.connection.write(&buf) {
            Ok(n) => sent = sent + n,
            Err(e) => panic!("{}", e)
        }
    }
    return sent;
}

fn read(client: &mut Client) -> Vec<u8> {
    let mut response: Vec<u8> = vec![];
    let mut buf = BufReader::new(client.connection);
    loop {
        match client.connection.read(&mut response) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
            },
            Err(e) => panic!("{}", e)
        }
    }
    return response;
}