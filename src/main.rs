use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::env;

use minigrep::Worker;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut buf = String::new();

    let workers = match parse_args(&args[1..], &mut buf) {
        Ok(vals) => vals,
        Err(msg) => panic!("{}", msg),
    };

    let mut n: usize = 0;
    for worker in workers {
        n += worker.run(false, &mut io::stdout()).unwrap();
    }

    println!("Found {} matches!", n);
}

fn parse_args<'a>(args: &'a [String], buf: &'a mut String) -> Result<Vec<Worker<'a>>, Box<dyn Error>> {
    if args.len() < 2 {
        return Err("expected at least 2 arguments".into());
    }

    File::open(&args[0])?.read_to_string(buf)?;
    let mut workers = Vec::with_capacity(args[1..].len());

    for v in &args[1..] {
        workers.push(Worker::new(buf, v.clone()));
    }

    Ok(workers)
}
