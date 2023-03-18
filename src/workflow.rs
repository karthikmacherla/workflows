
use std::{env, fs::{self, File}, io::{Write}, ffi::{OsString}, process::Command, fmt::format};

use crate::types::*;

// static HOME_PATH: &'static str = "C:\\Users\\kmacherla\\Desktop\\workflow";
static POWERSHELL_PATH: &'static str = "$env:APPDATA\\Microsoft\\Windows\\PowerShell\\PSReadLine\\ConsoleHost_history.txt";
static POWERSHELL_PATH_EXT: &'static str = "\\Microsoft\\Windows\\PowerShell\\PSReadLine\\ConsoleHost_history.txt";
static WORKFLOW_HOME: &'static str = "C:\\Users\\kmacherla\\Desktop\\workflows\\";
static SCRIPT_EXTENSION: &'static str = ".ps1";

pub fn start() -> Result<()> {
    // could do nothing ... or could push the current directory to the stack
    // let current_dir = env::current_dir()?;
    // let cmd = format!("cd {}", current_dir.display());

    // let app_data = env::var("APPDATA")?;
    // let pwsh_history = format!("{}{}", app_data, POWERSHELL_PATH_EXT);

    // let mut file = File::open(pwsh_history)?;
    // writeln!(file, "{}", cmd)?;
    // drop(file);
    Ok(())
}


pub fn list_workflows() -> Result<Vec<OsString>> {
    let mut wfs = Vec::new();
    let paths = fs::read_dir(WORKFLOW_HOME)?;

    for path in paths {
        let path = path?;
        wfs.push(path.file_name());
    }
    return Ok(wfs);
}

pub fn name_taken(name: &str) -> bool {
    if let Ok(list) = list_workflows() {
        let osstr = OsString::from(name);
        return list.contains(&osstr);
    }
    return false;
}

pub fn save(workflow: &[&str]) -> Result<()> {
    // save
    let mut name = String::new();
    loop {
        println!("Workflow name: ");
        std::io::stdin().read_line(&mut name)?;
        // check if the name is taken
        if name_taken(&name) {
            println!("Workflow name is taken already!");
            continue;
        } else {
            break;
        }
    }

    // create a new file with the following name
    let output_file = format!("{}{}{}", WORKFLOW_HOME, name.trim(), SCRIPT_EXTENSION);
    println!("{}", output_file);
    let mut file = File::create(output_file)?;

    for line in workflow.iter().rev() {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}


pub fn fin() -> Result<()>{
    let app_data = env::var("APPDATA")?;
    let full_path = format!("{}{}", app_data, POWERSHELL_PATH_EXT);

    let text = fs::read_to_string(full_path)?;    

    let mut workflow = Vec::new();
    let mut cnt = 0;

    for line in text.lines().rev() {
        if line.contains("workflows.exe start") || cnt > 50 {
            break;   
        } else {
            workflow.push(line);
            cnt += 1;
        }
    }
    // get rid of the first element which is known to be workflow.exe fin
    let workflow = &workflow[1..];
    

    println!("Here's your requested workflow:");
    println!("*******************************");
    for line in workflow.iter().rev() {
        println!("{}", line);
    }

    let mut save_workflow = false;
    loop {
        println!("Save workflow? (y/n): ");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer)?;
        let answer = answer.trim().to_lowercase();
        if answer.eq("n") {
            break;
        } else if answer.eq("y") {
            save_workflow = true;
            break;
        }
        println!("invalid option");
    }

    if save_workflow {
        save(workflow)?
    }

    Ok(())
}
