extern crate gcc;

fn main() {
    gcc::Config::new().file("src/sl.c")
                      .include("src")
                      .flag("-lcurses")
                      .compile("libsl.a");
}
