use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let (f_name, search) = match parse_args(&args[1..]) {
        Ok(vals) => vals,
        Err(msg) => panic!("{}", msg),
    };

    println!("Searching for {:?}", search);
    println!("In: {}", f_name);
}

fn parse_args(args: &[String]) -> Result<(&String, &[String]), String> {
    if args.len() < 2 {
        return Err(format!(
            "Expected at least two arguments but go {}",
            args.len()
        ));
    }

    Ok((&args[0], &args[1..]))
}
