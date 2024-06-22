use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub sign: Sign,
    pub mode: usize,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sign {
    pub mac: String,
    pub ip: String,
    pub devtype: String,
    pub key: String,
    pub t: i64,
    pub c: String,
    pub hv: String,
    pub sv: String,
    pub capability: Capability,
    pub candy: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capability {
    pub powermode: Vec<String>,
    pub algoset: Vec<String>,
    pub poolmax: i64,
    pub reboot: i64,
    pub update: i64,
    pub reset: i64,
    pub led: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    pub url: String,
    pub user: String,
    pub legal: bool,
    pub active: bool,
    #[serde(rename = "drag_id")]
    pub drag_id: i64,
    #[serde(rename = "pool_priority")]
    pub pool_priority: i64,
    pub pass: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Pool {
//     url: String,
//     user: String,
//     legal: bool,
//     active: bool,
//     #[serde(rename = "dragid")]
//     drag_id: i32,
//     #[serde(rename = "pool-priority")]
//     pool_priority: i32,
//     pass: String,
// }

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Message {
    MessageMode(MessageMode),
    MessageStatus(Box<MessageStatus>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageMode {
    pub mode: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "now_rate")]
    pub now_rate: f64,
    #[serde(rename = "avg_rate")]
    pub avg_rate: f64,
    #[serde(rename = "history_rate")]
    pub history_rate: Vec<f64>,
    #[serde(rename = "power_mode")]
    pub power_mode: String,
    #[serde(rename = "dig_time")]
    pub dig_time: i64,
    pub pool: Vec<Pool>,
    #[serde(rename = "hard_err")]
    pub hard_err: f64,
    pub refuse: f64,
    pub temperature: String,
    pub fan: String,
    pub led: i64,
    pub ip: String,
    pub key: String,
    pub coin: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
    pub key: String,
    pub coin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignResponse {
    pub result: i32,
    pub ms: String,
    pub mpt: u16,
    pub mu: String,
    pub mp: String,
    pub t: u64,
}

impl From<Status> for MessageStatus {
    fn from(item: Status) -> Self {
        Self {
            now_rate: item.now_rate,
            avg_rate: item.avg_rate,
            history_rate: item.history_rate,
            power_mode: item.power_mode,
            dig_time: item.dig_time as i32, // 注意这里的类型转换
            pool: item.pool,
            hard_err: item.hard_err,
            refuse: item.refuse,
            temperature: item.temperature,
            fan: item.fan,
            led: item.led as i32, // 注意这里的类型转换
            ip: item.ip,
            key: item.key,
            coin: item.coin,
        }
    }
}

impl From<usize> for MessageMode {
    fn from(value: usize) -> Self {
        Self { mode: value }
    }
}
