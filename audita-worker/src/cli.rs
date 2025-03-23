use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "audita-worker")]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "./audita.toml")]
    pub config: String,

    #[arg(long)]
    pub batch: usize,

    #[arg(long, default_value_t = false)]
    pub disable_elastic: bool,

    #[arg(long, default_value_t = false)]
    pub disable_ethereum: bool,
}
