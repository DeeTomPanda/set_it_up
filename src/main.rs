use std::{fs,env,io};
use std::process::Command;


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
        react_setup(project_name);
    }

}

fn react_setup(project_name:&str){
    let op=Command::new("yarn").args(["create","vite"]).output().expect("Failed for some reason");
    print!("{:?}",&op);
}

fn angular_setup(){}

fn react_native_setup(){}

fn express_setup(){}
