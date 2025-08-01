use aptos_crypto::SigningKey;
use aptos_crypto::{
	ed25519::Ed25519PrivateKey, Genesis, HashValue, Uniform, ValidCryptoMaterialStringExt,
};
use aptos_types::chain_id::ChainId;
use godfig::env_default;
use movement_signer::key::TryFromCanonicalString;
use movement_signer_loader::identifiers::{local::Local, SignerIdentifier};
use std::str::FromStr;
use url::Url;

// The default Maptos API listen hostname
env_default!(
	default_maptos_rest_listen_hostname,
	"MAPTOS_API_LISTEN_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default Maptos API listen port
env_default!(default_maptos_rest_listen_port, "MAPTOS_API_LISTEN_PORT", u16, 30731);

// The default Maptos API connection hostname
env_default!(
	default_maptos_rest_connection_hostname,
	"MAPTOS_API_CONNECTION_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default Maptos API connection port
env_default!(default_maptos_rest_connection_port, "MAPTOS_API_CONNECTION_PORT", u16, 30731);

// The default faucet API listen hostname
env_default!(
	default_maptos_faucet_rest_listen_hostname,
	"FAUCET_API_LISTEN_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default faucet API listen port
env_default!(default_maptos_faucet_rest_listen_port, "FAUCET_API_LISTEN_PORT", u16, 30732);

// The default faucet API connection hostname
env_default!(
	default_maptos_faucet_rest_connection_hostname,
	"FAUCET_API_CONNECTION_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default faucet API connection port
env_default!(default_maptos_faucet_rest_connection_port, "FAUCET_API_CONNECTION_PORT", u16, 30732);

// The default fin API listen hostname
env_default!(
	default_fin_rest_listen_hostname,
	"MAPTOS_FIN_VIEW_API_LISTEN_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default fin API listen port
env_default!(default_fin_rest_listen_port, "MAPTOS_FIN_VIEW_API_LISTEN_PORT", u16, 30733);

// The default fin API connection hostname
env_default!(
	default_fin_rest_connection_hostname,
	"MAPTOS_FIN_VIEW_API_CONNECTION_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

// The default Da Sequencer connection url
env_default!(
	default_da_sequencer_connection_url,
	"MAPTOS_DA_SEQUENCER_CONNECTION_URL",
	Url,
	"http://0.0.0.0:30730".parse().expect("Bad da sequencer connection url.")
);

env_default!(
	default_stream_heartbeat_interval_sec,
	"MOVEMENT_DA_STREAM_HEARTBEAT_INTERVAL_MILLISEC",
	u64,
	10
);

env_default!(
	default_propagate_execution_state,
	"MOVEMENT_DA_PROPAGATE_EXECUTION_STATE",
	bool,
	false
);

pub fn default_batch_signer_identifier() -> SignerIdentifier {
	match std::env::var("MAPTOS_DA_SEQUENCER_SIGNER_IDENTIFIER") {
		Ok(val) => SignerIdentifier::try_from_canonical_string(&val).unwrap(),
		Err(_) => {
			// encoded key string
			let key = Ed25519PrivateKey::generate(&mut rand::rngs::OsRng);

			let verifying_key = key.verifying_key();
			tracing::info!(
				"Using batch signing public key: {}",
				verifying_key.to_encoded_string().unwrap()
			);

			// remove the 0x prefix
			let key = key.to_encoded_string().unwrap();
			let key = key.trim_start_matches("0x");

			SignerIdentifier::Local(Local { private_key_hex_bytes: key.to_string() })
		}
	}
}

// The default chain id
env_default!(default_maptos_chain_id, "MAPTOS_CHAIN_ID", ChainId, ChainId::from_str("27").unwrap());

// The default read-only mode
env_default!(default_maptos_read_only, "MAPTOS_READ_ONLY", bool, false);

// The default private key
pub fn default_maptos_private_key() -> Ed25519PrivateKey {
	match std::env::var("MAPTOS_PRIVATE_KEY") {
		Ok(val) => match val.as_str() {
			"random" => Ed25519PrivateKey::generate(&mut rand::rngs::OsRng),
			_ => Ed25519PrivateKey::from_encoded_string(&val).unwrap(),
		},
		Err(_) => Ed25519PrivateKey::genesis(),
	}
}

// The default MAPTOS_PRIVATE_KEY_SIGNER_IDENTIFIER
pub fn default_maptos_private_key_signer_identifier() -> SignerIdentifier {
	match std::env::var("MAPTOS_PRIVATE_KEY_SIGNER_IDENTIFIER") {
		Ok(val) => SignerIdentifier::try_from_canonical_string(&val).unwrap(),
		Err(_) => {
			// encoded key string
			let key = default_maptos_private_key().to_encoded_string().unwrap();

			// remove the 0x prefix
			let key = key.trim_start_matches("0x");

			SignerIdentifier::Local(Local { private_key_hex_bytes: key.to_string() })
		}
	}
}

env_default!(
	default_maptos_indexer_grpc_listen_hostname,
	"MAPTOS_INDEXER_GRPC_LISTEN_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

env_default!(
	default_maptos_indexer_grpc_listen_port,
	"MAPTOS_INDEXER_GRPC_LISTEN_PORT",
	u16,
	30734
);

env_default!(
	default_maptos_indexer_grpc_connection_hostname,
	"MAPTOS_INDEXER_GRPC_CONNECTION_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

env_default!(
	default_maptos_indexer_grpc_connection_port,
	"MAPTOS_INDEXER_GRPC_CONNECTION_PORT",
	u16,
	30734
);

env_default!(
	default_maptos_indexer_healthcheck_hostname,
	"MAPTOS_INDEXER_HEALTHCHECK_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

env_default!(default_maptos_indexer_healthcheck_port, "MAPTOS_INDEXER_HEALTHCHECK_PORT", u16, 8084);

env_default!(
	default_maptos_indexer_grpc_inactivity_timeout,
	"MAPTOS_INDEXER_GRPC_INACTIVITY_TIMEOUT_SEC",
	u64,
	60
);

env_default!(
	default_maptos_indexer_grpc_ping_interval,
	"MAPTOS_INDEXER_GRPC_PING_INTERVAL_SEC",
	u64,
	10
);

env_default!(default_enable_pruning, "MAPTOS_ENABLE_PRUNING", bool, false);

env_default!(default_maptos_ledger_prune_window, "MAPTOS_LEDGER_PRUNING_WINDOW", u64, 50_000_000);

env_default!(
	default_maptos_state_merkle_prune_window,
	"MAPTOS_STATE_MERKLE_PRUNING_WINDOW",
	u64,
	100_000 // see comment in aptos-core, this essentially needs to exceed TPS
);

env_default!(
	default_maptos_epoch_snapshot_prune_window,
	"MAPTOS_EPOCH_SNAPSHOT_PRUNING_WINDOW",
	u64,
	50_000_000
);

env_default!(
	default_postgres_connection_string,
	"INDEXER_PROCESSOR_POSTGRES_CONNECTION_STRING",
	String,
	"postgresql://postgres:password@localhost:5432".to_string()
);

env_default!(
	default_indexer_processor_auth_token,
	"INDEXER_PROCESSOR_AUTH_TOKEN",
	String,
	"auth_token".to_string()
);

env_default!(
	default_genesis_timestamp_microseconds,
	"MAPTOS_GENESIS_TIMESTAMP_MICROSECONDS",
	u64,
	1_600_000_000_000
);

env_default!(
	default_genesis_block_hash_hex,
	"MAPTOS_GENESIS_BLOCK_HASH",
	String,
	HashValue::sha3_256_of(b"maptos").to_hex()
);

env_default!(
	default_health_server_hostname,
	"HEALTH_SERVER_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);

env_default!(default_health_server_port, "HEALTH_SERVER_PORT", u16, 18085);

env_default!(
	default_metrics_server_hostname,
	"METRICS_SERVER_HOSTNAME",
	String,
	"0.0.0.0".to_string()
);
env_default!(default_metrics_server_port, "METRICS_SERVER_PORT", u16, 18185);

env_default!(default_batch_production_time, "MAPTOS_BATCH_PRODUCTION_TIME_MS", u64, 2000);

env_default!(default_max_transactions_in_flight, "MAPTOS_MAX_TRANSACTIONS_IN_FLIGHT", u64);

env_default!(default_sequence_number_ttl_ms, "MAPTOS_SEQUENCE_NUMBER_TTL_MS", u64, 1000 * 60 * 3);

env_default!(default_gc_slot_duration_ms, "MAPTOS_GC_SLOT_DURATION_MS", u64, 1000 * 2);

env_default!(default_ingress_account_whitelist, "MAPTOS_INGRESS_ACCOUNT_WHITELIST", String);

env_default!(default_max_tx_per_batch, "MAPTOS_MAX_TX_PER_BATCH", u64, 1024 * 8);

env_default!(
	default_max_batch_size,
	"MAPTOS_MAX_BATCH_SIZE",
	u64,
	1024 * 1024 //1Mo
);
