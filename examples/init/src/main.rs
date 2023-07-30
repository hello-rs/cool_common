use tracing::info;

fn main() {
    // init cool lib
    cool::init();
    if cool::C_CONFIG.tracing.console {
        println!("info in console")
    }
    info!("print config {:?}", *cool::C_CONFIG);

    let res = cool::command("./", "cargo", vec!["--version"]).unwrap();
    info!("{:?}", res);
}
