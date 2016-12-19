use std::error::Error;
use std::ffi::CString;
use std::io;
use std::io::prelude::*;
use std::os::raw::c_char;
use std::process;

extern crate libc;
extern crate regex;
extern crate ncurses;
use regex::Regex;
use ncurses::*;

type Args<'a> = Vec<&'a str>;

#[link(name = "curses")]
extern {
    fn _sl(argv: *const *const c_char, argc: i32);
}

fn sl(argv: &Args, argc: i32) {
    // patch from the planet azyobuzi
    // https://gist.github.com/azyobuzin/9ec07d7ab465081537c11517c1eb227e
    // this is so kansya kangeki ame arare!!!
    let cstrs = argv.iter()
                    .map(|x| CString::new(*x))
                    .filter_map(|x| x.ok())
                    .collect::<Vec<_>>();
    let argv = cstrs.iter()
                    .map(|x| x.as_ptr())
                    .collect::<Vec<_>>();
    unsafe {
        _sl(argv, argc);
    }
}

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

fn launch(args: &Args) {
    println!("command: {:?}", *args);
    match process::Command::new(args[0]).args(&args[1..]).spawn() {
        Ok(mut child) => {
            child.wait().unwrap();
        },
        Err(err)   => println!("error:  {:?}", err.description()),

    }
}

// made in the planet azyobuzi
// https://github.com/azyobuzin/tweetust/blob/master/examples/cli.rs#L123
trait Command {
    fn run(&self, reader: &Args);
}

struct CommandMap {
    map: Vec<(&'static str, Box<Command>)>
}

impl CommandMap {
    fn new() -> CommandMap {
        CommandMap { map: Vec::new() }
    }

    fn add(&mut self, name: &'static str, cmd: Box<Command>) {
        self.map.push((name, cmd));
    }

    fn run(&self, reader: &Args) {
        let cmd = reader[0];
        if let Some(&(_, ref cmd)) = self.map.iter().filter(|&&(name, _)| name == cmd).nth(0) {
            cmd.run(reader);
        } else {
            launch(reader);
        }
    }
}

struct CmdExit;
impl Command for CmdExit {
    fn run(&self, _: &Args) {
        std::process::exit(0);
    }
}

struct CmdSl;
impl Command for CmdSl {
    fn run(&self, args: &Args) {
        sl(args, args.len() as i32);
    }
}

