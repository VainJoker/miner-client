use std::time::Duration;

use miner_client::pb::miner_sign::miner_sign_client::MinerSignClient;
use miner_client::pb::miner_sign::{sign_request, SignRequest};
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, MqttOptions};
use tokio::{task, time};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let addr = format!("http://{}:{}", "0.0.0.0", "9090");
    // let addr = addr.parse().unwrap_or_else(|e| {
    //     panic!("💥 Failed to connect bind TcpListener: {e:?}")
    // });
    let mut client = MinerSignClient::connect(addr).await?;
    let mac = "30:E2:97:3E:6F:10".to_string();

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
    // eprintln!("RESPONSE={:?}", response);
    //

    let mut mqtt_opts = MqttOptions::new(
        &mac,
        &res.ms,
        res.mpt as u16
    );
    mqtt_opts
        .set_keep_alive(std::time::Duration::from_secs(5));
    mqtt_opts.set_credentials(&res.mu, &res.mp);
    let (client, mut eventloop) = AsyncClient::new(mqtt_opts, 10);

    task::spawn(async move {
        requests(client,&mac,).await;
        time::sleep(Duration::from_secs(3)).await;
    });

    // loop {
    //     let event = eventloop.poll().await;
    //     match &event {
    //         Ok(v) => {
    //             println!("Event = {v:?}");
    //         }
    //         Err(e) => {
    //             println!("Error = {e:?}");
    //             return Ok(());
    //         }
    //     }
    // }
    // let topic = format!("/server/{mac}/#");
    // match client.subscribe(&topic, QoS::AtMostOnce).await {
    //     Ok(()) => {
    //         tracing::debug!("Subscribed to topic {}", topic);
    //     }
    //     Err(e) => {
    //         tracing::error!(
    //             "Error occurred while subscribing to topic {}: {}",
    //             topic,
    //             e
    //         );
    //     }
    // }
    Ok(())
}


async fn requests(client: AsyncClient,mac: &str) {
    let topic = format!("/server/{mac}/#");
    client
        .subscribe(&topic, QoS::AtMostOnce)
        .await
        .unwrap();

    for i in 1..=10 {
        client
            .publish(&topic, QoS::ExactlyOnce, false, vec![1; i])
            .await
            .unwrap();

        time::sleep(Duration::from_secs(1)).await;
    }

    time::sleep(Duration::from_secs(120)).await;
}
