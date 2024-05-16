# ===============================================================
#   Builder
# ===============================================================

FROM rust:slim-bullseye AS builder

RUN rustup target add x86_64-unknown-linux-gnu
RUN apt-get update && apt-get install cmake -y
RUN update-ca-certificates

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "10001" \
  "lorelei"

WORKDIR /lorelei

COPY ./ .

RUN cargo build --release --target x86_64-unknown-linux-gnu

# ===============================================================
#   Final image
# ===============================================================

FROM debian:bullseye-slim

RUN apt-get update && apt-get install python3-pip ffmpeg -y
RUN pip install -U yt-dlp

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER lorelei:lorelei

WORKDIR /lorelei

COPY --from=builder /lorelei/target/x86_64-unknown-linux-gnu/release/lorelei ./

CMD ["/lorelei/lorelei"]
