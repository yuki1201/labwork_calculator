use std::io;
extern crate libc;
use libc::{c_char,c_double};
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
    let mut ans = 0.0;
    //let inpt_string = inpt_string.ok();
    match inpt_string {
        Ok(v) => ans=main(v.to_string()),
        _ => println!("err"),
    };

    return ans;

}
fn main(mut inpt_string:String)->f32 {
    
    inpt_string=fn_preformater(inpt_string);
    let expr: Expressions = fn_parser(inpt_string);
    //expr = fn_calc_brac(expr);
    //println!("{}",fn_calc(expr));
    let ans = fn_calc(fn_calc_brac(expr));
    //println!("{}",ans);
    return ans;
}

fn fn_preformater(mut s:String) -> String{
	for _pat in 0..4 {
        s=s.replace("--","+");
		s=s.replace("--","+");
		s=s.replace("-+","+");
		s=s.replace("+-","+");
        s=s.replace("(-","(0-");
        s=s.replace("^(","^ (");
		s=s.replace("*(","* (");
		s=s.replace("+(","+ (");
		s=s.replace("-(","- (");
		s=s.replace("/(","/ (");
		s=s.replace(")^",") ^");
		s=s.replace(")-",") -");
		s=s.replace(")+",") +");
		s=s.replace(")/",") /");
		s=s.replace("))",") )");
		s=s.replace("((","( (");
	}
	match s.chars().nth(0).unwrap() {
		'(' => {
		},
		_ =>{
			s="0".to_string()+&s;
		},
	}
	return s;
}

fn fn_parser(s: String) ->Expressions{
	let mut nums_vec: Vec<f32>=vec![];
	let mut nums_string_vec: Vec<&str>= s.trim().split(['+','-','*','^','/','(',')',' '].as_ref()).collect();
	nums_string_vec.retain(|&x| x !="");
	for pat in nums_string_vec {
		nums_vec.push(str2num(pat));
	}
	//println!("{:?}",nums_vec);

	let mut sigs_vec: Vec<String>=vec![];
	let mut sigs_string_vec: Vec<&str>= s.trim().split(['1','2','3','4','5','6','7','8','9','0','.',' '].as_ref()).collect();
	sigs_string_vec.retain(|&x| x !="");
	for pat in sigs_string_vec {
		sigs_vec.push(pat.to_string());
	}
	//println!("{:?}",sigs_vec);

	Expressions {
	nums_vec: nums_vec,
	sigs_vec: sigs_vec,
	}
}

fn fn_calc_brac(mut expr: Expressions) -> Expressions {
	//println!("{:?}",expr.sigs_vec);
	//println!("{:?}",expr.nums_vec);
	let mut counter = 0;
	let mut brac_open_counter=0;
	let mut brac_expr=Expressions{
		nums_vec: vec![],
		sigs_vec: vec![],
	};
	while expr.sigs_vec.len()>counter{
		match &*expr.sigs_vec[counter] {
    	"(" => {
    		brac_open_counter=brac_open_counter+1;
    		counter=counter+1;
        	},
    	")" => {
        	expr.sigs_vec.remove(counter);
        	let mut for_counter = counter;
        	while  for_counter>0{
        		
        		println!("for_counter:{},brac_open_counter:{}", for_counter,brac_open_counter);
        		
        		match  &*expr.sigs_vec[for_counter-1]{
        			
        			"(" => {
        				
        				//expr.nums_vec.remove(for_counter-brac_open_counter);
        				expr.sigs_vec.remove(for_counter-1);
        				
        				brac_expr.nums_vec.insert(0,expr.nums_vec[for_counter-brac_open_counter]);
        				expr.nums_vec.remove(for_counter-brac_open_counter);
        				let buf = brac_expr.clone();
        				println!("brac{:?}",buf);
        				expr.nums_vec.insert(for_counter-brac_open_counter,fn_calc(buf));
        				brac_open_counter=0;
        				println!("{:?}",expr);
        				println!("chk");
        				counter=0;
        				while brac_expr.sigs_vec.len()>0{
        					brac_expr.sigs_vec.remove(0);
        				}
        				while brac_expr.nums_vec.len()>0{
        					brac_expr.nums_vec.remove(0);
        				}
        				break;
        			},
        			_ => {
        				brac_expr.nums_vec.insert(0,expr.nums_vec[for_counter-brac_open_counter]);
        				expr.nums_vec.remove(for_counter-brac_open_counter);
        				brac_expr.sigs_vec.insert(0,expr.sigs_vec[for_counter-1].to_string());
        				expr.sigs_vec.remove(for_counter-1);
        				//println!("brac_expr{:?}",brac_expr);
        			},
        			
        		}
        		for_counter=for_counter-1;
        		println!("{:?}",expr.sigs_vec);
				println!("{:?}",expr.nums_vec);
        		//for_counter=for_counter-1;
        		//println!("{:?}",for_counter);
        	}
    		},
        _ => {
        	counter=counter+1;
        	},
    	}
	}
	return expr;
}

fn fn_calc(mut expr: Expressions) -> f32 {

	let mut counter = 0;
	while expr.sigs_vec.len()-counter>0{
		match &*expr.sigs_vec[counter] {
        "^" => {
            let buf = expr.nums_vec[counter].powf(expr.nums_vec[counter+1]);
            expr.sigs_vec.remove(counter);
            expr.nums_vec.remove(counter);
            expr.nums_vec.remove(counter);
            expr.nums_vec.insert(counter,buf);
            },
            "^-" => {
            let buf = expr.nums_vec[counter].powf(-expr.nums_vec[counter+1]);
            expr.sigs_vec.remove(counter);
            expr.nums_vec.remove(counter);
            expr.nums_vec.remove(counter);
            expr.nums_vec.insert(counter,buf);
            },
        "*" => {
        	let buf = expr.nums_vec[counter]*expr.nums_vec[counter+1];
        	expr.sigs_vec.remove(counter);
        	expr.nums_vec.remove(counter);
        	expr.nums_vec.remove(counter);
        	expr.nums_vec.insert(counter,buf);
    		},
        "*-" => {
            let buf = expr.nums_vec[counter]*expr.nums_vec[counter+1]*-1.0;
            expr.sigs_vec.remove(counter);
            expr.nums_vec.remove(counter);
            expr.nums_vec.remove(counter);
            expr.nums_vec.insert(counter,buf);
            },
        "/" => {
        	let buf = expr.nums_vec[counter]/expr.nums_vec[counter+1];
        	expr.sigs_vec.remove(counter);
        	expr.nums_vec.remove(counter);
        	expr.nums_vec.remove(counter);
        	expr.nums_vec.insert(counter,buf);
    		},
        "/-" => {
            let buf = expr.nums_vec[counter]/expr.nums_vec[counter+1]*-1.0;
            expr.sigs_vec.remove(counter);
            expr.nums_vec.remove(counter);
            expr.nums_vec.remove(counter);
            expr.nums_vec.insert(counter,buf);
            },
        _ => {
        	//println!("その他");
        	counter=counter+1;
        	},
    	}	
		
	}
	let mut counter = 0;
	while expr.sigs_vec.len()-counter>0{
		match &*expr.sigs_vec[counter] {
        "+" => {
        	let buf = expr.nums_vec[counter]+expr.nums_vec[counter+1];
        	expr.sigs_vec.remove(counter);
        	expr.nums_vec.remove(counter);
        	expr.nums_vec.remove(counter);
        	expr.nums_vec.insert(counter,buf);
    		},
        "-" => {
        	let buf = expr.nums_vec[counter]-expr.nums_vec[counter+1];
        	expr.sigs_vec.remove(counter);
        	expr.nums_vec.remove(counter);
        	expr.nums_vec.remove(counter);
        	expr.nums_vec.insert(counter,buf);
    		},
        _ => {
        	//println!("その他");
        	counter=counter+1;
        	},
    	}
    }
	//println!("{:?}",exp.nums_vec);
	return expr.nums_vec[0];
}

fn str2num(s: &str) -> f32 {
	let num: f32 = s.parse().unwrap();
	return num
}
