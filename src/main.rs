use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener,TcpStream};
use byteorder::{ByteOrder, LittleEndian};
use tokio::io::{AsyncReadExt, BufReader,BufWriter, AsyncWriteExt};
use tokio::sync::mpsc;
use std::sync::Arc;
use std::{io};
mod tcp_server;
use tcp_server::{connection,proto,server};
struct Event {}

impl tcp_server::connection::RServer for  Event {
    fn react(&self,content: Vec<u8>,client_writer: &mut mpsc::UnboundedSender<Vec<u8>>) -> connection::Action {
        let a = String::from_utf8(content).map(|op| {println!("{}",op);1});
        println!("react a {:?}",a);
        client_writer.send("testdata".to_string().into_bytes());

        connection::Action::Normal
    }
}

#[tokio::main]
async fn main()   {

    let decoder = proto::decoder::new(1001, 4, 4,16*1024,false).unwrap();
    let event = Event{};
    let s = server::simpleTcpServer::new("0.0.0.0:8000".to_string(),decoder,event);
    s.run().await;
}
