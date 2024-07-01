use miner_client::{
    cfg,
    cmd::Cli,
    mqtt::{self, Options},
    req::abi,
    req::api,
    tp::read_file,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    cfg::init(&"./fixtures/config.toml".to_string());

    let cli = Cli::init();
    let config_path = cli.config.as_deref().unwrap().to_path_buf();
    let root = read_file(config_path);
    let mac = root.sign.mac.clone();

    let res = api::call(root.sign).await;
    //let res = abi::call(root.sign).await;

    mqtt::mqtt(
        Options {
            mac,
            host: res.ms,
            port: res.mpt as u16,
            mu: res.mu,
            mp: res.mp,
        },
        root.status.into(),
        root.mode.into(),
    )
    .await;

    Ok(())
}
