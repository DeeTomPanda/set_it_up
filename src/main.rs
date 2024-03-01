use std::io::Read;
use std::{fs,env,io};
use std::process::{Command,Stdio};
use std::io::Write;


fn main() {
    let mut buf:String=String::new();
    println!("Hello, welcome to SIUP");
    println!("Enter Your project name for a react setup");

    io::stdin().read_line(&mut buf).expect("error while parsing file name");
    let project_name:&str=buf.trim();


    // gets cwd
    let current_dir=env::current_dir().expect("Couldn't get current working directory");
    let mut project_path=current_dir.join(project_name);

    // Check if any project with same name exists
    if !(project_path).exists(){
        react_setup(project_name,false);
    }

}

fn react_setup(project_name:&str,TS:bool){
    let mut op=Command::new("sh").args(&["-c",&format!("yarn create vite {} --template react",project_name)])
    .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start command");
    

    let status=op.wait().expect("Encountered error while running command");


    // Check the exit status of the command
    if status.success() {
        println!("Command executed successfully");
    } else {
        eprintln!("Command failed with: {:?}", status);
    }    
}

fn angular_setup(){}

fn react_native_setup(){}

fn express_setup(){}
