use std::str::Chars;

pub fn token_cuter(cs: &mut Chars)-> Vec<String>{
    let mut result: Vec<String> = Vec::new();
    let s = cs.as_str().trim();
    loop {
        let nx = cs.next();
        match nx {
            Some(c) => {
                if c.is_whitespace() {
                    continue;
                }else if c == '"' {
                    result.push(str_cuter(cs));
                }else if c == '{' {
                    result.push(obj_cuter(cs));
                }else if c == '[' {
                    result.push(block_cuter(cs));
                }else if c == '(' {
                    result.push(paren_cuter(cs));
                }else if c == '%' {
                    result.push(file_cuter(cs));
                }else if c == '$' {
                    result.push("$".to_string() +  &one_token_cuter(cs));
                }else {
                    result.push(String::from(c) + &word_cuter(cs));
                }
            },

            None => break,
        }
    }

    return result;
}

pub fn one_token_cuter(cs: &mut Chars)-> String{
    let s = cs.as_str(); 
    
    let nx = cs.next();
    match nx {
        Some(c) => {
            if c.is_whitespace() {
                "".to_string()
            }else if c == '"' {
                "\"".to_string() + &str_cuter(cs)
            }else if c == '{' {
                "{".to_string() + &obj_cuter(cs)
            }else if c == '[' {
                "[".to_string() + &block_cuter(cs)
            }else if c == '(' {
                "(".to_string() + &paren_cuter(cs)
            }else if c == '%' {
                "%".to_string() + &file_cuter(cs)
            }else {
               String::from(c) + &word_cuter(cs)
            }
        },

        None => "".to_string(),
    }
    
}

pub fn str_cuter(cs: &mut Chars)-> String{
    let s = cs.as_str(); 
    loop {
        let nx = cs.next();
        match nx {
            Some(c) => {
                if c == '^' {
                    cs.next();
                    continue;
                }else if c == '"' {
                    let now_len = cs.size_hint().1.unwrap();
                    let now_idx = s.len() - now_len - 1;
                    return s[0..now_idx].to_string();
                }
            },

            None => break,
        }
    }
    return s.to_string();
}

pub fn obj_cuter(cs: &mut Chars)-> String{
    let s = cs.as_str();
    let mut o_floor = 1;
    loop {
        let nx = cs.next();
        match nx {
            Some(c) => {
                if c == '{' {
                    o_floor += 1;
                }else if c == '}' {
                    o_floor -= 1;
                    if o_floor == 0 {
                        let now_len = cs.size_hint().1.unwrap();
                        let now_idx = s.len() - now_len;
                        return "{".to_string() + &s[0..now_idx].to_string();
                    }
                }
            },

            None => break,
        }
    }
    return "{".to_string() + &s.to_string() + &"}".to_string();
}

pub fn block_cuter(cs: &mut Chars)-> String{
    let s = cs.as_str();
    let mut b_floor = 1;
    loop {
        let nx = cs.next();
        match nx {
            Some(c) => {
                if c == '[' {
                    b_floor += 1;
                }else if c == ']' {
                    b_floor -= 1;
                    if b_floor == 0 {
                        let now_len = cs.size_hint().1.unwrap();
                        let now_idx = s.len() - now_len;
                        return "[".to_string() + &s[0..now_idx].to_string();
                    }
                }
            },

            None => break,
        }
    }
    return "[".to_string() + &s.to_string() + &"]".to_string();
}

pub fn paren_cuter(cs: &mut Chars)-> String{
    let s = cs.as_str();
    let mut p_floor = 1;
    loop {
        let nx = cs.next();
        match nx {
            Some(c) => {
                if c == '(' {
                    p_floor += 1;
                }else if c == ')' {
                    p_floor -= 1;
                    if p_floor == 0 {
                        let now_len = cs.size_hint().1.unwrap();
                        let now_idx = s.len() - now_len;
                        return "(".to_string() + &s[0..now_idx].to_string();
                    }
                }
            },

            None => break,
        }
    }
    return "(".to_string() + &s.to_string() + &")".to_string();
}

pub fn file_cuter(cs: &mut Chars)-> String{
    let s = cs.as_str();
    loop {
        let nx = cs.next();
        match nx {
            Some(c) => {
                if c == '"' {
                    str_cuter(cs);
                }else if c.is_whitespace() {
                    let now_len = cs.size_hint().1.unwrap();
                    let now_idx = s.len() - now_len - 1;
                    return  s[0..now_idx].to_string();
                }
            },

            None => break,
        }
    }
    return s.to_string();
}

pub fn path_cuter(cs: &mut Chars)-> String{
    let s = cs.as_str();
    loop {
        let nx = cs.next();
        match nx {
            Some(c) => {
                if c == '"' {
                    str_cuter(cs);
                }else if c == '(' {
                    paren_cuter(cs);
                }else if c.is_whitespace() {
                    let now_len = cs.size_hint().1.unwrap();
                    let now_idx = s.len() - now_len - 1;
                    return  s[0..now_idx].to_string();
                }
            },

            None => break,
        }
    }
    return s.to_string();
}

pub fn word_cuter(cs: &mut Chars)-> String{
    let s = cs.as_str();
    loop {
        let nx = cs.next();
        match nx {
            Some(c) => {
                if c == '/' {
                    path_cuter(cs);
                    let now_len = cs.size_hint().1.unwrap();
                    let now_idx = s.len() - now_len - 1;
                    return  s[0..now_idx].to_string();
                }else if c == '(' {
                    paren_cuter(cs);
                    let mut now_len = cs.size_hint().1.unwrap();
                    let mut now_idx = s.len() - now_len;
                    if now_idx < s.len() && &s[now_idx..now_idx+1] == "[" {
                        block_cuter(cs);
                    }else if now_idx < s.len() && &s[now_idx..now_idx+1] == ":"{
                        cs.next();
                    }
                    now_len = cs.size_hint().1.unwrap();
                    now_idx = s.len() - now_len;
                    return  s[0..now_idx].to_string();
                }else if c == '[' {
                    block_cuter(cs);
                    let now_len = cs.size_hint().1.unwrap();
                    let now_idx = s.len() - now_len;
                    return  s[0..now_idx].to_string();
                }

                if c.is_whitespace() {
                    let now_len = cs.size_hint().1.unwrap();
                    let now_idx = s.len() - now_len - 1;
                    return  s[0..now_idx].to_string();
                }
            },

            None => break,
        }
    }
    return s.to_string();
}

pub fn is_num_str(s: String)-> i32 {

    if s.chars().count() == 0 {
        return -1;
    }
    if s == "-" {
        return -1;
    }

    let mut cs = s.chars();

    let c = cs.next().unwrap();
    if !c.is_ascii_digit() && c != '-' {
        return -1;
    }

    let mut dot = 0;

    loop {
        let c = cs.next();
        match c {
            Some(x) => {
                if x == '.' {
                    dot += 1;
                }else if !x.is_ascii_digit() {
                    return -1;
                }
            },
            None => break,
        }
    }


    return dot;
} 


