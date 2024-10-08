#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/79236386")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/79236386")]

pub mod assets;
pub mod builder;
pub mod cli;
pub mod config;
pub mod dioxus_crate;
pub mod dx_build_info;
pub mod error;
pub mod metadata;
pub mod serve;
pub mod settings;
pub mod tracer;

pub(crate) use cli::*;
pub(crate) use dioxus_crate::*;
pub(crate) use error::*;
pub(crate) use settings::*;
pub(crate) use tracer::{TraceMsg, TraceSrc};

use anyhow::Context;
use clap::Parser;

use Commands::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let log_control = tracer::build_tracing();

    match args.action {
        Translate(opts) => opts
            .translate()
            .context(error_wrapper("Translation of HTML into RSX failed")),

        New(opts) => opts
            .create()
            .context(error_wrapper("Creating new project failed")),

        Init(opts) => opts
            .init()
            .context(error_wrapper("Initializing a new project failed")),

        Config(opts) => opts
            .config()
            .context(error_wrapper("Configuring new project failed")),

        Autoformat(opts) => opts
            .autoformat()
            .context(error_wrapper("Error autoformatting RSX")),

        Check(opts) => opts
            .check()
            .await
            .context(error_wrapper("Error checking RSX")),

        Link(opts) => opts
            .link()
            .context(error_wrapper("Error with linker passthrough")),

        Build(mut opts) => opts
            .run()
            .await
            .context(error_wrapper("Building project failed")),

        Clean(opts) => opts
            .clean()
            .context(error_wrapper("Cleaning project failed")),

        Serve(opts) => opts
            .serve(log_control)
            .await
            .context(error_wrapper("Serving project failed")),

        Bundle(opts) => opts
            .bundle()
            .await
            .context(error_wrapper("Bundling project failed")),
    }
}

/// Simplifies error messages that use the same pattern.
fn error_wrapper(message: &str) -> String {
    format!("🚫 {message}:")
}
