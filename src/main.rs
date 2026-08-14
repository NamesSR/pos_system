#![allow(non_snake_case)]
#![allow(nonstandard_style)]


use std::{array, collections::HashMap, io, process::Command};

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
   let commands = ["help","mklb","shlbl","chcd","chch","chaddp"];
   for i in 0..2 {
       
      let sd = ReadCommand();
        match  sd.as_str() {
                "help" => help(commands),
                "mklb" => println!("mklb command"),
                "shlbl" => println!("shlbl command"),
                "chcd" => println!("chcd command"),
                "chch" => println!("chch command"),
                "chaddp" => println!("chaddp command"),
                _ => println!("Unknown command:"),
                
            }
   }
    
   
    let mut LableList: HashMap<u32,Lables> = HashMap::new();
    LableList.insert(0001, Lables { Productnum: (0001), PriceIncBTW: (9.99), PriceExuBTW: (9.99), Name: (String::from("Cable")), Btw: (BTWType::none) });
   match LableList.get(&0001) {
    Some(lable) => println!("{:?}", lable),
    None => println!("there are no lable's under that Productnum")
       
   }
}


fn ReadCommand() -> String
{
 let mut args: Vec<&str> = Vec::new();
 let mut input = String::new();
    io::stdin()
        .read_line(&mut input)  // 2. Pass buffer as a mutable reference
        .expect("Failed to read line");

    let mut cmd = input.trim();
    let mut i = 1;
    for token in cmd.split_whitespace() {
        if i < 3{
            
        println!("token {} ", token);
        args.push(token);
        i+=1;

    }
    
    }
   
    if args.len() > 1 {
        println!("{} {}", args[0], args[1]);
        if args[0] == "pos"{
           return args[1].to_string();
        }else {
             println!("no command found type pos first");
        }
    } else {
        
          
            println!("No argument provided.");
            return String::from(" ");
        
    }
    args.clear();
    //let mut args: Vec<String> = Vec::new();
    let mut i = 1;

    return String::from(" ");
}


fn help(command: [&str; 6]) {
    for s in command {
        println!("{}", s);
    }
}