//!Twitter accessing module
use ::egg_mode::{
    Token,
    KeyPair,
    FutureResponse,
    Response,
    tweet,
};

use ::tokio_core::reactor::{
    Handle
};

use super::common;
use ::config;

///Twitter client.
pub struct Client {
    ///Twitter access token.
    token: Token,
    ///Tokio Core's handle
    handle: Handle
}

impl Client {
    ///Creates new instances and initializes token.
    pub fn new(handle: Handle, config: config::Twitter) -> Self {
        let token = Token::Access {
            consumer: KeyPair::new(config.consumer.key, config.consumer.secret),
            access: KeyPair::new(config.access.key, config.access.secret)
        };

        Client {
            token,
            handle
        }
    }

    ///Posts new tweet.
    pub fn post(&self, message: &str, tags: &Option<Vec<String>>) -> FutureResponse<tweet::Tweet> {
        let message = common::message(message, tags);

        tweet::DraftTweet::new(&message).send(&self.token, &self.handle)
    }

    pub fn handle_post(response: Response<tweet::Tweet>) -> Result<(), String> {
        Ok(println!("Posted tweet(id={}):\n{}\n", response.response.id, response.response.text))
    }
}
