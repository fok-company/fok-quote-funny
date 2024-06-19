use std::{
  env,
  fs,
  collections::HashMap,
  time::{SystemTime, UNIX_EPOCH},
};

mod config_handler;

use config_handler::{ConfigObject, ConfigValue, ConfigValues};

fn mul_string(string: &str, replnum: i32) -> String  {
  let mut result = String::new();
  for i in 0..replnum {
    result+=string;
  }
  return result;
}

fn main() {
  let cloud_border = "-";
  let cloud_wall = "|";
  /*let plush = [
    "                              ████████",
    "                          ████        ████",
    "                        ██                ██",
    "                      ██                    ██",
    "                      ██    ████        ██████",
    "                    ██░░  ████  ██    ████  ████",
    "                    ██░░  ████████    ██████████  ██",
    "                  ██░░░░    ████        ████  ████",
    "                  ██░░░░░░██        ▒▒        ██",
    "          ████  ██░░░░░░░░░░      ██  ██    ██  ██",
    "        ██░░  ████░░░░░░██  ░░░░          ░░██",
    "      ██░░░░░░░░░░░░░░██  ░░  ░░░░░░░░░░░░██░░██",
    "        ██░░░░████████  ░░  ░░██░░░░░░████░░░░░░██",
    "          ████        ████████  ██████    ████████",

  ].to_vec();*/
  let mut quote = String::new();
  let mut author = String::from("Nullus");
  let args: Vec<String> = env::args().collect();

  //let confstr = fs::read_to_string(env!("CONFIG_FILE")).unwrap();
  let mut config = config_handler::parse(env!("CONFIG").to_string());
  //println!("{}", env!("CONFIG"));
  let plush = config.get("plush".to_string()).value.to_list().unwrap();
  //println!("{:#?}", config);

  let x = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
  let last_id = x as i32 % config.get("quotes".to_string()).value.to_list().unwrap().len() as i32;

  let mut quote = String::new();
  if args.len()>2 {
    author = args[2].clone()
  }
  if args.len()>1 {
    quote = args[1].clone();
  } else {
    let tuple = config.get("quotes".to_string()).value.to_list().unwrap()[last_id as usize].value.clone().to_list().unwrap();
    //println!("{}", quote);
    quote = tuple[0].value.clone().to_string().unwrap();
    author = tuple[1].value.clone().to_string().unwrap();
  }

  let border = mul_string(cloud_border,quote.len() as i32+8);
  println!(" {}\n|{}|\n|{}{}{}|\n|{}|\n {}\n{}\\/", border, mul_string(" ", quote.len() as i32+8),
    mul_string(" ", 4), quote, mul_string(" ", 4), mul_string(" ", quote.len() as i32+8), border, mul_string(" ", (quote.len() as i32)));

  for i in plush {
    println!("{}",i.value.clone().to_string().unwrap());
  }
  println!("{}~ {}",mul_string(" ", /*quote.len() as i32+*/56), author);
  /*if config.get("quotes".to_string()).value.to_list().unwrap().len()-1==last_id as usize{
    config.push("l_id".to_string(), ConfigObject{valuetype: ConfigValues::int, value: ConfigValue::int(0)});
  } else {
    config.push("l_id".to_string(), ConfigObject{valuetype: ConfigValues::int, value: ConfigValue::int(last_id+1)});
  }
  //println!("{:#?}", config);
  fs::write(env!("CONFIG_FILE"), config.to_string());
*/}
