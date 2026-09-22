// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod api;
mod utils;

use crate::{
    App, NAME, VERSION,
    server::utils::{ServerArgs, Service, init_logger},
};
use anyhow::{Context, Result, anyhow};
use log::{debug, error, info};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    process::exit,
    sync::Arc,
};
use tokio::net::TcpListener;

pub async fn handle_server(args: App) -> Result<()> {
    if let Err(e) = init_logger(args.debug).context("Failed to init logger") {
        eprintln!("CRITICAL: {e:?}");
        exit(1);
    }

    let header_text = format!(" Fluxmeter v{VERSION} ");
    let line_width: usize = 51;

    let total_dashes = line_width.saturating_sub(header_text.len());
    let left_dashes = "-".repeat(total_dashes / 2);
    let right_dashes = "-".repeat(total_dashes - left_dashes.len());

    info!(
        "\n===================================================\n{left_dashes}{header_text}{right_dashes}\n==================================================="
    );
    let server_args = ServerArgs {
        host: args.host,
        port: args.port,
    };
    if let Err(e) = serve(server_args).await {
        error!("{NAME} application logic: {e:?}");
        exit(1);
    }

    Ok(())
}

async fn serve(args: ServerArgs) -> Result<()> {
    let service = Service;
    debug!("service={service:?}");

    let app = api::routes(&service).with_state(Arc::new(service));
    let addr = SocketAddr::new(
        args.host
            .parse::<IpAddr>()
            .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
        args.port,
    );
    let listener = TcpListener::bind(addr)
        .await
        .context(anyhow!("Failed to bind to address: {addr}"))?;
    info!(
        "\n    listening on http://{}:{}\n    listening on http://localhost:{}\n",
        args.host, args.port, args.port
    );

    let shutdown_signal = async {
        let event: &str;

        #[cfg(unix)]
        {
            let ctrl_c = async {
                tokio::signal::ctrl_c()
                    .await
                    .expect("failed to install SIGINT handler");
            };
            let mut terminate =
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .expect("Failed to install signal handler");

            tokio::select! {
                _ = ctrl_c => event = "SIGINT",
                _ = terminate.recv() => event = "SIGTERM",
            };
        }

        #[cfg(windows)]
        {
            use tokio::signal::windows::{ctrl_break, ctrl_c};
            let mut sig_c = ctrl_c().expect("failed to install ctrl+c handler");
            let mut sig_break = ctrl_break().expect("failed to install ctrl+break handler");

            tokio::select! {
                _ = sig_c.recv() => event = "CTRL_C_EVENT",
                _ = sig_break.recv() => event = "CTRL_BREAK_EVENT",
            };
        }

        info!("{event} signal received, shutting down...");
    };

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal)
        .await
        .context("Failed to serve axum app")
}
