FROM rust:1.93

RUN apt-get update && apt-get install -y python3 python3-pip && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

EXPOSE 3001

CMD ["cargo", "run"]
