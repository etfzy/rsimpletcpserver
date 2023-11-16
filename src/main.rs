use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio::net::{TcpListener, TcpStream};
use std::{io};
mod tcp_server;
use tcp_server::{server,proto,connection};

fn call(content: Vec<u8>) -> connection::Action {
    let num = content.len();
    let s = String::from_utf8(content).unwrap_or(String::from(""));
    println!("content {:?} {:?}",num,s);
    return connection::Action::Normal
}

#[tokio::main]
async fn main()   {

    let request = proto::tlv::new(100, 4, 4);
    let response = proto::tlv::new(100, 4, 4);
    let proto = proto::proto::new(request, response);
    let s = server::simpleTcpServer::new("0.0.0.0:8020".to_string(),proto,call);
    s.run().await;
}
