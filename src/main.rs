use clap::Parser;

use crate::workflow::{fin, start, list, run};
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
   Start,
   Fin,
   List, 
   Run {
      workflow_name: String,
   },
   Print,
   Delete
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
      Print => (),
      Delete => ()
   };

   Ok(())
}