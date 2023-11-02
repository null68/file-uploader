#[macro_use]
extern crate rocket;
use rocket::{fs::relative, fs::FileServer, Config, Request, response::content::RawHtml};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Result};
use utils::{
    config::{setup_config, CDNConfig},
    store::setup_filedir, id::generate_secret_key,
};

mod routers;
mod utils;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserData {
    id: String,
    username: String,
}

#[get("/favicon.ico")]
fn favicon() -> Result<File> {
    let favicon_path = "src/assets/favicon.ico";
    File::open(favicon_path)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[catch(401)]
fn unauthorized() -> RawHtml<String> {
   RawHtml(String::from("<h1>401 Unauthorized</h1><br><a href=\"/login\">Login</a>"))
}

#[catch(403)]
fn forbidden() -> String {
    format!("Sorry, you don't have access to this resource.")
}

#[launch]
fn rocket() -> _ {
    setup_filedir();
    setup_config().expect("Failed to setup config");
    let router = routes![
        favicon,
        routers::files::index,
        routers::files::upload,
        routers::discord::callback,
        routers::discord::logout,
        routers::discord::login,
        routers::discord::error,
    ];
    let cfg = Config::figment().merge((
        "port",
        CDNConfig::new().unwrap().port(),
    )).merge((
        "secret_key",
        generate_secret_key()
    )).merge((
        "address",
        "0.0.0.0",
    ));
    rocket::custom(cfg)
        .register("/", catchers![not_found, unauthorized, forbidden])
        .mount("/", router)
        .mount("/", FileServer::from(relative!("files")))
}
