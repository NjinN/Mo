use crate::lang::Mtoken::*;

use crate::*;


pub fn init_eq()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_types("int!-int!-".to_string(), Mnative::new("eq".to_string(), eq_int_int));
    result.put_by_types("int!-float!-".to_string(), Mnative::new("eq".to_string(), eq_int_float));
    result.put_by_types("float!-int!-".to_string(), Mnative::new("eq".to_string(), eq_float_int));
    result.put_by_types("float!-float!-".to_string(), Mnative::new("eq".to_string(), eq_float_float));

    return result
}


fn eq_int_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    return PMtoken::new_bool(arg0 == arg1);
}

fn eq_int_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0) as f64;
    let arg1 = raw_arg_float!(args, 1);

    return PMtoken::new_bool(arg0 == arg1);
}

fn eq_float_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_int!(args, 1) as f64;

    return PMtoken::new_bool(arg0 == arg1);
}

fn eq_float_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    return PMtoken::new_bool(arg0 == arg1);
}


pub fn init_gt()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_types("int!-int!-".to_string(), Mnative::new("gt".to_string(), gt_int_int));
    result.put_by_types("int!-float!-".to_string(), Mnative::new("gt".to_string(), gt_int_float));
    result.put_by_types("float!-int!-".to_string(), Mnative::new("gt".to_string(), gt_float_int));
    result.put_by_types("float!-float!-".to_string(), Mnative::new("gt".to_string(), gt_float_float));

    return result
}


fn gt_int_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    return PMtoken::new_bool(arg0 > arg1);
}

fn gt_int_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0) as f64;
    let arg1 = raw_arg_float!(args, 1);

    return PMtoken::new_bool(arg0 > arg1);
}

fn gt_float_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_int!(args, 1) as f64;

    return PMtoken::new_bool(arg0 > arg1);
}

fn gt_float_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    return PMtoken::new_bool(arg0 > arg1);
}



pub fn init_lt()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_types("int!-int!-".to_string(), Mnative::new("lt".to_string(), lt_int_int));
    result.put_by_types("int!-float!-".to_string(), Mnative::new("lt".to_string(), lt_int_float));
    result.put_by_types("float!-int!-".to_string(), Mnative::new("lt".to_string(), lt_float_int));
    result.put_by_types("float!-float!-".to_string(), Mnative::new("lt".to_string(), lt_float_float));

    return result
}


fn lt_int_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    return PMtoken::new_bool(arg0 < arg1);
}

fn lt_int_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0) as f64;
    let arg1 = raw_arg_float!(args, 1);

    return PMtoken::new_bool(arg0 < arg1);
}

fn lt_float_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_int!(args, 1) as f64;

    return PMtoken::new_bool(arg0 < arg1);
}

fn lt_float_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    return PMtoken::new_bool(arg0 < arg1);
}


pub fn init_ge()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_types("int!-int!-".to_string(), Mnative::new("ge".to_string(), ge_int_int));
    result.put_by_types("int!-float!-".to_string(), Mnative::new("ge".to_string(), ge_int_float));
    result.put_by_types("float!-int!-".to_string(), Mnative::new("ge".to_string(), ge_float_int));
    result.put_by_types("float!-float!-".to_string(), Mnative::new("ge".to_string(), ge_float_float));

    return result
}


fn ge_int_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    return PMtoken::new_bool(arg0 <= arg1);
}

fn ge_int_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0) as f64;
    let arg1 = raw_arg_float!(args, 1);

    return PMtoken::new_bool(arg0 <= arg1);
}

fn ge_float_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_int!(args, 1) as f64;

    return PMtoken::new_bool(arg0 <= arg1);
}

fn ge_float_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    return PMtoken::new_bool(arg0 <= arg1);
}




pub fn init_le()-> MnativeMap {
    let mut result = MnativeMap::new();

    result.put_by_types("int!-int!-".to_string(), Mnative::new("le".to_string(), le_int_int));
    result.put_by_types("int!-float!-".to_string(), Mnative::new("le".to_string(), le_int_float));
    result.put_by_types("float!-int!-".to_string(), Mnative::new("le".to_string(), le_float_int));
    result.put_by_types("float!-float!-".to_string(), Mnative::new("le".to_string(), le_float_float));

    return result
}


fn le_int_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0);
    let arg1 = raw_arg_int!(args, 1);

    return PMtoken::new_bool(arg0 >= arg1);
}

fn le_int_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_int!(args, 0) as f64;
    let arg1 = raw_arg_float!(args, 1);

    return PMtoken::new_bool(arg0 >= arg1);
}

fn le_float_int(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_int!(args, 1) as f64;

    return PMtoken::new_bool(arg0 >= arg1);
}

fn le_float_float(args: Vec<PMtoken>, ctx: PMctx)-> PMtoken{
    let arg0 = raw_arg_float!(args, 0);
    let arg1 = raw_arg_float!(args, 1);

    return PMtoken::new_bool(arg0 >= arg1);
}



