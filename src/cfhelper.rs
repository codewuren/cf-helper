use std::fs::File;
//use std::fs;
use std::io::Read;
use std::io::Write;
use std::process;
use std::process::Command;
extern crate reqwest;
use codeforces_api::requests::CFAPIRequestable;
use codeforces_api::responses::CFResult;
use codeforces_api::requests::CFUserCommand;

pub struct Helper {
    
}

impl Helper {
    // Print the help message
    pub fn help(&self) {
        println!("Usage: ");
        println!("  cf-helper -g [TestName]\tGenerate a solution");
        println!("  cf-helper test [TestName]\tTest a solution");
    }

    // read a file and return the contents as a Vec<String>
    pub fn read_file_vec(&self, _filename: &String) -> Vec<String> {
        let mut file = match File::open(_filename) {
            Ok(file) => file,
            Err(_) => panic!("No such file!"),
        };
        let mut raw_file_contents = String::new();
        file.read_to_string(&mut raw_file_contents)
            .ok()
            .expect("failed to read!");
        let file_contents: Vec<String> = raw_file_contents.split("\n")
            .map(|s: &str| s.to_string())
            .collect();
        file_contents
    }

    //// read a file and return the contents as a String
    //pub fn read_file_string(&self, _filename: &String) -> String {
    //    let _str = fs::read_to_string(_filename).expect("Couldn't read the file!\n");
    //    _str
    //}

    // write the contents in _vec into a file
    pub fn write_file(&self, _filename: &String, _vec: Vec<String>) {
        // open the file
        let mut file = std::fs::File::create(_filename).expect("create failed!");
        
        // loop _vec and put each line into the target file
        for i in 0.._vec.len() {
            file.write_all(&_vec[i].as_bytes()).expect("write failed!");
            file.write_all("\n".as_bytes()).expect("write failed!");
        }
        println!("file {} create successful!", _filename);
    }

    // run a shell command
    pub fn run_command(&self, _command: String) -> String {
        println!("run command: {}", _command);

        // get the output and put them into str_output
        let output = Command::new("sh")
            .arg("-c")
            .arg(_command)
            .output()
            .expect("Wrong!");
        let mut str_output: String = String::from_utf8(output.stdout).unwrap();
        str_output.pop(); // delete the '\n'
        str_output
    }

    // generate the template code
    pub fn gen_code(&self, _filename: &mut String) {
        // Get user_home
        let mut template_path: String = self.run_command(String::from("echo $HOME"));
        template_path += "/.config/cf-helper/template";
        println!("template file located at: {}", template_path);

        // create the configuration folder
        self.run_command(String::from("mkdir ~/.config/cf-helper"));

        // read the contents of the template and write them into the target file
        let file_content: Vec<String> = self.read_file_vec(&template_path);
        self.write_file(&format!("{filename}.cpp", filename = _filename), file_content);
        self.write_file(&format!("{filename}.input", filename = _filename), vec![]);
    }

    // run a test
    pub fn test(&self, _filename: &String) {
        // Note: we only support the .cpp extension name
        println!("test started...");

        // get the full command and run
        let mut command: String = format!("g++ 
            {filename}.cpp 
            --std=c++11 
            -O2 
            -o {filename} 
            && ./{filename} 
            < {filename}.input 
            > {filename}.output", 
            filename = _filename);
        
        let mut output: String = self.run_command(command);
        println!("the output is:\n{}", output);

        command = format!("cat {filename}.input", filename = _filename);

        output = self.run_command(command);

        println!("the input is:\n{}", output);
    }

    // warn a error and quit if _exit_yon is true
    pub fn error(&self, _info: String, _exit_yon: bool) {
        println!("{}", String::from(_info));
        if _exit_yon {
            process::exit(-1);
        }
    }

    // print the version information
    pub fn print_version(&self) {
        println!("print_version()");
    }

    // get user info through CodeForces's API
    pub fn get_user_info(&self, _username: &String) {
        let handle_vec: Vec<String> = vec!(String::from(_username));

        // This is equivalent to the Codeforces `user.info` API method.
        let x: CFUserCommand = CFUserCommand::Info { 
            handles: handle_vec
        };

        // The `.get(..)` method on API commands returns a result with either
        // an error or an `Ok(CFResult)`.
        match x.get("2be5730459cc39ead0f78dc555d65118cb548121", "2a048d9e23c5d9d282c3059108888fa78e3b7243") {
            Ok(CFResult::CFUserVec(handles)) => {
                println!("Your user name: {:?}", handles[0].handle);
                println!("Your country name: {:?}", handles[0].country);
                println!("Your email: {:?}", handles[0].email);
                println!("Your rating: {:?}", handles[0].rating);
            },
            Ok(_) => {
                // In very rare cases, an unexpected type may be returned by
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

    // fetch problemset
    pub fn get_problem(&self) {

    }
}
