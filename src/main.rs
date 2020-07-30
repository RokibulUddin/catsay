extern crate colored;
extern crate structopt;

use std::io::{self, Read};

use colored::*;
use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Show a cat banner saying a message (done in Rust) [Author: Rokibul Uddin]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make that cat appear dead
    dead: bool,

    #[structopt(short = "c", long = "color")]
    /// Use color
    color: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file (ASCII)
    catfile: Option<std::path::PathBuf>,

    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options: Options = Options::from_args();
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message).with_context(|_| "could not read input from pipe")?;
    } else {
        if options.color {
            message = options.message.bright_yellow().underline().on_purple().to_string();
        } else {
            message = options.message;
        }
    }
    if message.to_lowercase() == "woof".to_string() {
        eprintln!("A cat shouldn't bark like a dog.");
    }
    let eye = if options.dead { "x" } else { "o" };
    let eye_to_print = if options.color { eye.red().bold() } else { ColoredString::from(eye) };
    println!("{}", message);
    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|_| format!("could not read file {:?}", path))?;
            let cat_picture = cat_template.replace("{eye}", eye_to_print.as_ref());
            println!("{}", &cat_picture);
        }
        None => {
            println!("\\");
            println!(" \\");
            println!("    /\\_/\\");
            println!("   ( {eye} {eye} )", eye = eye_to_print);
            println!("   =( I )=");
        }
    }
    Ok(())
}
