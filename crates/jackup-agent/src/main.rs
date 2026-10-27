use clap::Parser;
use jackup_config::AgentConfig;

#[derive(Debug, Parser)]
#[command(name = "jackup-agent", about = "Headless Jackup target-server agent")]
struct Args {
    #[arg(long, default_value = "127.0.0.1:8787")]
    listen: Option<String>,
}

fn main() {
    let args = Args::parse();
    let default_config = AgentConfig::default();
    let listen = args.listen.unwrap_or(default_config.listen_address);
    println!("jackup-agent foundation ready; control endpoint: {listen}");
}
