
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::*;
use crate::lang::*;

#[derive(Debug)]
pub enum MctxType {
    SysCtx,
    UsrCtx,
    TmpCtx,
}


#[derive(Debug)]
pub struct Mctx{
    pub t: MctxType,
    pub m: HashMap<String, PMtoken>,
    pub f: Option<PMctx>
}

#[derive(Debug)]
pub struct PMctx(pub Arc<RwLock<Mctx>>);


impl PMctx {
    pub fn new(t: MctxType)-> PMctx{
        PMctx(Arc::new(RwLock::new(
            Mctx{
                t: t,
                m: HashMap::new(),
                f: None,
            }
        )))
    }

    pub fn new_by_father(t: MctxType, f: PMctx)-> PMctx{
        PMctx(Arc::new(RwLock::new(
            Mctx{
                t: t,
                m: HashMap::new(),
                f: Some(f),
            }
        )))
    }

    pub fn new_by_map(t: MctxType, m: HashMap<String, PMtoken>)-> PMctx{
        PMctx(Arc::new(RwLock::new(
            Mctx{
                t: t,
                m: m,
                f: None,
            }
        )))
    }

    pub fn clone(&self)-> PMctx{
        PMctx(self.0.clone())
    }

    pub fn has_father(&self)-> bool{
        if let Some(ref x) = raw!(self).f{
            return true;
        }
        return false;
    }

    pub fn set_father(&mut self, f: PMctx){
        raw_mut!(self).f = Some(f)
    }

    pub fn clone_father(&self)-> Option<PMctx>{
        if let Some(ref x) = raw!(self).f {
            return Some(x.clone())
        }
        None
    }

    pub fn room(&self)-> i64{
        return raw!(self).m.capacity() as i64
    }

    pub fn len(&self)-> i64{
        return raw!(self).m.len() as i64
    }

    pub fn is_empty(&self)-> bool{
        raw!(self).m.is_empty()
    }

    pub fn fit_size(&mut self){
        raw_mut!(self).m.shrink_to_fit()
    }

    pub fn put_now(&mut self, k: String, v: PMtoken){
        raw_mut!(self).m.insert(k, v);
    }

    pub fn get_now(&self, k: String)-> PMtoken{
        if let Some(x) = raw!(self).m.get(&k){
            return x.clone()
        }
        PMtoken::new_nil()
    }

    pub fn remove_now(&mut self, k: String){
        raw_mut!(self).m.remove(&k);
    }


    pub fn put(&mut self, k:String, v: PMtoken){
        let mut temp = self.clone();
        loop {
            if raw!(temp).m.contains_key(&k){
                temp.put_now(k, v);
                return
            }
            let f: PMctx;
            if let Some(ref x) = raw!(temp).f{
                f = x.clone();
            }else{
                break
            }
            temp = f;
        }

        self.put_now(k, v)

    }


    pub fn get(&self, k: String)-> PMtoken{
        let mut temp = self.clone();
        loop {
            if raw!(temp).m.contains_key(&k){
                return temp.get_now(k);
            }
            let f: PMctx;
            if let Some(ref x) = raw!(temp).f{
                f = x.clone();
            }else{
                break
            }
            temp = f;
        }

        PMtoken::new_nil()
    }

    pub fn get_with_ctx(&self, k: String)-> (PMtoken, PMctx){
        let mut temp = self.clone();
        loop {
            if raw!(temp).m.contains_key(&k){
                return (temp.get_now(k), temp);
            }
            let f: PMctx;
            if let Some(ref x) = raw!(temp).f{
                f = x.clone();
            }else{
                break
            }
            temp = f;
        }

        (PMtoken::new_nil(), self.clone())
    }


    pub fn remove(&mut self, k: String){
        let mut temp = self.clone();
        loop {
            if raw!(temp).m.contains_key(&k){
                temp.remove(k);
                break
            }
            let f: PMctx;
            if let Some(ref x) = raw!(temp).f{
                f = x.clone();
            }else{
                break
            }
            temp = f;
        }
        if self.len() < self.room() / 4 {
            self.fit_size()
        }
    }


}




























