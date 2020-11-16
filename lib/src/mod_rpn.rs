pub mod mod_rpn {
    pub fn rpn(exp: &str) -> f32 {
        let mut stack= Vec::new();
        for token in exp.split_whitespace() {
            if let Ok(num) = token.parse::<f32>(){
                stack.push(num)
            }
            else {
                match token {
                    "+" => apply2(&mut stack,|x, y| x + y),
                    "-" => apply2(&mut stack,|x, y| x - y),
                    "*" => apply2(&mut stack,|x, y| x * y),
                    "/" => apply2(&mut stack,|x, y| x / y),
                    _ => panic!("Unknown Operator:{}",token),
                }
            }
        }
        return stack.pop().expect("stack underflow");
    }

    pub fn apply2<F:Fn(f32,f32)->f32>(stack :&mut Vec<f32>,fun:F){
        if let (Some(y),Some(x)) = (stack.pop(),stack.pop()) {
            stack.push(fun(x,y));
        }
        else {
            panic!("Stack underflow");
        }
}
}