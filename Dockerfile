FROM rust:1.87-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM python:3.12-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

RUN mkdir -p ./engines

COPY --from=builder /app/target/release/ByteSlayer-ChessBot ./engines/ByteSlayer-ChessBot

COPY . .

RUN pip install --no-cache-dir -r requirements.txt

RUN chmod +x ./engines/ByteSlayer-ChessBot

ENV PYTHONPATH="/app"
CMD ["python", "lichess-bot.py"]
