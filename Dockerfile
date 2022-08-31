FROM rust:1.63.0-bullseye as builder
WORKDIR /usr/src/myapp
COPY src ./src
COPY Cargo.toml ./
COPY Cargo.lock ./
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN cargo install --path .

FROM rust:1.63.0-bullseye
RUN apt-get update && apt-get install -y inetutils-traceroute && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rustcanhazip /usr/local/bin/rustcanhazip
EXPOSE 8000
CMD ["rustcanhazip"]