use crate::lang::*;


pub fn init_op(c: PMctx){
    let mut ctx = &mut (c.clone());
    ctx.put_now("+".to_string(), PMtoken::new_op("+".to_string(), "add".to_string(), Vec::new()));
    ctx.put_now("-".to_string(), PMtoken::new_op("-".to_string(), "sub".to_string(), Vec::new()));
    ctx.put_now("*".to_string(), PMtoken::new_op("*".to_string(), "mul".to_string(), Vec::new()));
    ctx.put_now("/".to_string(), PMtoken::new_op("/".to_string(), "div".to_string(), Vec::new()));

    ctx.put_now("=".to_string(), PMtoken::new_op("=".to_string(), "eq".to_string(), Vec::new()));
    ctx.put_now(">".to_string(), PMtoken::new_op(">".to_string(), "gt".to_string(), Vec::new()));
    ctx.put_now("<".to_string(), PMtoken::new_op("<".to_string(), "lt".to_string(), Vec::new()));
    ctx.put_now(">=".to_string(), PMtoken::new_op(">=".to_string(), "ge".to_string(), Vec::new()));
    ctx.put_now("<=".to_string(), PMtoken::new_op("<=".to_string(), "le".to_string(), Vec::new()));


}
