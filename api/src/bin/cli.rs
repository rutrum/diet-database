use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let table = &args[1];
    match table.as_ref() {
        "bowel" => manage_bowel(&args[2..]),
        _ => help(),
    }
}

fn manage_bowel(args: &[String]) {
    if args.len() < 1 {
        println!("need subcommand for bowel");
        help();
        return;
    }
}

fn help() {
    println!("Bad, try again.");
}
