use std::path::PathBuf;

use crate::entity::Root;

pub fn read_file(file: PathBuf) -> Root {
    let content = std::fs::read_to_string(file).unwrap();
    let v: Root = serde_json::from_str(&content).unwrap();
    v
}

#[cfg(test)]

mod tests {

    use super::*;

    #[tokio::test]
    async fn test_read_file() {
        read_file(PathBuf::from(
            "/home/vainjoker/Codes/minerhub/miner-client/fixtures/status.json",
        ));
    }
}
