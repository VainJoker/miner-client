use crate::{
    cfg,
    pb::miner_sign::{SignRequest, SignResponse},
    utils,
};

pub async fn call(mut sign: SignRequest) -> SignResponse {
    let client = reqwest::Client::new();
    let sign_config = cfg::config().sign.clone();
    let addr = format!("http://{}:{}", sign_config.host, sign_config.port);
    let c = &format!(
        "{}{}{}{}{}",
        sign.mac, sign.ip, sign.devtype, sign.key, sign.t
    )
    .replace('\"', "");
    sign.c = utils::tool_md5v(c);
    println!("{}",sign.c);
    let response = client.post(addr).json(&sign).send().await.unwrap();

    response.json::<SignResponse>().await.unwrap()
}

// #[cfg(test)]
//
// mod tests{
//
//     use super::*;
//
//     #[tokio::test]
//     async fn test_api() {
//         call().await;
//     }
// }
