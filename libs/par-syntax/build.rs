fn main() {
    println!("cargo::rerun-if-changed=parser.lalrpop");
    lalrpop::Configuration::new()
        .set_out_dir("src")
        .process_file("parser.lalrpop")
        .unwrap();
}
