extern crate yukikaze;

use self::yukikaze::client;
use self::yukikaze::client::config::Config;
pub use self::yukikaze::client::request::multipart;
pub use self::yukikaze::client::request::Builder;
pub use self::yukikaze::client::Request;
pub use self::yukikaze::futures;
pub use self::yukikaze::futures::{Future, IntoFuture};
pub use self::yukikaze::header;
pub use self::yukikaze::http::Method;
pub use self::yukikaze::mime::Mime;
pub use self::yukikaze::rt::{AutoClient, AutoRuntime, Guard};

use config::Settings;

use std::time::Duration;

static mut TIMEOUT: u64 = 5;

struct Conf;

impl Config for Conf {
    fn timeout() -> Duration {
        get_timeout()
    }
}

pub fn init(settings: &Settings) -> Guard {
    unsafe {
        TIMEOUT = settings.timeout;
    }

    let client = client::Client::<Conf>::new();
    yukikaze::rt::set(client);
    yukikaze::rt::init()
}

pub fn get_timeout() -> Duration {
    unsafe { Duration::from_secs(TIMEOUT) }
}
