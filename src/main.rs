use std::io;
use std::io::prelude::*;

extern crate libc;
extern crate regex;
use regex::Regex;

extern crate rust_shell;
use rust_shell::commands::*;
use rust_shell::models::*;

macro_rules! cmds_map {
    ($(($name:expr, $cmd:expr),)*) => {{
        let mut cmds = CommandMap::new();
        $(cmds.add($name, Box::new($cmd));)*
        cmds
    }}
}

fn main() {
    let cmds = cmds_map! {
        ("sl", CmdSl),
        ("exit", CmdExit),
    };
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        println!("line: {:?}", line);
        let args = parse_args(&line);
        println!("args: {:?}", args);
        if !args.is_empty() { cmds.run(&args); }
    }
}

fn parse_args(line: &str) -> Vec<&str> {
    let delimiter = Regex::new(r"[ \t\r\n\a]+").unwrap();
    delimiter.split(&line.trim_left()).collect()
}

