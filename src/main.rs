mod cfhelper;
use cfhelper::Helper;
use std::env;


fn main() {

    // create a new Helper obeject
    let helper = Helper {
        api_key: String::from("2be5730459cc39ead0f78dc555d65118cb548121"),
        api_secret: String::from("2a048d9e23c5d9d282c3059108888fa78e3b7243")
    };

    // get the arguments
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
        else if args[i] == "-i" || args[i] == "-user_info" {
            helper.get_user_info(&args[i + 1]);
        }
        else if args[i] == "-s" || args[i] == "-search" {
            helper.search_problems(&args[i + 1]);
        }
    }
}
