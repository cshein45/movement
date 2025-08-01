use super::partial::MovementPartialNode;
use anyhow::Context;
use godfig::{backend::config_file::ConfigFile, Godfig};
use maptos_opt_executor::executor::TxExecutionResult;
use movement_config::Config;
use tokio::signal::unix::signal;
use tokio::signal::unix::SignalKind;
use tokio::sync::mpsc::unbounded_channel;

#[derive(Clone)]
pub struct Manager {
	godfig: Godfig<Config, ConfigFile>,
}

// Implements a very simple manager using a marker strategy pattern.
impl Manager {
	pub async fn new(file: tokio::fs::File) -> Result<Self, anyhow::Error> {
		let godfig = Godfig::new(ConfigFile::new(file), vec![]);
		Ok(Self { godfig })
	}

	pub async fn try_run(&self) -> Result<(), anyhow::Error> {
		let (stop_tx, stop_rx) = tokio::sync::watch::channel(());
		tokio::spawn({
			let mut sigterm =
				signal(SignalKind::terminate()).context("can't register to SIGTERM.")?;
			let mut sigint =
				signal(SignalKind::interrupt()).context("can't register to SIGKILL.")?;
			let mut sigquit = signal(SignalKind::quit()).context("can't register to SIGKILL.")?;
			async move {
				loop {
					tokio::select! {
						_ = sigterm.recv() => (),
						_ = sigint.recv() => (),
						_ = sigquit.recv() => (),
					};
					tracing::info!("Receive Terminate Signal");
					if let Err(err) = stop_tx.send(()) {
						tracing::warn!("Can't update stop watch channel because :{err}");
						return Err::<(), anyhow::Error>(anyhow::anyhow!(err));
					}
				}
			}
		});

		let config = self.godfig.try_wait_for_ready().await?;

		let (mempool_tx_exec_result_sender, mempool_commit_tx_receiver) =
			unbounded_channel::<Vec<TxExecutionResult>>();

		let node = MovementPartialNode::try_from_config(config, mempool_tx_exec_result_sender)
			.await
			.context("Failed to create the executor")?;

		let join_handle = tokio::spawn(node.run(mempool_commit_tx_receiver, stop_rx));
		join_handle.await??;

		Ok(())
	}
}
