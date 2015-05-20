extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/cgroonga.c")
        .include("src")
        .compile("librurooonga.a");
}
