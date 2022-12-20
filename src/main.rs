use dotenv::dotenv;
use log::{error, info};
use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
    Bot,
};
use uuid::Uuid;
mod handlers;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();
    info!("Starting Kinopoisk bot...");

    let bot = Bot::from_env();
    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |bot: Bot, q: InlineQuery| async move {
            let movies = handlers::search_for_movie(q.query);
            let results: Vec<InlineQueryResult> = movies
                .await
                .into_iter()
                .map(|movie| {
                    InlineQueryResult::Article(
                        InlineQueryResultArticle::new(
                            Uuid::new_v4().to_string(),
                            movie.get_title(),
                            InputMessageContent::Text(InputMessageContentText::new(
                                movie.get_kp_url(),
                            )),
                        )
                        .description(movie.description.unwrap_or_default())
                        .thumb_url(movie.poster_preview_url.parse().unwrap()),
                    )
                })
                .collect();
            let response = bot.answer_inline_query(&q.id, results).send().await;
            if let Err(err) = response {
                error!("Error in handler: {:?}", err);
            }
            respond(())
        },
    ));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}
