mod diff_lib;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    target: String,

    #[arg(short, long, default_value = "")]
    out: String,

    #[arg(long, help = "Enable multi-threading (default: single-thread)")]
    multi_thread: bool,
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut source = diff_lib::comparsion_source::ComparsionSource::new_with_mode(args.multi_thread);
    println!("read base path....");
    source.read_base_path(args.source).await;

    println!("compare ....");
    source.compare_start(args.target.clone()).await;
    println!("compare end");
    source.result_output(args.out, args.target);
}
