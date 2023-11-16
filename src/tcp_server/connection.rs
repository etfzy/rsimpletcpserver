use tokio::net::tcp::OwnedReadHalf;
use tokio::net::{TcpListener,TcpStream};
use byteorder::{ByteOrder, LittleEndian};
use tokio::io::{AsyncReadExt, BufReader, AsyncWriteExt};
use std::sync::Arc;
use super::proto::{self, tlv};


pub enum Action {
    Normal,
    Close,
} 

pub type CallBack = fn(content: Vec<u8>)-> Action;




#[derive(Debug)]
pub struct connection {
    proto: proto::proto,
    call: Arc<CallBack>
}


impl connection {
    pub fn new(proto: proto::proto,call:Arc::<CallBack>) -> connection {
        return connection {proto:proto,call}
    }
    pub async fn process_content(&self,buf_reader: &mut BufReader<OwnedReadHalf>,content_len: u64) -> Result<Vec<u8>,String> {
        let usize_len = content_len as usize;
        let mut vec_content=vec![0u8;usize_len];
        loop {
            match buf_reader.read_exact(&mut vec_content).await {
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
    
    pub async fn process_header(&self,buf_reader: &mut BufReader<OwnedReadHalf>) -> Result<u64,String> {
        let mut vec_header = vec![0u8;4];
        loop {
            match buf_reader.read_exact(&mut vec_header).await {
                Err(e) => {
                    return Err(format!("clent close for error: {}",e));
                }
                Ok(0) => {
                    return Err(String::from("client close"));
                }
                Ok(n) => {
                    println!("read buffer length {}",n);
                    let header = LittleEndian::read_u32(&vec_header);
                    return Ok(header as u64);
                }
            }
        }
    
        return Err(String::from("client close"));
    
    }
    
    
    pub async fn process_flag(&self,buf_reader: &mut BufReader<OwnedReadHalf>) -> Result<(),String> {
        let mut vec_header = vec![0u8;4];
        loop {
            match buf_reader.read_exact(&mut vec_header).await {
                Err(e) => {
                    return Err(format!("clent close for error: {}",e));
                }
                Ok(0) => {
                    return Err(String::from("client close"));
                }
                Ok(n) => {
                    println!("read buffer length {}",n);
                    let flag = LittleEndian::read_u32(&vec_header);
    
                    if let 1001 = flag {
                        println!("flag check ok:{}",flag);
                        return Ok(());
                    }
                    println!("flag check failed:{}",flag);
                    return Err(format!("clent close for flag error: {}",flag));
                }
            }
        }
    
        return Err(String::from("client close"));
    
    }
    
  
    pub fn run (&self,content: Vec<u8>)-> Action {
        return (*self.call)(content);
    }
    pub async fn process_socket(stream: TcpStream,conn: connection) {
 
        stream.set_nodelay(true);
        let (mut client_reader,  mut client_writer) = stream.into_split();
        
        let mut buf_reader = tokio::io::BufReader::new(client_reader);
    
        let mut count = 0;
        loop {
            match conn.process_flag(&mut buf_reader).await {
                Ok(_) => { 
                    match conn.process_header(&mut buf_reader).await {
                        Ok(num) =>{
                            println!("receive {}",num);
    
                            if num > 16*1024 {
                                println!("receive too lang {}",num);
                                break; 
                            }
    
                            match conn.process_content(&mut buf_reader,num).await {
                                Ok(content) => {

                                        conn.run(content);

                                }
                                Err(_) => {
                                    break;
                                }
                            }
                        }
                        Err(_)=> {
                            println!("stop thread for process_header");
                            break;
                        }
                    }
                }
                Err(_)=> {
                    println!("stop thread for process_flag");
                    break ;
                }
            }
            count = count+1;
            println!("stop client read{}",count);
        }    
    }
}


