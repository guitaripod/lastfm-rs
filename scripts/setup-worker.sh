#!/bin/bash
#
# Bootstraps a self-hosted lastfm-rs Cloudflare Worker:
#   - copies wrangler.toml.example -> wrangler.toml (gitignored)
#   - prompts for the LASTFM_API_KEY and LASTFM_API_SECRET secrets
#
# After it finishes, run `wrangler deploy`.

set -euo pipefail

cd "$(dirname "$0")/.."

echo "lastfm-rs worker setup"
echo "======================"

if ! command -v wrangler >/dev/null 2>&1; then
    echo "Error: wrangler is not installed. Install it with: npm install -g wrangler" >&2
    exit 1
fi

if [ ! -f wrangler.toml.example ]; then
    echo "Error: wrangler.toml.example not found. Run this from the repository root." >&2
    exit 1
fi

if [ -f wrangler.toml ]; then
    read -r -p "wrangler.toml already exists. Overwrite it from the template? [y/N] " answer
    case "$answer" in
        y | Y) ;;
        *)
            echo "Keeping the existing wrangler.toml; aborting setup."
            exit 0
            ;;
    esac
fi

cp wrangler.toml.example wrangler.toml
echo "Created wrangler.toml from the template."
echo ""

echo "Set your Last.fm secrets (get them at https://www.last.fm/api/account/create):"
wrangler secret put LASTFM_API_KEY || echo "  skipped/failed: LASTFM_API_KEY"
wrangler secret put LASTFM_API_SECRET || echo "  skipped/failed: LASTFM_API_SECRET"

echo ""
echo "Setup complete. Review wrangler.toml, then deploy with:"
echo "  wrangler deploy"
echo ""
echo "Finally, point the CLI at your worker:"
echo "  lastfm-cli config set worker_url https://<your-worker>.workers.dev"
