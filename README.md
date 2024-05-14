# discord-channel-backup
A simple tool for downloading all of the messages in a Discord channel and saving them to a postgres database. Requires a discord bot present in the channel which has permission to read messages and message content intent.

## Database table setup
Run the query or manually create a table with the columns:
```sql
CREATE TABLE messages
(
    channel_id BIGINT NOT NULL,
    id BIGINT NOT NULL,
    author_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    data JSONB NOT NULL,
    CONSTRAINT message_id_pkey PRIMARY KEY (id)
);
```

## Running the tool
Clone the repository:
```bash
git clone https://github.com/mokiros/discord-channel-backup.git
cd discord-channel-backup
```
Set up the environment variables:
```bash
touch .env
# Token for your discord bot
echo "DISCORD_BOT_TOKEN=your_token" >> .env
# Postgres database where the messages will be saved to
echo "DB_URL=postgres://username:password@localhost:5432/db_name" >> env
```
Either build the binaries or run it directly from cargo:
```bash
cargo build --release
cargo run --release -- --help
```

## Command line arguments
```
discord-channel-backup --help

Usage: discord-channel-backup [OPTIONS] --channel <CHANNEL>

Options:
  -c, --channel <CHANNEL>      (required) ID of the channel
  -e, --estimated <ESTIMATED>  Estimated amount of messages in the channel
  -b, --before <BEFORE>        Start from a specific message ID
  -h, --help                   Print help
```

## Notes
* `--estimated` is optional, but recommended. To get it, search for `from: #channel` in your Discord client.
* If the backup is interrupted or you need to start from a specific date/message, use the `--before` option.
