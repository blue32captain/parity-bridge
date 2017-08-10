use std::path::{PathBuf, Path};
use std::fs;
use std::io::Read;
use std::time::Duration;
use web3::types::{Address, Bytes};
use error::{ResultExt, Error};
use {toml, ethabi};

const DEFAULT_POLL_INTERVAL: u64 = 1;
const DEFAULT_CONFIRMATIONS: u64 = 12;

/// Application config.
#[derive(Debug, PartialEq)]
pub struct Config {
	pub mainnet: Node,
	pub testnet: Node,
}

impl Config {
	pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
		let mut file = fs::File::open(path).chain_err(|| "Cannot open config")?;
		let mut buffer = String::new();
		file.read_to_string(&mut buffer);
		Self::load_from_str(&buffer)
	}

	fn load_from_str(s: &str) -> Result<Config, Error> {
		let config: load::Config = toml::from_str(s).chain_err(|| "Cannot parse config")?;
		Config::from_load_struct(config)
	}

	fn from_load_struct(config: load::Config) -> Result<Config, Error> {
		let result = Config {
			mainnet: Node::from_load_struct(config.mainnet, NodeDefaults::mainnet())?,
			testnet: Node::from_load_struct(config.testnet, NodeDefaults::testnet())?,
		};

		Ok(result)
	}
}

#[derive(Debug, PartialEq)]
pub struct Node {
	pub account: Address,
	pub contract: ContractConfig,
	pub ipc: PathBuf,
	pub deploy_tx: TransactionConfig,
	pub poll_interval: Duration,
	pub required_confirmations: u64,
}

struct NodeDefaults {
	ipc: PathBuf,
}

impl NodeDefaults {
	fn mainnet() -> Self {
		NodeDefaults {
			ipc: "".into(),
		}
	}

	fn testnet() -> Self {
		NodeDefaults {
			ipc: "".into(),
		}
	}
}

impl Node {
	fn from_load_struct(node: load::Node, defaults: NodeDefaults) -> Result<Node, Error> {
		let result = Node {
			account: node.account,
			contract: ContractConfig {
				bin: Bytes(fs::File::open(node.contract.bin)?.bytes().collect::<Result<_, _>>()?),
				abi: ethabi::Contract::load(fs::File::open(node.contract.abi)?)?,
			},
			ipc: node.ipc.unwrap_or(defaults.ipc),
			deploy_tx: TransactionConfig {
				gas: node.deploy_tx.as_ref().and_then(|tx| tx.gas).unwrap_or_default(),
				gas_price: node.deploy_tx.as_ref().and_then(|tx| tx.gas_price).unwrap_or_default(),
				value: node.deploy_tx.as_ref().and_then(|tx| tx.value).unwrap_or_default(),
			},
			poll_interval: Duration::from_secs(node.poll_interval.unwrap_or(DEFAULT_POLL_INTERVAL)),
			required_confirmations: node.required_confirmations.unwrap_or(DEFAULT_CONFIRMATIONS),
		};
	
		Ok(result)
	}
}

#[derive(Debug, PartialEq)]
pub struct TransactionConfig {
	pub gas: u64,
	pub gas_price: u64,
	pub value: u64,
}

#[derive(Debug, PartialEq)]
pub struct ContractConfig {
	pub bin: Bytes,
	pub abi: ethabi::Contract,
}

/// Some config values may not be defined in `toml` file, but they should be specified at runtime.
/// `load` module separates `Config` representation in file with optional from the one used 
/// in application.
mod load {
	use std::path::PathBuf;
	use web3::types::Address;

	#[derive(Deserialize)]
	#[serde(deny_unknown_fields)]
	pub struct Config {
		pub mainnet: Node,
		pub testnet: Node,
	}

	#[derive(Deserialize)]
	pub struct Node {
		pub account: Address,
		pub contract: ContractConfig,
		pub ipc: Option<PathBuf>,
		pub deploy_tx: Option<TransactionConfig>,
		pub poll_interval: Option<u64>,
		pub required_confirmations: Option<u64>,
	}

	#[derive(Deserialize)]
	pub struct TransactionConfig {
		pub gas: Option<u64>,
		pub gas_price: Option<u64>,
		pub value: Option<u64>,
	}

	#[derive(Deserialize)]
	pub struct ContractConfig {
		pub bin: PathBuf,
		pub abi: PathBuf,
	}
}

#[cfg(test)]
mod tests {
	use std::time::Duration;
	use ethabi;
	use super::{Config, Node, TransactionConfig, ContractConfig};

	#[test]
	fn load_full_setup_from_str() {
		let toml = r#"
[mainnet]
account = "0x1B68Cb0B50181FC4006Ce572cF346e596E51818b"
ipc = "/mainnet.ipc"
poll_interval = 2
required_confirmations = 100

[mainnet.contract]
bin = "contracts/EthereumBridge.bin"
abi = "contracts/EthereumBridge.abi"

[testnet]
account = "0x0000000000000000000000000000000000000001"
ipc = "/testnet.ipc"
deploy_tx = { gas = 20, value = 15 }

[testnet.contract]
bin = "contracts/KovanBridge.bin"
abi = "contracts/KovanBridge.abi"
"#;

		let expected = Config {
			mainnet: Node {
				account: "0x1B68Cb0B50181FC4006Ce572cF346e596E51818b".parse().unwrap(),
				ipc: "/mainnet.ipc".into(),
				contract: ContractConfig {
					bin: include_bytes!("../contracts/EthereumBridge.bin").to_vec().into(),
					abi: ethabi::Contract::load(include_bytes!("../contracts/EthereumBridge.abi") as &[u8]).unwrap(),
				},
				deploy_tx: TransactionConfig {
					gas: 0,
					gas_price: 0,
					value: 0,
				},
				poll_interval: Duration::from_secs(2),
				required_confirmations: 100,
			},
			testnet: Node {
				account: "0x0000000000000000000000000000000000000001".parse().unwrap(),
				contract: ContractConfig {
					bin: include_bytes!("../contracts/KovanBridge.bin").to_vec().into(),
					abi: ethabi::Contract::load(include_bytes!("../contracts/KovanBridge.abi") as &[u8]).unwrap(),
				},
				ipc: "/testnet.ipc".into(),
				deploy_tx: TransactionConfig {
					gas: 20,
					gas_price: 0,
					value: 15,
				},
				poll_interval: Duration::from_secs(1),
				required_confirmations: 12,
			}
		};

		let config = Config::load_from_str(toml).unwrap();
		assert_eq!(expected, config);
	}

	#[test]
	fn laod_minimal_setup_from_str() {
		let toml = r#"
[mainnet]
account = "0x1B68Cb0B50181FC4006Ce572cF346e596E51818b"

[mainnet.contract]
bin = "contracts/EthereumBridge.bin"
abi = "contracts/EthereumBridge.abi"

[testnet]
account = "0x0000000000000000000000000000000000000001"

[testnet.contract]
bin = "contracts/KovanBridge.bin"
abi = "contracts/KovanBridge.abi"
"#;
		let expected = Config {
			mainnet: Node {
				account: "0x1B68Cb0B50181FC4006Ce572cF346e596E51818b".parse().unwrap(),
				ipc: "".into(),
				contract: ContractConfig {
					bin: include_bytes!("../contracts/EthereumBridge.bin").to_vec().into(),
					abi: ethabi::Contract::load(include_bytes!("../contracts/EthereumBridge.abi") as &[u8]).unwrap(),
				},
				deploy_tx: TransactionConfig {
					gas: 0,
					gas_price: 0,
					value: 0,
				},
				poll_interval: Duration::from_secs(1),
				required_confirmations: 12,
			},
			testnet: Node {
				account: "0x0000000000000000000000000000000000000001".parse().unwrap(),
				ipc: "".into(),
				contract: ContractConfig {
					bin: include_bytes!("../contracts/KovanBridge.bin").to_vec().into(),
					abi: ethabi::Contract::load(include_bytes!("../contracts/KovanBridge.abi") as &[u8]).unwrap(),
				},
				deploy_tx: TransactionConfig {
					gas: 0,
					gas_price: 0,
					value: 0,
				},
				poll_interval: Duration::from_secs(1),
				required_confirmations: 12,
			}
		};

		let config = Config::load_from_str(toml).unwrap();
		assert_eq!(expected, config);
	}
}
