use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(required = true)]
    text: Vec<String>,

    #[arg(short = 'n', long)]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    if args.omit_newline {
        print!("{}", args.text.join(" "))
    } else {
        println!("{}", args.text.join(" "))
    }
}
