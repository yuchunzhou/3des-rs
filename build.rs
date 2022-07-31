fn main() {
    cc::Build::new()
        .file("src/3des.c")
        .include("src")
        .compile("3des");
}
