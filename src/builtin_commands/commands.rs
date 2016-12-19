use std::process;
use std::ffi::CString;
use std::os::raw::c_char;
use builtin_commands::models::*;

extern crate libc;

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
        _sl(argv[..].as_ptr(), argc);
    }
}

pub struct CmdExit;
impl Command for CmdExit {
    fn run(&self, _: &Args) {
        process::exit(0);
    }
}

pub struct CmdSl;
impl Command for CmdSl {
    fn run(&self, args: &Args) {
        sl(args, args.len() as i32);
    }
}

pub struct CmdCd;
impl Command for CmdCd {
    fn run(&self, args: &Args) {println!("cd!!");}
}

