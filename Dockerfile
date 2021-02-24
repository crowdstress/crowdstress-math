FROM ghcr.io/crowdstress/rust-opencv:latest AS builder
WORKDIR /app

# Setup git credentials
ARG GIT_CREDENTIALS
RUN git config --global credential.helper store && \
    echo "$GIT_CREDENTIALS" > ~/.git-credentials && \
    chmod 600 ~/.git-credentials

# Build
COPY src ./src
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Bundle
FROM ghcr.io/crowdstress/ubuntu-opencv:latest
WORKDIR /app
COPY --from=builder /app/target/release/crowdstress-api .
ENTRYPOINT ["/app/crowdstress-api"]
