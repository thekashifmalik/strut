#![feature(phase)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

#[phase(plugin)]
extern crate seq_macros;
extern crate seq;

extern crate strut;

use strut::{Strut, types};


static TITLE: &'static str = "Strut";
static VERSION: &'static str = "0.1";


fn main() {
    println!("Testing {} v{}...", TITLE, VERSION);
    let mut strut = Strut::new();

    strut.set("1", types::Bool(true));
    strut.set("2", types::Int(3));
    strut.set("3", types::Float(3.1));
    strut.set("4", types::Str("world".to_string()));
    strut.set("5", types::Array(seq![
        types::Bool(false),
        types::Int(123),
        types::Str("hello world".to_string()),
    ]));
    strut.set("6", types::Map(seq!{
        "key1".to_string() => types::Int(123),
        "key2".to_string() => types::Array(seq![
            types::Bool(false),
            types::Float(5.23),
        ]),
    }));

    // Print gets.
    println!("\nPrinting gets...")
    match strut.get("1") {
        Some(v) => println!("{}", v),
        None => {},
    }
    match strut.get("2") {
        Some(v) => println!("{}", v),
        None => {},
    }
    match strut.get("3") {
        Some(v) => println!("{}", v),
        None => {},
    }
    match strut.get("4") {
        Some(v) => println!("{}", v),
        None => {},
    }
    match strut.get("5") {
        Some(v) => println!("{}", v),
        None => {},
    }
    match strut.get("6") {
        Some(v) => println!("{}", v),
        None => {},
    }

    // Print typed gets.
    println!("\nPrinting typed gets...")
    match strut.get_bool("1") {
        Some(v) => println!("{}", v),
        None => {},
    }
    match strut.get_int("2") {
        Some(v) => println!("{}", v),
        None => {},
    }
    match strut.get_float("3") {
        Some(v) => println!("{}", v),
        None => {},
    }
    match strut.get_str("4") {
        Some(v) => println!("{}", v),
        None => {},
    }
    match strut.get_array("5") {
        Some(v) => println!("{}", v),
        None => {},
    }
    match strut.get_map("6") {
        Some(v) => println!("{}", v),
        None => {},
    }

    println!("\nStrut -> {}\n", strut);
}
