use tracing::info;

fn main() {
    // init cool_common lib
    cool_common::init();
    if cool_common::C_CONFIG.tracing.console {
        println!("info in console")
    }
    info!("print config {:?}", *cool_common::C_CONFIG);

    let res = cool_common::command("./", "cargo", vec!["--version"]).unwrap();
    info!("{:?}", res);
}
