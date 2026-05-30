#!/bin/bash

WORKER_URL="${WORKER_URL:-http://localhost:8787}"

echo "Testing Last.fm Proxy Worker"
echo "============================"
echo ""

echo "1. Testing health endpoint:"
curl -s "$WORKER_URL/health" | head -5
echo -e "\n"

echo "2. Testing artist info endpoint:"
curl -s "$WORKER_URL/artist/getInfo?artist=The%20Beatles" | jq '.' 2>/dev/null | head -20 || echo "Failed to get artist info"
echo -e "\n"

echo "3. Testing track search:"
curl -s "$WORKER_URL/track/search?track=Yesterday&limit=5" | jq '.' 2>/dev/null | head -20 || echo "Failed to search tracks"
echo -e "\n"

echo "4. Testing album info:"
curl -s "$WORKER_URL/album/getInfo?artist=Pink%20Floyd&album=The%20Wall" | jq '.' 2>/dev/null | head -20 || echo "Failed to get album info"
echo -e "\n"

echo "5. Testing chart endpoint:"
curl -s "$WORKER_URL/chart/getTopArtists?limit=5" | jq '.' 2>/dev/null | head -20 || echo "Failed to get top artists"