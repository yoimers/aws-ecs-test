FROM rust:1.72

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

EXPOSE 8080
ENTRYPOINT ["cargo", "run", "-r"]