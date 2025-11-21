FROM rust:1-alpine3.22 AS build

RUN apk add npm zlib-dev zlib-static build-base openssl-dev	openssl-libs-static --no-cache

WORKDIR /backend

COPY Cargo.lock .
COPY Cargo.toml .

RUN mkdir src \
  && echo "// dummy file" > src/lib.rs \
  && cargo build

COPY src src

RUN cargo build --release


FROM alpine:3.22

WORKDIR /backend

COPY start.sh .
RUN chmod +x start.sh

COPY --from=build /backend/target/release/basic_git_ui .

ENTRYPOINT ["./start.sh"]
