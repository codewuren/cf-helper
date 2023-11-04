use codeforces_api::requests::{CFAPIRequestable, CFProblemsetCommand, CFUserCommand};
use std::fs::File;
use std::io::{Read, Write};
use std::process;
use std::process::Command;
//use codeforces_api::responses::CFProblem;
use codeforces_api::responses::CFResult;

pub struct Helper {
    pub api_key: String,
    pub api_secret: String,
}

impl Helper {
    // print the help message
    pub fn help(&self) {
        println!("Usage: ");
        println!("  cf -g [TestName]\tGenerate a solution");
        println!("  cf test [TestName]\tTest a solution");
    }

    // read a file and return the contents as a Vec<String>
    pub fn read_file_vec(&self, filename: &String) -> Vec<String> {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => panic!("No such file!"),
        };
        let mut raw_file_contents = String::new();
        file.read_to_string(&mut raw_file_contents)
            .ok()
            .expect("failed to read!");
        let file_contents: Vec<String> = raw_file_contents
            .split("\n")
            .map(|s: &str| s.to_string())
            .collect();
        file_contents
    }

    // // read a file and return the contents as a String
    // pub fn read_file_string(&self, _filename: &String) -> String {
    //     let _str = fs::read_to_string(_filename).expect("Couldn't read the file!\n");
    //     _str
    // }

    // write the contents in _vec into a file
    pub fn write_file(&self, _filename: &String, vec: Vec<String>) {
        // open the file
        let mut file = std::fs::File::create(_filename).expect("create failed!");

        // loop _vec and put each line into the target file
        for i in 0..vec.len() {
            file.write_all(&vec[i].as_bytes()).expect("write failed!");
            file.write_all("\n".as_bytes()).expect("write failed!");
        }
        println!("file {} create successful!", _filename);
    }

    // run a shell command
    pub fn run_command(&self, command: String) -> String {
        println!("run command: {}", command);

        // get the output and put them into str_output
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Wrong!");
        let str_output: String = String::from_utf8(output.stdout).unwrap();
        str_output
    }

    // generate the template code
    pub fn gen_code(&self, filename: &mut String) {
        // get user_home
        let mut template_path: String = self.run_command(String::from("echo $HOME"));

        // remove the '\n'
        template_path.pop();

        // get the template path
        template_path += "/.config/cf-helper/template";

        // create the configuration folder
        self.run_command(String::from("mkdir ~/.config/cf-helper"));

        // print template path
        println!("template file located at: {}", template_path);

        // read the contents of the template and write them into the target file
        let file_content: Vec<String> = self.read_file_vec(&template_path);
        self.write_file(
            &format!("{filename}.cpp", filename = filename),
            file_content,
        );
        self.write_file(&format!("{filename}.input", filename = filename), vec![]);
    }

    // run a test
    pub fn test(&self, filename: &String) {
        // note: we only support the .cpp extension name
        println!("test started...");

        // get the full command and run
        let mut command: String = format!(
            "g++ \
            {filename}.cpp \
            --std=c++11 \
            -O2 \
            -o {filename} \
            && ./{filename} \
            < {filename}.input \
            > {filename}.output && cat {filename}.output",
            filename = filename
        );

        let mut output: String = self.run_command(command);
        if !output.is_empty() {
            println!("the output is:\n{}", output);
        } else {
            println!("no output");
        }

        command = format!("cat {filename}.input", filename = filename);

        output = self.run_command(command);

        if !output.is_empty() {
            println!("the input is:\n{}", output);
        } else {
            println!("no input");
        }
    }

    // warn a error and quit if exit_yon is true
    pub fn error(&self, info: String, exit_yon: bool) {
        println!("{}", String::from(info));
        if exit_yon {
            process::exit(-1);
        }
    }

    // print the version information
    pub fn print_version(&self) {
        println!("print_version()");
    }

    // get user info through CodeForces's API
    pub fn get_user_info(&self, username: &String) {
        let username_vec: Vec<String> = vec![String::from(username)];

        // this is equivalent to the Codeforces `user.info` API method.
        let x: CFUserCommand = CFUserCommand::Info {
            handles: username_vec,
        };

        // the `.get(..)` method on API commands returns a result with either
        // an error or an `Ok(CFResult)`.
        match x.get(self.api_key.as_str(), self.api_secret.as_str()) {
            Ok(CFResult::CFUserVec(handles)) => {
                println!("Your user name: {:?}", handles[0].handle);
                println!("Your country name: {:?}", handles[0].country);
                println!("Your email: {:?}", handles[0].email);
                println!("Your rating: {:?}", handles[0].rating);
            }
            Ok(_) => {
                // in very rare cases, an unexpected type may be returned by
                // `.get()`. If this happens, then you may wish to throw a
                // custom error.
                panic!("`.get()` returned an unexpected type.");
            }
            Err(e) => {
                // Errors returned are of a custom Error type. This could be
                // returned if, for example, an invalid API key/secret was used
                // or if there was no internet connection.
                panic!("something failed {:?}", e);
            }
        }
    }

    // search problemset
    pub fn search_problems(&self, problem: &String) {
        let x: CFProblemsetCommand = CFProblemsetCommand::Problems {
            tags: Some(vec![String::from(problem)]),
            problemset_name: None,
        };

        match x.get(self.api_key.as_str(), self.api_secret.as_str()) {
            Ok(CFResult::CFProblemset(problem)) => {
                for i in 0..problem.problems.len() {
                    println!(
                        "Problem {idx}: {name}",
                        idx = i,
                        name = problem.problems[i].name
                    );
                }
            }
            _ => {
                // Errors returned are of a custom Error type. This could be
                // returned if, for example, an invalid API key/secret was used
                // or if there was no internet connection.
                panic!("something failed!");
            }
        }
    }
}
