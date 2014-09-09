#![feature(phase)]

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

#[phase(plugin)]
extern crate seq_macros;
extern crate seq;

extern crate debug;
extern crate strut;

use std::io::{
    Acceptor,
    Listener,
    TcpListener,
    TcpStream,
};
use strut::{Strut, types};


static TITLE: &'static str = "Strut";
static VERSION: &'static str = "0.1";


#[deriving(Show)]
struct Token {
    text: String
}

#[deriving(Show)]
struct Statement {
     text: String,
 }

enum StatementType {
    Get,
    Set,
    Delete,
}

impl Statement {

    fn new(mut text: String) -> Result<Statement, ()> {
        let valid_statment_regex = regex!(r"^.+\n$");
        let text_slice = text.as_slice();

        if valid_statment_regex.is_match(text_slice) {

            let trimmed = text_slice.trim_chars('\n').trim_chars(' ').to_string();
            return Ok(Statement{text: trimmed});
        }
        Err(())
    }
}


fn main() {
    println!("{} v{}", TITLE, VERSION);

    // Create instance of memory.
    let mut strut = Strut::new();

    // Attach to port and wait for connections.
    let listener = TcpListener::bind("127.0.0.1", 5747);
    let mut acceptor = listener.listen();
    println!("listening on {}:{}...", "127.0.0.1", 5747u);

    for stream in acceptor.incoming() {

        let string = match stream {
            Err(_) => { continue },
            Ok(mut s) => {
                match s.read_to_string() {
                    Err(_) => { continue },
                    Ok(s) => s,
                }
            },
        };

        let mut statement = match Statement::new(string) {
            Err(_) => { continue },
            Ok(s) => s,
        };

        match strut.evaluate(statement.text.as_slice()) {
            None => continue,
            Some(d) => {},
        };

    }

}
