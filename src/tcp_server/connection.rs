use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener,TcpStream};
use byteorder::{ByteOrder, LittleEndian};
use tokio::io::{AsyncReadExt, BufReader, BufWriter,AsyncWriteExt};
use tokio::sync::mpsc;
use std::sync::Arc;
use std::rc;
use super::proto::{self, decoder};

pub trait RServer {
    fn on_open(&self,stream: &TcpStream,addr: String){}
    fn on_close(&self,addr: String){}
    fn react(&self,content: Vec<u8>,client_writer:  &mut mpsc::UnboundedSender<Vec<u8>>) -> Action;
}


pub enum Action {
    Normal,
    Close,
} 


#[derive(Debug)]
pub struct ConnectionReader{
    proto: proto::decoder,
    addr:String,
    sender:  mpsc::UnboundedSender<Vec<u8>>,
    buf_reader: BufReader<OwnedReadHalf>,
}


impl ConnectionReader{
    pub fn new(proto: proto::decoder,addr:String,writer: mpsc::UnboundedSender<Vec<u8>>,buf_reader:  BufReader<OwnedReadHalf>) -> ConnectionReader {
        return ConnectionReader {
            proto:proto,
            addr:addr,
            sender:writer,
            buf_reader:buf_reader,
        }
    }

    pub async fn process_content(&mut self,content_len: u64) -> Result<Vec<u8>,String> {
        let usize_len = content_len as usize;
        let mut vec_content=vec![0u8;usize_len];
        loop {
            match self.buf_reader.read_exact(&mut vec_content).await {
                Err(e) => {
                    return Err(format!("clent close for error: {}",e));
                }
                Ok(0) => {
                    return Err(String::from("client close"));
                }
                Ok(n) => {
                    return Ok(vec_content);
                }
            }
        }
    
        return Err(String::from("client close"));
    }
    
    pub async fn process_length(&mut self) -> Result<u64,String> {
        let mut vec_header = vec![0u8;self.proto.len_len as usize];
        loop {
            match self.buf_reader.read_exact(&mut vec_header).await {
                Err(e) => {
                    return Err(format!("clent close for error: {}",e));
                }
                Ok(0) => {
                    return Err(String::from("client close"));
                }
                Ok(n) => {
                    
                    let length = self.proto.decode_length(vec_header);
                   
                    return Ok(length);
                }
            }
        }
        return Err(String::from("client close"));
    }
    
    
    pub async fn process_flag(&mut self) -> Result<(),String> {

        if self.proto.flag_len == 0 {
            return Ok(());
        }

        let mut vec_flag = vec![0u8;self.proto.flag_len as usize];
        loop {
            match self.buf_reader.read_exact(&mut vec_flag).await {
                Err(e) => {
                    return Err(format!("clent close for error: {}",e));
                }
                Ok(0) => {
                    return Err(String::from("client close"));
                }
                Ok(n) => {
                    let flag = self.proto.decode_flag(vec_flag);
                    if flag == self.proto.flag {
                        return Ok(());
                    }
                    return Err(format!("clent close for flag error: {}",flag));
                }
            }
        }
    
        return Err(String::from("client close"));
    
    }
    
    pub async fn run <T:RServer>(&mut self,rserver:Arc<T>){
        loop {
            match self.process_flag().await {
                Ok(_) => { 
                    match self.process_length().await {
                        Ok(length) =>{
                            if length > self.proto.max_size || length <=0 {
                                println!("receive msg length error {}",length);
                                return;
                            }
    
                            match self.process_content(length).await {
                                Ok(content) => {
                                    
                                    match rserver.react(content,&mut self.sender) {
                                            Action::Close => {
                                            return ;
                                        },
                                        _ => {}
                                    }
                                }
                                Err(_) => {
                                    return;
                                }
                            }
                        }
                        Err(_)=> {
                            return;
                        }
                    }
                }
                Err(_)=> {
                    return;
                }
            }

        }   
        
    }
}


#[derive(Debug)]
pub struct ConnectionSender{
    proto: proto::decoder,
    addr:String,
    receiver:  mpsc::UnboundedReceiver<Vec<u8>>,
    buf_writer: BufWriter<OwnedWriteHalf>
}



impl ConnectionSender {
    pub fn new(proto: proto::decoder,addr:String,receiver: mpsc::UnboundedReceiver<Vec<u8>>,writer: BufWriter<OwnedWriteHalf>) -> ConnectionSender {
        return ConnectionSender {
            proto:proto,
            addr:addr,
            receiver:receiver,
            buf_writer:writer,
        }
    }

    pub async fn run(&mut self) {
        while let Some(v) = self.receiver.recv().await {
            if let Err(e) = self.buf_writer.write_all(&v).await {
                break;
            }
        }
    }

}
