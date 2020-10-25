use std::collections::HashSet;

use crate::*;

use crate::lang::*;

#[derive(Debug)]
pub struct MsetFunc {
    pub name:   String,
    pub args:   PMtoken,
}


impl MsetFunc {
    pub fn new(name: String, args: PMtoken)-> MsetFunc{
        MsetFunc{name: name, args: args}
    }

    pub fn clone(&self)-> MsetFunc{
        MsetFunc{
            name: self.name.clone(),
            args: self.args.clone(),
        }
    }


    pub fn dim_func(&self, blk: PMtoken, ctx: PMctx)->PMtoken{
        
        if !blk.is_block(){
            return PMtoken::new_err("Args type mismatch".to_string())
        }
    
        let mut arg1 = raw_block!(blk);
    
        // println!("{}", arg0.aim.clone());
        let mut func = ctx.get(self.name.clone());
        // func.echo();
        if func.is_nil() {
            let func_map = MfuncMap::new();
            func = PMtoken::new_func(func_map);
            ctx.clone().put_now(self.name.clone(), func.clone());
        }
    
    
    
        let arg_list = raw_read_paren!(self.args);
        if is_all_word(&arg_list){
            let mut local_set: HashSet<String> = HashSet::new();
            let mut i = 0;
            while i < arg_list.len() {
                local_set.insert(raw_arg_word!(arg_list, i).k.clone());
                i += 1;
            }
    
            bind_ctx_local(&mut arg1, ctx.clone(), local_set);
    
            let rfunc = Mfunc::new_one(copy_token_list(&arg_list), copy_token_list(&arg1));
    
            match &mut *(func.0.write().unwrap()) {
                Mtoken::Func{v} => {
                    v.count_map.insert(arg_list.len() as u32, rfunc);
                    return func.clone()
                },
    
                _ => {
                    return PMtoken::new_err("No such func!".to_string())
                }
            }
    
    
        }else if is_word_pair(&arg_list){
            let mut format_arg_list: Vec<PMtoken> = Vec::new();
            let mut local_set: HashSet<String> = HashSet::new();
            let mut types_str = "".to_string();
            let mut i = 0;
            while i < arg_list.len() {
                let key_name = raw_arg_set_word!(arg_list, i).k.clone();
                local_set.insert(key_name.clone());
                format_arg_list.push(PMtoken::new_word(Mref::new(key_name.clone())));
                types_str += &raw_arg_datatype!(arg_list, i + 1).to_string();
                types_str += "-";
                i += 2;
            }
    
            bind_ctx_local(&mut arg1, ctx.clone(), local_set);
    
            let rfunc = Mfunc::new_one(copy_token_list(&format_arg_list), copy_token_list(&arg1));
    
            match &mut *(func.0.write().unwrap()) {
                Mtoken::Func{v} => {
                    v.types_map.insert(types_str, rfunc);
                    return func.clone()
                },
    
                _ => {
                    return PMtoken::new_err("No such func!".to_string())
                }
            }
    
        }
        
        
        return PMtoken::new_err("Args type mismatch".to_string())
        
    }

}

fn is_all_word(args: &Vec<PMtoken>)-> bool{
    for item in args.iter() {
        if !item.is_word(){
            return false
        }
    }
    return true
}

fn is_word_pair(args: &Vec<PMtoken>)-> bool{
    if args.len() % 2 != 0 {
        return false
    }

    let mut i = 0;
    while i < args.len() {
        if !args.get(i).unwrap().is_set_word() || !args.get(i+1).unwrap().is_datatype(){
            return false
        }
        i += 2;
    }

    return true
}

