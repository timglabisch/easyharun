#[cfg(test)]
mod integration_test {
    use std::time::Duration;
    use easyharun_lib::config::{Config, ConfigContainer};
    use crate::config::config_provider::ConfigProvider;
    use crate::Core;

    #[tokio::test]
    async fn it_works() {

        let (config_reader, _) = ConfigProvider::new(Config {
            container: vec![
                ConfigContainer {
                    name: "foo".to_string(),
                    image: "easyharun_test_container".to_string(),
                    replica_id: 1,
                    container_port: 123,
                    health_checks: vec!["check".to_string()],
                    proxies: vec!["proxy".to_string()]
                }
            ],
            ..Config::default()
        });

        let (_, core) = Core::spawn(config_reader);

        ::tokio::time::sleep(Duration::from_secs(100)).await;

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}