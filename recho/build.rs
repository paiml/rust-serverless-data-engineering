// Emits the contract assertion environment variables for contracts/recho-handler-v1.yaml
// and rebuilds when the contract changes.
fn main() {
  println!("cargo:rerun-if-changed=../contracts/recho-handler-v1.yaml");
  println!("cargo:rustc-env=CONTRACT_RECHO_HANDLER_V1_PRECONDITIONS=3");
}
