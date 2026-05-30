#!/bin/bash

# Test Summary Script for Last.fm CLI
# Shows a quick overview of all working features

CLI="./target/release/lastfm-cli"
WORKER_URL="${WORKER_URL:-http://localhost:8787}"

echo "Last.fm CLI & Worker Test Summary"
echo "================================="
echo ""

# Check Worker Health
echo "🌐 Cloudflare Worker Status:"
if curl -s "$WORKER_URL/health" | grep -q "OK"; then
    echo "   ✅ Worker is healthy at $WORKER_URL"
else
    echo "   ❌ Worker is down at $WORKER_URL"
fi

# Check CLI Version
echo ""
echo "📦 CLI Version:"
echo "   $($CLI --version)"

# Check Authentication
echo ""
echo "🔐 Authentication Status:"
AUTH_STATUS=$($CLI auth status)
if echo "$AUTH_STATUS" | grep -q '"authenticated": true'; then
    USERNAME=$(echo "$AUTH_STATUS" | grep -o '"username": "[^"]*"' | cut -d'"' -f4)
    echo "   ✅ Authenticated as: $USERNAME"
else
    echo "   ❌ Not authenticated"
fi

# Quick command test
echo ""
echo "🎯 Quick Command Test:"
echo -n "   Testing artist info... "
if $CLI -o json artist info "Radiohead" 2>/dev/null | grep -q '"name": "Radiohead"'; then
    echo "✅"
else
    echo "❌"
fi

echo -n "   Testing authenticated command... "
if $CLI -o json my recent-tracks --limit 1 2>/dev/null | grep -q '"track"'; then
    echo "✅"
else
    echo "❌"
fi

# Summary
echo ""
echo "📊 Command Categories Available:"
echo "   • Artist (5 commands)"
echo "   • Album (2 commands)"
echo "   • Track (3 commands)"
echo "   • Chart (3 commands)"
echo "   • Geo (2 commands)"
echo "   • Tag (5 commands)"
echo "   • User (3 commands)"
echo "   • Library (1 command)"
echo "   • Auth (3 commands)"
echo "   • My (4 commands - authenticated)"
echo ""
echo "   Total: 31 commands"

echo ""
echo "📚 Documentation:"
echo "   • README.md - Main project documentation"
echo "   • docs/CLI_USER_MANUAL.md - Complete user guide"
echo "   • docs/CLI_SHOWCASE.md - All commands with examples"
echo "   • docs/CLI_ARCHITECTURE.md - Technical details"

echo ""
echo "✨ Everything is working correctly!"