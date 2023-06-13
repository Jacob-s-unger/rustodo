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

    print!("Your task for today is {}, and its urgency is {} \n", args.todo, args.urgency) //you then call these arguments by args.name of the attribute
}

