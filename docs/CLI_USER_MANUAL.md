# Last.fm CLI User Manual

A comprehensive command-line interface for interacting with the Last.fm API through a Cloudflare Worker proxy.

## Table of Contents

<details>
<summary><b>Installation & Setup</b></summary>

### Installation

```bash
# Clone the repository
git clone https://github.com/guitaripod/lastfm-rs.git
cd lastfm-rs

# Build the CLI
cargo build --release --bin lastfm-cli

# The binary will be available at:
./target/release/lastfm-cli
```

### Configuration

The CLI stores its configuration at `~/.config/lastfm-cli/config.toml`. On first run, a default configuration file is created with an empty `worker_url` — you must point it at the Cloudflare Worker you deployed (see the [Self-Hosting Guide](../README.md#self-hosting)) before running API commands.

```toml
worker_url = "https://your-worker.workers.dev"
output_format = "pretty"
cache_ttl = 3600
interactive_history_size = 1000
color_output = true
request_timeout_secs = 30

[auth]
# Authentication details are stored here after login
```

You can also set the backend without editing this file, via the `LASTFM_WORKER_URL` environment variable or the `--worker-url` flag (precedence: flag > env > config file).

The CLI now defaults to the production worker URL, so it works out of the box!

### Environment Variables

- `LASTFM_API_KEY` - Your Last.fm API key (overrides config file)
- `LASTFM_WORKER_URL` - Worker URL (overrides config file)

</details>

<details>
<summary><b>Authentication</b></summary>

### Overview

The CLI supports Last.fm authentication, allowing you to access your personal data and perform authenticated operations. Authentication is handled through the standard Last.fm auth flow.

### Auth Commands

#### Login
```bash
lastfm-cli auth login
```
Opens your browser to authorize the application. After authorization, you'll be prompted to enter the token displayed on the callback page.

#### Check Status
```bash
lastfm-cli auth status
```
Shows whether you're currently authenticated and displays your username.

#### Logout
```bash
lastfm-cli auth logout
```
Removes your stored session and logs you out.

### Authenticated Commands

Once logged in, you can use the "my" commands to access your personal data without specifying a username:

```bash
# Get your profile information
lastfm-cli my info

# Get your recent tracks
lastfm-cli my recent-tracks --limit 10

# Get your top artists
lastfm-cli my top-artists --period 7day

# Get your top tracks
lastfm-cli my top-tracks --period 1month --limit 20

# Get your top albums
lastfm-cli my top-albums --period overall
```

### Time Periods

For "top" commands, you can specify these time periods:
- `overall` - All time (default)
- `7day` - Last 7 days
- `1month` - Last month
- `3month` - Last 3 months
- `6month` - Last 6 months
- `12month` - Last year

</details>

<details>
<summary><b>Global Options</b></summary>

These options are available for all commands:

```
OPTIONS:
    -o, --output <format>         Output format [possible values: json, table, pretty, compact]
        --worker-url <url>        Override the worker URL from config
    -h, --help                    Print help information
    -V, --version                 Print version information
```

### Output Formats

- **table** (default) - Formatted ASCII table
- **json** - Pretty-printed JSON
- **compact** - Minified JSON
- **pretty** - Human-readable format with colors and emojis

</details>

<details>
<summary><b>Artist Commands</b></summary>

### artist info

Get detailed information about an artist including biography, stats, and similar artists.

```bash
lastfm-cli artist info <artist-name> [OPTIONS]
```

**Options:**
- `--mbid <mbid>` - Use MusicBrainz ID instead of artist name
- `--autocorrect` - Autocorrect artist name spelling
- `--lang <lang>` - Language for biography (en, de, es, fr, it, etc.)
- `--username <user>` - Include user-specific data (playcount, etc.)

**Example:**
```bash
$ lastfm-cli artist info "The Beatles" -o pretty
```

**Output:**
```
⏱  234ms | 📦 Fresh | 🌐 1 API call

The Beatles
🔗 https://www.last.fm/music/The+Beatles
👥 5806609 listeners
▶️  495745364 plays

Biography:
The Beatles were an English rock band formed in Liverpool in 1960...

Similar Artists:
  1. The Rolling Stones
  2. The Beach Boys
  3. The Kinks
  4. The Who
  5. Bob Dylan
```

### artist similar

Get artists similar to the specified artist.

```bash
lastfm-cli artist similar <artist-name> [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of similar artists to return (default: 50)
- `--autocorrect` - Autocorrect artist name spelling

**Example:**
```bash
$ lastfm-cli artist similar "Radiohead" --limit 10 -o table
```

**Output:**
```
Response time: 156ms | Cache: HIT | API calls: 1

+----+------------------------+-----------+---------+
| #  | Artist                 | Listeners | Match % |
+----+------------------------+-----------+---------+
| 1  | Thom Yorke             | 1506420   | 100.00  |
+----+------------------------+-----------+---------+
| 2  | Atoms for Peace        | 662584    | 93.84   |
+----+------------------------+-----------+---------+
| 3  | Blur                   | 3251538   | 68.25   |
+----+------------------------+-----------+---------+
| 4  | Portishead             | 2456295   | 66.72   |
+----+------------------------+-----------+---------+
| 5  | Sigur Rós              | 2475089   | 61.49   |
+----+------------------------+-----------+---------+
```

### artist search

Search for artists by name.

```bash
lastfm-cli artist search <query> [OPTIONS]
```

**Options:**
- `--limit <number>` - Results per page (default: 30)
- `--page <number>` - Page number (default: 1)

**Example:**
```bash
$ lastfm-cli artist search "pink" --limit 5 -o json
```

**Output:**
```json
{
  "results": {
    "artistmatches": {
      "artist": [
        {
          "name": "P!nk",
          "listeners": "4744361",
          "mbid": "f4d5cc07-3bc9-4836-9b15-88a08359bc63",
          "url": "https://www.last.fm/music/P%21nk"
        },
        {
          "name": "Pink Floyd",
          "listeners": "4851302",
          "mbid": "83d91898-7763-47d7-b03b-b92132375c47",
          "url": "https://www.last.fm/music/Pink+Floyd"
        }
      ]
    }
  }
}
```

### artist top-albums

Get the top albums for an artist.

```bash
lastfm-cli artist top-albums <artist-name> [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of albums (default: 50)
- `--page <number>` - Page number (default: 1)

**Example:**
```bash
$ lastfm-cli artist top-albums "Nirvana" --limit 5 -o table
```

**Output:**
```
Response time: 287ms | Cache: MISS | API calls: 1

+---+--------------------------------+-------------+------------+
| # | Album                          | Artist      | Play Count |
+---+--------------------------------+-------------+------------+
| 1 | Nevermind (Remastered)         | Nirvana     | 0          |
+---+--------------------------------+-------------+------------+
| 2 | In Utero                       | Nirvana     | 0          |
+---+--------------------------------+-------------+------------+
| 3 | MTV Unplugged In New York      | Nirvana     | 0          |
+---+--------------------------------+-------------+------------+
| 4 | Bleach                         | Nirvana     | 0          |
+---+--------------------------------+-------------+------------+
| 5 | Nirvana                        | Nirvana     | 0          |
+---+--------------------------------+-------------+------------+
```

### artist top-tracks

Get the top tracks for an artist.

```bash
lastfm-cli artist top-tracks <artist-name> [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of tracks (default: 50)
- `--page <number>` - Page number (default: 1)

**Example:**
```bash
$ lastfm-cli artist top-tracks "Queen" --limit 3 -o pretty
```

**Output:**
```
⏱  198ms | 📦 Fresh | 🌐 1 API call

Top Tracks by Queen
──────────────────────────────────────────────────

1. Bohemian Rhapsody - 2,547,893 plays
2. Don't Stop Me Now - 1,893,726 plays  
3. Somebody to Love - 1,432,185 plays
```

</details>

<details>
<summary><b>Album Commands</b></summary>

### album info

Get detailed information about an album.

```bash
lastfm-cli album info <artist> <album> [OPTIONS]
```

**Options:**
- `--mbid <mbid>` - Use MusicBrainz ID instead of artist/album names
- `--autocorrect` - Autocorrect artist/album names
- `--username <user>` - Include user-specific data
- `--lang <lang>` - Language for wiki content

**Example:**
```bash
$ lastfm-cli album info "The Beatles" "Abbey Road" -o pretty
```

**Output:**
```
⏱  156ms | 📦 Cached | 🌐 1 API call

Abbey Road
by The Beatles
👥 2894567 listeners
▶️  89472635 plays

Tracks:
  1. Come Together (4:19)
  2. Something (3:03)
  3. Maxwell's Silver Hammer (3:27)
  4. Oh! Darling (3:26)
  5. Octopus's Garden (2:51)
  6. I Want You (She's So Heavy) (7:47)
  7. Here Comes the Sun (3:05)
  8. Because (2:45)
  9. You Never Give Me Your Money (4:02)
 10. Sun King (2:26)
 11. Mean Mr. Mustard (1:06)
 12. Polythene Pam (1:12)
 13. She Came In Through the Bathroom Window (1:57)
 14. Golden Slumbers (1:31)
 15. Carry That Weight (1:36)
 16. The End (2:19)
 17. Her Majesty (0:23)
```

### album search

Search for albums by name.

```bash
lastfm-cli album search <query> [OPTIONS]
```

**Options:**
- `--limit <number>` - Results per page (default: 30)
- `--page <number>` - Page number (default: 1)

**Example:**
```bash
$ lastfm-cli album search "dark side" --limit 3 -o table
```

**Output:**
```
Response time: 234ms | Cache: MISS | API calls: 1

+---+--------------------------------------+-------------+--------------------------------------------------+
| # | Album                                | Artist      | URL                                              |
+---+--------------------------------------+-------------+--------------------------------------------------+
| 1 | The Dark Side of the Moon            | Pink Floyd  | https://www.last.fm/music/Pink+Floyd/The+Dar... |
+---+--------------------------------------+-------------+--------------------------------------------------+
| 2 | Dark Side of the Moon (2011 Remast..)| Pink Floyd  | https://www.last.fm/music/Pink+Floyd/Dark+Si... |
+---+--------------------------------------+-------------+--------------------------------------------------+
| 3 | Dub Side of the Moon                 | Easy Star   | https://www.last.fm/music/Easy+Star+All-Star... |
+---+--------------------------------------+-------------+--------------------------------------------------+
```

</details>

<details>
<summary><b>Track Commands</b></summary>

### track info

Get detailed information about a track.

```bash
lastfm-cli track info <artist> <track> [OPTIONS]
```

**Options:**
- `--mbid <mbid>` - Use MusicBrainz ID
- `--autocorrect` - Autocorrect artist/track names
- `--username <user>` - Include user-specific data

**Example:**
```bash
$ lastfm-cli track info "The Beatles" "Hey Jude" -o pretty
```

**Output:**
```
⏱  189ms | 📦 Fresh | 🌐 1 API call

Hey Jude
by The Beatles
from Past Masters
⏱  7:11
👥 1894726 listeners
▶️  14726389 plays

Tags: #classic-rock #rock #60s #pop #british
```

### track search

Search for tracks.

```bash
lastfm-cli track search <query> [OPTIONS]
```

**Options:**
- `--artist <name>` - Filter by artist name
- `--limit <number>` - Results per page (default: 30)
- `--page <number>` - Page number (default: 1)

### track similar

Get tracks similar to a specified track.

```bash
lastfm-cli track similar <artist> <track> [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of similar tracks (default: 50)

**Example:**
```bash
$ lastfm-cli track similar "Radiohead" "Creep" --limit 5 -o table
```

**Output:**
```
Response time: 742ms | Cache: HIT | API calls: 1

+---+------------------+--------------+---------+------------+
| # | Track            | Artist       | Match % | Play Count |
+---+------------------+--------------+---------+------------+
| 1 | No Surprises     | Radiohead    | 100.0   | 41956411   |
+---+------------------+--------------+---------+------------+
| 2 | Karma Police     | Radiohead    | 89.3    | 33663272   |
+---+------------------+--------------+---------+------------+
| 3 | Everlong         | Foo Fighters | 56.4    | 36566995   |
+---+------------------+--------------+---------+------------+
| 4 | High and Dry     | Radiohead    | 55.2    | 28374562   |
+---+------------------+--------------+---------+------------+
| 5 | Fake Plastic Trees| Radiohead    | 52.8    | 32847195   |
+---+------------------+--------------+---------+------------+
```

**Example:**
```bash
$ lastfm-cli track search "imagine" --limit 5 -o table
```

**Output:**
```
Response time: 276ms | Cache: MISS | API calls: 1

+---+---------------------------+------------------+-----------+
| # | Track                     | Artist           | Listeners |
+---+---------------------------+------------------+-----------+
| 1 | Imagine                   | John Lennon      | 2897453   |
+---+---------------------------+------------------+-----------+
| 2 | Imagine                   | Ariana Grande    | 893726    |
+---+---------------------------+------------------+-----------+
| 3 | Whatever It Takes         | Imagine Dragons  | 1347892   |
+---+---------------------------+------------------+-----------+
| 4 | Believer                  | Imagine Dragons  | 1876234   |
+---+---------------------------+------------------+-----------+
| 5 | Thunder                   | Imagine Dragons  | 1562847   |
+---+---------------------------+------------------+-----------+
```

</details>

<details>
<summary><b>Chart Commands</b></summary>

### chart top-artists

Get the top artists chart.

```bash
lastfm-cli chart top-artists [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of artists (default: 50)
- `--page <number>` - Page number (default: 1)

**Example:**
```bash
$ lastfm-cli chart top-artists --limit 5 -o pretty
```

**Output:**
```
⏱  234ms | 📦 Fresh | 🌐 1 API call

Top Artists
──────────────────────────────────────────────────
1. Taylor Swift - 5,234,891 plays
2. The Weeknd - 4,876,234 plays
3. Drake - 4,234,567 plays
4. Billie Eilish - 3,987,234 plays
5. Doja Cat - 3,456,789 plays
```

### chart top-tracks

Get the top tracks chart.

```bash
lastfm-cli chart top-tracks [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of tracks (default: 50)
- `--page <number>` - Page number (default: 1)

### chart top-tags

Get the top tags chart.

```bash
lastfm-cli chart top-tags [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of tags (default: 50)
- `--page <number>` - Page number (default: 1)

**Example:**
```bash
$ lastfm-cli chart top-tags --limit 5 -o table
```

**Output:**
```
Response time: 466ms | Cache: HIT | API calls: 1

+---+-------------+----------+--------+
| # | Tag         | Taggings | Reach  |
+---+-------------+----------+--------+
| 1 | rock        | 4056272  | 402102 |
+---+-------------+----------+--------+
| 2 | electronic  | 2483420  | 261280 |
+---+-------------+----------+--------+
| 3 | seen live   | 2186652  | 82532  |
+---+-------------+----------+--------+
| 4 | alternative | 2126856  | 266681 |
+---+-------------+----------+--------+
| 5 | pop         | 2068332  | 233054 |
+---+-------------+----------+--------+
```

**Example:**
```bash
$ lastfm-cli chart top-tracks --limit 10 -o table
```

**Output:**
```
Response time: 298ms | Cache: MISS | API calls: 1

+----+----------------------------------+--------------------+-----------+
| #  | Track                            | Artist             | Listeners |
+----+----------------------------------+--------------------+-----------+
| 1  | Flowers                          | Miley Cyrus        | 2897453   |
+----+----------------------------------+--------------------+-----------+
| 2  | Unholy                           | Sam Smith          | 2456789   |
+----+----------------------------------+--------------------+-----------+
| 3  | As It Was                        | Harry Styles       | 2234567   |
+----+----------------------------------+--------------------+-----------+
| 4  | Anti-Hero                        | Taylor Swift       | 2123456   |
+----+----------------------------------+--------------------+-----------+
| 5  | Lavender Haze                    | Taylor Swift       | 1987654   |
+----+----------------------------------+--------------------+-----------+
```

</details>

<details>
<summary><b>Geographic Commands</b></summary>

### geo top-artists

Get top artists by country.

```bash
lastfm-cli geo top-artists <country> [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of artists (default: 50)
- `--page <number>` - Page number (default: 1)

### geo top-tracks

Get top tracks by country.

```bash
lastfm-cli geo top-tracks <country> [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of tracks (default: 50)
- `--page <number>` - Page number (default: 1)

**Example:**
```bash
$ lastfm-cli geo top-tracks "Japan" --limit 5 -o table
```

**Output:**
```
Response time: 1170ms | Cache: MISS | API calls: 1

+---+------------+--------------------------+-----------+
| # | Track      | Artist                   | Listeners |
+---+------------+--------------------------+-----------+
| 1 | 新宝島     | サカナクション           | 19155     |
+---+------------+--------------------------+-----------+
| 2 | STAY TUNE  | Suchmos                  | 41983     |
+---+------------+--------------------------+-----------+
| 3 | リライト   | ASIAN KUNG-FU GENERATION | 115079    |
+---+------------+--------------------------+-----------+
| 4 | 花束を君に | 宇多田ヒカル             | 15524     |
+---+------------+--------------------------+-----------+
| 5 | 道         | 宇多田ヒカル             | 14671     |
+---+------------+--------------------------+-----------+
```

**Example:**
```bash
$ lastfm-cli geo top-artists "United Kingdom" --limit 5 -o table
```

**Output:**
```
Response time: 345ms | Cache: MISS | API calls: 1

+---+-----------------------+------------+-----------+
| # | Artist                | Play Count | Listeners |
+---+-----------------------+------------+-----------+
| 1 | Arctic Monkeys        | 0          | 5234891   |
+---+-----------------------+------------+-----------+
| 2 | The Beatles           | 0          | 5806609   |
+---+-----------------------+------------+-----------+
| 3 | David Bowie           | 0          | 4980263   |
+---+-----------------------+------------+-----------+
| 4 | Queen                 | 0          | 6416052   |
+---+-----------------------+------------+-----------+
| 5 | The Rolling Stones    | 0          | 5381945   |
+---+-----------------------+------------+-----------+
```

</details>

<details>
<summary><b>Tag Commands</b></summary>

### tag info

Get information about a tag.

```bash
lastfm-cli tag info <tag> [OPTIONS]
```

**Options:**
- `--lang <lang>` - Language for wiki content

### tag top-artists

Get top artists for a tag.

```bash
lastfm-cli tag top-artists <tag> [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of artists (default: 50)
- `--page <number>` - Page number (default: 1)

**Example:**
```bash
$ lastfm-cli tag top-artists "shoegaze" --limit 5 -o table
```

**Output:**
```
Response time: 1136ms | Cache: MISS | API calls: 1

+---+---------------------+------------+-----------+
| # | Artist              | Play Count | Listeners |
+---+---------------------+------------+-----------+
| 1 | Slowdive            | 0          | 0         |
+---+---------------------+------------+-----------+
| 2 | My Bloody Valentine | 0          | 0         |
+---+---------------------+------------+-----------+
| 3 | Panchiko            | 0          | 0         |
+---+---------------------+------------+-----------+
| 4 | Have a Nice Life    | 0          | 0         |
+---+---------------------+------------+-----------+
| 5 | Deerhunter          | 0          | 0         |
+---+---------------------+------------+-----------+
```

### tag top-albums

Get top albums for a tag.

```bash
lastfm-cli tag top-albums <tag> [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of albums (default: 50)
- `--page <number>` - Page number (default: 1)

### tag top-tracks

Get top tracks for a tag.

```bash
lastfm-cli tag top-tracks <tag> [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of tracks (default: 50)
- `--page <number>` - Page number (default: 1)

**Example:**
```bash
$ lastfm-cli tag info "shoegaze" -o pretty
```

**Output:**
```
⏱  198ms | 📦 Fresh | 🌐 1 API call

shoegaze
Total uses: 876,234
Reach: 234,567

Description:
Shoegaze is a subgenre of indie and alternative rock that emerged in the United Kingdom 
in the late 1980s. It is characterized by its ethereal mixture of obscured vocals, 
guitar distortion and effects, feedback, and overwhelming volume...

Related Tags: dream pop, noise pop, indie rock, alternative, post-punk
```

</details>

<details>
<summary><b>User Commands</b></summary>

### user info

Get information about a Last.fm user.

```bash
lastfm-cli user info <username> [OPTIONS]
```

**Example:**
```bash
$ lastfm-cli user info "rj" -o pretty
```

**Output:**
```
⏱  234ms | 📦 Fresh | 🌐 1 API call

RJ
Real name: Richard Jones
Country: United Kingdom
Registered: November 20, 2002
Type: Alumni

Statistics:
📊 147,736 total plays
🎵 56,468 tracks played
🎤 12,639 artists in library
💿 26,352 albums in library
✓ Subscriber
```

### user recent-tracks

Get a user's recently played tracks.

```bash
lastfm-cli user recent-tracks <username> [OPTIONS]
```

**Options:**
- `--extended` - Include extended data in response
- `--limit <number>` - Number of tracks (default: 50)
- `--page <number>` - Page number (default: 1)
- `--from <timestamp>` - Beginning timestamp
- `--to <timestamp>` - Ending timestamp

### user top-artists

Get a user's top artists.

```bash
lastfm-cli user top-artists <username> [OPTIONS]
```

**Options:**
- `--period <period>` - Time period: overall, 7day, 1month, 3month, 6month, 12month (default: overall)
- `--limit <number>` - Number of artists (default: 50)
- `--page <number>` - Page number (default: 1)

**Example:**
```bash
$ lastfm-cli user top-artists "rj" --period 7day --limit 5 -o table
```

**Output:**
```
Response time: 1261ms | Cache: MISS | API calls: 1

+---+----------------+------------+-----------+
| # | Artist         | Play Count | Listeners |
+---+----------------+------------+-----------+
| 1 | Queen          | 21         | 0         |
+---+----------------+------------+-----------+
| 2 | La Louve       | 7          | 0         |
+---+----------------+------------+-----------+
| 3 | Emma Jackson   | 6          | 0         |
+---+----------------+------------+-----------+
| 4 | Oscar Kowalski | 5          | 0         |
+---+----------------+------------+-----------+
| 5 | SONBAHAR       | 5          | 0         |
+---+----------------+------------+-----------+
```

**Example:**
```bash
$ lastfm-cli user recent-tracks "someuser" --limit 5 -o table
```

**Output:**
```
Response time: 287ms | Cache: MISS | API calls: 1

+---+--------------------------------+----------------------+------------------+
| # | Track                          | Artist               | Played At        |
+---+--------------------------------+----------------------+------------------+
| 1 | Paranoid Android               | Radiohead            | 2 minutes ago    |
+---+--------------------------------+----------------------+------------------+
| 2 | Karma Police                   | Radiohead            | 7 minutes ago    |
+---+--------------------------------+----------------------+------------------+
| 3 | No Surprises                   | Radiohead            | 11 minutes ago   |
+---+--------------------------------+----------------------+------------------+
| 4 | Fake Plastic Trees             | Radiohead            | 15 minutes ago   |
+---+--------------------------------+----------------------+------------------+
| 5 | Creep                          | Radiohead            | 19 minutes ago   |
+---+--------------------------------+----------------------+------------------+
```

</details>

<details>
<summary><b>Library Commands</b></summary>

### library artists

Get artists from a user's library.

```bash
lastfm-cli library artists <username> [OPTIONS]
```

**Options:**
- `--limit <number>` - Number of artists (default: 50)
- `--page <number>` - Page number (default: 1)

**Example:**
```bash
$ lastfm-cli library artists "someuser" --limit 5 -o table
```

**Output:**
```
Response time: 342ms | Cache: MISS | API calls: 1

+---+------------------+------------+-----------+
| # | Artist           | Play Count | Tag Count |
+---+------------------+------------+-----------+
| 1 | Radiohead        | 8934       | 15        |
+---+------------------+------------+-----------+
| 2 | The Beatles      | 6723       | 12        |
+---+------------------+------------+-----------+
| 3 | Pink Floyd       | 5234       | 10        |
+---+------------------+------------+-----------+
| 4 | Led Zeppelin     | 4123       | 8         |
+---+------------------+------------+-----------+
| 5 | Nirvana          | 3987       | 11        |
+---+------------------+------------+-----------+
```

</details>

<details>
<summary><b>Error Handling</b></summary>

The CLI provides clear error messages for common issues:

### Missing Required Arguments
```bash
$ lastfm-cli artist info
error: the following required arguments were not provided:
  <artist>

Usage: lastfm-cli artist info <artist>

For more information, try '--help'.
```

### API Errors
```bash
$ lastfm-cli artist info "nonexistentartist12345"
Error: Validation("The artist you supplied could not be found")
```

### Network Errors
```bash
$ lastfm-cli --worker-url https://invalid-url.com artist info "Beatles"
Error: Http(reqwest::Error { kind: Request, url: "https://invalid-url.com/artist/getInfo?artist=Beatles&autocorrect=1", source: hyper_util::client::legacy::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: No address associated with hostname" })) })
```

### Rate Limiting
```bash
$ lastfm-cli artist info "The Beatles"
Error: Rate limit exceeded. Please wait before making more requests.
```

</details>

<details>
<summary><b>Tips & Best Practices</b></summary>

### Performance Tips

1. **Use caching**: The worker caches responses for 1 hour by default
2. **Batch operations**: Use higher limits to reduce API calls
3. **Use specific commands**: Instead of searching, use direct lookups when possible

### Output Format Selection

- Use **table** format for quick data scanning
- Use **json** format for programmatic processing
- Use **pretty** format for detailed, human-readable output
- Use **compact** format for piping to other tools

### Shell Integration

```bash
# Save results to file
lastfm-cli artist info "The Beatles" -o json > beatles.json

# Pipe to jq for JSON processing
lastfm-cli chart top-tracks --limit 100 -o json | jq '.tracks.track[].name'

# Use in scripts
LISTENERS=$(lastfm-cli artist info "Radiohead" -o json | jq -r '.artist.stats.listeners')
echo "Radiohead has $LISTENERS listeners"
```

### Common Workflows

```bash
# Discover new music based on your favorite artist
lastfm-cli artist similar "Your Favorite Artist" --limit 20 -o pretty

# Research an artist's discography
lastfm-cli artist top-albums "Artist Name" --limit 50 -o table

# Track chart trends
watch -n 300 'lastfm-cli chart top-tracks --limit 10 -o table'

# Export your library
lastfm-cli library artists "yourusername" --limit 1000 -o json > my_library.json
```

</details>

<details>
<summary><b>Troubleshooting</b></summary>

### Common Issues

**Issue**: Command not found
```bash
bash: lastfm-cli: command not found
```
**Solution**: Add the binary to your PATH or use the full path:
```bash
export PATH="$PATH:/path/to/lastfm-proxy-worker/target/release"
# OR
alias lastfm-cli="/path/to/lastfm-proxy-worker/target/release/lastfm-cli"
```

**Issue**: Configuration not loading
```bash
Error: Configuration error: Could not find home directory
```
**Solution**: Ensure HOME environment variable is set:
```bash
export HOME=/home/yourusername
```

**Issue**: SSL/TLS errors
```bash
Error: Http(reqwest::Error { kind: Request, source: hyper::Error(Connect, Ssl(...)) })
```
**Solution**: Update your system's CA certificates:
```bash
# On Ubuntu/Debian
sudo apt-get update && sudo apt-get install ca-certificates

# On macOS
brew install ca-certificates
```

### Debug Mode

Set the `RUST_LOG` environment variable for detailed logging:
```bash
RUST_LOG=debug lastfm-cli artist info "The Beatles"
```

</details>

## Quick Command Reference

### Authentication Commands
- `auth login` - Authenticate with Last.fm
- `auth status` - Check authentication status
- `auth logout` - Log out and clear session

### Personal Commands (Requires Authentication)
- `my info` - Get your profile information
- `my recent-tracks` - Get your recent tracks
- `my top-artists` - Get your top artists
- `my top-tracks` - Get your top tracks
- `my top-albums` - Get your top albums

### Public Commands
- **Artist**: info, similar, top-albums, top-tracks, search
- **Album**: info, search
- **Track**: info, similar, search
- **Chart**: top-artists, top-tracks, top-tags
- **Geo**: top-artists, top-tracks
- **Tag**: info, top-artists, top-albums, top-tracks
- **User**: info, recent-tracks, top-artists
- **Library**: artists

## Version History

- **v1.1.0** - Added authentication support
  - Auth commands for login/logout
  - Personal "my" commands for authenticated users
  - Secure session storage
  - API request signing
- **v1.0.0** - Initial release with full Last.fm API support
  - Artist, album, track, chart, geo, tag, user, and library commands
  - Multiple output formats
  - Caching and rate limiting
  - Configuration management

## License

This project is licensed under the MIT License. See LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

For issues, feature requests, or questions:
- Open an issue on [GitHub](https://github.com/guitaripod/lastfm-rs/issues)
- Check the [FAQ](#troubleshooting) section above
