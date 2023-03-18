use clap::Parser;

use crate::workflow::{fin, start};
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
   Run,
   Delete
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[command(subcommand)]
   action: Action,

   name: String
}

// Rust program that listens to terminal commands


fn main()  -> Result<()> {
   let args = Args::parse();
   println!("{:?}", args);

   use Action::*;
   match args.action {
      Start => start()?,
      Fin => fin()?,
      Run => (),
      _ => ()
   };

   Ok(())
}