use std::io::{self, Write};
fn main() {
  print!("{}[2J", 27 as char);
  println!("=== [ Calculator started. Start evaluating math operation ] ===");    

loop{
    let input = input(">");
    if input == "clear" {
      print!("{}[2J", 27 as char);
      println!("Terminal is clear now. Give me expression to evaluate.");    
      print!("\r");
      continue
    }
    if input == "exit" {
      print!("{}[2J", 27 as char);
      break
    }

    match input.chars().next(){
      Some(x) => {
        // if input starts with 0 => exit kro
        if x == '0' {
          println!("Exiting!");
          break;
        }
      },
      None =>{
        // agar khali enter kia he user ne to bh continue kro
        println!("Couldn't calculate expression.Try again!");
        continue;
      }
    }

    let char_vec: Vec<char> = input.trim().chars().collect();
    let validated_vec = validate_values(char_vec);
    let len_constraints:usize=String::from("2").parse().unwrap();
    let mut is_print:bool = true;
    if !(&validated_vec.len() > &len_constraints) {
          is_print = false;          
    }

      let mut operation = Operation::new();
      for c in validated_vec {
        match c.as_ref() {
          "+" | "-" | "/" | "*" |  "^"  => {
              operation.operator=Some(c);
          },
          _ => {
              if let None = operation.operand1 {
                  operation.operand1 = Some(c.parse().unwrap());
              }
              else {
                  operation.operand2 = Some(c.parse().unwrap());
              }
          },
      };
    }// end for loop

    if is_print {
       print_result(operation);
    } else {
        println!("Couldn't calculate expression.Try again!");
    }
  }
}

#[derive(Debug)]
struct Operation{
  operand1: Option<f64>,
  operator: Option<String>,
  operand2: Option<f64>,
}
impl Operation {
  fn new()->Operation{
    Operation{
        operand1: None,
        operator: None,
        operand2: None,
      }
  }
}

fn print_result(operation:Operation){
      let number1: f64 = operation.operand1.unwrap();
      let number2: f64 = operation.operand2.unwrap();
      match Some(operation.operator) {
        Some(ref s) if s == &Some("+".to_string()) => {
          println!("{:?}",number1+number2);
        },
        Some(ref s) if s == &Some("-".to_string()) => {
          println!("{:?}",number1-number2);
        },
        Some(ref s) if s == &Some("/".to_string()) => {
          println!("{:?}",number1/number2);
        },
        Some(ref s) if s == &Some("*".to_string()) => {
          println!("{:?}",number1*number2);
        },
        Some(ref s) if s == &Some("^".to_string()) => {
          let result:f64=power(number1 as i32,number2 as i32) as f64;
          println!("{:?}",result);
        },
        _ => println!("Invalid Operator used !"),
      }
}



fn validate_values(char_vec:Vec<char>)->Vec<String>{
    let mut validated_vec = Vec::new();
    validated_vec.push(String::from(""));

    let mut index=0;
    for c in char_vec {
        match c {
          '+' | '-' | '*' | '/' | '^' => {
              validated_vec.push(String::from(&c.to_string()[..]));
              validated_vec.push(String::from(""));
              index =index+2;
            },
          ' ' => {
            },          
          '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0'
            => {
            let value = &mut validated_vec[index];
            value.push_str(&c.to_string()[..]);
            validated_vec[index] = value.to_string();
            },
          _ => {
              //println!("Try again with correct operadns and operator");
              return Vec::new()
            },
        }
    }
validated_vec
}

#[doc="created function to reuse and reduce LOC for input."]
#[allow(dead_code)]
pub fn input(message:&str)->String{
print!("{}",message);
let mut input = String::new();
let _ = io::stdout().flush();
std::io::stdin().read_line(&mut input).ok().expect("Couldn't read line");
input.pop();
input
}

fn power(x:i32, y:i32)->i32{
    if y == 0 {
      1
    }
    else if y % 2 == 0 {
        (power(x, y / 2) * power(x, y / 2))
    }
    else {
        (x * power(x, y / 2) * power(x, y / 2)) 
    }    
}
