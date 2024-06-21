use std::time::Duration;
use miner_client::cfg;

use miner_client::pb::miner_sign::miner_sign_client::MinerSignClient;
use miner_client::pb::miner_sign::{sign_request, SignRequest};
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, MqttOptions};
use serde::{Deserialize, Serialize};
use tokio::{task, time};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Message {
    MessageMode(MessageMode),
    MessageStatus(Box<MessageStatus>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageMode {
    pub mode: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageStatus {
    #[serde(rename = "nowrate")]
    pub now_rate: f64,
    #[serde(rename = "avgrate")]
    pub avg_rate: f64,
    #[serde(rename = "historyrate")]
    pub history_rate: Vec<f64>,
    #[serde(rename = "powermode")]
    pub power_mode: String,
    #[serde(rename = "digtime")]
    pub dig_time: i32,
    pub pool: Vec<Pool>,
    #[serde(rename = "harderr")]
    pub hard_err: f64,
    pub refuse: f64,
    pub temperature: String,
    pub fan: String,
    pub led: i32,
    pub ip: String,
    key: String,
    pub coin: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pool {
    url: String,
    user: String,
    legal: bool,
    active: bool,
    #[serde(rename = "dragid")]
    drag_id: i32,
    #[serde(rename = "pool-priority")]
    pool_priority: i32,
    pass: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    cfg::init(&"./fixtures/config.toml".to_string());

    let addr = format!("http://{}:{}", "0.0.0.0", "9090");
    let mut client = MinerSignClient::connect(addr).await?;
    let mac = "30:E2:97:3E:6F:14".to_string();

    let request = tonic::Request::new(
        SignRequest { 
            mac: mac.clone(),
            ip: "192.168.110.97".to_string(),
            devtype: "Goldshell-MiniDOGEPro".to_string(),
            key: "dd99c54c08bb2752f5f8ad6e526243cbea722".to_string(),
            t: 1_717_640_795,
            c: "d785383619c1ea6e9f3ef8130069bc88".to_string(),
            hv: "GP.CI.IA".to_string(),
            sv: "2.2.8".to_string(),
            capability: Some(
                sign_request::Capability{ 
                    powermode: vec!["Hashrate".to_string(),"Idle".to_string()],
                    algoset: vec!["scrypt(LTC)".to_string()],
                    poolmax: 3,
                    reboot: 1,
                    update: 1,
                    reset: 1,
                    led: 1,
               }),
            candy: vec![150,89,150,129,190,124,65,153,135,189,66,132,170,74,175,156,238,39,196,111,200,243,50,53,154,210,163,106,151,224,42,175,102,23,96,77,198,255,84,55,237,74,67,246,85,182,232,190,80,129,210,94,190,155,96,101,88,225,218,5,123,223,129,195,243,64,143,230,110,47,50,74]
        });
    let response = client.sign(request).await?;
    let res = response.into_inner();

    let mut mqtt_opts = MqttOptions::new(
        &mac,
        &res.ms,
        res.mpt as u16
    );
    mqtt_opts
        .set_keep_alive(std::time::Duration::from_secs(30));
    mqtt_opts.set_credentials(&res.mu, &res.mp);
    let (client, mut eventloop) = AsyncClient::new(mqtt_opts, 10);

    task::spawn(async move {
        requests(client,&mac).await;
    });

    loop {
        let event = eventloop.poll().await;
        match &event {
            Ok(v) => {
                println!("Event = {v:?}");
            }
            Err(e) => {
                println!("Error = {e:?}");
                return Ok(());
            }
        }
    }
}


async fn requests(client: AsyncClient,mac: &str) {
    let server_topic = format!("/server/{mac}/#");
    let mode_topic = format!("/client/{mac}/property/upload");
    let status_topic = format!("/client/{mac}/work/status/upload");
    client
        .subscribe(&server_topic, QoS::AtMostOnce)
        .await
        .unwrap();
    let client1 = client.clone();
    let client2 = client.clone();

    let mode = tokio::spawn(async move {
        loop {
            let message = Message::MessageMode(MessageMode { mode: 1 });
            let message = serde_json::to_vec(&message).unwrap();
            client1
                .publish(&mode_topic, QoS::AtLeastOnce, false, message)
                .await
                .unwrap();

            time::sleep(Duration::from_secs(5)).await;
        }
    });

    let status = tokio::spawn(async move {
        loop{
            let message = Message::MessageStatus(Box::new(MessageStatus {
                now_rate: 220.692,
                avg_rate: 210.25,
                history_rate: vec![216.759,205.234,207.718,227.19,213.357,208.49700000000004,225.146,215.259,200.357,210.665,226.502,223.918,206.987,206.52599999999995,207.542,211.443,217.706,214.922,198.74200000000005,205.44299999999996,202.125,197.71200000000005,214.31,203.07,191.48,214.429,220.8,237.94099999999992,217.15400000000008,228.028,207.86000000000004,205.56,210.539,215.239,201.03299999999996,205.002,229.43400000000008,217.49,218.593,200.359,210.262,221.445,208.655,204.15400000000005,199.252,214.253,208.55900000000003,202.75099999999995,219.104,211.51,210.228,196.18700000000004,199.43099999999995,212.485,208.74400000000003,215.789,206.98,194.602,226.64,237.462,214.545,199.75599999999997,222.671,203.95,204.168,214.7,224.52,225.678,238.774,227.976,198.18799999999996,200.90599999999995,221.554,201.24900000000005,216.951,196.898,186.727,223.518,216.12,213.865,202.498,193.208,207.081,220.146,210.347,224.34,232.418,209.47900000000004,199.605,207.709,211.987,207.56200000000004,221.45,210.75099999999992,207.46099999999996,206.514,196.697,201.60299999999995,184.502,208.28,211.963,201.82,217.742,216.014,206.66099999999997,218.612,211.317,219.583,215.622,202.584,199.05599999999995,202.00799999999995,220.327,209.36000000000004,231.382,220.773,235.612,202.87400000000005,234.212,213.366,213.032,189.87200000000004,210.18,216.919,213.293,202.03099999999995,201.07400000000004,206.24,217.094,215.932,196.03299999999996,203.162,189.30900000000003,185.718,214.018,214.25,220.201,226.638,229.302,207.261,213.565,222.299,222.156,207.055,214.113,206.24599999999995,221.01,205.525,214.341,201.07299999999995,192.60400000000004,203.00299999999996,202.37799999999996,208.71900000000005,210.085,231.908,211.142,202.59599999999995,216.491,197.79499999999996,209.12,204.96099999999996,202.86700000000005,220.258,223.724,195.29499999999996,197.174,216.609,197.31799999999996,187.433,215.394,235.96,229.265,196.06,221.818,210.062,210.00599999999991,206.91400000000004,217.205,208.638,220.345,215.05,235.841,185.528,211.757,197.79,209.107,218.965,215.86900000000009,210.368,197.274,205.19099999999997,211.457,228.044,197.77900000000005,209.12400000000005,192.696,230.418,204.5,199.11000000000004,202.707,202.606,208.56799999999996,200.24099999999996,224.29,210.41,206.487,198.40900000000005,210.645,215.47099999999992,192.82400000000004,198.377,212.493,200.857,201.53,218.297,218.94,213.901,219.535,202.19,220.894,216.00599999999991,212.507,203.12,205.59400000000005,211.231,226.861,218.27900000000008,199.55099999999996,191.78299999999996,218.982,219.448,200.34400000000005,216.356,203.386,214.811,211.90400000000008,201.50799999999995,221.03599999999992,210.418,218.882,205.138,218.547,211.122,212.549,243.66099999999992,191.766,199.65900000000005,206.636,203.882,217.02,218.397,207.56299999999996,208.435,212.27900000000008,212.78599999999992,219.135,210.156,216.651,224.93400000000008,214.893,195.761,197.053,213.854,219.43,213.015,223.581,204.81900000000005,198.44299999999996,220.55900000000008,212.595,221.862,212.985,223.44099999999992,212.826,210.018,205.554,200.745,194.14599999999996,208.15099999999995,218.886,209.236,227.052,210.841,211.336,202.81,206.67700000000005,221.995],
                power_mode: "Hashrate".to_string(),
                dig_time: 1314,
                pool: vec![Pool{ url: "stratum+tcp://192.168.111.225:4001".to_string(), user: "shminer.3134".to_string(), legal: true, active: true, drag_id: 0, pool_priority: 0, pass: "123".to_string() }],
                hard_err: 0.998_976,
                refuse: 0.001_772_264_067_346_034_5,
                temperature: "78.6 °C".to_string(),
                fan: "1980 / 1920".to_string(),
                led: 0,
                ip: "192.168.110.97".to_string(),
                key: "dd99c54c08bb2752f5f8ad6e526243cbea722".to_string(),
                coin: "scrypt(LTC)".to_string(),
            }));
            let message = serde_json::to_vec(&message).unwrap();
        client2
            .publish(&status_topic, QoS::AtLeastOnce, false, message)
            .await
            .unwrap();

            time::sleep(Duration::from_secs(5)).await;
        }
    });

    let _ = tokio::join!(mode, status);

}
