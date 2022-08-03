fn main() {
    println!("cargo:rerun-if-changed=src/3des.c");
    println!("cargo:rerun-if-changed=src/3des.h");

    cc::Build::new()
        .file("src/3des.c")
        .include("src")
        .compile("3des");
}
