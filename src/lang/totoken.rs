extern crate regex;
use regex::Regex;

use crate::lang::*;


pub fn to_token(s: String, ctx: PMctx)-> PMtoken{
    let s = s.as_str().trim();
    let len = s.chars().count();
    if len == 0 {
        return PMtoken::new_nil()
    }

    let first_c = s.chars().nth(0).unwrap();
    let last_c = s.chars().last().unwrap();

    if s == "none" {
        return PMtoken::new_none()
    }

    if s == "false" {
        return PMtoken::new_bool(false)
    }

    if s == "true" {
        return PMtoken::new_bool(true)
    }

    if first_c == '$' {
        let wrap = Mwrap::new(s.to_string(), to_token(s[1..].to_string(), ctx));
        return PMtoken::new_wrap(wrap)
    }

    if last_c == '!' {
        return PMtoken::str_to_rtype(s.to_string())
    }

    if first_c == '\'' {
        return PMtoken::new_lit_word(s[1..].to_string())
    }

    let num_dot = is_num_str(s.to_string());
    if  num_dot == 0 {
        return PMtoken::new_int(s.parse().unwrap())
    }else if num_dot == 1 {
        return PMtoken::new_float(s.parse().unwrap())
    }

    if first_c == '"' {
        return PMtoken::new_str(s[1..s.len()-1].to_string())
    }

    if first_c == '[' {
        let list = to_tokens(s[1..s.len()-1].to_string(), ctx);
        return PMtoken::new_block(list);
    }

    if first_c == '(' {
        let list = to_tokens(s[1..s.len()-1].to_string(), ctx);
        return PMtoken::new_paren(list);
    }


    let mut r = Regex::new(r"^[^\(]+\[.*\]$");
    if r.unwrap().is_match(s) {
        let block_idx = s.find('[').unwrap();
        let callee = s[0..block_idx].to_string();
        let block_str = s[block_idx+1..s.len()-1].to_string();
        let block = to_tokens(block_str, ctx);
    
        return PMtoken::new_call(Mcall::new_with_block(callee, PMtoken::new_block(block)))
    }
   

    r = Regex::new(r"^[^\[]+\(.*\)\[.*\]$");
    if r.unwrap().is_match(s) {
        let paren_idx = s.find('(').unwrap();
        let callee = s[0..paren_idx].to_string();
        let paren_str = paren_cuter(&mut s[paren_idx+1..].chars());
        let block_idx = paren_idx + paren_str.as_str().len();
        let paren_str = s[paren_idx+1..block_idx-1].to_string();
        let block_str = s[block_idx+1..s.len()-1].to_string();
        let paren = to_tokens(paren_str, ctx.clone());
        let block = to_tokens(block_str, ctx);
  

        return PMtoken::new_call(Mcall::new(callee, "".to_string(), PMtoken::new_paren(paren), PMtoken::new_block(block), PMtoken::new_nil()));
    }

    r = Regex::new(r"^.+\(.*\)$");
    if r.unwrap().is_match(s) {
        let paren_idx = s.find('(').unwrap();
        let callee = s[0..paren_idx].to_string();
        let paren_str = s[paren_idx+1..s.len()-1].to_string();
        let paren = to_tokens(paren_str, ctx);
    
        return PMtoken::new_call(Mcall::new_with_paren(callee, PMtoken::new_paren(paren)))

    }

    r = Regex::new(r"^.+\(.*\):$");
    if r.unwrap().is_match(s) {
        let paren_idx = s.find('(').unwrap();
        let name = s[0..paren_idx].to_string();
        let paren_str = s[paren_idx+1..s.len()-2].to_string();
        let paren = to_tokens(paren_str, ctx);
    
        return PMtoken::new_set_func(MsetFunc::new(name, PMtoken::new_paren(paren)))

    }


    if last_c == ':' {
        return PMtoken::new_set_word(Mref::new_with_ctx(s[0..s.len()-1].to_string(), ctx.get_with_ctx(s.to_string()).1 ))
    }

    if s.ends_with(":=") {
        return PMtoken::new_put_word(s[0..s.len()-2].to_string())
    }



    PMtoken::new_word(Mref::new_with_ctx(s.to_string(), ctx.get_with_ctx(s.to_string()).1 ))
}


pub fn to_tokens(s: String, ctx: PMctx)-> Vec<PMtoken>{
    let mut result: Vec<PMtoken> = Vec::new();
    let list = token_cuter(&mut s.chars());
    for item in list {
        result.push(to_token(item, ctx.clone()))
    }
    return result
}



