mod foklang;
use std::{
  env,
  fs,
  collections::HashMap,
  time::{SystemTime, UNIX_EPOCH},
};


fn mul_string(string: &str, replnum: i32) -> String  {
  let mut result = String::new();
  for i in 0..replnum {
    result+=string;
  }
  return result;
}

fn stringify(l: Vec<foklang::AST::Proventus>) -> String {
  let mut result = String::new();
  for i in l {
    match i.value {
      foklang::AST::Fructa::Ustulo(c) => {result+=&c.to_string();},
      _ => {}
    }
  }
  result
}

fn main() {

  let tokenizer = foklang::tokenizer::Tokenizer {};
  let mut parser = foklang::parser::Parser {};
  let error_handler = foklang::error_handler::ErrorHandler {};
  let mut env = foklang::env::Environment{ error_handler: error_handler, ..Default::default() };


  foklang::builtins::declare_builtins(&mut env);
  let mut interpreter = foklang::interpreter::Interpreter {error_handler: error_handler};

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
  //println!("{:#?}", env!("CONFIG").to_string());
  let mut config = interpreter.evaluate(parser.parse(tokenizer.tokenize(env!("CONFIG").to_string())), &mut env);
  //println!("{:#?}", config);
  //println!("{}", env!("CONFIG"));
  let plush = foklang::builtins::get(foklang::builtins::Arguments{
      function: foklang::builtins::FunctionArgs::get(config.clone(), foklang::AST::Proventus{
          value: foklang::AST::Fructa::Filum(String::from("plush")), id: -1
      })
  });
  let quotes = foklang::builtins::get(foklang::builtins::Arguments{
      function: foklang::builtins::FunctionArgs::get(config, foklang::AST::Proventus {
          value: foklang::AST::Fructa::Filum(String::from("quotes")), id: -1
      })
  });
  //println!("{:#?}", plush);
  
  //config.get("plush".to_string()).value.to_list().unwrap();
  

  //println!("{:#?}", config);

  let x = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
  let last_id = x as i32 % match quotes.value {foklang::AST::Fructa::Inventarii(ref i) => i.len(), _ => panic!("d")} as i32;



  if args.len()>2 {
    author = args[2].clone()
  }
  if args.len()>1 {
    quote = args[1].clone();
  } else {
    let temp = match quotes.value {foklang::AST::Fructa::Inventarii(i) => i, _ => panic!("e")};
    let tuple = match &temp[last_id as usize].value {
      foklang::AST::Fructa::Inventarii(i) => i, 
      _ => panic!("f")
    };
    //println!("{:#?}", tuple);
    quote = match &tuple[0].value {
      foklang::AST::Fructa::Inventarii(i) => stringify(i.to_vec()),
      _ => "I bork".to_string()
    };
    
    //println!("{}", quote);
    author = match &tuple[1].value {
      foklang::AST::Fructa::Inventarii(i) => stringify(i.to_vec()),
      _ => "I bork sequel".to_string()
    };
  }



  let border = mul_string(cloud_border,quote.len() as i32+8);
  println!(" {}\n|{}|\n|{}{}{}|\n|{}|\n {}\n{}\\/", border, mul_string(" ", quote.len() as i32+8),
    mul_string(" ", 4), quote, mul_string(" ", 4), mul_string(" ", quote.len() as i32+8), border, mul_string(" ", (quote.len() as i32)));

  let plushm = match plush.value {
    foklang::AST::Fructa::Inventarii(i) => i,
    _ => panic!("A")
  };
  for i in plushm {
    println!("{}", format!("{}", match i.value {foklang::AST::Fructa::Inventarii(i) =>  stringify(i.to_vec()), _ => panic!("a")}).replace("esc", "\x1B"));
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
