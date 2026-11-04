fn main() {
    // sqlx::migrate! tracks existing files, but Cargo must also notice newly
    // added migrations when building incrementally.
    println!("cargo:rerun-if-changed=migrations");
}
