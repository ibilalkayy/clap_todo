use clap:: {
    Args,
    Parser,
    Subcommand
};

use serde::Serialize;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Todo {
    #[clap(subcommand)]
    pub manage_tasks: ManageTasks,
}

#[derive(Debug, Subcommand)]
pub enum ManageTasks {
    /// Add the task
    Add(AddTask),

    /// Update the task
    Update(UpdateTask),

    /// View the task
    View(ViewTask),
}

#[derive(Debug, Args, Serialize)]
pub struct AddTask {
    /// The title of the task
    pub title: String,

    /// The description of the task
    pub description: String,

    /// The name of the file
    pub filename: String,
}

#[derive(Debug, Args)]
pub struct UpdateTask {
    /// The title of the task
    pub title: String,

    /// The description of the task
    pub description: String,

    /// The name of the file
    pub filename: String,
}

#[derive(Debug, Args)]
pub struct ViewTask {
    /// The title of the task
    pub title: String,

    /// The name of the file
    pub filename: String,
}