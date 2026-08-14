#![allow(non_snake_case)]

use std::{collections::{HashMap, hash_map}, io};
use std::env;
 #[derive(Debug)]
enum BTWType {
    
    none,
    low,
    high
    
    
}
#[derive(Debug)]
struct Lables{

    Productnum: u32,
    PriceIncBTW: f32,
    PriceExuBTW: f32,
    Name: String,
    Btw: BTWType
   
}

fn main() {
   
   
    readLine();
//     let mut LableList: HashMap<u32,Lables> = HashMap::new();
//     LableList.insert(0001, Lables { Productnum: (0001), PriceIncBTW: (9.99), PriceExuBTW: (9.99), Name: (String::from("Cable")), Btw: (BTWType::none) });
//    match LableList.get(&0001) {
//     Some(lable) => println!("{:?}", lable),
//     None => println!("there are no lable's under that Productnum")
       
//    }
}

fn readLine()
{
 let mut args: Vec<String> = Vec::new();
  args.push(" ".to_string());
 let mut input = String::new();
    io::stdin()
        .read_line(&mut input)  // 2. Pass buffer as a mutable reference
        .expect("Failed to read line");

    let mut cmd = input.trim().to_string();
    let mut i = 1;
    for token in cmd.split_whitespace() {
        if i < 3{
            
        println!("token {} ", token);
        args.push(token.to_string());
        i+=1;

    }
    
    }
   
    if args.len() > 2 {
        println!("{} {}", args[1], args[2]);
    } else {
         println!("{}", args[1]);
            println!("No argument provided.");
        
    }
    args.clear();
    let mut i = 1;
    
}