use clap::Parser;

use crate::cmds::project::{ProjectListCliArgs, ProjectMetadataGetCliArgs};

use super::common::{validate_project_repo_path, GetArgs, ListArgs};

#[derive(Parser)]
pub struct ProjectCommand {
    #[clap(subcommand)]
    subcommand: ProjectSubcommand,
}

#[derive(Parser)]
enum ProjectSubcommand {
    #[clap(about = "Gather project information metadata")]
    Info(ProjectInfo),
    #[clap(about = "List project/repository tags")]
    Tags(ListProject),
}

#[derive(Parser)]
struct ProjectInfo {
    /// ID of the project
    #[clap(long, group = "id_or_repo")]
    pub id: Option<i64>,
    /// Path of the project in the format `OWNER/PROJECT_NAME`
    #[clap(long, group = "id_or_repo", value_name = "OWNER/PROJECT_NAME", value_parser=validate_project_repo_path)]
    pub repo: Option<String>,
    #[clap(flatten)]
    pub get_args: GetArgs,
}

#[derive(Parser)]
pub struct ListProject {
    #[clap(flatten)]
    pub list_args: ListArgs,
}

impl From<ProjectCommand> for ProjectOptions {
    fn from(options: ProjectCommand) -> Self {
        match options.subcommand {
            ProjectSubcommand::Info(options) => options.into(),
            ProjectSubcommand::Tags(options) => options.into(),
        }
    }
}

impl From<ProjectInfo> for ProjectOptions {
    fn from(options: ProjectInfo) -> Self {
        ProjectOptions::Info(
            ProjectMetadataGetCliArgs::builder()
                .id(options.id)
                .path(options.repo)
                .get_args(options.get_args.into())
                .build()
                .unwrap(),
        )
    }
}

impl From<ListProject> for ProjectOptions {
    fn from(options: ListProject) -> Self {
        ProjectOptions::Tags(
            ProjectListCliArgs::builder()
                .list_args(options.list_args.into())
                .tags(true)
                .build()
                .unwrap(),
        )
    }
}

pub enum ProjectOptions {
    Info(ProjectMetadataGetCliArgs),
    Tags(ProjectListCliArgs),
}

#[cfg(test)]
mod test {
    use crate::cli::{Args, Command};

    use super::*;

    #[test]
    fn test_project_cli_info() {
        let args = Args::parse_from(vec!["gr", "pj", "info", "--id", "1"]);
        let project_info = match args.command {
            Command::Project(ProjectCommand {
                subcommand: ProjectSubcommand::Info(options),
            }) => {
                assert_eq!(options.id, Some(1));
                options
            }
            _ => panic!("Expected ProjectCommand::Info"),
        };
        let options: ProjectOptions = project_info.into();
        match options {
            ProjectOptions::Info(options) => {
                assert_eq!(options.id, Some(1));
            }
            _ => panic!("Expected ProjectOptions::Info"),
        }
    }

    #[test]
    fn test_project_cli_list_tags() {
        let args = Args::parse_from(vec!["gr", "pj", "tags"]);
        let list_project = match args.command {
            Command::Project(ProjectCommand {
                subcommand: ProjectSubcommand::Tags(options),
            }) => options,
            _ => panic!("Expected ProjectCommand::Info"),
        };
        let options: ProjectOptions = list_project.into();
        match options {
            ProjectOptions::Tags(cli_args) => {
                assert!(cli_args.tags);
                assert!(!cli_args.stars);
            }
            _ => panic!("Expected ProjectOptions::Info"),
        }
    }
}
