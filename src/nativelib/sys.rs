use crate::lang::*;

use crate::*;

use std::process::exit;
use std::io;
use std::io::Write;
use std::time::{Duration, SystemTime};

pub fn init_quit()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_count(0, Mnative::new("quit".to_string(), quit));
    result.put_by_count(1, Mnative::new("quit".to_string(), quit_with_code));

    return result
}

fn quit(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    exit(0);
}

fn quit_with_code(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    exit(arg0 as i32);
}


pub fn init_print()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_count(1, Mnative::new("print".to_string(), println));
    result.put_by_count(2, Mnative::new("print".to_string(), print));

    return result
}

fn println(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    args.get(0).unwrap().echo();
    PMtoken::new_nil()
}

fn print(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    if !args.get(1).unwrap().is_bool(){
        return PMtoken::new_err("Args type mismatch".to_string())
    }

    let arg1 = raw_arg_bool!(args, 1);

    let str = args.get(0).unwrap().to_string();
    if arg1 {
        println!("{}", str);
    }else{
        print!("{}", str);
        io::stdout().flush().unwrap();
    }

    PMtoken::new_nil()
}


pub fn init_cost()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_count(1, Mnative::new("cost".to_string(), cost));

    return result
}

fn cost(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    if !args.get(0).unwrap().is_block(){
        return PMtoken::new_err("Args type mismatch".to_string())
    }

    let mut arg0 = raw_arg_block!(args, 0);

    let now = SystemTime::now();
    bind_ctx(&mut arg0, ctx.clone());
    let temp = Msolver::solver_eval(arg0, ctx.clone());
    if temp.is_err(){
        return temp
    }

    match now.elapsed(){
        Ok(elapsed) => {
            PMtoken::new_float(elapsed.as_secs_f64())
        },
        Err(e) => {
            PMtoken::new_err("cost fail".to_string())
        }
    }

}


pub fn init_reduce()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_count(1, Mnative::new("reduce".to_string(), reduce));

    return result
}

fn reduce(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    if !args.get(0).unwrap().is_block(){
        return PMtoken::new_err("Args type mismatch".to_string())
    }
    
    let mut arg0 = raw_arg_block!(args, 0);
    
    bind_ctx(&mut arg0, ctx.clone());
    PMtoken::new_block(Msolver::solver_reduce(arg0, ctx.clone()))


}