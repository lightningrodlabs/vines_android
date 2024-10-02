fn main() {
  println!("cargo:rerun-if-changed=../workdir/vines.happ");
  tauri_build::build()
}
