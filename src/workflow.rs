#![allow(dead_code)]
#![allow(unused_imports)]
use std::{env, fs::{self, File, OpenOptions}, io::{Write, BufReader, BufRead}, ffi::{OsString}, process::{Command, Stdio}, fmt::format, path::PathBuf};

use crate::types::*;

static SCRIPT_EXTENSION: &'static str = ".ps1";
static APPDATA: &'static str = "APPDATA";
static POWERSHELL_HISTORY_PATH: &'static str = "\\Microsoft\\Windows\\PowerShell\\PSReadLine\\ConsoleHost_history.txt";

//should have $HOME prepended
static HOME: &'static str = "USERPROFILE";
static WORKFLOW_HOME_PATH: &'static str = "\\workflows\\home\\";
static WORKFLOW_START_PATH: &'static str = "\\workflows\\current_dir";
static WORKFLOW_CONFIG_PATH: &'static str = "\\workflows\\config";


fn prepend_env(env: &str, path: &str) -> Result<PathBuf> {
    let my_directory = env::var(env)?;
    return Ok(PathBuf::from(my_directory + path));  
}

pub fn list_workflows() -> Result<Vec<OsString>> {
    let mut wfs = Vec::new();
    fs::create_dir_all(prepend_env(HOME, WORKFLOW_HOME_PATH)?)?;
    let paths = fs::read_dir(prepend_env(HOME, WORKFLOW_HOME_PATH)?)?;

    for path in paths {
        let path = path?;
        if let Some(file_name) = path.path().file_stem() {
            wfs.push(file_name.to_os_string());
        }
    }
    return Ok(wfs);
}

pub fn is_existing_workflow(name: &str) -> bool {
    if let Ok(list) = list_workflows() {
        let osstr = OsString::from(name);
        return list.contains(&osstr);
    }
    return false;
}

pub fn get_alias() -> Option<String> {
    fs::read_to_string(prepend_env(HOME, WORKFLOW_CONFIG_PATH).unwrap_or_default()).ok()
}

pub fn save(workflow: &[&str]) -> Result<()> {
    // save
    let mut name = String::new();
    loop {
        println!("Workflow name: ");
        std::io::stdin().read_line(&mut name)?;
        // check if the name is taken
        if is_existing_workflow(&name) {
            println!("Workflow name is taken already!");
            continue;
        } else {
            break;
        }
    }

    // create a new file with the following name
    let wf_home_path = prepend_env(HOME, WORKFLOW_HOME_PATH)?;
    let output_file = format!("{}{}{}", wf_home_path.as_path().display(), name.trim(), SCRIPT_EXTENSION);
    let mut file = File::create(output_file)?;

    for line in workflow.iter().rev() {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}


/**
 * Start of the workflow commands we need to support
 * - start
 * - fin
 * - list
 * - run
 * - delete
 * - printS
 */
pub fn start() -> Result<()> {
    fs::create_dir_all(prepend_env(HOME, WORKFLOW_HOME_PATH)?)?;
    let mut file = File::create(prepend_env(HOME, WORKFLOW_START_PATH)?)?;

    // write the current directory to the temp file
    let current_dir = env::current_dir()?;
    let cmd = format!("cd {}", current_dir.display());
    writeln!(file, "{}", cmd)?;
    drop(file);
    Ok(())
}

pub fn fin() -> Result<()>{
    let pwsh_history = prepend_env(APPDATA, POWERSHELL_HISTORY_PATH)?;
    let text = fs::read_to_string(pwsh_history)?;    

    let mut workflow = Vec::new();
    let mut cnt = 0;
    
    let mut start_alias = "workflows.exe start".to_string();
    start_alias = match get_alias() {
        Some(s) => if !s.is_empty() { s.trim().to_string() } else { start_alias },
        None => start_alias
    };
    
    for line in text.lines().rev() {
        if line.contains(&start_alias) || cnt > 50 {
            break;   
        } else {
            workflow.push(line);
            cnt += 1;
        }
    }

    // double check that this folder exists
    fs::create_dir_all(prepend_env(HOME, WORKFLOW_HOME_PATH)?)?;
    // finally we need to read from the config and get the starting path of the workflow
    let start_cmd = fs::read_to_string(prepend_env(HOME, WORKFLOW_START_PATH)?)?;
    if !start_cmd.contains("cd") {
        return Err(Error::StartPathNotSetError);
    }
    workflow.push(start_cmd.trim());
    
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

pub fn list() -> Result<()> {
    let res = list_workflows()?;

    println!("Here are your current workflows:");
    for wf in &res {
        println!("- {}", wf.to_str().unwrap_or_default());
    }

    println!();
    println!("To run a workflow, try:");
    println!("\t `workflow.exe run <name>`");

    Ok(())
}

pub fn run(name: &str) -> Result<()> {
    if !is_existing_workflow(name) {
        println!("Not a valid workflow!");
        println!();
        return list()
    }

    let wf_home_path = prepend_env(HOME, WORKFLOW_HOME_PATH)?;
    let script_path = format!("{}{}{}", wf_home_path.as_path().display(), name, SCRIPT_EXTENSION);

    // run the workflow
    let mut output = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Unrestricted")
        .arg("-File")
        .arg(script_path)
        .env("LS_COLORS", "rs=0:di=38;5;27:mh=44;38;5;15")
        .stdout(Stdio::inherit())
        .spawn()?;

    output.wait()?;

    Ok(())
}

pub fn delete(name: &str) -> Result<()> {
    if !is_existing_workflow(name) {
        println!("Not a valid workflow!");
        println!();
        return list()
    }


    let wf_home_path = prepend_env(HOME, WORKFLOW_HOME_PATH)?;
    let script_path = format!("{}{}{}", wf_home_path.as_path().display(), name, SCRIPT_EXTENSION);

    match fs::remove_file(script_path) {
        Ok(()) => println!("Successfully deleted workflow: {}", name),
        Err(e) => println!("Error deleting workflow: {}", e)
    }

    Ok(())
}

pub fn print(name: &str) -> Result<()> {
    if !is_existing_workflow(name) {
        println!("Not a valid workflow!");
        println!();
        return list()
    }

    let wf_home_path = prepend_env(HOME, WORKFLOW_HOME_PATH)?;
    let script_path = format!("{}{}{}", wf_home_path.as_path().display(), name, SCRIPT_EXTENSION);

    let script = File::open(script_path)?;
    let rdr = BufReader::new(script);

    println!("Displaying workflow for {}:", name);
    for line in rdr.lines() {
        let line = line?;
        println!("{}", line);
    }


    println!("\nTo manually update an existing script, open the script located at: {}", wf_home_path.as_path().display());

    Ok(())
}

pub fn test() -> Result<()> {
    if let Ok(shell) = env::var("SHELL") {
        println!("The shell running this program is: {}", shell);
    } else {
        println!("Unable to determine the shell.");
    }

    Ok(())
}


pub fn alias(name: Option<String>) -> Result<()> {
    fs::create_dir_all(prepend_env(HOME, WORKFLOW_HOME_PATH)?)?;

    // this is a get alias command
    if name.is_none() {
        // open the config and read out the current alias
        let alias = fs::read_to_string(prepend_env(HOME, WORKFLOW_CONFIG_PATH)?).unwrap_or_default();
        println!("The current start is (empty if no alias set): {}", alias.trim());
        return Ok(())
    }

    let name = name.unwrap();
    let mut file = File::create(prepend_env(HOME, WORKFLOW_CONFIG_PATH)?)?;
    writeln!(file, "{}", &name)?;

    println!("Successfully set the start alias to: {}", &name);
    Ok(())
}