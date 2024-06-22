use md5;

pub fn tool_md5v(input: &str) -> String {
    format!("{:x}", md5::compute(input.as_bytes()))
}

#[cfg(test)]

mod tests {

    use super::*;

    #[tokio::test]
    async fn test_md5() {
        // signObj.Mac + signObj.Ip + signObj.DevType + signObj.Key + tString
        let res = tool_md5v(
            "28:E2:97:3E:6F:06192.168.110.97Goldshell-MiniDOGEProdf286c6992ed05a12eccaa549b600a50849291717640785",
        );
        eprintln!("{res}");
        assert_eq!(res, "8586989e6dc724c8f8890ce377049896");
    }
}
