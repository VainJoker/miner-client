use crate::{
    cfg,
    pb::miner_sign::{
        miner_sign_client::MinerSignClient, SignRequest,
        SignResponse,
    },
};

pub async fn call(sign: SignRequest) -> SignResponse {
    let sign_config = cfg::config().sign.clone();
    let addr = format!("http://{}:{}", sign_config.host, sign_config.port);
    let mut client = MinerSignClient::connect(addr).await.unwrap();
    let request = tonic::Request::new(sign);
    // SignRequest {
    //     mac: sign.mac,
    //     ip: "192.168.110.97".to_string(),
    //     devtype: "Goldshell-MiniDOGEPro".to_string(),
    //     key: "dd99c54c08bb2752f5f8ad6e526243cbea722".to_string(),
    //     t: 1_717_640_795,
    //     c: "d785383619c1ea6e9f3ef8130069bc88".to_string(),
    //     hv: "GP.CI.IA".to_string(),
    //     sv: "2.2.8".to_string(),
    //     capability: Some(
    //         sign_request::Capability{
    //             powermode: vec!["Hashrate".to_string(),"Idle".to_string()],
    //             algoset: vec!["scrypt(LTC)".to_string()],
    //             poolmax: 3,
    //             reboot: 1,
    //             update: 1,
    //             reset: 1,
    //             led: 1,
    //         }),
    //         candy:
    // vec![150,89,150,129,190,124,65,153,135,189,66,132,170,74,175,156,238,39,
    // 196,111,200,243,50,53,154,210,163,106,151,224,42,175,102,23,96,77,198,
    // 255,84,55,237,74,67,246,85,182,232,190,80,129,210,94,190,155,96,101,88,
    // 225,218,5,123,223,129,195,243,64,143,230,110,47,50,74] });
    let res = client.sign(request).await.unwrap();
    res.into_inner()
}
