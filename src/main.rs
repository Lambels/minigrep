use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::{env, process};

use minigrep::Worker;

const USAGE_PROMPT: &str = "USAGE:\nminigrep <file-name> [...queries]";

fn main() {
    let mut buf = String::new();

    let workers = parse_args(env::args(), &mut buf).unwrap_or_else(exit_with_help);

    let mut n: usize = 0;
    for worker in workers {
        n += worker.run(false, &mut io::stdout()).unwrap();
    }

    println!("Found {} matches!", n);
}

fn exit_with_help<'a>(err: Box<dyn Error>) -> Vec<Worker<'a>> {
    println!("{}", err);
    println!();
    println!("{}", USAGE_PROMPT);

    process::exit(1);
}

fn parse_args<'a>(
    mut args: impl ExactSizeIterator<Item = String>,
    buf: &'a mut String,
) -> Result<Vec<Worker<'a>>, Box<dyn Error>> {
    args.next(); // skip the executalble path.

    let file_path = args.next().ok_or("file path argument not found")?;

    if args.len() < 1 {
        return Err("expected at least one search query".into());
    }

    File::open(file_path)?.read_to_string(buf)?;

    // use iterator to move the values from the iterator to the worker, this avoids an allocation
    // over the previous implementation with the string slice where a clone had to be made, since
    // the slice didnt own the data.
    Ok(args.map(|val| Worker::new(buf, val)).collect())
}
