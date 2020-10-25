
mod lang;
use lang::*;

mod nativelib;
use nativelib::*;

mod oplib;
use oplib::*;

use std::io;
use std::io::Write;

fn main(){
    let ctx = PMctx::new(MctxType::UsrCtx);

    init_native(ctx.clone());
    init_op(ctx.clone());

    loop {
        print!("{}", ">> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf);

        let result = Msolver::solver_eval_str(buf, ctx.clone());
        if !result.is_nil(){
            result.echo();
        }

        println!("{}", "");
        
        
    }

}



// fn main() {
//     println!("Hello, world!");
//     println!("{:?}", token_cuter(&mut "123 \"456^\"789\" 0 f(1 2): ".chars()));
//     println!("{:?}", PMtoken::new_err("hello".to_string()));

//     let mut ctx = PMctx::new(MctxType::SysCtx);

//     let code = to_tokens("i: 321".to_string(), ctx.clone());
//     let mut solver = Msolver::new(code);
//     let result = solver.eval_blk(ctx.clone());
//     println!("{:?}", result);
//     println!("{:?}", Msolver::solver_reduce_str("i: 456 321 123".to_string(), ctx.clone()));
// }
