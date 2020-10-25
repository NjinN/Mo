use crate::lang::*;

use crate::*;


pub fn init_if()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_count(2, Mnative::new("if".to_string(), rif));

    return result
}


fn rif(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_bool!(args, 0);
    let mut arg1 = raw_arg_block!(args, 1);

    bind_ctx(&mut arg1, ctx.clone());

    if arg0 {
        return Msolver::solver_eval(copy_token_list(&arg1), ctx.clone());
    }

    PMtoken::new_nil()
}


pub fn init_either()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_count(3, Mnative::new("either".to_string(), either));

    return result
}


fn either(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_bool!(args, 0);
    let mut arg1 = raw_arg_block!(args, 1);
    let mut arg2 = raw_arg_block!(args, 2);

    bind_ctx(&mut arg1, ctx.clone());

    if arg0 {
        return Msolver::solver_eval(copy_token_list(&arg1), ctx.clone());
    }else{
        return Msolver::solver_eval(copy_token_list(&arg2), ctx.clone());
    }

    PMtoken::new_nil()
}







