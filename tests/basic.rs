use nessus_launcher::{NessusClient, NessusConfig};

#[test]
fn test_config_missing_env() {
    std::env::remove_var("NESSUS_HOST");
    std::env::remove_var("NESSUS_USERNAME");
    std::env::remove_var("NESSUS_PASSWORD");

    let cfg = NessusConfig::from_env();
    assert!(cfg.is_err());
}

#[test]
fn test_config_valid() {
    std::env::set_var("NESSUS_HOST", "https://example.com");
    std::env::set_var("NESSUS_USERNAME", "admin");
    std::env::set_var("NESSUS_PASSWORD", "pass");

    let cfg = NessusConfig::from_env();
    assert!(cfg.is_ok());
}

#[test]
fn test_client_constructs() {
    std::env::set_var("NESSUS_HOST", "https://example.com");
    std::env::set_var("NESSUS_USERNAME", "admin");
    std::env::set_var("NESSUS_PASSWORD", "pass");

    let cfg = NessusConfig::from_env().unwrap();
    let client = NessusClient::new(cfg);
    assert!(client.is_ok());
}

