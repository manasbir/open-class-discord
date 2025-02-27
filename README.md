# Open Class Discord

A Discord bot to help University of Waterloo students find open classrooms.

## Installation
While available to the public as a discord bot, to run on Cloudflare Workers follow the steps below.

1. Clone the repository
```bash
git clone https://github.com/manasbir/open-class-discord.git
cd open-class-discord
```

2. Create the .env and populate the environment variables
```bash
echo UWATERLOO_API_KEY="<INSERT_API_KEY>" >> .env
echo DISCORD_APPLICATION_ID="<INSERT_APPLICATION_ID>" >> .env
echo DISCORD_PUBLIC_KEY="<INSERT_PUBLIC_KEY>" >> .env
echo DISCORD_TOKEN="<INSERT_BOT_TOKEN>" >> .env
echo ADMIN_DISCORD_ID="<INSERT_YOUR_DISCORD_ID>" >> .env

echo > /workers/.dev.vars < .env
```

3. Register the commands with Discord
```bash
cd register-commands
cargo run
```


4. Deploy!
```bash
cd ../workers
wrangler publish
```

## Design Choices

### Architecture
- Command-based Discord bot using Serenity
- Asynchronous classroom data fetching
- In-memory caching of classroom schedules
- Periodic updates from UW Open Data API

### Technology Stack
- Rust
- SQL (Cloudflare D1)

<!-- ## Technical Challenges -->

<!-- ### Solved Challenges
1. Rate Limiting
    - Problem: UW API and Discord API rate limits
    - Solution: Implemented caching and request queuing

2. Data Processing
    - Problem: Complex classroom schedule data format
    - Solution: Custom serialization/deserialization with serde

### Current Challenges
1. Real-time schedule updates
2. Handling concurrent user requests efficiently -->