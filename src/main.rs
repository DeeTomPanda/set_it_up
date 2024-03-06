use std::io::Read;
use std::{fs,env,io};
use std::process::{Command,Stdio};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use colored::Colorize;
use std::time::Duration;



macro_rules! log{
    // tt matches anything 
    ($($arg:tt)*)=>{
        print!("{}",$($arg)*);
        std::io::stdout().flush().expect("Flush failed");
    }
 }
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
        // angular_setup(project_name);
    }

}


fn react_setup(project_name:&str,ts_flag:bool){
    

    let project_name=project_name.to_string();
    let stop_flag = Arc::new(Mutex::new(false));
    let stop_flag_clone = stop_flag.clone();

    print!("{}","Downloading".yellow());

    let progress_handle=thread::spawn(
    move||{
        while !*stop_flag.lock().unwrap(){
            log!(".".yellow());
            thread::sleep(Duration::from_millis(1000));
    }});
    
    let download_thread=thread::spawn(move||{
       let mut op=Command::new("yarn").args(&["create","vite",&project_name,"--template","react"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
        .expect("Failed to start command");

     

    let status=op.wait().expect("failed to complete the installation");
 
    

    // Check exit status of the command
    if status.success() {
        *stop_flag_clone.lock().unwrap() = true;        
        println!("\nNavigate to {} and start working!",&project_name.green());
    } else {
        eprintln!("Command failed with: {:?}", status);
    }
    
 
 });

download_thread.join().expect("Error in running thread");
 
}

fn angular_setup(project_name:&str){
    
    let project_name=project_name.to_string();

    print!("{}","Downloading".yellow());
    
    let download_thread=thread::spawn(move||{
        let mut output=Command::new("ng")
            .args(&["new",&project_name,"--defaults","--style=scss"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start ng command");

        let status=output.wait().expect("Failed to wait for ng");
        
        if status.success() {
            // *stop_flag_clone.lock().unwrap() = true;        
            println!("\nNavigate to {} and start working!",&project_name.green());
        } else {
            eprintln!("Command failed with: {:?}", status);
        }
    });
    download_thread.join().expect("Error in running thread");
}

fn react_native_setup(){}

fn express_setup(){}
