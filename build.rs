use cc;

fn main() {
    cc::Build::new()
        .file("src/implementation_packed_c.c")
        .compile("foo")
}