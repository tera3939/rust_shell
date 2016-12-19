use std::error::Error;
use std::process;

pub type Args<'a> = Vec<&'a str>;

#[macro_export]
macro_rules! cmds_map {
    ($(($name:expr, $cmd:expr),)*) => {{
        let mut cmds = CommandMap::new();
        $(cmds.add($name, Box::new($cmd));)*
        cmds
    }}
}

fn launch(args: &Args) {
    match process::Command::new(args[0]).args(&args[1..]).spawn() {
        Ok(mut child) => {
            // TODO: ここでunwrap()は暴力的な気がするのでなんとかしよう
            child.wait().unwrap();
        },
        Err(err)   => println!("error:  {:?}", err.description()),

    }
}

// made in the planet azyobuzi
// https://github.com/azyobuzin/tweetust/blob/master/examples/cli.rs#L123
pub trait Command {
    fn run(&self, reader: &Args);
}

pub struct CommandMap {
    pub map: Vec<(&'static str, Box<Command>)>
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

