pub fn main() -> std::io::Result<()> {
    ocaml_build::Sigs::new("src/my_project.ml").generate()
}
