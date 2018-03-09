use std::process::Command;

fn main() {
	// rerun build script if bridge contract has changed.
	// without this cargo doesn't since the bridge contract
	// is outside the crate directories
	println!("cargo:rerun-if-changed=../contracts/BridgrableToken.sol");

	match Command::new("solc")
		.arg("--abi")
		.arg("--bin")
		.arg("--optimize")
		.arg("--output-dir").arg("../compiled_contracts")
		.arg("--overwrite")
		.arg("../contracts/BridgeableToken.sol")
		.status()
	{
		Ok(exit_status) => {
			if !exit_status.success() {
				if let Some(code) = exit_status.code() {
					panic!("`solc` exited with error exit status code `{}`", code);
				} else {
					panic!("`solc` exited because it was terminated by a signal");
				}
			}
		},
		Err(err) => {
			if let std::io::ErrorKind::NotFound = err.kind() {
				panic!("`solc` executable not found in `$PATH`. `solc` is required to compile the bridge contracts. please install it: https://solidity.readthedocs.io/en/develop/installing-solidity.html");
			} else {
				panic!("an error occurred when trying to spawn `solc`: {}", err);
			}
		}
	}
}
