use std::{
  io,
  fs,
  collections::HashMap,
};
#[derive(Clone, Debug)]
pub enum ConfigValues {
  Empty,
  string,
  int,
  tuple,
  list,
}
#[derive(Clone, Debug)]
pub enum ConfigValue {
  Nullus,
  string(String),
  int(i32),
  tuple(Vec<ConfigObject>),
  list(Vec<ConfigObject>),
}


#[derive(Clone, Debug)]
pub struct ConfigObject {
  pub valuetype: ConfigValues,
  pub value: ConfigValue,
}
#[derive(Clone, Debug)]
pub struct Config {
  keys: Vec<String>,
  values: Vec<ConfigObject>,
}



impl ConfigValue {
  pub fn to_int(self) -> Option<i32> {
    match self {
      Self::int(s) => Some(s),
      _ => None,
    }
  }
  pub fn to_string(self) -> Option<String> {
    match self {
      Self::string(s) => Some(s),
      _ => None,
    }
  }
  pub fn to_list(self) -> Option<Vec<ConfigObject>> {

    match self {
      Self::list(s) => Some(s),
      Self::tuple(s) => Some(s),
      _ => None,
    }
  }
  pub fn as_str(self) -> String {
    let mut result=String::new();
    match self {
      Self::list(s) => {
          result+="[";
          for i in s {
            if !result.ends_with("[") {
              result+="`";
            }
            result+=&i.value.as_str();
          }
          result+="]";
          result
      },    
      Self::tuple(s) => {
        result+="(";
        for i in s {
          if !result.ends_with("(") {
            result+=",";
          }
          result+=&i.value.as_str();

        }
        result+=")";
        result
    },
      Self::string(s) => "\"".to_owned() + &s + "\"",
      Self::int(s) => s.to_string(),
      _ => "nullus".to_string()

    }
  }
}

impl ConfigObject {
  fn new() -> ConfigObject {
    return ConfigObject{ valuetype: ConfigValues::Empty, value: ConfigValue::Nullus}
  }
  
  fn from(value: String) -> Self {
    if value.starts_with("\"") && value.ends_with("\"") {
      let mut chars = value.chars(); chars.next(); chars.next_back();
      return ConfigObject{valuetype: ConfigValues::string, value: ConfigValue::string(chars.as_str().to_string())};
    }
 
    else if value.starts_with("(") && value.ends_with(")") {
      let mut tuple: Vec<ConfigObject> = [].to_vec();
      let mut chars = value.chars(); chars.next(); chars.next_back();


      for i in chars.as_str().to_string().split(",") {
        tuple.push(ConfigObject::from(fix_string(i.to_string())));
      }

      return ConfigObject{valuetype: ConfigValues::tuple, value: ConfigValue::tuple(tuple)};
    }
    
    else if value.starts_with("[") && value.ends_with("]") {
      let mut vector: Vec<ConfigObject> = [].to_vec();
      let mut chars = value.chars(); chars.next(); chars.next_back();

      for i in chars.as_str().to_string().split("`") {
        vector.push(ConfigObject::from(fix_string(i.to_string())));
      }
      return ConfigObject{valuetype: ConfigValues::list, value: ConfigValue::list(vector)};
    }
    else {
      let mut tmp = value.chars().collect::<Vec<char>>();
      if tmp.len()>0 {
        while tmp[0].is_numeric() && tmp.len()>0 {
          tmp.remove(0);
          if tmp.len()==0 {
            return ConfigObject{valuetype: ConfigValues::int, value: ConfigValue::int(value.parse::<i32>().unwrap())};
          }
        }
      }
      return ConfigObject{valuetype: ConfigValues::Empty, value: ConfigValue::Nullus};
    }
  }
}

impl Config {
  pub fn new() -> Config {
    return Config {keys: [].to_vec(), values: [].to_vec()};
  }
  pub fn push(&mut self, key: String, value: ConfigObject) -> &mut Self {
    for i in 0..self.keys.len() {
      if self.keys[i]==key {
        self.values[i]=value;
        return self
      }
    }
    self.keys.push(key);
    self.values.push(value);
    return self
  }
  pub fn get(&mut self, key: String) -> ConfigObject {
    let mut id = 0;
    for i in self.keys.clone() {
      if key==i {
        break;
      }
      id+=1;
    }
    self.values[id].clone()
  }
  pub fn to_string(&mut self) -> String {
    let mut result = String::new();

    for i in 0..(self.keys.len()) {
      result+=&(self.keys[i.clone()].clone()+"="+&(self.values[i].value.clone().as_str()));
      if i!=self.keys.len()-1 {
        result+=";\n";
      }
    }
    result
  }
}




pub fn parse(string: String) -> Config {
  let mut config = Config::new();
  for i in string.split(";") {
    config.push(i.split("=").collect::<Vec<&str>>()[0].to_string().replace("\n","").replace(" ",""),
        ConfigObject::from(fix_string(i.split("=").collect::<Vec<&str>>()[1].to_string())));
  }
  return config
}


fn fix_string(string: String) -> String {
  let mut chars = string.chars();

  while chars.as_str().to_string().starts_with(" ") {
    chars.next();
  }
  while chars.as_str().to_string().ends_with(" ") {
    chars.next_back();
  }
  return chars.as_str().to_string().replace("\n","");
}

