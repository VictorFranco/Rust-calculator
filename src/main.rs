mod stack;
use stack::Stack as Stack;
fn main(){
    println!("Calculator");
    let mut stack:Stack=Stack::create_stack();
    let instructions=["1","2","+"];
    let mut num1:f64=0.0;
    let mut num2:f64=0.0;    
    for instruction in instructions.iter(){    
        let expression=*instruction;
        if expression=="+"||expression=="-"||expression=="*"||expression=="/" {
            Stack::pop(&mut stack,&mut num1);
            Stack::pop(&mut stack,&mut num2);
            let result=match &*expression{
                "+"=>num1+num2,
                "-"=>num1-num2,
                "*"=>num1*num2,
                "/"=>num1/num2,
                _=>0.0
            };
            Stack::push(&mut stack,result);
        }else{
            let number:f64=expression.parse::<f64>().unwrap();
            Stack::push(&mut stack,number);
        }
    }
    let mut math_expre=String::new();
    for instruction in instructions.iter(){
        math_expre.push_str(instruction);
        math_expre.push_str(" ");
    }
    println!(">>> {}",math_expre);
    println!("{:?}",stack.array[(stack.sp+1) as usize]);
}