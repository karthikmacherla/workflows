use clap::Parser;

use crate::workflow::{fin, start, list, run, print, delete, test, alias};
use crate::types::*;

pub mod workflow;
pub mod types;
/**
 * Plan:
 * 
 * 
 */
#[derive(clap::Subcommand, Debug)]  
enum Action {
   /// start recording your current workflow (if you mess up just hit this command to reset)
   Start,
   /// finish recording your current workflow and save if desired
   Fin,
   /// lists your current workflows
   List, 
   /// runs the workflow with the name you've provided
   Run {
      workflow_name: String,
   },
   /// prints the generated workflow script for the inputted name
   Print {
      workflow_name: String,
   },
   /// deletes the workflow for the following name
   Delete {
      workflow_name: String,
   },
   /// sets the alias to be detected to the inputted name. 
   /// is set to the default path of the executable
   Alias {
      workflow_name: Option<String>,
   },
   Test
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[command(subcommand)]
   action: Action,
}

// Rust program that listens to terminal commands


fn main()  -> Result<()> {
   let args = Args::parse();


   use Action::*;
   match args.action {
      Start => start()?,
      Fin => fin()?,
      Run { workflow_name } => run(&workflow_name)?,
      List => list()?,
      Print { workflow_name } => print(&workflow_name)?,
      Delete { workflow_name } => delete(&workflow_name)?,
      Alias { workflow_name } => alias(workflow_name)?,
      Test => test()?,

   };

   Ok(())
}