FROM rust:1.87-slim as builder
WORKDIR /app
COPY . .
WORKDIR /app/ByteSlayer
RUN cargo build --release

FROM python:3.9-slim
WORKDIR /app
COPY --from=builder /app/ByteSlayer/target/release/byteslayer ./engines/ByteSlayer-ChessBot
COPY . .
RUN pip install -r requirements.txt
RUN chmod +x ./engines/ByteSlayer-ChessBot
CMD ["python", "lichess-bot.py"]
