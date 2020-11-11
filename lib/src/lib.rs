extern crate libc;
use libc::c_char;
use std::ffi::CStr;

#[derive(Clone, Debug)]
struct Expressions {
	nums_vec: Vec<f32>,
	sigs_vec: Vec<String>,
}
#[no_mangle]

pub extern fn rust_fn(name: *const c_char) -> f32 {
    let hoge = unsafe{CStr::from_ptr(name)};
    let inpt_string=hoge.to_str();
    let mut ans=0.0;
    match inpt_string {
        Ok(v) => {
            let mut inpt_string: Vec<char> = v.to_string().chars().collect();
            ans=fn_expr(&mut inpt_string);
            println!("{:?}",ans);
        },
        _ => println!("err"),
    };

    return ans;

}
fn fn_number(inpt_string:&mut Vec<char>)->f32 {

    let mut return_str = String::new();
    println!("{:?}",inpt_string);
    loop {
        if inpt_string.len()>0&&(inpt_string[0].is_digit(10)||inpt_string[0] == '.'){
            return_str.push(inpt_string[0]);
            inpt_string.remove(0);
        }
        else{
            return return_str.parse::<f32>().unwrap();
        }
    }
}


fn fn_expr(inpt_string:&mut Vec<char>) -> f32 {
    let mut num = fn_number(inpt_string);
    while inpt_string.len()>0{
        match inpt_string[0] {
            '+' => {
                inpt_string.remove(0);
                num=num+fn_number(inpt_string);
            },
            '-' => {
                inpt_string.remove(0);
                num=num-fn_number(inpt_string);
            },
            _ => return num,
        }

    }
    return num
}