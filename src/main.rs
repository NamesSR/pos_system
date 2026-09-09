#![allow(non_snake_case)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]


use std::{array, collections::HashMap, io, process::Command};
use typed_money::{Amount, USD, EUR, BTC, ETH, LTC, ADA, USDT, USDC, CAD, CNY, THB, NGN, Rate, RoundingMode};
use rusqlite::{params, Connection, Result};
 #[derive(Debug, Clone, Copy)]
pub enum BTWType {
    
    None,
    Low,
    High
    
    
}
impl BTWType {
     pub fn as_str(&self) -> &'static str {
        match self {
            BTWType::High => "high",
            BTWType::Low => "low",
            BTWType::None => "none",
        }
    }
}
#[derive(Debug)]
struct Lables{

    Productnum: u32,
    PriceIncBTW: Amount<EUR>,
    PriceExuBTW: Amount<EUR>,
    Name: String,
    Btw: BTWType
   
}

fn main() {
  
    for  i in 1..4 {
        
        ReadCommand();
    }
 
}


fn ReadCommand()
{
    let commands = ["help","mklb","shlbl","cocs","cocd","addp"];
 let mut args: Vec<&str> = Vec::new();
 
    let cmd = readlineString();
    //let mut i = 1;
    for token in cmd.split_whitespace() {
        
            
        println!("token {} ", token);
        args.push(token);
       // i+=1;

    
    
    }
   
    if args.len() > 1 {
        println!("{} {}", args[0], args[1]);
        if args.len() > 2{
            if args[0] == "pos"{
                 match  args[1] {
                    "help"  => print!("to many arguments"),
                    "mklb"  => println!("to many arguments"),
                    "shlbl" => print!("to many arguments"),
                    "cocs"  => println!("cocs command"),
                    "cocd"  => println!("cocd command"),
                    "addp"  => println!("addp command"),
                    _       => println!("Unknown command:"),
                
                }
            }
        }else {
            if args[0] == "pos"{
                 match  args[1] {
                    "help"  => help(commands),
                    "mklb"  => MakeLable().expect("Failed to make label"),
                    "shlbl" => Showlbl().expect("there are no lables in the DataBase"),
                    "cocs"  => println!("cocs command"),
                    "cocd"  => println!("cocd command"),
                    "addp"  => println!("addp command"),
                    _       => println!("Unknown command:"),
                
                }
            } else {
             println!("no command found type pos first");
            }
        } 
      args.clear();
      //let mut args: Vec<String> = Vec::new();
    

    
    }else {
            println!("no command found");
        }
}

fn help(command: [&str; 6]) {
    for s in command {
        println!("{}", s);
    }
}
fn readlineString() ->String{
let mut input = String::new();

     io::stdin()
        .read_line(&mut input)  // 2. Pass buffer as a mutable reference
        .expect("Failed to read line");
    return input.trim().to_string(); 
}

fn readlineint32()-> i32{
   
     let num_value: i32 = match readlineString().parse::<i32>() 
       {
          Ok(n) => n,
          Err(e) => 
           {
              println!("{e}");
              return 0;
            }
        };
  
     return num_value;
}
fn readlineint64()-> i64{
   
     let num_value: i64 = match readlineString().parse::<i64>() 
       {
          Ok(n) => n,
          Err(e) => 
           {
              println!("{e}");
              return 0;
            }
        };
  
     return num_value;
}fn Showlbl() -> Result<()>{
    
    // create a new in-memory database
    //let conn = Connection::open_in_memory()?;
    
    
   
    // create a new database file
    let conn = Connection::open("pos_labels.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS labels (
            productnum INTEGER PRIMARY KEY,
            priceincbtw INTEGER NOT NULL,
            priceexbtw INTEGER NOT NULL,
            name TEXT NOT NULL,
            btw TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;
  
    
    

    // create a new database file
    

        let mut stmt = conn.prepare("SELECT productnum, priceincbtw, priceexbtw, name, btw FROM labels")?;
        // load lables from the database based on product number
       // let mut stmt = conn.prepare(
       //     "SELECT product_num, price_inc_btw_cents, price_ex_btw_cents, name, btw 
       //      FROM labels WHERE product_num = ?0"
       // )?;
        let lables_iter = stmt.query_map([], |row| {
            Ok(Lables {
                Productnum: row.get(0)?,
                PriceIncBTW: Amount::<EUR>::from_minor(row.get(1)?),
                PriceExuBTW: Amount::<EUR>::from_minor(row.get(2)?),
                Name: row.get(3)?,
                Btw: match row.get::<_, String>(4)?.as_str() {
                    "high" => BTWType::High,
                    "low" => BTWType::Low,
                    "none" => BTWType::None,
                    _ => BTWType::None, // Default case
                },
            })
        })?;
    
    
       let mut found_any = false;
        // show list of lables
        for Lables in lables_iter {
            if let Ok(found_lables) = Lables{
                found_any = true;
                println!("Found lable {:?}", found_lables);
            }
            
        }
        if !found_any{
            println!("there are no labels in the database")
        }

    Ok(())
    
}
fn AddProductToCheckOut() -> Result<()>{
     // create a new in-memory database
    //let conn = Connection::open_in_memory()?;


    // create a new database file
      let conn = Connection::open("pos_labels.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS labels (
            productnum INTEGER PRIMARY KEY,
            priceincbtw INTEGER NOT NULL,
            priceexbtw INTEGER NOT NULL,
            name TEXT NOT NULL,
            btw TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;
  
    
   // load lables from the database based on product number
       let mut stmt = conn.prepare(
           "SELECT product_num, price_inc_btw_cents, price_ex_btw_cents, name, btw 
            FROM labels WHERE product_num = ?0"
       )?; 
        
        let mut lables_iter = stmt.query_map([], |row| {
            Ok(Lables {
                Productnum: row.get(0)?,
                PriceIncBTW: Amount::<EUR>::from_minor(row.get(1)?),
                PriceExuBTW: Amount::<EUR>::from_minor(row.get(2)?),
                Name: row.get(3)?,
                Btw: match row.get::<_, String>(4)?.as_str() {
                    "high" => BTWType::High,
                    "low" => BTWType::Low,
                    "none" => BTWType::None,
                    _ => BTWType::None, // Default case
                },
            })
        })?;

        Ok(())

    // create a new database file
    //  let conn = Connection::open("pos_labels.db")?;
}
fn MakeLable() -> Result<()> {
    // create a new in-memory database
   // let conn = Connection::open_in_memory()?;


    // create a new database file
      let conn = Connection::open("pos_labels.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS labels (
            productnum INTEGER PRIMARY KEY,
            priceincbtw INTEGER NOT NULL,
            priceexbtw INTEGER NOT NULL,
            name TEXT NOT NULL,
            btw TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;
     let s1 =  readlineString();
     let s2 = match readlineString().as_str() {
                "high" => BTWType::High,
                "low" => BTWType::Low,
                "none" => BTWType::None,
                _ => BTWType::None, // Default case
            };
         let s3: i32 = readlineint32();
         let s4: i64 = readlineint64();
    let me = Lables {
        Productnum: s3 as u32,
        PriceIncBTW: Amount::<EUR>::from_minor(s4),
        PriceExuBTW: calculateExcluBTW(Amount::<EUR>::from_minor(s4), s2),
        Name: s1,
        Btw: s2
    };

     let inc_cents: i64 = me.PriceIncBTW.to_minor().try_into().unwrap_or(0);
     let ex_cents: i64 = me.PriceExuBTW.to_minor().try_into().unwrap_or(0);


    conn.execute(
        "INSERT INTO labels (productnum, priceincbtw, priceexbtw, name, btw) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&me.Productnum, &inc_cents, &ex_cents, &me.Name, &me.Btw.as_str()),
    )?;


   
    Ok(())
    

}
fn calculateExcluBTW(price: Amount<EUR>, btw: BTWType) -> Amount<EUR> {
    let result = match btw {
        BTWType::High => price / 121 * 100,
        BTWType::Low => price / 109 * 100,
        BTWType::None => price,
    };
    result.round(RoundingMode::HalfUp)
}