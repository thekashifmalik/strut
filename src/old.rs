// use std::io;
// static PROMPT: &'static str = ">>> ";
// static SEPERATOR: char = '\n';


// enum Command {
//     Get,
//     Set(String),
//     Index(String),
//     Method(String),
// }


    // loop {
    //     for line in io::stdin().lines() {

    //         let statement = match line {
    //             Err(_) => {
    //                 print!("{}", PROMPT);
    //                 continue
    //             },
    //             Ok(string) => { Statement::new(string) },
    //         };

    //         let key = statement.token.as_slice();
    //         let value = match strut.get(key) {
    //             None => { continue },
    //             Some(v) => v,
    //         };

    //         print!("{}\n", value);
    //         print!("{}", PROMPT);
    //     }
    // }