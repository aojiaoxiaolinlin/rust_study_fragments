use clap::Parser;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value = "18")]
    age: u8,
}

fn main() {
    let args = Args::parse();
    dbg!(args);
}
