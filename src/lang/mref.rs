use crate::lang::*;

#[derive(Debug)]
pub struct Mref{
    pub k: String,
    pub c: Option<PMctx>
}

impl Mref {
    pub fn new(k: String)-> Mref{
        Mref{k: k, c: None}
    }

    pub fn new_with_ctx(k: String, c: PMctx)-> Mref{
        Mref{k: k, c: Some(c)}
    }

    pub fn clone(&self)-> Mref{
        if let Some(ref x) = self.c{
            Mref{k: self.k.clone(), c: Some(x.clone())}
        }else{
            Mref{k: self.k.clone(), c: None}
        }
    }

    pub fn set_key(&mut self, k: String){
        self.k = k.clone()
    }

    pub fn set_ctx(&mut self, c: PMctx){
        self.c = Some(c)
    }

}

























