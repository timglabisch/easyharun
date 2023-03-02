mod container_manager;
mod config;
mod docker;
mod brain;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {

}

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();

    println!("Hello, world!");
}
