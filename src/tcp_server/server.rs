use std::net::SocketAddr;
use std::{io,vec,fmt};
use tokio::net::{TcpListener,TcpStream};
use tokio::task::JoinError;
use tokio::task::JoinSet;
use super::proto::{self, decoder};
use super::connection::{ConnectionReader,Action,RServer,ConnectionSender};
use std::sync::Arc;
use tokio::{self,sync};

pub struct  simpleTcpServer<T:RServer+Send+Sync+'static> {
    addr :String,
    proto: proto::decoder,
    rserver:Arc<T>,
}


impl <T:RServer+Send+Sync+'static> simpleTcpServer<T>{
    pub fn new(addr :String,proto: proto::decoder,rserver:T) -> simpleTcpServer<T> {
        return simpleTcpServer{
            addr:addr,
            proto:proto,
            rserver:Arc::new(rserver),
        }
    }

    pub async fn run(&self) -> io::Result<()>  {
        println!("listen tcp server {}",self.addr);
        let listener = TcpListener::bind(self.addr.to_string()).await?;
        loop {

            match listener.accept().await {
                Ok(v) =>{
                    let  stream: TcpStream = v.0;
                    let addr = v.1;

                    println!("{:?}",addr);
                    
                    let arc_rserver = self.rserver.clone();
                    let arc_close = self.rserver.clone();
                    let proto_clone = self.proto.clone();
                                        
                    tokio::spawn(async move {
                        arc_rserver.on_open(&stream, addr.to_string());
                        stream.set_nodelay(true);
                        let (mut client_reader,  mut client_writer) = stream.into_split();
                   
                        let mut buf_reader = tokio::io::BufReader::new(client_reader);
                        let mut buf_writer = tokio::io::BufWriter::new(client_writer);
                        

                        let (tx,mut  rx) = sync::mpsc::unbounded_channel::<Vec<u8>>();
                        let mut conn_reader: ConnectionReader = ConnectionReader::new(proto_clone,addr.to_string(),tx,buf_reader);

                        let mut conn_writer: ConnectionSender = ConnectionSender::new(proto_clone,addr.to_string(),rx,buf_writer);
                        
                        let mut set = JoinSet::new();
                        set.spawn(async move {
                            conn_writer.run().await;

                        });

                        set.spawn(async move {
                            conn_reader.run(arc_rserver).await;
                        });
            
                        set.join_next().await;

                        set.abort_all();
                        println!("stop thread remote {}",addr.clone());
                        arc_close.on_close(addr.to_string());

                    });
                    
                } 
                Err(e) => println!(" {:?}", e),
            }
        }
    }
}