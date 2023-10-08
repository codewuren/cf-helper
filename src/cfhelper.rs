use std::fs::File;
//use std::fs;
use std::io::Read;
use std::io::Write;
use std::process;
use std::process::Command;

pub struct Helper {
    
}

impl Helper {
    // Print the help message
    pub fn help(&self) {
        println!("Usage: ");
        println!("  cf-helper gen [TestName]\tGenerate a solution");
        println!("  cf-helper test [TestName]\tTest a solution");
        println!("  cf-helper user-info\t\tGet user info");
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
        _filename.push_str(".cpp");
        self.write_file(_filename, file_content);
    }

    // run a test
    pub fn test(&self, _filename: &String) {
        // Note: we only support the .cpp extension name
        println!("test started...");
        let mut command: String = String::from("g++ ");
        command += _filename;
        command += ".cpp";
        command += " --std=c++11 -O2 -o ";
        command += _filename;
        command += " && ./";
        command += _filename;

        self.run_command(command);
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
}
