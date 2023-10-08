mod cfhelper;
use cfhelper::Helper;
use std::env;


fn main() {
    let helper = Helper {  };
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        helper.help();
        helper.error(String::from("No arguments passed in!"), false);
    }
    for i in 0..args.len() {
        if args[i] == "--help" || args[i] == "-h" {
            helper.help();
        }
        else if args[i] == "-g" {
            helper.gen_code(&mut args[i + 1]);
        }
        else if args[i] == "-v" {
            helper.print_version();
        }
        else if args[i] == "test" {
            helper.test(&args[i + 1]);
        }
    }
}
