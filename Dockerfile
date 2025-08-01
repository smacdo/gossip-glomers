# Build our gossip-glomer Rust application as "app".
FROM rust:slim-bookworm AS rust_builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo install --locked --path .

# Install Maelstrom and friends.
FROM rust:slim-bookworm
RUN apt-get update
RUN apt-get install -y --no-install-recommends default-jdk graphviz gnuplot git
RUN apt-get install -y --no-install-recommends curl bzip2

WORKDIR /opt
RUN curl -L -o maelstrom.tar.bz2 https://github.com/jepsen-io/maelstrom/releases/latest/download/maelstrom.tar.bz2
RUN tar -xvjf maelstrom.tar.bz2

# Copy the gossip-glomer application into the final image.
WORKDIR /app
COPY --from=rust_builder /app/target/release/gossip-glomers .

EXPOSE 8080

ENV RUST_LOG="debug"
ENV RUST_BACKTRACE=1

CMD ["/opt/maelstrom/maelstrom", "test", "--log-stderr", "-w", "echo", "--bin", "/app/gossip-glomers", "--time-limit", "5"]
