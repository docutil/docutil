fn main() {
    tracing_subscriber::fmt::init();
    
    let doc = std::fs::read_to_string("demo.yml").expect("demo.yml not found");
    let playbook = playbook::try_from(&doc).expect("unable to read a playbook");

    playbook.start().unwrap();
}
