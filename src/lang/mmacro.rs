
#[macro_export]
macro_rules! raw {
    ($name:expr) => {
        $name.0.read().unwrap()
    };
}


#[macro_export]
macro_rules! raw_mut {
    ($name:expr) => {
        $name.0.write().unwrap()
    };
}

#[macro_export]
macro_rules! raw_set_word {
    ($tk:expr) => {
        {
            match  &(*$tk.0.read().unwrap()) {
                Mtoken::SetWord{v} => v.clone(),
                _ => {
                    return PMtoken::new_err("Arg type mismatch".to_string());                        
                }
            }
        };
    };
}

#[macro_export]
macro_rules! raw_set_func {
    ($tk:expr) => {
        {
            match  &(*$tk.0.read().unwrap()) {
                Mtoken::SetFunc{v} => v.clone(),
                _ => {
                    return PMtoken::new_err("Arg type mismatch".to_string());                        
                }
            }
        };
    };
}




#[macro_export]
macro_rules! raw_read_block {
    ($tk:expr) => {
        {
            match &*$tk.0.read().unwrap() {
                Mtoken::Block{v} => copy_token_list(v),
                _ => return PMtoken::new_err("Arg type mismatch".to_string()) 
            }
        }
    }
}

#[macro_export]
macro_rules! raw_paren {
    ($tk:expr) => {
        {
            match &*$tk.0.read().unwrap() {
                Mtoken::Paren{v} => copy_token_list(v),
                _ => Vec::new()
            }
        }
    }
}

#[macro_export]
macro_rules! raw_read_paren {
    ($tk:expr) => {
        {
            match &*$tk.0.read().unwrap() {
                Mtoken::Paren{v} => copy_token_list(v),
                _ => return PMtoken::new_err("Arg type mismatch".to_string()) 
            }
        }
    }
}



#[macro_export]
macro_rules! raw_block {
    ($tk:expr) => {
        {
            match  &mut (*$tk.0.write().unwrap()) {
                Mtoken::Block{v} => copy_token_list(v),
                _ => {
                    return PMtoken::new_err("Arg type mismatch".to_string());                        
                }
            }
        };
    };
}

#[macro_export]
macro_rules! raw_read_int {
    ($tk:expr) => {
        {
            match  &mut (*$tk.0.read().unwrap()) {
                Mtoken::Int{v} => v.clone(),
                _ => {
                    return PMtoken::new_err("Arg type mismatch".to_string());                        
                }
            }
        };
    };
}

#[macro_export]
macro_rules! raw_int {
    ($tk:expr) => {
        {
            match  &mut (*$tk.0.write().unwrap()) {
                Mtoken::Int{v} => v.clone(),
                _ => {
                    return PMtoken::new_err("Arg type mismatch".to_string());                        
                }
            }
        };
    };
}



#[macro_export]
macro_rules! raw_arg_int {
    ($args:expr, $idx:expr) => {
        {
            if let Some(ref p) = $args.get($idx){
                match  &mut (*p.0.write().unwrap()) {
                    Mtoken::Int{v} => v.clone(),
                    _ => {
                        return PMtoken::new_err("Arg type mismatch".to_string());                        
                    }
                }
            }else{
                return PMtoken::new_err("Arg type mismatch".to_string());
            }
        };
    };
}

#[macro_export]
macro_rules! raw_arg_float {
    ($args:expr, $idx:expr) => {
        {
            if let Some(ref p) = $args.get($idx){
                match  &mut (*p.0.write().unwrap()) {
                    Mtoken::Float{v} => v.clone(),
                    _ => {
                        return PMtoken::new_err("Arg type mismatch".to_string());                        
                    }
                }
            }else{
                return PMtoken::new_err("Arg type mismatch".to_string());
            }
        };
    };
}

#[macro_export]
macro_rules! raw_arg_str {
    ($args:expr, $idx:expr) => {
        {
            if let Some(ref p) = $args.get($idx){
                match  &mut (*p.0.read().unwrap()) {
                    Mtoken::Str{v} => v.clone(),
                    _ => {
                        return PMtoken::new_err("Arg type mismatch".to_string());                        
                    }
                }
            }else{
                return PMtoken::new_err("Arg type mismatch".to_string());
            }
        };
    };
}

#[macro_export]
macro_rules! raw_arg_bool {
    ($args:expr, $idx:expr) => {
        {
            if let Some(ref p) = $args.get($idx){
                match  &mut (*p.0.write().unwrap()) {
                    Mtoken::Bool{v} => v.clone(),
                    _ => {
                        return PMtoken::new_err("Arg type mismatch".to_string());                        
                    }
                }
            }else{
                return PMtoken::new_err("Arg type mismatch".to_string());
            }
        };
    };
}

#[macro_export]
macro_rules! raw_arg_block {
    ($args:expr, $idx:expr) => {
        {
            if let Some(ref p) = $args.get($idx){
                match  &mut (*p.0.write().unwrap()) {
                    Mtoken::Block{v} => copy_token_list(v),
                    _ => {
                        return PMtoken::new_err("Arg type mismatch".to_string());                        
                    }
                }
            }else{
                return PMtoken::new_err("Arg type mismatch".to_string());
            }
        };
    };
}

#[macro_export]
macro_rules! raw_arg_paren {
    ($args:expr, $idx:expr) => {
        {
            if let Some(ref p) = $args.get($idx){
                match  &mut (*p.0.read().unwrap()) {
                    Mtoken::Paren{v} => copy_token_list(v),
                    _ => {
                        return PMtoken::new_err("Arg type mismatch".to_string());                        
                    }
                }
            }else{
                return PMtoken::new_err("Arg type mismatch".to_string());
            }
        };
    };
}


#[macro_export]
macro_rules! raw_arg_call {
    ($args:expr, $idx:expr) => {
        {
            if let Some(ref p) = $args.get($idx){
                match  &mut *(p.0.write().unwrap()) {
                    Mtoken::Call{v} => v.clone(),
                    _ => {
                        return PMtoken::new_err("Arg type mismatch".to_string());                        
                    }
                }
            }else{
                return PMtoken::new_err("Arg type mismatch".to_string());
            }
        };
    };
}

#[macro_export]
macro_rules! raw_arg_word {
    ($args:expr, $idx:expr) => {
        {
            if let Some(ref p) = $args.get($idx){
                match  & *(p.0.read().unwrap()) {
                    Mtoken::Word{v} => v.clone(),
                    _ => {
                        return PMtoken::new_err("Arg type mismatch".to_string());                        
                    }
                }
            }else{
                return PMtoken::new_err("Arg type mismatch".to_string());
            }
        };
    };
}


#[macro_export]
macro_rules! raw_arg_set_word {
    ($args:expr, $idx:expr) => {
        {
            if let Some(ref p) = $args.get($idx){
                match  & *(p.0.read().unwrap()) {
                    Mtoken::SetWord{v} => v.clone(),
                    _ => {
                        return PMtoken::new_err("Arg type mismatch".to_string());                        
                    }
                }
            }else{
                return PMtoken::new_err("Arg type mismatch".to_string());
            }
        };
    };
}

#[macro_export]
macro_rules! raw_arg_datatype {
    ($args:expr, $idx:expr) => {
        {
            if let Some(ref p) = $args.get($idx){
                match  & *(p.0.read().unwrap()) {
                    Mtoken::Datatype{v} => v.clone(),
                    _ => {
                        return PMtoken::new_err("Arg type mismatch".to_string());                        
                    }
                }
            }else{
                return PMtoken::new_err("Arg type mismatch".to_string());
            }
        };
    };
}












