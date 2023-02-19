use clap::Parser;
use jd_mini_proj_5::count_bigrams;

#[derive(Parser)]
#[clap(version = "1.0", author = "Jackie Du", about = None)]
struct Args {
    #[clap(short, long)]
    input_string: String,
}

fn main() {
    let args = Args::parse();
    let res = count_bigrams(args.input_string);

    // print hashmap result in a nice format
    for (key, value) in res {
        println!("{}: {}", key, value);
    }
}
