// Easy Error enum magic
use anyhow::Result;

// Import core GHActions macros
use ghactions::{info, debug, warn, group, groupend};
use octocrab::{params::State, models::issues::Issue};


async fn workflow() -> Result<()> {
    println!("PLEASE WORK!!");

    let mut action = match ghactions::init() {
        Ok(a) => a,
        Err(_err) => {
            eprintln!("Failed to load ghactions...");
            std::process::exit(1);
        }
    };

    if ! action.in_action() {
        warn!("Failed to load action.yml file");
    }

    debug!("GitHub Action Name :: {}", &action.name.clone().unwrap_or_else(|| "N/A".to_string()));

    group!("Main Workflow");

    info!("Repository: `{}`", action.repository.display());

    let client = action.client
        .expect("Failed loading client..");

    // https://docs.rs/octocrab/latest/octocrab/index.html
    // Example to get all the active issues
    let issues_pages = client.issues(action.repository.owner, action.repository.name)
        .list()
        .state(State::Open)
        .per_page(50)
        .send().await?;

    for issue in client.all_pages::<Issue>(issues_pages).await? {
        info!(" >> {} -> {}", issue.id, issue.title);
    }

    groupend!();

    Ok(())
}

#[tokio::main]
async fn main() {
    match workflow().await {
        Ok(_) => println!("Successfully Exiting..."),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}


