// Local only: export arc-swap's shared debt list head so dlopen-ed plugins use the same list.
fn main() {
    println!("cargo:rustc-link-arg-bins=-Wl,--export-dynamic-symbol=PUMPKIN_ARC_SWAP_LIST_HEAD");
}
