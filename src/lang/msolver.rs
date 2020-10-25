use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

use crate::*;

use crate::lang::*;

pub fn bind_ctx(list: &mut Vec<PMtoken>, ctx: PMctx){
    bind_ctx_raw(list, ctx.clone(), HashSet::new())
}

pub fn bind_ctx_local(list: &mut Vec<PMtoken>, ctx: PMctx, local: HashSet<String>){
    bind_ctx_raw(list, ctx.clone(), local)
}

pub fn bind_ctx_raw(list: &mut Vec<PMtoken>, ctx: PMctx, local: HashSet<String>){
    let mut put_word_set = local;
    for item in list.iter() {
        match &mut *raw_mut!(item) {
            Mtoken::Word{v} | Mtoken::SetWord{v} => {
                let bind = ctx.get_with_ctx(v.k.clone());
                if bind.0.is_nil(){
                    if put_word_set.contains(&(v.k.clone())){
                        v.c = None;
                    }else{
                        v.c = Some(ctx.clone());
                    }
                }else{
                    v.c = Some(bind.1.clone())
                }

            },


            Mtoken::Call{v} => {
                let bind = ctx.get_with_ctx(v.callee.clone());
                if bind.0.is_nil() {
                    if put_word_set.contains(&(v.callee.clone())){
                        v.ctx = None;
                    }else{
                        v.ctx = Some(ctx.clone());
                    }
                }else{
                     v.ctx = Some(ctx.clone());
                }
            },

            _ => {},
        }
    }


}


pub struct Msolver {
    pub inp: Vec<PMtoken>,
    pub inp_len: usize,
    pub idx: usize,
    pub now_tk: PMtoken,
    pub next_tk: PMtoken,
}

impl Msolver {
    pub fn solver_eval_str(code: String, ctx: PMctx)-> PMtoken{
        let inp = to_tokens(code, ctx.clone());
        Msolver::new(inp).eval_blk(ctx)
    }

    pub fn solver_eval(inp: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
        Msolver::new(inp).eval_blk(ctx)
    }

    pub fn solver_reduce_str(code: String, ctx: PMctx)-> Vec<PMtoken>{
        let inp = to_tokens(code, ctx.clone());
        Msolver::new(inp).reduce_blk(ctx)
    }

    pub fn solver_reduce(inp: Vec<PMtoken>, ctx: PMctx)-> Vec<PMtoken>{
        Msolver::new(inp).reduce_blk(ctx)
    }

    pub fn new(inp: Vec<PMtoken>)-> Msolver{
        let inp_len = inp.len();
        Msolver{
            inp: inp,
            inp_len: inp_len,
            idx: 0,
            now_tk: PMtoken::new_nil(),
            next_tk: PMtoken::new_nil(),
        }
    }

    pub fn pre_read(&mut self, ctx: PMctx){
        if self.idx >= self.inp_len {
            return
        }
        if !self.next_tk.is_nil() {
            self.now_tk = self.next_tk.clone();
        }else{
            self.now_tk = self.inp.get(self.idx).unwrap().clone().get_val(ctx.clone());
            // now_tk.echo();
        }
        
        if self.idx < self.inp_len - 1 {
            self.next_tk = self.inp.get(self.idx+1).unwrap().clone().get_val(ctx.clone());
            // next_tk.echo();
        }
        self.idx += 1
    }

    pub fn eval_blk(&mut self, ctx: PMctx)-> PMtoken{
        let mut temp = PMtoken::new_nil();
        while self.idx < self.inp_len {
            temp = self.eval_one(ctx.clone(), true)
        }

        return temp
    }

    pub fn reduce_blk(&mut self, ctx: PMctx)-> Vec<PMtoken>{
        let mut result: Vec<PMtoken> = Vec::new();
        
        while self.idx < self.inp_len {
            result.push(self.eval_one(ctx.clone(), true))
        }
        return result
    }

    pub fn eval_one(&mut self, ctx: PMctx, pre_read: bool)-> PMtoken{
        if pre_read {
           self.pre_read(ctx.clone()); 
        }
        
        if self.now_tk.is_set_word(){
            let k = raw_set_word!(self.now_tk).k.clone();
            let v = self.eval_one(ctx.clone(), true);
            ctx.clone().put(k, v.clone());
            self.pre_read(ctx.clone());
            return v
        }else if self.now_tk.is_set_func(){
            let set_func = raw_set_func!(self.now_tk.clone());
            self.pre_read(ctx.clone());
            return set_func.dim_func(self.now_tk.clone(), ctx.clone())

        }else if self.next_tk.is_op(){
            let arg_l = self.now_tk.clone();
            let op = self.next_tk.clone();
            self.pre_read(ctx.clone());
            self.pre_read(ctx.clone());
            let arg_r = self.now_tk.clone();
            let op_args = vec![arg_l, arg_r];
            let temp: PMtoken;
            match &mut *raw_mut!(op){
                Mtoken::Op{v} => {
                    v.paren = PMtoken::new_paren(op_args);
                    temp = v.run(ctx.clone())
                },
                _ => return PMtoken::new_err("Error grammar!".to_string()),
            }

            if self.next_tk.is_op(){
                self.now_tk = temp; 
                return self.eval_one(ctx.clone(), false)
            }else{
                return temp
            }
        }
        
        let result = self.now_tk.clone();
        // self.pre_read(ctx.clone());
        return result
    }

}




// pub fn eval_str(code: String, ctx: PMctx)-> PMtoken{
//     eval_blk(to_tokens(code, ctx.clone()), ctx)
// }


// pub fn eval_blk(inp: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
//     let mut now_tk = PMtoken::new_nil();
//     let mut next_tk = PMtoken::new_nil();

//     let mut i = 0;
//     while i < inp.len() {
//         if !next_tk.is_nil() {
//             now_tk = next_tk.clone();
//             // if now_tk.is_word(){
//             //     // now_tk.echo();
//             //     now_tk = now_tk.get_word_val(ctx.clone());
//             // }
//         }else{
//             now_tk = inp.get(i).unwrap().clone().get_val(ctx.clone());
//             // now_tk.echo();
//         }
        
//         if i < inp.len() - 1 {
//             next_tk = inp.get(i+1).unwrap().clone().get_val(ctx.clone());
//             // next_tk.echo();
//         }



//     }





//     PMtoken::new_nil()
// }


// pub fn eval_one(inp: &Vec<PMtoken>, ctx: PMctx, idx: &mut i32, now_tk: PMtoken, next_tk: PMtoken)-> PMtoken{
//     if now_tk.is_set_word() {

        
//     }

//     if next_tk.is_op(){

//     }

    


//     PMtoken::new_nil()
// }

// pub fn eval_op(inp: &Vec<PMtoken>, ctx: PMctx, idx: &mut i32, op: PMtoken, op_l: PMtoken)-> PMtoken{
//     let mut now_tk = PMtoken::new_nil();
//     let mut next_tk = PMtoken::new_nil();
//     let inp_len = inp.len() as i32;
//     *idx += 1;
//     if *idx < inp_len {
//         now_tk = inp.get(*idx as usize).unwrap().clone().get_val(ctx.clone());
//     }else{
//         return PMtoken::new_err("Incomplete Op Expr".to_string())
//     }

//     if *idx < inp_len - 1 {
//         next_tk = inp.get((*idx + 1) as usize).unwrap().clone().get_val(ctx.clone());
//     }

//     if next_tk.is_op(){

//     }else{

//     }


//     PMtoken::new_nil()
// }















