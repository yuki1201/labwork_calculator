extern crate libc;
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]

pub extern fn rust_fn(name: *const c_char,tf:bool) -> f32 {
    let hoge = unsafe{CStr::from_ptr(name)};
    let inpt_string=hoge.to_str();
    let mut ans=0.0;
    match inpt_string {
        Ok(v) => {
            let mut inpt_string: Vec<char> = v.to_string().chars().collect();
            if tf {
                ans=fn_expr(&mut inpt_string);
            }
        },
        _ => println!("err"),
    };

    return ans;

}
fn fn_number(inpt_string:&mut Vec<char>)->f32 {

    let mut return_str = String::new();
    while  inpt_string.len()>0&&((inpt_string[0].is_digit(10)||inpt_string[0] == '.')||(return_str.len()==0&&inpt_string[0] == '-')){
        return_str.push(inpt_string[0]);
        inpt_string.remove(0);
    }
    println!("{:?}",return_str);
    return return_str.parse::<f32>().unwrap();
}



fn fn_expr(inpt_string:&mut Vec<char>) -> f32 {
    let mut num = fn_term(inpt_string);
    while inpt_string.len()>0{
        match inpt_string[0] {
            '+' => {
                inpt_string.remove(0);
                num=num+fn_term(inpt_string);
            },
            '-' => {
                inpt_string.remove(0);
                num=num-fn_term(inpt_string);
            },
            _ => return num,
        }

    }
    return num
}

fn fn_term(inpt_string:&mut Vec<char>) -> f32 {
    let mut num = fn_factor(inpt_string);
    while inpt_string.len()>0{
        match inpt_string[0] {
            '*' => {
                inpt_string.remove(0);
                num=num*fn_factor(inpt_string);
            },
            '/' => {
                inpt_string.remove(0);
                num=num/fn_factor(inpt_string);
            },
            _ => return num,
        }

    }
    return num
}

fn fn_factor(inpt_string:&mut Vec<char>) -> f32 {
    if inpt_string[0]=='(' {
        inpt_string.remove(0);
        let num = fn_expr(inpt_string);
        if inpt_string[0]==')' {
            inpt_string.remove(0);
        }
        return num;
    }
    return fn_number(inpt_string);
}