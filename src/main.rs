mod diff_lib;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    target: String,
}
fn main() {
    let args = Args::parse();
    let mut source = diff_lib::comparsion_source::ComparsionSource::new();
    source.read_base_path(args.source);
    source.compare_start(args.target);
    println!("source files => {}", &source.file_list.len());
    println!(
        "not compared files =>  {},",
        &source.not_compared_list().len()
    );
    println!("compare error files => {:?}", &source.compare_error);
}
