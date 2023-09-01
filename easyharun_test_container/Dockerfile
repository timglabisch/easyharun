FROM rust:latest as builder

WORKDIR /rust_build
ADD ./ /rust_build
RUN apt-get update -y \
    && apt install -y protobuf-compiler \
    && cargo build --release

FROM ubuntu:latest
COPY --from=builder /rust_build/target/release/easyharun_test_container /easyharun_test_container
ENTRYPOINT ["/easyharun_test_container"]
