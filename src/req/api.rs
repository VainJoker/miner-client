use crate::utils;

pub async fn sign() {
    let client = reqwest::Client::new();
    let sign_str = r#"
    {
    "mac": "28:E2:97:3E:6F:06",
    "ip": "192.168.110.97",
    "devtype": "Goldshell-MiniDOGEPro",
    "key": "df286c6992ed05a12eccaa549b600a5084929",
    "t": 1717640785,
    "c": "8586989e6dc724c8f8890ce377049896",
    "hv": "GP.CI.IA",
    "sv": "2.2.8",
    "capability": {
        "powermode": [
            "Hashrate",
            "Idle"
        ],
        "algoset": [
            "scrypt(LTC)"
        ],
        "poolmax": 3,
        "reboot": 1,
        "update": 1,
        "reset": 1,
        "led": 1
    },
    "candy": [150,89,150,129,190,124,65,153,135,189,66,132,170,74,175,156,238,39,196,111,200,243,50,53,154,210,163,106,151,224,42,175,102,23,96,77,198,255,84,55,237,74,67,246,85,182,232,190,80,129,210,94,190,155,96,101,88,225,218,5,123,223,129,195,243,64,143,230,110,47,50,74]
    }
    "#;

    let mut m :serde_json::Value = serde_json::from_str(sign_str).unwrap();
    let t_str = m["t"].to_string();
    eprintln!("{}",&format!("{}{}{}{}{}",m["mac"],m["ip"],m["devtype"],m["key"],t_str));
    let c = utils::tool_md5v(&format!("{}{}{}{}{}",m["mac"],m["ip"],m["devtype"],m["key"],m["t"]));
    eprintln!("{:#?}", m["c"]);
    m["c"] = serde_json::Value::String(c);
    eprintln!("{:#?}", m["c"]);
    let response = client
        .post("http://192.168.110.200:8022/sign/post")
        .json(&m)
        .send()
        .await
        .unwrap();

    eprintln!("{:?}", response.text().await.unwrap());
}


#[cfg(test)]

mod tests{

    use super::*;

    #[tokio::test]
    async fn test_api() {
        sign().await;
    }
}
