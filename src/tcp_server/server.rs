use std::{io,vec,fmt};
use tokio::net::{TcpListener,TcpStream};

use super::proto::{self, tlv};
use super::connection::{connection,CallBack};
use std::sync::Arc;
pub struct simpleTcpServer {
    addr :String,
    proto: proto::proto,
    call: Arc<CallBack>,
}



impl simpleTcpServer {
    pub fn new(addr :String,proto: proto::proto,call_back: CallBack) -> simpleTcpServer {
        return simpleTcpServer{
            addr:addr,
            proto:proto,
            call:Arc::new(call_back),
        }
    }

    pub async fn run(&self) -> io::Result<()>  {
        let listener = TcpListener::bind(self.addr.to_string()).await?;
        loop {

            match listener.accept().await {
                Ok(v) =>{
                    let  stream = v.0;
                    let addr = v.1;
                    println!("new connection: {:?}", addr);

                    let p = self.proto.clone();
                    let call = Arc::clone(&self.call);
                    let c: connection = connection::new(p,call);
                    tokio::spawn(async move {
                        connection::process_socket(stream,c).await;
                    });
                    
                } 
                Err(e) => println!(" {:?}", e),
            }
        }
    }
}