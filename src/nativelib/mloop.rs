use crate::lang::Mtoken::*;

use crate::*;
use std::collections::HashSet;
// use std::time::{Duration, SystemTime};

pub fn init_loop()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_count(2, Mnative::new("loop".to_string(), rloop));

    return result
}

fn rloop(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let mut arg1 = raw_arg_block!(args, 1);

    bind_ctx(&mut arg1, ctx.clone());

    let mut i = 0;
    while i < arg0 {
        Msolver::solver_eval(copy_token_list(&arg1), ctx.clone());
        i += 1;
    }

    PMtoken::new_nil()
}


pub fn init_repeat()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_count(3, Mnative::new("repeat".to_string(), repeat));

    return result
}

fn repeat(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_word!(args, 0);
    let arg1 = raw_arg_int!(args, 1);
    let mut arg2 = raw_arg_block!(args, 2);

    let mut lctx = PMctx::new_by_father(MctxType::UsrCtx, ctx.clone());
    lctx.put(arg0.k.clone(), PMtoken::new_int(1));
  
    let mut local_set: HashSet<String> = HashSet::new();
    local_set.insert(arg0.k.clone());

    bind_ctx_local(&mut arg2, lctx.clone(), local_set);

    loop {
        let mut temp = lctx.get_now(arg0.k.clone());
        let times = raw_int!(temp);
        if times > arg1 {
            break;
        }
        Msolver::solver_eval(copy_token_list(&arg2), lctx.clone());

        // if let Mtoken::Integer{ref mut v} = *lctx.get_now(arg0.key.clone()).0.write().unwrap(){
        //     *v = times + 1
        // }

        lctx.put_now(arg0.k.clone(), PMtoken::new_int(times + 1));
    }

    PMtoken::new_nil()
}






