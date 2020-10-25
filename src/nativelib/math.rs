use crate::lang::Mtoken::*;

use crate::*;

pub fn init_add()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_types("int!-int!-".to_string(), Mnative::new("add".to_string(), add_int_int));
    result.put_by_types("int!-float!-".to_string(), Mnative::new("add".to_string(), add_int_float));
    result.put_by_types("float!-int!-".to_string(), Mnative::new("add".to_string(), add_float_int));
    result.put_by_types("float!-float!-".to_string(), Mnative::new("add".to_string(), add_float_float));

    return result
}


fn add_int_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    let sum = arg0 + arg1;
    return PMtoken::new_int(sum);
}

fn add_int_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    let sum = (arg0 as f64) + arg1;
    return PMtoken::new_float(sum);
}

fn add_float_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    let sum = arg0 + (arg1 as f64);
    return PMtoken::new_float(sum);
}

fn add_float_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    let sum = arg0 + arg1;
    return PMtoken::new_float(sum);
}


pub fn init_sub()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_types("int!-int!-".to_string(), Mnative::new("sub".to_string(), sub_int_int));
    result.put_by_types("int!-float!-".to_string(), Mnative::new("sub".to_string(), sub_int_float));
    result.put_by_types("float!-int!-".to_string(), Mnative::new("sub".to_string(), sub_float_int));
    result.put_by_types("float!-float!-".to_string(), Mnative::new("sub".to_string(), sub_float_float));

    return result
}


fn sub_int_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    let sum = arg0 - arg1;
    return PMtoken::new_int(sum);
}

fn sub_int_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    let sum = (arg0 as f64) - arg1;
    return PMtoken::new_float(sum);
}

fn sub_float_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    let sum = arg0 - (arg1 as f64);
    return PMtoken::new_float(sum);
}

fn sub_float_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    let sum = arg0 - arg1;
    return PMtoken::new_float(sum);
}


pub fn init_mul()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_types("int!-int!-".to_string(), Mnative::new("mul".to_string(), mul_int_int));
    result.put_by_types("int!-float!-".to_string(), Mnative::new("mul".to_string(), mul_int_float));
    result.put_by_types("float!-int!-".to_string(), Mnative::new("mul".to_string(), mul_float_int));
    result.put_by_types("float!-float!-".to_string(), Mnative::new("mul".to_string(), mul_float_float));

    return result
}


fn mul_int_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    let sum = arg0 * arg1;
    return PMtoken::new_int(sum);
}

fn mul_int_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    let sum = (arg0 as f64) * arg1;
    return PMtoken::new_float(sum);
}

fn mul_float_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    let sum = arg0 * (arg1 as f64);
    return PMtoken::new_float(sum);
}

fn mul_float_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    let sum = arg0 * arg1;
    return PMtoken::new_float(sum);
}


pub fn init_div()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_types("int!-int!-".to_string(), Mnative::new("div".to_string(), div_int_int));
    result.put_by_types("int!-float!-".to_string(), Mnative::new("div".to_string(), div_int_float));
    result.put_by_types("float!-int!-".to_string(), Mnative::new("div".to_string(), div_float_int));
    result.put_by_types("float!-float!-".to_string(), Mnative::new("div".to_string(), div_float_float));

    return result
}


fn div_int_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    let sum = (arg0 as f64) / (arg1 as f64);
    return PMtoken::new_float(sum);
}

fn div_int_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    let sum = (arg0 as f64) / (arg1 as f64);
    return PMtoken::new_float(sum);
}

fn div_float_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    let sum = (arg0 as f64) / (arg1 as f64);
    return PMtoken::new_float(sum);
}

fn div_float_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    let sum = arg0 /  arg1;
    return PMtoken::new_float(sum);
}