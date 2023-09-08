#[cfg(test)]
mod integration_test {
    use std::time::Duration;
    use easyharun_lib::config::{Config, ConfigContainer, ConfigContainerProxy, ConfigFileHealthCheck, ConfigFileProxy};
    use easyharun_test_container::TestClient;
    use crate::config::config_provider::ConfigProvider;
    use crate::Core;

    #[tokio::test(flavor = "multi_thread", worker_threads = 5)]
    async fn it_works() {

        let (config_reader, _) = ConfigProvider::new(Config {
            proxy: vec![
                ConfigFileProxy {
                    name: "foo_grpc".to_string(),
                    listen: "127.0.0.1:5345".to_string(),
                },
                ConfigFileProxy {
                    name: "foo_health".to_string(),
                    listen: "127.0.0.1:3000".to_string(),
                }
            ],
            health_check: vec![
                ConfigFileHealthCheck {
                    name: "check".to_string(),
                    check: "http".to_string(),
                    url: "http://127.0.0.1:{{container.port_dynamic_host_3000}}".to_string(),
                    timeout_ms: 1000
                }
            ],
            container: vec![
                ConfigContainer {
                    name: "foo".to_string(),
                    image: "easyharun_test_container".to_string(),
                    replica_id: 1,
                    container_ports: vec![5345, 3000],
                    health_checks: vec!["check".to_string()],
                    proxies: vec![
                        ConfigContainerProxy {
                            container_port: 3000,
                            name: "foo_health".to_string(),
                        },
                        ConfigContainerProxy {
                            container_port: 5345,
                            name: "foo_grpc".to_string(),
                        }
                    ]
                }
            ],
            ..Config::default()
        });

        let (_, core) = Core::spawn(config_reader, true);

        ::tokio::time::sleep(Duration::from_secs(1000)).await;

        let x = TestClient::connect("foo", Duration::from_secs(10)).await;

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}