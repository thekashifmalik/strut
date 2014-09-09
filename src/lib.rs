#![feature(phase)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::collections::HashMap;


pub mod types  {


    use std::collections::HashMap;
    use std::from_str;

    #[deriving(Show, PartialEq)]
    pub enum Data {
        Bool(bool),
        Int(i64),
        Float(f64),
        Str(String),
        Array(Vec<Data>),
        Map(HashMap<String, Data>),
    }

    pub fn from_str(string: &str) -> Result<Data, ()> {
        let int_regex = regex!(r"^\d+$");
        let float_regex = regex!(r"^\d*\.\d*$");
        let str_regex = regex!("^[\"].*[\"]$");  // No support for newlines

        // Try bool
        match string {
            "True" => { return Ok(Bool(true)) },
            "False" => { return Ok(Bool(false)) },
            _ => {},
        }

        // Try int
        if int_regex.is_match(string) {
            return Ok(Int(from_str::from_str(string).unwrap()))
        }

        // Try float
        if float_regex.is_match(string) {
            return Ok(Float(from_str::from_str(string).unwrap()))
        }

        // Try str
        if str_regex.is_match(string) {
            return Ok(Str(String::from_str(string.slice_chars(1, string.char_len() - 1))))
        }

        return Err(())
    }

}



#[deriving(Show)]
pub struct Strut {
    memory: HashMap<String, types::Data>
}

impl Strut {

    pub fn new() -> Strut {
        Strut{memory: HashMap::new()}
    }

    pub fn get(&self, key_str: &str) -> Option<&types::Data> {
        self.memory.find(&key_str.to_string())
    }

    pub fn get_str(&self, key_str: &str) -> Option<&String> {
        match self.get(key_str) {
            None => { return None },
            Some(data) => {
                match *data {
                    types::Str(ref v) => { return Some(v) },
                    _ => { return None },
                }
            }
        }
    }

    pub fn get_bool(&self, key_str: &str) -> Option<&bool> {
        match self.get(key_str) {
            None => { return None },
            Some(data) => {
                match *data {
                    types::Bool(ref v) => { return Some(v) },
                    _ => { return None },
                }
            }
        }
    }

    pub fn get_int(&self, key_str: &str) -> Option<&i64> {
        match self.get(key_str) {
            None => { return None },
            Some(data) => {
                match *data {
                    types::Int(ref v) => { return Some(v) },
                    _ => { return None },
                }
            }
        }
    }

    pub fn get_float(&self, key_str: &str) -> Option<&f64> {
        match self.get(key_str) {
            None => { return None },
            Some(data) => {
                match *data {
                    types::Float(ref v) => { return Some(v) },
                    _ => { return None },
                }
            }
        }
    }

    pub fn get_array(&self, key_str: &str) -> Option<&Vec<types::Data>> {
        match self.get(key_str) {
            None => { return None },
            Some(data) => {
                match *data {
                    types::Array(ref v) => { return Some(v) },
                    _ => { return None },
                }
            }
        }
    }

    pub fn get_map(&self, key_str: &str) -> Option<&HashMap<String, types::Data>> {
        match self.get(key_str) {
            None => { return None },
            Some(data) => {
                match *data {
                    types::Map(ref v) => { return Some(v) },
                    _ => { return None },
                }
            }
        }
    }
    pub fn set(&mut self, key_str: &str, data: types::Data){
        self.memory.insert(key_str.to_string(), data);
    }

    pub fn evaluate(&mut self, text: &str) -> Option<&types::Data>{

        let get_regex = regex!(r"^\S+$");
        let set_regex = regex!(r"^\S+\s*=\s*\S+.*");
        let del_regex = regex!(r"^del \S+$");


        if set_regex.is_match(text) {

            let mut tokens = text.split('=');
            let ident = tokens.next().unwrap().trim();
            let value_str = tokens.last().unwrap().trim();

            let value = match types::from_str(value_str) {
                Ok(d) => d,
                Err(_) => return None,
            };
            println!("SET \"{}\" to {}", ident, value);
            self.set(ident, value);

        }

        if get_regex.is_match(text) {
            let ident = text;
            let value = self.get(ident);
            println!("GET \"{}\" got {}", ident, value);
            return value;
        }

        // if del_regex.is_match(text) {
        //     println!("DEL");
        //     return;
        // }

        return None;
    }

}

