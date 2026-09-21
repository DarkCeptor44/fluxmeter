// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod server;

use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;
use std::process::exit;

use crate::server::handle_server;

pub const NAME: &str = env!("CARGO_BIN_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct App {
    #[arg(
        short = 'H',
        long,
        help = "Host to listen on",
        env = "FM_HOST",
        default_value = "0.0.0.0"
    )]
    host: String,

    #[arg(
        short,
        long,
        help = "Port to listen on",
        env = "FM_PORT",
        default_value_t = 7890
    )]
    port: u16,

    #[arg(long, help = "Enable debug logging", env = "FM_DEBUG", default_value_t)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    if let Err(e) = main_impl().await {
        eprintln!("{NAME}: {e:?}");
        exit(1);
    }
}

async fn main_impl() -> Result<()> {
    let args = App::parse();

    handle_server(args).await
}
