use crate::lang::*;
use crate::nativelib::*;

pub fn init_native(c: PMctx){
    let mut ctx = &mut (c.clone());
    ctx.put_now("add".to_string(), PMtoken::new_native(init_add()));
    ctx.put_now("sub".to_string(), PMtoken::new_native(init_sub()));
    ctx.put_now("mul".to_string(), PMtoken::new_native(init_mul()));
    ctx.put_now("div".to_string(), PMtoken::new_native(init_div()));


    ctx.put_now("quit".to_string(), PMtoken::new_native(init_quit()));
    ctx.put_now("q".to_string(), PMtoken::new_native(init_quit()));
    ctx.put_now("print".to_string(), PMtoken::new_native(init_print()));
    ctx.put_now("cost".to_string(), PMtoken::new_native(init_cost()));
    ctx.put_now("reduce".to_string(), PMtoken::new_native(init_reduce()));

    ctx.put_now("func".to_string(), PMtoken::new_native(init_func()));

    ctx.put_now("loop".to_string(), PMtoken::new_native(init_loop()));
    ctx.put_now("repeat".to_string(), PMtoken::new_native(init_repeat()));

    ctx.put_now("if".to_string(), PMtoken::new_native(init_if()));
    ctx.put_now("either".to_string(), PMtoken::new_native(init_either()));

    ctx.put_now("eq".to_string(), PMtoken::new_native(init_eq()));
    ctx.put_now("gt".to_string(), PMtoken::new_native(init_gt()));
    ctx.put_now("lt".to_string(), PMtoken::new_native(init_lt()));
    ctx.put_now("ge".to_string(), PMtoken::new_native(init_ge()));
    ctx.put_now("le".to_string(), PMtoken::new_native(init_le()));

}
