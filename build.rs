// Contract build script, shared by the recho crate via `build = "../build.rs"`.
// Emits the assertion environment variables for contracts/recho-handler-v1.yaml
// (three equations, one precondition and one postcondition each) and rebuilds
// when the contract changes.
fn main() {
  println!("cargo:rerun-if-changed=contracts/recho-handler-v1.yaml");
  println!("cargo:rerun-if-changed=../contracts/recho-handler-v1.yaml");
  println!("cargo:rustc-env=PRE_COUNT=3");
  println!("cargo:rustc-env=POST_COUNT=3");
}
