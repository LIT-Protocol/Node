pub fn get_web_addr_from_chain_info(ip: u32, port: u32) -> String {
    let ip = std::net::Ipv4Addr::from(ip).to_string();
    format!("{}:{}", ip, port)
}

#[cfg(test)]
mod tests {
    use super::get_web_addr_from_chain_info;

    struct TestCase {
        pub ip: u32,
        pub port: u32,
        pub expected_out: String,
    }

    fn get_test_cases() -> Vec<TestCase> {
        vec![TestCase {
            ip: 1,
            port: 2,
            expected_out: "0.0.0.1:2".to_string(),
        }]
    }

    #[test]
    fn test_get_web_addr_from_chain_info() {
        let test_cases = get_test_cases();

        for tc in test_cases {
            let actual_web_addr = get_web_addr_from_chain_info(tc.ip, tc.port);
            assert_eq!(tc.expected_out, actual_web_addr);
        }
    }
}
