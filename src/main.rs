extern crate clap;
extern crate testcase_diff;
use clap::{App, Arg};
use testcase_diff::default;
use testcase_diff::simple;

fn main() {
    let app = App::new("testcase_diff")
        .version("0.1.0")
        .author("kobyn")
        .about("CLI tool")
        .arg(Arg::with_name("correct").required(true))
        .arg(Arg::with_name("input").required(true))
        .arg(
            Arg::with_name("flg")
                .help("use simple flag")
                .short("s")
                .long("simple"),
        )
        .arg(Arg::with_name("eps"));

    let matches = app.get_matches();

    let correct = matches.value_of("correct").unwrap();
    let input = matches.value_of("input").unwrap();
    if matches.is_present("flg") {
        let ret = simple::diff(correct, input);
        match ret {
            Ok(()) => println!("Accepted!"),
            Err(e) => {
                let (ok, ng) = *e;
                simple::print(&ok, &ng);
            }
        }
    } else {
        let ret = default::diff(correct, input);
        match ret {
            Ok(()) => println!("Accepted!"),
            Err(e) => {
                let p = *e;
                default::print(&p);
            }
        }
    }
}
