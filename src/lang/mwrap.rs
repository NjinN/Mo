use crate::lang::mtoken::*;

#[derive(Debug)]
pub struct Mwrap {
    pub str: String,
    pub raw: PMtoken,
}

impl Mwrap {
    pub fn new(str: String, raw: PMtoken)-> Mwrap{
        Mwrap{str: str, raw: raw}
    }
}

