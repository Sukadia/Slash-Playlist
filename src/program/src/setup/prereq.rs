use std::process::Command;

pub fn check_version(commandname: String, args: Vec<String>, filterop: Option<&dyn Fn(String) -> String>){
    let filter = filterop.unwrap_or(&|s| s);

    let output = Command::new(&commandname)
        .args(&args)
        .output();

    if let Ok(output) = output {
        let mut version = String::from_utf8(output.stdout).unwrap().replace("\n", "");
        version = filter(version);
        println!("Found: {} {}",&commandname,version);
    }else{
        println!("Did not find {}!",&commandname);
    }
}