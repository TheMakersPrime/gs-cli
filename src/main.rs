mod error;
mod gs_args;
mod gs_sheet;
mod print;
mod sheet;
mod sheet_model;

use crate::gs_args::{Command, GSArgs, GitBranch};
use crate::print::{fprint, MessageType};
use clap::Parser;
use std::process;

#[tokio::main]
async fn main() {
    let args = GSArgs::parse();

    let credentials = args.credential.as_str();
    let sheet_id = args.sheet_id.as_str();
    let sheet_name = args.sheet_name.as_str();

    if credentials.is_empty() {
        fprint("Credential is missing".to_string(), MessageType::Error);
        process::exit(0);
    }

    if sheet_id.is_empty() {
        fprint("Sheet ID is missing".to_string(), MessageType::Error);
        process::exit(0);
    }

    if sheet_name.is_empty() {
        fprint("Sheet name is missing".to_string(), MessageType::Error);
        process::exit(0);
    }

    let hub = match sheet::get(credentials.to_string()).await {
        Ok(sheet) => sheet,
        Err(error) => {
            fprint(error.to_string(), MessageType::Error);
            process::exit(0);
        }
    };

    let command_result = match args.command {
        Command::Add(pr_data) => gs_sheet::add(&hub, pr_data, sheet_id, sheet_name).await,
        Command::Done(branch) => match branch.command {
            GitBranch::Rc(pr_title) => {
                gs_sheet::done(&hub, pr_title.title, "rc", sheet_id, sheet_name).await
            }
            GitBranch::Master(pr_title) => {
                gs_sheet::done(&hub, pr_title.title, "master", sheet_id, sheet_name).await
            }
        },
        Command::Fetch => gs_sheet::fetch(&hub, sheet_id, sheet_name).await,
    };

    match command_result {
        Ok(sheet) => fprint(sheet, MessageType::Success),
        Err(error) => {
            fprint(error.to_string(), MessageType::Error);
            process::exit(0);
        }
    }
}
