/// This script sets Alice's balance to 3_000_000_000 on the Relay chain.
///
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

#[cfg(feature = "paseo")]
mod paseo_interface;
#[cfg(not(feature = "paseo"))]
mod rococo_interface;

#[cfg(not(feature = "paseo"))]
mod relay {
	pub(crate) use crate::rococo_interface::api as runtime;
	pub(crate) type RuntimeCall = runtime::runtime_types::rococo_runtime::RuntimeCall;
	pub(crate) const UNIT: u128 = 1_000_000_000_000;
}

#[cfg(feature = "paseo")]
mod relay {
	pub(crate) use crate::paseo_interface::api as runtime;

	pub(crate) type RuntimeCall = runtime::runtime_types::paseo_runtime::RuntimeCall;
	pub(crate) const UNIT: u128 = 10_000_000_000;
}

use relay::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("Connecting to the Relay chain...");
	let relay_api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:8833").await?;

	println!("Preparing to set Alice's balance...");
	let new_balance = UNIT * 3_000_000_000;
	println!("New balance to be set for Alice: {}", new_balance);
	let set_alice_balance = RuntimeCall::Balances(runtime::balances::Call::force_set_balance {
		who: dev::alice().public_key().into(),
		new_free: new_balance,
	});

	println!("Creating SUDO call to set Alice's balance...");
	let sudo_set_balance = runtime::tx().sudo().sudo(set_alice_balance);
	let from = dev::alice();

	println!("Submitting the transaction to set Alice's balance...");
	let _ = relay_api
		.tx()
		.sign_and_submit_then_watch_default(&sudo_set_balance, &from)
		.await?
		.wait_for_finalized_success()
		.await?;

	println!("Alice's balance has been successfully set to: {}", new_balance);
	Ok(())
}
