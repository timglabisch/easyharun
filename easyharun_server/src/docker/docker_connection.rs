use anyhow::Context;
use bollard::Docker;

pub fn docker_create_connection() -> Result<Docker, ::anyhow::Error> {
    Docker::connect_with_socket_defaults().context("could not connect docker.")
}