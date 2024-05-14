use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use serenity::all::MessagePagination;
use sqlx::{postgres::PgPoolOptions, types::Json};
use std::env;

/// Fetch messages from a discord channel
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
	/// (required) ID of the channel
	#[arg(short, long)]
	channel: u64,

	/// Estimated amount of messages in the channel
	#[arg(short, long)]
	estimated: Option<u32>,

	/// Start from a specific message ID
	#[arg(short, long)]
	before: Option<u64>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
	dotenvy::dotenv().expect("Could not load environment variables");
	let args = Args::parse();
	let token = env::var("DISCORD_BOT_TOKEN").expect("Bot token not found");
	let db_url = env::var("DB_URL").expect("Database url not found");
	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect(db_url.as_str())
		.await
		.expect("Could not connect to database");
	let http = serenity::http::Http::new(&token);
	let bar = ProgressBar::new(args.estimated.unwrap_or(0).into()).with_style(
		ProgressStyle::with_template(
			"[{elapsed_precise}/{duration_precise}] {wide_bar} {pos:>7}/{len}",
		)
		.unwrap(),
	);
	let mut target: Option<MessagePagination> =
		args.before.map(|id| MessagePagination::Before(id.into()));
	loop {
		let messages = http
			.get_messages(args.channel.into(), target, Some(100))
			.await
			.expect("Failed to fetch messages");
		for msg in messages.iter() {
			sqlx::query(r#"INSERT INTO "messages" (channel_id, id, author_id, content, data) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET content = $4, data = $5;"#)
				.bind::<i64>(msg.channel_id.into())
				.bind::<i64>(msg.id.into())
				.bind::<i64>(msg.author.id.into())
				.bind::<&String>(&msg.content)
				.bind::<Json<&serenity::all::Message>>(sqlx::types::Json(msg))
				.execute(&pool)
				.await
				.expect(format!("Failed to save message {} to db", msg.id).as_str());
			bar.inc(1);
		}
		target = messages.last().map(|m| MessagePagination::Before(m.id));
		if target.is_none() {
			break;
		};
	}
	bar.finish();
}
