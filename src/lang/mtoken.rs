
use std::sync::{Arc, RwLock};

use crate::*;
use crate::lang::*;

#[derive(Debug)]
pub enum Mtoken{
    Nil,
    None,
    Err{v: String},
    Datatype{v: String},
    LitWord{v: String},
    Bool{v: bool},
    Char{v: char},
    Int{v: i64},
    Float{v: f64},
    Str{v: String},
    Block{v: Vec<PMtoken>},
    Paren{v: Vec<PMtoken>},
    Object{},
    Wrap{v: Mwrap},
    Call{v: Mcall},
    SetFunc{v: MsetFunc},
    Word{v: Mref},
    SetWord{v: Mref},
    PutWord{v: String},
    Op{v: Mcall},
    Native{v: MnativeMap},
    Func{v: MfuncMap},

    Undefined,

}


#[derive(Debug)]
pub struct PMtoken(pub Arc<RwLock<Mtoken>>);

impl PMtoken {

    pub fn new_nil()-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Nil{})))
    }

    pub fn new_none()-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::None{})))
    }

    pub fn new_err(v: String)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Err{v})))
    }

    pub fn new_datatype(v: String)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Datatype{v})))
    }

    pub fn new_lit_word(v: String)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::LitWord{v})))
    }

    pub fn new_bool(v: bool)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Bool{v})))
    }

    pub fn new_char(v: char)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Char{v})))
    }

    pub fn new_int(v: i64)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Int{v})))
    }

    pub fn new_float(v: f64)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Float{v})))
    }

    pub fn new_str(v: String)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Str{v})))
    }

    pub fn new_block(v: Vec<PMtoken>)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Block{v})))
    }

    pub fn new_paren(v: Vec<PMtoken>)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Paren{v})))
    }

    pub fn new_object()-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Object{ })))
    }

    pub fn new_wrap(v: Mwrap)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Wrap{v})))
    }

    pub fn new_call(v: Mcall)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Call{v})))
    }

    pub fn new_set_func(v: MsetFunc)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::SetFunc{v})))
    }

    pub fn new_word(v: Mref)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Word{v})))
    }

    pub fn new_set_word(v: Mref)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::SetWord{v})))
    }

    pub fn new_put_word(v: String)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::PutWord{v})))
    }

    pub fn new_op(op_name: String, callee: String, args: Vec<PMtoken>)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Op{v: Mcall::new_for_op(op_name, callee, args)})))
    }

    pub fn new_native(v: MnativeMap)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Native{v})))
    }

    pub fn new_func(v: MfuncMap)-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Func{v})))
    }

    pub fn new_undefined()-> PMtoken{
        PMtoken(Arc::new(RwLock::new(Mtoken::Undefined{ })))
    }



    pub fn clone(&self)-> PMtoken{
        PMtoken(self.0.clone())
    }

    pub fn get_rtype(&self)-> PMtoken{
        match &*self.0.read().unwrap() {
            Mtoken::Nil => PMtoken::new_datatype("nil!".to_string()),
            Mtoken::None => PMtoken::new_datatype("none!".to_string()),
            Mtoken::Err{v} => PMtoken::new_datatype("err!".to_string()),
            Mtoken::Datatype{v} => PMtoken::new_datatype("datatype!".to_string()),
            Mtoken::LitWord{v} => PMtoken::new_datatype("lit_word!".to_string()),
            Mtoken::Bool{v} => PMtoken::new_datatype("bool!".to_string()),
            Mtoken::Char{v} => PMtoken::new_datatype("char!".to_string()),
            Mtoken::Int{v} => PMtoken::new_datatype("int!".to_string()),
            Mtoken::Float{v} => PMtoken::new_datatype("float!".to_string()),
            Mtoken::Str{v} => PMtoken::new_datatype("str!".to_string()),
            Mtoken::Block{v} => PMtoken::new_datatype("block!".to_string()),
            Mtoken::Paren{v} => PMtoken::new_datatype("paren!".to_string()),
            Mtoken::Object{} => PMtoken::new_datatype("object!".to_string()),
            Mtoken::Wrap{v} => PMtoken::new_datatype("wrap!".to_string()),
            Mtoken::Call{v} => PMtoken::new_datatype("call!".to_string()),
            Mtoken::SetFunc{v} => PMtoken::new_datatype("set_func!".to_string()),
            Mtoken::Word{v} => PMtoken::new_datatype("word!".to_string()),
            Mtoken::SetWord{v} => PMtoken::new_datatype("set_word!".to_string()),
            Mtoken::PutWord{v} => PMtoken::new_datatype("put_word!".to_string()),
            Mtoken::Op{v} => PMtoken::new_datatype("op!".to_string()),
            Mtoken::Native{v} => PMtoken::new_datatype("native!".to_string()),
            Mtoken::Func{v} => PMtoken::new_datatype("func!".to_string()),

            _ => PMtoken::new_datatype("undefined!".to_string()),
        }
    }


    pub fn str_to_rtype(tp: String)-> PMtoken{
        match tp.as_str() {
            "nil!" => PMtoken::new_datatype("nil!".to_string()),
            "none!" => PMtoken::new_datatype("none!".to_string()),
            "err!" => PMtoken::new_datatype("err!".to_string()),
            "datatype!" => PMtoken::new_datatype("datatype!".to_string()),
            "lit_word!" => PMtoken::new_datatype("lit_word!".to_string()),
            "bool!" => PMtoken::new_datatype("bool!".to_string()),
            "char!" => PMtoken::new_datatype("char!".to_string()),
            "int!" => PMtoken::new_datatype("int!".to_string()),
            "float!" => PMtoken::new_datatype("float!".to_string()),
            "str!" => PMtoken::new_datatype("str!".to_string()),
            "block!" => PMtoken::new_datatype("block!".to_string()),
            "paren!" => PMtoken::new_datatype("paren!".to_string()),
            "object!" => PMtoken::new_datatype("object!".to_string()),
            "wrap!" => PMtoken::new_datatype("wrap!".to_string()),
            "call!" => PMtoken::new_datatype("call!".to_string()),
            "set_func!" => PMtoken::new_datatype("set_func!".to_string()),
            "word!" => PMtoken::new_datatype("word!".to_string()),
            "set_word!" => PMtoken::new_datatype("set_word!".to_string()),
            "put_word!" => PMtoken::new_datatype("put_word!".to_string()),
            "op!" => PMtoken::new_datatype("op!".to_string()),
            "native!" => PMtoken::new_datatype("native!".to_string()),
            "func!" => PMtoken::new_datatype("func!".to_string()),

            _ => PMtoken::new_datatype("undefined".to_string()),
        }
    }


    pub fn to_string(&self)-> String{
        match &*self.0.read().unwrap(){
            Mtoken::Nil => "nil".to_string(),
            Mtoken::None => "none".to_string(),
            Mtoken::Err{v} => v.clone(),
            Mtoken::Datatype{v} => v.clone(),
            Mtoken::LitWord{v} => v.clone(),
            Mtoken::Bool{v} => v.to_string(),
            Mtoken::Char{v} => v.to_string(),
            Mtoken::Int{v} => v.to_string(),
            Mtoken::Float{v} => v.to_string(),
            Mtoken::Str{v} => v.clone(),
            Mtoken::Block{v} => {
                let mut result = "[".to_string();
                for item in v.iter() {
                    result += &item.to_string();
                    result += &" ";
                }
                if result.len() > 1 {
                    result.pop();
                }
                result += &"]";
                return result
            },
            Mtoken::Paren{v} => {
                let mut result = "(".to_string();
                for item in v.iter() {
                    result += &item.to_string();
                    result += &" ";
                }
                if result.len() > 1 {
                    result.pop();
                }
                result += &")";
                return result
            }
            Mtoken::Object{} => "{}".to_string(),
            Mtoken::Wrap{v} => v.str.clone(),
            Mtoken::Call{v} => v.callee.clone(),
            Mtoken::SetFunc{v} => {
                let mut result = v.name.clone() + "(";
                for item in raw_paren!(v.args) {
                    result += &item.to_string();
                    result += &" ";
                }
                if result.len() > 1 {
                    result.pop();
                }
                result += &"):";
                return result
            },
            Mtoken::Word{v} => v.k.clone(),
            Mtoken::SetWord{v} => v.k.clone() + ":",
            Mtoken::PutWord{v} => v.clone() + ":=",
            Mtoken::Op{v} => v.op_name.clone(),
            Mtoken::Native{v} => "native".to_string(),
            Mtoken::Func{v} => "func".to_string(),

            _ => "undefined".to_string(),
        }

    }


    pub fn echo(&self){
        println!("{}", self.to_string())
    }


    pub fn is_nil(&self)-> bool{
        match *raw!(self){
            Mtoken::Nil => true,
            _ => false,
        }
    }

    pub fn is_none(&self)-> bool{
        match *raw!(self){
            Mtoken::None => true,
            _ => false,
        }
    }

    pub fn is_err(&self)-> bool{
        match &*raw!(self){
            Mtoken::Err{v} => true,
            _ => false,
        }
    }

    pub fn is_bool(&self)-> bool{
        match *raw!(self){
            Mtoken::Bool{v} => true,
            _ => false,
        }
    }

    pub fn is_datatype(&self)-> bool{
        match &*raw!(self){
            Mtoken::Datatype{v} => true,
            _ => false,
        }
    }

    pub fn is_block(&self)-> bool{
        match &*raw!(self){
            Mtoken::Block{v} => true,
            _ => false,
        }
    }

    pub fn is_paren(&self)-> bool{
        match &*raw!(self){
            Mtoken::Paren{v} => true,
            _ => false,
        }
    }

    pub fn is_wrap(&self)-> bool{
        match &*raw!(self){
            Mtoken::Wrap{v} => true,
            _ => false,
        }
    }

    pub fn is_call(&self)-> bool{
        match &*raw!(self){
            Mtoken::Call{v} => true,
            _ => false,
        }
    }

    pub fn is_set_func(&self)-> bool{
        match &*raw!(self){
            Mtoken::SetFunc{v} => true,
            _ => false,
        }
    }

    pub fn is_word(&self)-> bool{
        match &*raw!(self){
            Mtoken::Word{v} => true,
            _ => false,
        }
    }

    pub fn is_set_word(&self)-> bool{
        match &*raw!(self){
            Mtoken::SetWord{v} => true,
            _ => false,
        }
    }

    pub fn is_put_word(&self)-> bool{
        match &*raw!(self){
            Mtoken::PutWord{v} => true,
            _ => false,
        }
    }

    pub fn is_op(&self)-> bool{
        match &*raw!(self){
            Mtoken::Op{v} => true,
            _ => false,
        }
    }

   





    pub fn get_val(&self, ctx: PMctx)-> PMtoken{
        match &*raw!(self) {
            Mtoken::Word{v} => {
                if let Some(ref x) = v.c {
                    x.get(v.k.clone())
                }else{
                    ctx.get(v.k.clone())
                }
            },

            Mtoken::LitWord{v} => {
                PMtoken::new_word(Mref::new(v.clone()))
            },

            Mtoken::Paren{v} => {
                Msolver::solver_eval(copy_token_list(&v), ctx)
            }

            Mtoken::Wrap{v} => {
                v.raw.clone()
            },

            Mtoken::Call{v} => {
                v.run(ctx.clone())
            },

            _ => self.clone()
        }
    }






}


pub fn copy_token_list(list: &Vec<PMtoken>)-> Vec<PMtoken>{
    let mut result = Vec::new();
    for item in list.iter(){
        result.push(item.clone());
    }
    result
}





















