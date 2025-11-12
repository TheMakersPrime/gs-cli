use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct GSArgs {
    #[command(subcommand)]
    pub command: Command,

    #[arg(long, global = true, required = false)]
    pub credential: String,

    #[arg(long, global = true, required = false)]
    pub sheet_id: String,

    #[arg(long, global = true, required = false)]
    pub sheet_name: String,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add PR to sheet
    #[command(arg_required_else_help = true)]
    Add(PRData),

    /// Mark PR as done
    Done(GitBranchWrapper),

    /// Fetch prettified google sheet data
    Fetch,
}

#[derive(Debug, Args)]
pub struct GitBranchWrapper {
    #[command(subcommand)]
    pub command: GitBranch,
}

#[derive(Debug, Subcommand)]
pub enum GitBranch {
    /// Mark PR as merged in rc
    #[command(arg_required_else_help = true)]
    Rc(PRTitle),

    /// Mark PR as merged in master
    #[command(arg_required_else_help = true)]
    Master(PRTitle),
}

#[derive(Debug, Args)]
pub struct PRTitle {
    /// List of pr title to be marked as done
    /// Ech item to be preceded with -t or --title
    #[arg(short, long)]
    pub title: Vec<String>,
}

#[derive(Debug, Args)]
pub struct PRData {
    /// PR information;
    /// List of data to be added to the sheet
    /// Each item to be preceded with -d or --data
    #[arg(short, long)]
    pub data: Vec<String>,
}
