FROM ubuntu:latest AS builder

WORKDIR /app
ADD . .

ARG GIT_CREDENTIALS

ENV TZ=Europe/Moscow
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt update && \
    apt install -y \
    curl \
    git \
    clang \
    libclang-dev \
    libopencv-dev && \
    echo "OpenCV version $(opencv_version)"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup default nightly

RUN git config --global credential.helper store
RUN echo "$GIT_CREDENTIALS" > ~/.git-credentials
RUN chmod 600 ~/.git-credentials

RUN cargo build

FROM alpine

WORKDIR /app
COPY --from=builder /app/target/release/crowdstress-api .

CMD ["/app/crowdstress-api"]
