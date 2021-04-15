use std::io::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut client = RedisClient::new("localhost:6379").await?;

    let response = client.ping().await?;

    println!("{:?}", response);

    Ok(())
}

struct RedisClient {
    stream: TcpStream
}

pub  const PING_COMMAND : & 'static [u8; 14] =  b"*1\r\n$4\r\nPING\r\n";

impl RedisClient {


    async fn new(address: &str) -> Result<RedisClient, Error> {
        let stream = TcpStream::connect(address).await?;
        Ok(RedisClient {
            stream
        })
    }

    async fn ping(&mut self) -> Result<String,Error> {
        self.stream.write_all(PING_COMMAND).await?;
        let mut buf = vec![0; 1024];
        let size = self.stream.read(&mut buf).await?;
        let response = RedisClient::parse_response(&mut buf, &size);
        Ok(response)
    }

    fn parse_response(buf: &mut Vec<u8>, size: &usize) -> String {
        let s = &buf[1..size - 2];
        let response = std::str::from_utf8(s).unwrap();
        String::from(response)
    }
}
