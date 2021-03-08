FROM ubuntu-rust-opencv:latest AS builder
WORKDIR /app

# Setup git credentials
ARG GIT_CREDENTIALS
RUN git config --global credential.helper store && \
    echo "$GIT_CREDENTIALS" > ~/.git-credentials && \
    chmod 600 ~/.git-credentials

# Build
COPY src ./src
COPY Cargo.toml Cargo.lock Rocket.toml ./
RUN cargo build --release

# Bundle
FROM ubuntu-opencv:latest
WORKDIR /app
COPY --from=builder /app/target/release/crowdstress-math .
COPY --from=builder /app/Rocket.toml .
ENTRYPOINT ["/app/crowdstress-math"]
