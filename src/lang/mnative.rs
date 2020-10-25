
use std::collections::HashMap;

use crate::lang::*;

#[derive(Debug)]
pub struct Mnative {
    pub name: String,
    pub exec: fn(Vec<PMtoken>, PMctx)-> PMtoken,
}

impl Mnative {
    pub fn new(name: String, exec: fn(Vec<PMtoken>, PMctx)-> PMtoken)-> Self{
        Mnative{
            name: name,
            exec: exec,
        }
    }
    
    
    pub fn run(&self, args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
        (self.exec as fn(Vec<PMtoken>, PMctx)-> PMtoken)(args, ctx)
    }
}


#[derive(Debug)]
pub struct MnativeMap {
    pub count_map:  HashMap<u32, Mnative>,
    pub types_map:  HashMap<String, Mnative>,
}

impl MnativeMap {
    pub fn new()-> Self{
        MnativeMap{
            count_map:  HashMap::new(),
            types_map:  HashMap::new(),
        }
    }

    pub fn put_by_count(&mut self, count: u32, native: Mnative){
        self.count_map.insert(count, native);
    }

    pub fn put_by_types(&mut self, types: String, native: Mnative){
        self.types_map.insert(types, native);
    }

    pub fn get_by_count(&self, count: u32)-> Option<&Mnative>{
        self.count_map.get(&count)
    }

    pub fn get_by_types(&self, types: String)-> Option<&Mnative>{
        self.types_map.get(&types)
    }

    pub fn run(&self, args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{

        if self.types_map.len() == 0 {
            let args_count = args.len() as u32;
            let native = self.count_map.get(&args_count);
            match native {
                Some(nv) => {
                    nv.run(args, ctx)
                },
                None => PMtoken::new_err("No such native!".to_string()),
            }

        }else{
            let mut types_str = "".to_string();
 
            for item in args.iter() {
                types_str += &item.get_rtype().to_string();
                types_str += "-";
            }

            let native = self.types_map.get(&types_str);
            if let Some(ref nv) = native{
                return nv.run(args, ctx)
            }

            let args_count = args.len() as u32;
            let native = self.count_map.get(&args_count);
            match native {
                Some(nv) => {
                    nv.run(args, ctx)
                },
                None => PMtoken::new_err("No such native!".to_string()),
            }

        }
    }


}


