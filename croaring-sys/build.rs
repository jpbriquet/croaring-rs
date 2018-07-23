extern crate gcc;

fn main() {
    gcc::Build::new()
      .flag("-std=c11")
      .flag("-march=native")
      .flag("-O3")
      .file("CRoaring/roaring.c")
      .compile("libroaring.a");
}
