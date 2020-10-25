use crate::*;
use crate::lang::*;

#[derive(Debug)]
pub struct Mcall{
    pub callee:  String,
    pub op_name: String,
    pub ctx: Option<PMctx>,
    pub paren:   PMtoken,
    pub block:   PMtoken,
    pub object:  PMtoken,

}

impl Mcall {
    pub fn new(callee: String, op_name: String, paren: PMtoken, block: PMtoken, object: PMtoken)-> Mcall{
        Mcall{
            callee: callee,
            op_name: op_name,
            ctx: None,
            paren: paren,
            block: block,
            object: object,
        }
    }

    pub fn new_with_callee(callee: String)-> Mcall{
        Mcall{
            callee: callee,
            op_name: "".to_string(),
            ctx: None,
            paren: PMtoken::new_nil(),
            block: PMtoken::new_nil(),
            object: PMtoken::new_nil(),
        }
    }

    pub fn new_with_paren(callee: String, paren: PMtoken)-> Mcall{
        Mcall{
            callee: callee,
            op_name: "".to_string(),
            ctx: None,
            paren: paren,
            block: PMtoken::new_nil(),
            object: PMtoken::new_nil(),
        }
    }

    pub fn new_with_block(callee: String, block: PMtoken)-> Mcall{
        Mcall{
            callee: callee,
            op_name: "".to_string(),
            ctx: None,
            paren: PMtoken::new_nil(),
            block: block,
            object: PMtoken::new_nil(),
        }
    }

    pub fn new_for_op(op_name: String, callee: String, args: Vec<PMtoken>)-> Mcall{
        Mcall{
            callee: callee, 
            op_name: op_name, 
            ctx: None, 
            paren: PMtoken::new_block(args),
            block: PMtoken::new_nil(),
            object: PMtoken::new_nil(),
        }
    }

    pub fn clone(&self)-> Mcall{
        let mut c : Option<PMctx> = None;
        if let Some(ref x) = self.ctx{
            c = Some(x.clone())
        }
        Mcall{
            callee: self.callee.clone(),
            op_name: self.op_name.clone(),
            ctx: c,
            paren: self.paren.clone(),
            block: self.block.clone(),
            object: self.object.clone(),
        }
    }

    pub fn run(&self, ctx: PMctx)-> PMtoken{
        let mut func: PMtoken;
        if let Some(ref x) = self.ctx{
            func = x.get(self.callee.clone())
        }else{
            func = ctx.get(self.callee.clone())
        }

        let mut act_args: Vec<PMtoken> = Vec::new();

        if self.paren.is_paren(){
            let mut ori_args = raw_read_paren!(self.paren);
            bind_ctx(&mut ori_args, ctx.clone());
            act_args = Msolver::solver_reduce(ori_args, ctx.clone());
        }
        
        
        if self.block.is_block(){
            act_args.push(self.block.clone());
        }

        match &*raw!(func){
            Mtoken::Native{v} => {
                return v.run(act_args, ctx.clone())
            },
            Mtoken::Func{v} => {
                return v.run(act_args, ctx.clone())
            },

            _ => {}
        }



        return PMtoken::new_err("No such native! or func!".to_string())
    }
}


