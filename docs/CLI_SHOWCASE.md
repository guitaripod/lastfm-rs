# Last.fm CLI Complete Command Showcase

This document demonstrates all available commands in the Last.fm CLI with real examples and outputs, including authentication and personal commands for authenticated users.

## Table of Contents
- [Authentication Commands](#authentication-commands)
- [Personal Commands (My)](#personal-commands-my)
- [Artist Commands](#artist-commands)
- [Album Commands](#album-commands)
- [Track Commands](#track-commands)
- [Chart Commands](#chart-commands)
- [Geographic Commands](#geographic-commands)
- [Tag Commands](#tag-commands)
- [User Commands](#user-commands)
- [Library Commands](#library-commands)
- [Output Formats](#output-formats)
- [Configuration](#configuration)
- [Error Handling](#error-handling)

---

## Authentication Commands

### 1. Auth Status (Not Logged In)
Check authentication status when not logged in.

```bash
$ ./target/release/lastfm-cli auth status
```

Output:
```json
{
  "authenticated": false,
  "username": null
}
```

### 2. Auth Login
Authenticate with Last.fm (interactive flow).

```bash
$ ./target/release/lastfm-cli auth login
```

Output:
```
Opening browser for authorization...
If the browser doesn't open, visit this URL:
https://www.last.fm/api/auth/?api_key=REDACTED_API_KEY&cb=http://localhost:8080/auth/callback

After authorizing, you'll be redirected to a page showing an auth token.
Please enter the token here:
Token: [user enters token]
✓ Successfully authenticated as 'rj'
```

### 3. Auth Status (Logged In)
Check authentication status after logging in.

```bash
$ ./target/release/lastfm-cli auth status -o pretty
```

Output:
```json
{
  "authenticated": true,
  "username": "rj"
}
```

### 4. Auth Logout
Log out and clear stored session.

```bash
$ ./target/release/lastfm-cli auth logout
```

Output:
```
Successfully logged out
{
  "authenticated": false,
  "message": "Successfully logged out"
}
```

---

## Personal Commands (My)

These commands work only when authenticated (as rj) and automatically use your username.

### 1. My Info
Get your own profile information.

```bash
$ ./target/release/lastfm-cli my info -o table
```

Output:
```
Response time: 1405ms | Cache: MISS | API calls: 1

+------+--------------------------------------+
| Key  | Value                                |
+------+--------------------------------------+
| Name | rj                                   |
+------+--------------------------------------+
| Real | RJ                                   |
| Name |                                      |
+------+--------------------------------------+
| Play | 151049                               |
| Count|                                      |
+------+--------------------------------------+
| Artists | 8788                              |
+------+--------------------------------------+
| Albums  | 8447                              |
+------+--------------------------------------+
| Tracks  | 29264                             |
+------+--------------------------------------+
| Since   | Apr 2010                          |
+------+--------------------------------------+
| URL  | https://www.last.fm/user/rj          |
+------+--------------------------------------+
```

### 2. My Recent Tracks
Get your recently played tracks.

```bash
$ ./target/release/lastfm-cli my recent-tracks --limit 3 -o table
```

Output:
```
Response time: 1289ms | Cache: MISS | API calls: 1

+---+----------------------------------------+-------------------+---------------------------------------+--------------------+
| # | Track                                  | Artist            | Album                                 | Date               |
+---+----------------------------------------+-------------------+---------------------------------------+--------------------+
| 1 | XPLR                                   | GHOSTEMANE        | HIADICA                               | 07 Jul 2025, 14:52 |
+---+----------------------------------------+-------------------+---------------------------------------+--------------------+
| 2 | System Blower                          | Death Grips       | The Money Store                       | 07 Jul 2025, 14:49 |
+---+----------------------------------------+-------------------+---------------------------------------+--------------------+
| 3 | I Staple Tapeworms on My Penis         | Passenger of Shit | I Staple Tapeworms on My Penis - Sin... | 07 Jul 2025, 14:44 |
+---+----------------------------------------+-------------------+---------------------------------------+--------------------+
```

### 3. My Top Artists
Get your top artists for a specific time period.

```bash
$ ./target/release/lastfm-cli my top-artists --period 7day --limit 5 -o table
```

Output:
```
Response time: 1320ms | Cache: MISS | API calls: 1

+---+---------------------+------------+-----------+
| # | Artist              | Play Count | Listeners |
+---+---------------------+------------+-----------+
| 1 | Metallica           | 16         | 0         |
+---+---------------------+------------+-----------+
| 2 | Drake               | 14         | 0         |
+---+---------------------+------------+-----------+
| 3 | Final Fantasy Union | 12         | 0         |
+---+---------------------+------------+-----------+
| 4 | Pendulum            | 12         | 0         |
+---+---------------------+------------+-----------+
| 5 | System of a Down    | 10         | 0         |
+---+---------------------+------------+-----------+
```

### 4. My Top Tracks
Get your most played tracks.

```bash
$ ./target/release/lastfm-cli my top-tracks --period 1month --limit 3 -o table
```

Output:
```
Response time: 1276ms | Cache: MISS | API calls: 1

+---+-------------+------------------+-------+
| # | Track       | Artist           | Plays |
+---+-------------+------------------+-------+
| 1 | Psycho      | System of a Down | 21    |
+---+-------------+------------------+-------+
| 2 | Aerials     | System of a Down | 18    |
+---+-------------+------------------+-------+
| 3 | Watercolour | Pendulum         | 8     |
+---+-------------+------------------+-------+
```

### 5. My Top Albums
Get your most played albums.

```bash
$ ./target/release/lastfm-cli my top-albums --period overall --limit 3 -o table
```

Output:
```
Response time: 1317ms | Cache: MISS | API calls: 1

+---+-----------------------------------+-------------+-------+
| # | Album                             | Artist      | Plays |
+---+-----------------------------------+-------------+-------+
| 1 | beerbongs & bentleys              | Post Malone | 4775  |
+---+-----------------------------------+-------------+-------+
| 2 | FINAL FANTASY X ORIGINAL SOUNDTR… | Various     | 2727  |
|   |                                   | Artists     |       |
+---+-----------------------------------+-------------+-------+
| 3 | Stoney (Deluxe)                   | Post Malone | 2333  |
+---+-----------------------------------+-------------+-------+
```

---

## Artist Commands

### 1. Artist Info
Get detailed information about an artist including biography, stats, and similar artists.

```bash
$ ./target/release/lastfm-cli artist info "The Beatles" -o table
```

Output:
```
Response time: 781ms | Cache: MISS | API calls: 1

+------------+---------------------------------------+
| Field      | Value                                 |
+------------+---------------------------------------+
| Name       | The Beatles                           |
+------------+---------------------------------------+
| Listeners  | "5806609"                             |
+------------+---------------------------------------+
| Play Count | "879284673"                           |
+------------+---------------------------------------+
| URL        | https://www.last.fm/music/The+Beatles |
+------------+---------------------------------------+
```

### 2. Artist Similar
Find artists similar to a specified artist.

```bash
$ ./target/release/lastfm-cli artist similar "Radiohead" --limit 5 -o table
```

Output:
```
Response time: 605ms | Cache: MISS | API calls: 1

+---+-----------------+---------+
| # | Artist          | Match % |
+---+-----------------+---------+
| 1 | Thom Yorke      | 100.0   |
+---+-----------------+---------+
| 2 | Atoms for Peace | 55.3    |
+---+-----------------+---------+
| 3 | Jeff Buckley    | 47.8    |
+---+-----------------+---------+
| 4 | The Strokes     | 37.1    |
+---+-----------------+---------+
| 5 | Deftones        | 33.1    |
+---+-----------------+---------+
```

### 3. Artist Search
Search for artists by name.

```bash
$ ./target/release/lastfm-cli artist search "Pink" --limit 3 -o table
```

Output:
```
Response time: 1232ms | Cache: MISS | API calls: 1

+---+----------------+-----------+------------------------------------------+
| # | Artist         | Listeners | URL                                      |
+---+----------------+-----------+------------------------------------------+
| 1 | Pink Floyd     | 4851302   | https://www.last.fm/music/Pink+Floyd     |
+---+----------------+-----------+------------------------------------------+
| 2 | pinkpantheress | 2331604   | https://www.last.fm/music/pinkpantheress |
+---+----------------+-----------+------------------------------------------+
| 3 | Pink Sweat$    | 752288    | https://www.last.fm/music/Pink+Sweat$    |
+---+----------------+-----------+------------------------------------------+
```

### 4. Artist Top Albums
Get the most popular albums by an artist.

```bash
$ ./target/release/lastfm-cli artist top-albums "Nirvana" --limit 5 -o table
```

Output:
```
Response time: 1180ms | Cache: MISS | API calls: 1

+---+---------------------------+---------+------------+
| # | Album                     | Artist  | Play Count |
+---+---------------------------+---------+------------+
| 1 | Nevermind                 | Nirvana | 0          |
+---+---------------------------+---------+------------+
| 2 | In Utero                  | Nirvana | 0          |
+---+---------------------------+---------+------------+
| 3 | Nevermind (Remastered)    | Nirvana | 0          |
+---+---------------------------+---------+------------+
| 4 | MTV Unplugged in New York | Nirvana | 0          |
+---+---------------------------+---------+------------+
| 5 | Nirvana                   | Nirvana | 0          |
+---+---------------------------+---------+------------+
```

### 5. Artist Top Tracks
Get the most popular tracks by an artist.

```bash
$ ./target/release/lastfm-cli artist top-tracks "Queen" --limit 5 -o table
```

Output:
```
Response time: 1128ms | Cache: MISS | API calls: 1

+---+----------------------------------------------+--------+------------+
| # | Track                                        | Artist | Play Count |
+---+----------------------------------------------+--------+------------+
| 1 | Bohemian Rhapsody                            | Queen  | 13948654   |
+---+----------------------------------------------+--------+------------+
| 2 | Bohemian Rhapsody - Remastered 2011          | Queen  | 10301634   |
+---+----------------------------------------------+--------+------------+
| 3 | Don't Stop Me Now - Remastered 2011          | Queen  | 10476592   |
+---+----------------------------------------------+--------+------------+
| 4 | Don't Stop Me Now                            | Queen  | 9074974    |
+---+----------------------------------------------+--------+------------+
| 5 | Another One Bites The Dust - Remastered 2011 | Queen  | 8920733    |
+---+----------------------------------------------+--------+------------+
```

---

## Album Commands

### 1. Album Info
Get detailed information about an album.

```bash
$ ./target/release/lastfm-cli album info "The Beatles" "Abbey Road" -o table
```

Output:
```
Response time: 588ms | Cache: MISS | API calls: 1

+------------+-------------+
| Field      | Value       |
+------------+-------------+
| Album      | Abbey Road  |
+------------+-------------+
| Artist     | The Beatles |
+------------+-------------+
| Listeners  | "1112612"   |
+------------+-------------+
| Play Count | "44341250"  |
+------------+-------------+
```

### 2. Album Search
Search for albums by name.

```bash
$ ./target/release/lastfm-cli album search "Dark Side" --limit 3 -o table
```

Output:
```
Response time: 567ms | Cache: MISS | API calls: 1

+---+-----------------------------------------------------+-------------+---------------------------------------------------------------+
| # | Album                                               | Artist      | URL                                                           |
+---+-----------------------------------------------------+-------------+---------------------------------------------------------------+
| 1 | The Dark Side Of The Moon                           | Pink Floyd  | https://www.last.fm/music/Pink+Floyd/The+Dark+Side+Of+The... |
+---+-----------------------------------------------------+-------------+---------------------------------------------------------------+
| 2 | The Dark Side Of The Moon (2011 Remastered Version) | Pink Floyd  | https://www.last.fm/music/Pink+Floyd/The+Dark+Side+Of+The... |
+---+-----------------------------------------------------+-------------+---------------------------------------------------------------+
| 3 | DARK SIDE OF THE CLOUDS                             | $uicideboy$ | https://www.last.fm/music/$uicideboy$/DARK+SIDE+OF+THE+CL... |
+---+-----------------------------------------------------+-------------+---------------------------------------------------------------+
```

---

## Track Commands

### 1. Track Info
Get information about a specific track.

```bash
$ ./target/release/lastfm-cli track info "The Beatles" "Hey Jude" -o table
```

Output:
```
Response time: 629ms | Cache: MISS | API calls: 1

+------------+-------------+
| Field      | Value       |
+------------+-------------+
| Track      | Hey Jude    |
+------------+-------------+
| Artist     | The Beatles |
+------------+-------------+
| Listeners  | "1104023"   |
+------------+-------------+
| Play Count | "7107395"   |
+------------+-------------+
```

### 2. Track Similar
Find tracks similar to a specified track.

```bash
$ ./target/release/lastfm-cli track similar "Radiohead" "Creep" --limit 5 -o table
```

Output:
```
Response time: 1165ms | Cache: MISS | API calls: 1

+---+-------------------+-----------------+---------+------------+
| # | Track             | Artist          | Match % | Play Count |
+---+-------------------+-----------------+---------+------------+
| 1 | No Surprises      | Radiohead       | 100.0   | 41956411   |
+---+-------------------+-----------------+---------+------------+
| 2 | Karma Police      | Radiohead       | 89.3    | 33663272   |
+---+-------------------+-----------------+---------+------------+
| 3 | Everlong          | Foo Fighters    | 56.4    | 36566995   |
+---+-------------------+-----------------+---------+------------+
| 4 | Where Is My Mind? | Pixies          | 50.6    | 18036908   |
+---+-------------------+-----------------+---------+------------+
| 5 | Zombie            | The Cranberries | 42.6    | 18971683   |
+---+-------------------+-----------------+---------+------------+
```

### 3. Track Search
Search for tracks by name.

```bash
$ ./target/release/lastfm-cli track search "Imagine" --limit 3 -o table
```

Output:
```
Response time: 587ms | Cache: MISS | API calls: 1

+---+---------------------------+---------------+-----------+
| # | Track                     | Artist        | Listeners |
+---+---------------------------+---------------+-----------+
| 1 | Imagine                   | Ariana Grande | 590422    |
+---+---------------------------+---------------+-----------+
| 2 | Imagine - Remastered 2010 | John Lennon   | 531395    |
+---+---------------------------+---------------+-----------+
| 3 | Imagine                   | John Lennon   | 893731    |
+---+---------------------------+---------------+-----------+
```

---

## Chart Commands

### 1. Chart Top Artists
Get the current top artists globally.

```bash
$ ./target/release/lastfm-cli chart top-artists --limit 5 -o table
```

Output:
```
Response time: 579ms | Cache: MISS | API calls: 1

+---+--------------------+-----------+------------+
| # | Artist             | Listeners | Play Count |
+---+--------------------+-----------+------------+
| 1 | Kendrick Lamar     | 4504184   | 848101950  |
+---+--------------------+-----------+------------+
| 2 | The Weeknd         | 4667392   | 904367015  |
+---+--------------------+-----------+------------+
| 3 | Lady Gaga          | 6811557   | 773863504  |
+---+--------------------+-----------+------------+
| 4 | Tyler, the Creator | 3707803   | 816064985  |
+---+--------------------+-----------+------------+
| 5 | Radiohead          | 7271048   | 1068152617 |
+---+--------------------+-----------+------------+
```

### 2. Chart Top Tracks
Get the current top tracks globally.

```bash
$ ./target/release/lastfm-cli chart top-tracks --limit 5 -o table
```

Output:
```
Response time: 1091ms | Cache: MISS | API calls: 1

+---+--------------------+-------------------+-----------+
| # | Track              | Artist            | Listeners |
+---+--------------------+-------------------+-----------+
| 1 | Manchild           | Sabrina Carpenter | 589081    |
+---+--------------------+-------------------+-----------+
| 2 | Good Luck, Babe!   | Chappell Roan     | 1704527   |
+---+--------------------+-------------------+-----------+
| 3 | back to friends    | Sombr             | 650530    |
+---+--------------------+-------------------+-----------+
| 4 | BIRDS OF A FEATHER | Billie Eilish     | 1810311   |
+---+--------------------+-------------------+-----------+
| 5 | Party 4 U          | Charli XCX        | 827347    |
+---+--------------------+-------------------+-----------+
```

### 3. Chart Top Tags
Get the most popular tags/genres.

```bash
$ ./target/release/lastfm-cli chart top-tags --limit 5 -o table
```

Output:
```
Response time: 671ms | Cache: MISS | API calls: 1

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

---

## Geographic Commands

### 1. Geo Top Artists
Get top artists by country.

```bash
$ ./target/release/lastfm-cli geo top-artists "United States" --limit 5 -o table
```

Output:
```
Response time: 1244ms | Cache: MISS | API calls: 1

+---+-------------+------------+-----------+
| # | Artist      | Play Count | Listeners |
+---+-------------+------------+-----------+
| 1 | Radiohead   | 0          | 7271048   |
+---+-------------+------------+-----------+
| 2 | David Bowie | 0          | 4980263   |
+---+-------------+------------+-----------+
| 3 | The Weeknd  | 0          | 4667392   |
+---+-------------+------------+-----------+
| 4 | The Beatles | 0          | 5806609   |
+---+-------------+------------+-----------+
| 5 | Kanye West  | 0          | 7205419   |
+---+-------------+------------+-----------+
```

### 2. Geo Top Tracks
Get top tracks by country (example with Japan).

```bash
$ ./target/release/lastfm-cli geo top-tracks "Japan" --limit 5 -o table
```

Output:
```
Response time: 589ms | Cache: MISS | API calls: 1

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

---

## Tag Commands

### 1. Tag Info
Get information about a music tag/genre.

```bash
$ ./target/release/lastfm-cli tag info "shoegaze" -o pretty
```

Output:
```
⏱  1165ms | 📦 Fresh | 🌐 1 API call

shoegaze
👥 41888 reach
📊 223765 total taggings

Wiki:
Shoegaze is a subgenre of indie rock characterized by its ethereal mixture of obscured vocals, guitar distortion and effects, feedback, and overwhelming volume. It emerged in Ireland and the United Kingdom in the late 1980s among neo-psychedelic groups...
```

### 2. Tag Similar
Find tags similar to a specified tag.

```bash
$ ./target/release/lastfm-cli tag similar "shoegaze" --limit 5 -o table
```

Output:
```
Response time: 1165ms | Cache: MISS | API calls: 1

+---+--------------------+
| # | Tag                |
+---+--------------------+
| 1 | dream pop          |
+---+--------------------+
| 2 | noise pop          |
+---+--------------------+
| 3 | post-punk          |
+---+--------------------+
| 4 | indie              |
+---+--------------------+
| 5 | alternative        |
+---+--------------------+
```

### 3. Tag Top Artists
Get top artists for a specific tag.

```bash
$ ./target/release/lastfm-cli tag top-artists "shoegaze" --limit 5 -o table
```

Output:
```
Response time: 561ms | Cache: MISS | API calls: 1

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

### 4. Tag Top Albums
Get top albums for a specific tag.

```bash
$ ./target/release/lastfm-cli tag top-albums "rock" --limit 3 -o table
```

Output:
```
Response time: 600ms | Cache: MISS | API calls: 1

+---+-------------------------+-------------+------------+
| # | Album                   | Artist      | Play Count |
+---+-------------------------+-------------+------------+
| 1 | Twilight                | bôa         | 0          |
+---+-------------------------+-------------+------------+
| 2 | Abbey Road (Remastered) | The Beatles | 0          |
+---+-------------------------+-------------+------------+
| 3 | Minutes to Midnight     | Linkin Park | 0          |
+---+-------------------------+-------------+------------+
```

### 5. Tag Top Tracks
Get top tracks for a specific tag.

```bash
$ ./target/release/lastfm-cli tag top-tracks "electronic" --limit 3 -o table
```

Output:
```
Response time: 574ms | Cache: MISS | API calls: 1

+---+----------------+-----------+-----------+
| # | Track          | Artist    | Listeners |
+---+----------------+-----------+-----------+
| 1 | Feel Good Inc. | Gorillaz  | 0         |
+---+----------------+-----------+-----------+
| 2 | Poker Face     | Lady Gaga | 0         |
+---+----------------+-----------+-----------+
| 3 | Bad Romance    | Lady Gaga | 0         |
+---+----------------+-----------+-----------+
```

---

## User Commands

### 1. User Info
Get information about a Last.fm user.

```bash
$ ./target/release/lastfm-cli user info "rj" -o pretty
```

Output:
```
⏱  626ms | 📦 Fresh | 🌐 1 API call

rj
📍 London, United Kingdom
🔗 https://www.last.fm/user/rj
👥 26352 albums
🎵 12639 artists
▶️  1045485 plays
📅 Since Dec 2002
🏳️  0 age
```

### 2. User Recent Tracks
Get a user's recently played tracks.

```bash
$ ./target/release/lastfm-cli user recent-tracks "rj" --limit 3 -o table
```

Output:
```
Response time: 592ms | Cache: MISS | API calls: 1

+---+----------------------------------------------+--------+-----------------------------------+--------------------+
| # | Track                                        | Artist | Album                             | Date               |
+---+----------------------------------------------+--------+-----------------------------------+--------------------+
| 1 | We Will Rock You - Remastered 2011           | Queen  | News of the World (2011 Remaster) | 06 Jul 2025, 14:21 |
+---+----------------------------------------------+--------+-----------------------------------+--------------------+
| 2 | Another One Bites The Dust - Remastered 2011 | Queen  | The Game (2011 Remaster)          | 06 Jul 2025, 14:18 |
+---+----------------------------------------------+--------+-----------------------------------+--------------------+
| 3 | We Are The Champions - Remastered 2011       | Queen  | Greatest Hits (Remastered)        | 06 Jul 2025, 14:11 |
+---+----------------------------------------------+--------+-----------------------------------+--------------------+
```

### 3. User Top Artists
Get a user's top artists for a specific time period.

```bash
$ ./target/release/lastfm-cli user top-artists "rj" --period 7day --limit 5 -o table
```

Output:
```
Response time: 599ms | Cache: MISS | API calls: 1

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

Available periods: `overall`, `7day`, `1month`, `3month`, `6month`, `12month`

---

## Library Commands

### Library Artists
Get artists from a user's library.

```bash
$ ./target/release/lastfm-cli library artists "rj" -o table
```

Output (showing first 5):
```
Response time: 572ms | Cache: MISS | API calls: 1

+----+--------------------------+-----------+------------+
| #  | Artist                   | Listeners | Play Count |
+----+--------------------------+-----------+------------+
| 1  | Dream Theater            | 0         | 1877       |
+----+--------------------------+-----------+------------+
| 2  | Dire Straits             | 0         | 1686       |
+----+--------------------------+-----------+------------+
| 3  | Snoop Dogg               | 0         | 1226       |
+----+--------------------------+-----------+------------+
| 4  | Miles Davis              | 0         | 978        |
+----+--------------------------+-----------+------------+
| 5  | Metallica                | 0         | 968        |
+----+--------------------------+-----------+------------+
```

---

## Output Formats

The CLI supports four different output formats:

### 1. Table Format (default)
Human-readable ASCII tables with response metadata.

```bash
$ ./target/release/lastfm-cli artist info "Radiohead" -o table
```

### 2. JSON Format
Pretty-printed JSON for programmatic use.

```bash
$ ./target/release/lastfm-cli artist info "Radiohead" -o json
```

Output (truncated):
```json
{
  "artist": {
    "bio": {
      "content": "Radiohead are an English alternative rock band...",
      "links": {
        "link": {
          "#text": "",
          "href": "https://last.fm/music/Radiohead/+wiki",
          "rel": "original"
        }
      }
    },
    "image": [
      {
        "#text": "https://lastfm.freetls.fastly.net/i/u/34s/2a96cbd8b46e442fc41c2b86b821562f.png",
        "size": "small"
      }
    ],
    ...
  }
}
```

### 3. Compact Format
Minified JSON for piping to other tools.

```bash
$ ./target/release/lastfm-cli artist search "Beatles" --limit 2 -o compact
```

Output:
```json
{"results":{"@attr":{"for":"Beatles"},"artistmatches":{"artist":[{"image":[...],"listeners":"5806609","mbid":"b10bbbfc-cf9e-42e0-be17-e2c3e1d2600d","name":"The Beatles","streamable":"0","url":"https://www.last.fm/music/The+Beatles"},...]}}}
```

### 4. Pretty Format
Human-friendly format with colors and emojis (when color output is enabled).

```bash
$ ./target/release/lastfm-cli artist info "Pink Floyd" -o pretty
```

Output:
```
⏱  1139ms | 📦 Fresh | 🌐 1 API call

Pink Floyd
🔗 https://www.last.fm/music/Pink+Floyd
👥 "4851302" listeners
▶️  "501816852" plays

Biography:
Pink Floyd are an English rock band formed in London in 1965...

Similar Artists:
  1. Roger Waters
  2. David Gilmour
  3. Syd Barrett
  4. Led Zeppelin
  5. King Crimson
```

---

## Configuration

The CLI configuration is stored at `~/.config/lastfm-cli/config.toml`:

```toml
worker_url = "https://your-worker.workers.dev"
output_format = "pretty"
cache_ttl = 3600
interactive_history_size = 1000
color_output = true
request_timeout_secs = 30
```

You can override any setting with command-line options:

```bash
# Use a different output format
$ ./target/release/lastfm-cli artist info "The Beatles" -o json

# Use a different worker URL
$ ./target/release/lastfm-cli --worker-url https://custom.workers.dev artist info "The Beatles"
```

---

## Error Handling

The CLI provides clear error messages for common issues:

### Artist Not Found
```bash
$ ./target/release/lastfm-cli artist info "NonExistentArtist12345" -o table
```

Output:
```
Error: Validation("The artist you supplied could not be found")
```

### Invalid Parameter
```bash
$ ./target/release/lastfm-cli user top-artists "testuser" --period 2weeks -o table
```

Output:
```
error: invalid value '2weeks' for '--period <period>'
  [possible values: overall, 7day, 1month, 3month, 6month, 12month]

For more information, try '--help'.
```

### Missing Required Argument
```bash
$ ./target/release/lastfm-cli artist info
```

Output:
```
error: the following required arguments were not provided:
  <artist>

Usage: lastfm-cli artist info <artist>

For more information, try '--help'.
```

---

## Advanced Usage

### Piping and Processing
```bash
# Get top 10 artists and extract just their names
$ ./target/release/lastfm-cli chart top-artists --limit 10 -o json | jq -r '.artists.artist[].name'

# Export user's library to a file
$ ./target/release/lastfm-cli library artists "username" -o json > my_library.json

# Monitor currently playing track
$ watch -n 10 './target/release/lastfm-cli user recent-tracks "username" --limit 1 -o pretty'
```

### Scripting Examples
```bash
# Check if an artist exists
if ./target/release/lastfm-cli artist info "$ARTIST" -o json >/dev/null 2>&1; then
    echo "Artist exists"
else
    echo "Artist not found"
fi

# Get listener count for an artist
LISTENERS=$(./target/release/lastfm-cli artist info "Radiohead" -o json | jq -r '.artist.stats.listeners')
echo "Radiohead has $LISTENERS listeners"
```

---

## Summary

The Last.fm CLI provides comprehensive access to the Last.fm API with 32 commands across 10 categories:

- **Authentication Commands** (3): status, login, logout
- **Personal Commands (My)** (5): info, recent-tracks, top-artists, top-tracks, top-albums
- **Artist Commands** (5): info, similar, search, top-albums, top-tracks
- **Album Commands** (2): info, search
- **Track Commands** (3): info, similar, search
- **Chart Commands** (3): top-artists, top-tracks, top-tags
- **Geo Commands** (2): top-artists, top-tracks
- **Tag Commands** (5): info, similar, top-artists, top-albums, top-tracks
- **User Commands** (3): info, recent-tracks, top-artists
- **Library Commands** (1): artists

All commands support multiple output formats (table, json, compact, pretty) and optional parameters, and run against the Cloudflare Worker you self-host (configured via `worker_url`). Once authenticated, the personal "my" commands provide quick access to your own Last.fm data without needing to specify your username.
