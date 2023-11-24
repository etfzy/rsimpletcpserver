use byteorder::{BigEndian, ByteOrder, LittleEndian};
use tokio::io::AsyncWriteExt;

#[derive(Debug,Clone,Copy)]
pub struct decoder {
    pub flag: u64,
    pub flag_len: u64,
    pub len_len: u64,
    pub max_size: u64,
    pub big_endian: bool,
}


impl decoder {
    pub fn new(flag: u64,flag_len: u64,len_len: u64,max_size: u64,bigendian: bool)->Result<decoder,String> {
        let mut len_list:Vec<u64> = Vec::new();
        len_list.push(2);
        len_list.push(4);
        len_list.push(8);
        
        if !len_list.contains(&(flag_len)) {
            return Err("flag length is not in 1,2,4,8".to_string());
        }

        if !len_list.contains(&len_len) {
            return Err("len length is not in 1,2,4,8".to_string());
        }

        return Ok(decoder {
            flag:flag,
            flag_len:flag_len,
            len_len:len_len,
            max_size: max_size,
            big_endian:bigendian,
        })
    }


    pub fn decode_flag(&self,data: Vec<u8>) -> u64 {
        let mut result = 0 as u64;
        if self.big_endian {
            match self.flag_len {
                2 => {
                    result = BigEndian::read_u16(&data) as u64;
                }
                4 => {
                    result = BigEndian::read_u32(&data) as u64;
                }
                8 => {
                    result = BigEndian::read_u64(&data) as u64;
                }
                _=> {
                    result = 0
                }
            }
        }else {
            match self.flag_len {
                2 => {
                    result = LittleEndian::read_u16(&data) as u64;
                }
                4 => {
                    result = LittleEndian::read_u32(&data) as u64;
                }
                8 => {
                    result = LittleEndian::read_u64(&data) as u64;
                }
                _=> {
                    result = 0
                }
            }
        }

        return result;
    }


    pub fn decode_length(&self,data: Vec<u8>) -> u64 {
        let mut result = 0 as u64;
        if self.big_endian {
            match self.len_len {
                2 => {
                    result = BigEndian::read_u16(&data) as u64;
                }
                4 => {
                    result = BigEndian::read_u32(&data) as u64;
                }
                8 => {
                    result = BigEndian::read_u64(&data) as u64;
                }
                _=> {
                    result = 0
                }
            }
        }else {
            match self.len_len {
                2 => {
                    result = LittleEndian::read_u16(&data) as u64;
                }
                4 => {
                    result = LittleEndian::read_u32(&data) as u64;
                }
                8 => {
                    result = LittleEndian::read_u64(&data) as u64;
                }
                _=> {
                    result = 0
                }
            }
        }

        return result;
    }
}

