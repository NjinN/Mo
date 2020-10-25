
use std::collections::HashMap;

use crate::*;

use crate::lang::*;

#[derive(Debug)]
pub struct Mfunc {
    pub args:   Vec<PMtoken>,
    pub body:   Vec<PMtoken>,
}

impl Mfunc {
    pub fn new()-> Self{
        Mfunc{args: Vec::new(), body: Vec::new()}
    }

    pub fn new_one(args: Vec<PMtoken>, body: Vec<PMtoken>,)-> Self{
        Mfunc{args: args, body: body}
    }

    pub fn run(&self, act_args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
        let mut fctx = PMctx::new_by_father(MctxType::UsrCtx, ctx.clone());

        let mut i = 0;
        while i < self.args.len() {
            let arg_key = raw_arg_word!(self.args, i).k.clone();
            let arg_val = act_args.get(i).unwrap().clone();
            fctx.put_now(arg_key.clone(), arg_val);
            i += 1;
        }

        Msolver::solver_eval(copy_token_list(&self.body), fctx)
        
    }


}


#[derive(Debug)]
pub struct MfuncMap {
    pub count_map:  HashMap<u32, Mfunc>,
    pub types_map:   HashMap<String, Mfunc>,
}

impl MfuncMap {
    pub fn new()-> Self{
        MfuncMap{
            count_map:  HashMap::new(),
            types_map:  HashMap::new(),
        }
        
    }


    pub fn run(&self, args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
        if self.types_map.len() == 0 {
            let args_count = args.len() as u32;

            let func = self.count_map.get(&args_count);
            match func {
                Some(nv) => {
                    nv.run(args, ctx)
                },
                None => PMtoken::new_err("No such func!".to_string()),
            }

        }else{
            let mut types_str = "".to_string();
 
            for item in args.iter() {
                types_str += &item.get_rtype().to_string();
                types_str += "-";
            }
            
            let func = self.types_map.get(&types_str);
            if let Some(ref nv) = func{
                return nv.run(args, ctx)
            }

            let args_count = args.len() as u32;
            let func = self.count_map.get(&args_count);
            match func {
                Some(nv) => {
                    nv.run(args, ctx)
                },
                None => PMtoken::new_err("No such func!".to_string()),
            }

        }
    }
    
}




