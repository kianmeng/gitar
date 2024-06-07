use crate::display;
use crate::remote::MergeRequestListBodyArgs;
/// Common functions and macros that are used by multiple commands
use crate::Result;
use crate::{api_traits::MergeRequest, remote::ListRemoteCliArgs};
use std::fmt::Display;
use std::io::Write;
use std::sync::Arc;

use crate::api_traits::{
    Cicd, CicdRunner, CodeGist, CommentMergeRequest, Deploy, DeployAsset, RemoteProject,
    TrendingProjectURL,
};

use super::cicd::{RunnerListBodyArgs, RunnerListCliArgs};
use super::gist::{GistListBodyArgs, GistListCliArgs};
use super::merge_request::{CommentMergeRequestListBodyArgs, CommentMergeRequestListCliArgs};
use super::project::{ProjectListBodyArgs, ProjectListCliArgs};
use super::release::{ReleaseAssetListBodyArgs, ReleaseAssetListCliArgs, ReleaseBodyArgs};
use super::trending::TrendingCliArgs;
use super::{cicd::PipelineBodyArgs, merge_request::MergeRequestListCliArgs};

macro_rules! query_pages {
    ($func_name:ident, $trait_name:ident) => {
        pub fn $func_name<W: Write>(remote: Arc<dyn $trait_name>, mut writer: W) -> Result<()> {
            process_num_metadata(remote.num_pages(), MetadataName::Pages, &mut writer)
        }
    };
    ($func_name:ident, $trait_name:ident, $body_args:ident) => {
        pub fn $func_name<W: Write>(
            remote: Arc<dyn $trait_name>,
            body_args: $body_args,
            mut writer: W,
        ) -> Result<()> {
            process_num_metadata(
                remote.num_pages(body_args),
                MetadataName::Pages,
                &mut writer,
            )
        }
    };
}

macro_rules! query_num_resources {
    ($func_name:ident, $trait_name:ident) => {
        pub fn $func_name<W: Write>(remote: Arc<dyn $trait_name>, mut writer: W) -> Result<()> {
            process_num_metadata(remote.num_resources(), MetadataName::Resources, &mut writer)
        }
    };
    ($func_name:ident, $trait_name:ident, $body_args:ident) => {
        pub fn $func_name<W: Write>(
            remote: Arc<dyn $trait_name>,
            body_args: $body_args,
            mut writer: W,
        ) -> Result<()> {
            process_num_metadata(
                remote.num_resources(body_args),
                MetadataName::Resources,
                &mut writer,
            )
        }
    };
}

#[derive(Debug)]
pub enum MetadataName {
    Pages,
    Resources,
}

impl Display for MetadataName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetadataName::Pages => write!(f, "pages"),
            MetadataName::Resources => write!(f, "resources"),
        }
    }
}

pub fn process_num_metadata<W: Write, T: Display>(
    num_metadata: Result<Option<T>>,
    resource_name: MetadataName,
    mut writer: W,
) -> Result<()> {
    let none_msg_info = format!(
        "Number of {resource_name} not available.\n",
        resource_name = resource_name
    );
    match num_metadata {
        Ok(Some(count)) => writer.write_all(format!("{total}\n", total = count).as_bytes())?,
        Ok(None) => {
            writer.write_all(none_msg_info.as_bytes())?;
        }
        Err(e) => {
            return Err(e);
        }
    };
    Ok(())
}

query_pages!(num_release_pages, Deploy);
query_pages!(
    num_release_asset_pages,
    DeployAsset,
    ReleaseAssetListBodyArgs
);
query_pages!(num_cicd_pages, Cicd);
query_pages!(num_runner_pages, CicdRunner, RunnerListBodyArgs);

query_pages!(
    num_merge_request_pages,
    MergeRequest,
    MergeRequestListBodyArgs
);
query_pages!(num_project_pages, RemoteProject, ProjectListBodyArgs);
query_num_resources!(num_project_resources, RemoteProject, ProjectListBodyArgs);
query_pages!(
    num_comment_merge_request_pages,
    CommentMergeRequest,
    CommentMergeRequestListBodyArgs
);

query_num_resources!(num_release_resources, Deploy);
query_num_resources!(
    num_release_asset_resources,
    DeployAsset,
    ReleaseAssetListBodyArgs
);
query_num_resources!(num_cicd_resources, Cicd);
query_num_resources!(num_runner_resources, CicdRunner, RunnerListBodyArgs);
query_num_resources!(
    num_merge_request_resources,
    MergeRequest,
    MergeRequestListBodyArgs
);
query_pages!(
    num_comment_merge_request_resources,
    CommentMergeRequest,
    CommentMergeRequestListBodyArgs
);

macro_rules! list_resource {
    ($func_name:ident, $trait_name:ident, $body_args:ident, $cli_args:ident, $embeds_list_args: literal) => {
        pub fn $func_name<W: Write>(
            remote: Arc<dyn $trait_name>,
            body_args: $body_args,
            cli_args: $cli_args,
            mut writer: W,
        ) -> Result<()> {
            let objs = list_remote_objs!(remote, body_args, cli_args.list_args, writer);
            display::print(&mut writer, objs, cli_args.list_args.get_args)?;
            Ok(())
        }
    };
    ($func_name:ident, $trait_name:ident, $body_args:ident, $cli_args:ident) => {
        pub fn $func_name<W: Write>(
            remote: Arc<dyn $trait_name>,
            body_args: $body_args,
            cli_args: $cli_args,
            mut writer: W,
        ) -> Result<()> {
            let objs = list_remote_objs!(remote, body_args, cli_args, writer);
            display::print(&mut writer, objs, cli_args.get_args)?;
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! list_remote_objs {
    ($remote:expr, $body_args:expr, $cli_args:expr, $writer:expr) => {{
        let objs = $remote.list($body_args)?;
        if $cli_args.flush {
            return Ok(());
        }
        if objs.is_empty() {
            $writer.write_all(b"No resources found.\n")?;
            return Ok(());
        }
        objs
    }};
}

list_resource!(
    list_merge_requests,
    MergeRequest,
    MergeRequestListBodyArgs,
    MergeRequestListCliArgs,
    true
);

list_resource!(list_pipelines, Cicd, PipelineBodyArgs, ListRemoteCliArgs);
list_resource!(
    list_runners,
    CicdRunner,
    RunnerListBodyArgs,
    RunnerListCliArgs,
    true
);

list_resource!(list_releases, Deploy, ReleaseBodyArgs, ListRemoteCliArgs);
list_resource!(
    list_release_assets,
    DeployAsset,
    ReleaseAssetListBodyArgs,
    ReleaseAssetListCliArgs,
    true
);

list_resource!(
    list_user_projects,
    RemoteProject,
    ProjectListBodyArgs,
    ProjectListCliArgs,
    true
);

list_resource!(
    list_user_gists,
    CodeGist,
    GistListBodyArgs,
    GistListCliArgs,
    true
);

list_resource!(
    list_merge_request_comments,
    CommentMergeRequest,
    CommentMergeRequestListBodyArgs,
    CommentMergeRequestListCliArgs,
    true
);

list_resource!(list_trending, TrendingProjectURL, String, TrendingCliArgs);
