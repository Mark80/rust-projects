use std::io::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut client = RedisClient::new("localhost:6379").await?;
    let response = client.ping().await?;
    let r2 = client.set("foo2".to_string(), "bar".to_string()).await?;
    println!("{:?}", response);
    println!("{:?}", r2);

    Ok(())
}

struct RedisClient {
    stream: TcpStream
}


/*For Simple Strings the first byte of the reply is "+"
For Errors the first byte of the reply is "-"
For Integers the first byte of the reply is ":"
For Bulk Strings the first byte of the reply is "$"
For Arrays the first byte of the reply is "*"*/
enum RESP {
    SimpleStrings(String),
    Errors(Vec<u8>),
    Integers(i64),
    BulkString(Vec<u8>),
    Arrays(Vec<RESP>),
}

impl RESP {

    fn serialize(self, buf: &mut Vec<u8>)  {

        match self {
            RESP::Arrays(values) => {
                buf.push(b'*');
                buf.append(&mut format!("{}", values.len()).into_bytes());
                buf.push('\r' as u8 );
                buf.push('\n' as u8 );
                for v in values {
                    v.serialize(buf);
                }
            }

            RESP::BulkString(mut data) => {
                buf.push(b'$');
                buf.append(&mut format!("{}", data.len()).into_bytes());
                buf.push('\r' as u8 );
                buf.push('\n' as u8 );
                buf.append(&mut data);
                buf.push('\r' as u8 );
                buf.push('\n' as u8 );
            }
            _ => unimplemented!()
        }

    }
}

impl RedisClient {
    async fn new(address: &str) -> Result<RedisClient, Error> {
        let stream = TcpStream::connect(address).await?;
        Ok(RedisClient {
            stream
        })
    }

    async fn ping(&mut self) -> Result<String, Error> {
        let mut ping_command = String::from("");
        ping_command.push_str("*1\r\n");
        ping_command.push_str("$4\r\n");
        ping_command.push_str("PING\r\n");

        let response = self.execute_command(ping_command).await;
        Ok(response)
    }

    async fn set(&mut self, key: String, value: String) -> Result<String, Error> {
        let set_command = format!("*3\r\n$3\r\nSET\r\n${}\r\n{}\r\n${}\r\n{}\r\n", key.len(), key, value.len(), value);
        let response = self.execute_command(set_command).await;
        Ok(response)
    }

    async fn get(&mut self, key: String) -> Result<String, Error> {
        let set_command = format!("*2\r\n$3\r\nGET\r\n${}\r\n{}\r\n", key.len(), key);
        let response = self.execute_command2(set_command).await;
        Ok(response)
    }

    async fn execute_command2(&mut self, command: String) -> String {
        self.stream.write_all(command.as_bytes()).await;
        let mut buf = vec![0; 1024];
        let size = self.stream.read(&mut buf).await.unwrap();
        RedisClient::parse_response2(&mut buf, &size)
    }

    async fn execute_command(&mut self, set_command: String) -> String {
        self.stream.write_all(set_command.as_bytes()).await;
        let mut buf = vec![0; 1024];
        let size = self.stream.read(&mut buf).await.unwrap();
        RedisClient::parse_response(&mut buf, &size)
    }

    fn parse_response2(buf: &mut Vec<u8>, size: &usize) -> String {
        let s = &buf[4..size - 2];
        let response = std::str::from_utf8(s).unwrap();
        String::from(response)
    }

    fn parse_response(buf: &mut Vec<u8>, size: &usize) -> String {
        let s = &buf[1..size - 2];
        let response = std::str::from_utf8(s).unwrap();
        String::from(response)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pong() {
        let mut client = RedisClient::new("localhost:6379").await.unwrap();
        let response = client.ping().await.unwrap();
        assert_eq!(response, "PONG");
    }

    #[tokio::test]
    async fn test_set_get_delete() {
        let mut client = RedisClient::new("localhost:6379").await.unwrap();
        let response = client.get("foo2".to_string()).await.unwrap();
        assert_eq!(response, "bar");
    }
}