#![allow(unused)] //Allows the command to run when not filling in the arguments, basically error supression for running

use clap::Parser; //this calls the clap Parser package into scope
use rusqlite::{params, Connection, Result}; //working

//This generates the parser implementation
#[derive(Parser)] /// Search for a pattern in a file and display the lines that contain it.
struct Cli { //This is our Struct
    id: i32,
    todo: String, //This is the actual to do
    urgency: String, //This is the urgency of the request, this will be used later then asking what we should do first
}

fn main() {
    let args = Cli::parse(); //the passed args are run throught the Cli Parse functionality, this is the bread and butter of breaking up arguments
    // let todo = std::io::read_to_string(&args.todo).expect("please repeat your task"); 
    // let urgent = std::io::read_to_string(&args.urgency).expect("Whoops, missed an input");

    print!("Your task for today is {}, and its urgency is {} \n", args.todo, args.urgency)
}


//So when we run the command 'cargo run -- main src/main.rs' we are looking for the pattern = "main" in the file = 'src/main.rs'
// once the pattern is found in that file, we print that entire line, so the output that we get is 'fn main() {'
// NOTE: it will print all instances of that found pattern in the file, meaning it is iterating over the entire file...!x
// running different input arguments will cement this idea...
//Try things like 'cargo run -- pattern src/main.rs' and ' cargo run -- Cli src/main.rs' or even running it on different files! Just make sure to specify path right 