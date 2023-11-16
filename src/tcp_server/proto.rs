#[derive(Debug,Clone,Copy)]
pub struct tlv {
    flag: u64,
    flag_len: u64,
    len_len: u64,
}

#[derive(Debug,Clone,Copy)]
pub struct proto {
    request: tlv,
    response: tlv,
}

impl tlv {
    pub fn new(flag: u64,flag_len: u64,len_len: u64)->tlv {
        return tlv {
            flag:flag,
            flag_len:flag_len,
            len_len:len_len,
        }
    }
}

impl proto {
    pub fn new(request: tlv,response: tlv)->proto {
        return proto{
            request:request,
            response:response,
        }
    }
}