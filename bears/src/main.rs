use bears::Cli;
use bears_ecology::trace_init;
use bears_species::BeaErr;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    tracing::trace!("Environmental variables loaded.");

    // let url = EnvError::from_env("BEA_URL")?;
    // let url = UrlParseError::into_url(&url)?;
    // let key = EnvError::from_env("API_KEY")?;
    //
    // let style = indicatif::ProgressStyle::with_template(
    //     "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Downloading data.'}",
    // )
    // .unwrap();
    let cli = Cli::parse();
    cli.act().await?;
    Ok(())
}
