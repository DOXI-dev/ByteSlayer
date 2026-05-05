![Status](https://img.shields.io/badge/Status-Work%20In%20Progress-yellow)
![Version](https://img.shields.io/badge/Version-0.1.0-blue)
![License](https://img.shields.io/badge/License-MIT-green)

[![Rust](https://img.shields.io/badge/Language-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Language-Python-3776AB?logo=python&logoColor=white)](https://www.python.org/)
[![Docker](https://img.shields.io/badge/Deployment-Docker-2496ED?logo=docker&logoColor=white)](https://www.docker.com/)
[![Render](https://img.shields.io/badge/Hosted%20on-Render-46E3B7?logo=render&logoColor=white)](https://render.com/)
[![Flask](https://img.shields.io/badge/Framework-Flask-000000?logo=flask&logoColor=white)](https://flask.palletsprojects.com/)

[![Lichess](https://img.shields.io/badge/Play%20on-Lichess-white?logo=lichess&logoColor=black)](https://lichess.org/@/ByteSlayer-ChessBot)

# ByteSlayer
ByteSlayer is a ChessEngine written in Rust, work in progress for the ChessInterface & DiscordBot.

While not aiming to beat Stockfish (yet!), ByteSlayer is a modern alternative for those looking for a fast, Rust-based engine with a focus on clean code and easy Lichess integration.

## Prerequisites

Before setting up ByteSlayer, make sure you have the following installed:

- **Python 3.10+** — [Download Python](https://www.python.org/downloads/)
- **Rust (latest stable)** — [Install Rust](https://www.rust-lang.org/tools/install)
- **A Lichess account** with a [Bot API Token](https://lichess.org/account/oauth/token) (create one with the scope `bot:play`)

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/DOXI-dev/ByteSlayer.git
cd ByteSlayer
```

### 2. Install Python Dependencies

```bash
pip install -r requirements.txt
```

### 3. Build the Rust Engine

```bash
cd engines
cargo build --release
```

The compiled engine binary will be available at `engines/target/release/ByteSlayer-ChessBot` (or the appropriate name for your platform).

### 4. Configure the Bot

1. Open `config.yml` and set your Lichess API token:

   ```yaml
   token: your_lichess_bot_token_here
   ```

   You can generate a token at [lichess.org/account/oauth/token](https://lichess.org/account/oauth/token) with the `bot:play` scope.

2. Review the `config.yml` file for additional settings such as engine options, challenge preferences, and time controls.

### 5. Run the Bot

```bash
python lichess-bot.py
```

### Docker (Optional)

A `Dockerfile` is included for containerized deployment:

```bash
docker build -t byteslayer .
docker run -e LICHESS_BOT_TOKEN=your_token_here byteslayer
```

## Contribute
Contributions are welcome! Whether it's to optimize the Rust search algorithm, add features, or anything else, feel free to open a Pull Request.
