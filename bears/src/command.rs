use bears_ecology::{
    History, Mode, Overwrite, Style, download_with_history, init_queue, initial_load,
};
use bears_species::{BeaErr, Dataset, write_json};
use clap::Parser;
// use indicatif::ProgressBar;

#[derive(Parser, derive_getters::Getters)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'c', long, help = "Command to execute.")]
    command: Action,
    #[arg(short = 'd', long, help = "Dataset on which to apply command.")]
    dataset: Option<Dataset>,
    #[arg(
        short = 'y',
        long,
        help = "Use history log for context.",
        default_value_t = false
    )]
    history: bool,
    #[arg(short = 'o', long, help = "Output file name & path.")]
    output: Option<std::path::PathBuf>,
    #[arg(
        short = 'x',
        long,
        help = "Overwrite existing data.",
        default_value_t = false
    )]
    overwrite: bool,
    #[arg(short = 's', long, help = "Source of file.")]
    source: Option<std::path::PathBuf>,
}

impl Cli {
    #[tracing::instrument(skip_all)]
    pub async fn act(&self) -> Result<(), BeaErr> {
        self.command.act(self).await
    }
}

/// Variants of the `Action` enum encapsulate the different actions a user can select, exposing the
/// different capabilities of the library.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::FromStr)]
pub enum Action {
    Load,
    Download,
    NextError,
    Queue,
}

impl Action {
    #[tracing::instrument(skip_all)]
    pub async fn act(&self, cli: &Cli) -> Result<(), BeaErr> {
        match self {
            Self::Download => {
                if let Some(dataset) = cli.dataset {
                    tracing::info!("Downloading {dataset}.");
                    let styles = Style::try_new()?;
                    let style = styles["queue_download"].clone();
                    if cli.history {
                        download_with_history(dataset, style).await?;
                    } else {
                        let queue = init_queue(dataset)?;
                        tracing::info!("Queue length: {}", queue.len());
                        if cli.overwrite {
                            queue.download(Overwrite::Yes).await?;
                        } else {
                            queue.download(Overwrite::No).await?;
                        }
                    }
                    tracing::info!("Datasets downloaded.");
                } else {
                    tracing::warn!("Dataset parameter is missing, add '-d MyDataset' to args.");
                }
            }
            Self::Load => {
                if let Some(dataset) = cli.dataset {
                    tracing::info!("Loading {dataset}.");
                    let result = initial_load(dataset, None).await?;
                    tracing::info!("{} datasets loaded.", result.len());
                } else {
                    tracing::warn!("Dataset parameter is missing, add '-d MyDataset' to args.");
                }
            }
            Self::NextError => {
                if let Some(dataset) = cli.dataset {
                    let mut queue = init_queue(dataset)?;
                    tracing::info!("Queue length: {}", queue.len());
                    let history = History::try_from((dataset, Mode::Load))?;
                    queue.errors(&history, bears_ecology::Scope::History)?;
                    if let Some(req) = queue.first() {
                        tracing::info!("Loading first MNE error.");
                        req.load()?;
                    }
                    tracing::info!("MNE file successfully loaded.");
                } else {
                    tracing::warn!("Dataset parameter is missing, add '-d MyDataset' to args.");
                }
            }
            Self::Queue => {
                if let Some(dataset) = cli.dataset {
                    let queue = init_queue(dataset)?;
                    if let Some(out) = &cli.output {
                        write_json(&queue, out)?;
                    } else {
                        tracing::warn!("Output path is missing, add '-o MyPath' to args.");
                    }
                } else {
                    tracing::warn!("Dataset parameter is missing, add '-d MyDataset' to args.");
                }
            }
        }
        Ok(())
    }
}
