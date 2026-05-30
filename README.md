# lastfm-rs 🎵

A blazing-fast Rust-based Last.fm SDK for Cloudflare Workers and a feature-rich CLI that lets you access both public and personal Last.fm data.

## ✨ Key Features

- **🔐 Full Authentication Support** - Access your personal Last.fm data (loved tracks, recent plays, top artists)
- **🚀 High Performance** - Built with Rust for optimal speed and efficiency
- **💾 Smart Caching** - KV-based caching with intelligent TTL management
- **🛡️ Rate Limiting** - Built-in protection against API abuse
- **📊 31+ Commands** - Comprehensive coverage across 10 categories
- **🎨 Multiple Output Formats** - JSON, table, pretty-print, and compact modes

## ⚠️ This project is self-hosted

lastfm-rs ships with **no backend**. The CLI talks to a Cloudflare Worker that **you deploy and own** — your Last.fm API key, your KV cache, your rate limits, your users' data. There is no shared or default endpoint baked in. Deploy the worker first, then point the CLI at it.

## 🏠 Self-Hosting

### Prerequisites

- A Cloudflare account (the free tier is enough)
- [`wrangler`](https://developers.cloudflare.com/workers/wrangler/install-and-update/) installed and authenticated (`wrangler login`)
- A Last.fm API account — create one to get an **API key** and **shared secret**: https://www.last.fm/api/account/create

### Deploy your worker

```bash
git clone https://github.com/guitaripod/lastfm-rs
cd lastfm-rs

# Creates KV namespaces, writes their IDs into wrangler.toml, and sets your secrets
./scripts/setup-worker.sh

wrangler deploy
```

`setup-worker.sh` copies `wrangler.toml.example` to `wrangler.toml`, creates the `CACHE` and `RATE_LIMIT` KV namespaces (production + preview), fills in their IDs, and prompts for the two required secrets:

| Secret | Required | Purpose |
| --- | --- | --- |
| `LASTFM_API_KEY` | yes | Your Last.fm API key |
| `LASTFM_API_SECRET` | yes | Your Last.fm shared secret, used to sign authenticated requests server-side |

Prefer to do it by hand? See [`wrangler.toml.example`](wrangler.toml.example), then:

```bash
wrangler kv namespace create CACHE
wrangler kv namespace create CACHE --preview
wrangler kv namespace create RATE_LIMIT
wrangler kv namespace create RATE_LIMIT --preview
# paste the four IDs into wrangler.toml
wrangler secret put LASTFM_API_KEY
wrangler secret put LASTFM_API_SECRET
wrangler deploy
```

### Continuous deployment (optional)

The included GitHub Actions workflow auto-deploys on push to `master`. Because `wrangler.toml` is gitignored (it holds your namespace IDs), CI rebuilds it from `wrangler.toml.example`. Add these to your repository so it can:

- Secret `CLOUDFLARE_API_TOKEN`
- Variables `CACHE_ID`, `CACHE_PREVIEW_ID`, `RATE_LIMIT_ID`, `RATE_LIMIT_PREVIEW_ID`

## 🚀 Using the CLI

### Install

```bash
# From source
cargo install --path . --bin lastfm-cli

# Or download a pre-built binary (if available)
curl -L https://github.com/guitaripod/lastfm-rs/releases/latest/download/lastfm-cli-linux-x64 -o lastfm-cli
chmod +x lastfm-cli
```

### Point it at your worker

The CLI refuses to run API commands until a backend is configured. Set yours once:

```bash
lastfm-cli config set worker_url https://your-worker.workers.dev
```

Or override per-invocation with an environment variable or flag (precedence: `--worker-url` > `LASTFM_WORKER_URL` > config file):

```bash
export LASTFM_WORKER_URL=https://your-worker.workers.dev
lastfm-cli --worker-url https://your-worker.workers.dev artist info "Boards of Canada"
```

## 🎯 Example Commands

### Personal Data (Authenticated)

```bash
# Login to Last.fm
lastfm-cli auth login

# Get your recent tracks
lastfm-cli my recent-tracks --limit 10

# See your top artists this month
lastfm-cli my top-artists --period 1month

# Check your loved tracks
lastfm-cli my loved-tracks
```

### Public Data

```bash
# Get artist info with beautiful formatting
lastfm-cli artist info "Taylor Swift" -o pretty

# Search for similar tracks
lastfm-cli track similar "The Beatles" "Hey Jude" -o table

# Discover music by country
lastfm-cli geo top-tracks "Japan" --limit 20

# Explore genres
lastfm-cli tag top-artists "shoegaze" -o json | jq '.topartists.artist[0:5]'
```

### Advanced Usage

```bash
# Compare your music taste with a friend
lastfm-cli user compare "YourUsername" "FriendUsername"

# Export your library to CSV
lastfm-cli my top-tracks --period overall --limit 1000 -o json | \
  jq -r '.toptracks.track[] | [.name, .artist.name, .playcount] | @csv' > my_music.csv

# Track listening habits over time
lastfm-cli my weekly-chart-list | \
  lastfm-cli my weekly-track-chart --from $(date -d '1 month ago' +%s)
```

## 📚 Documentation

- **[CLI User Manual](docs/CLI_USER_MANUAL.md)** - Complete guide to all CLI features
- **[CLI Command Showcase](docs/CLI_SHOWCASE.md)** - Live examples of every command
- **[Testing Summary](docs/TESTING_SUMMARY.md)** - Quality assurance details

## 🛠️ Configuration

Config lives at `~/.config/lastfm-cli/config.toml`. See [`config.toml.example`](config.toml.example).

```bash
# Set your worker URL (required)
lastfm-cli config set worker_url https://your-worker.workers.dev

# Other options: output format, cache TTL, request timeout, etc.
lastfm-cli config set output_format pretty

# View all settings
lastfm-cli config list
```

## 🏗️ Architecture

- **Worker**: Rust-based Cloudflare Worker with caching, rate limiting, and CORS support
- **CLI**: Modern command-line interface with authentication, multiple output formats, and comprehensive API coverage
- **Shared Core**: Common types and utilities for consistent behavior

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.
