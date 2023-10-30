use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use rocket::{
    http::{Cookie, CookieJar, SameSite, Status},
    response::{Flash, Redirect},
    serde::json::Json,
};

use serde::{Deserialize, Serialize};

use crate::{UserData, utils::config::CDNConfig};

#[derive(Deserialize)]
pub struct OAuthData {
    token_type: String,
    access_token: String,
}
#[derive(Deserialize, Serialize)]
pub struct Error {
    message: String,
}
#[get("/callback?<code>")]
pub async fn callback(cookies: &CookieJar<'_>, code: &str) -> Flash<Redirect> {
    let config = CDNConfig::new().unwrap();
    if !code.is_empty() {
        let res = reqwest::Client::new()
            .post("https://discord.com/api/oauth2/token")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(format!(
                "client_id={}&client_secret={}&code={}&grant_type=authorization_code&redirect_uri={}&scope=identify",
                config.client_id(),
                config.client_secret(),
                code,
                config.redirect_uri()
            ))
            .send().await;

        if let Ok(res) = res {
            let data: OAuthData = res.json().await.unwrap();
            let res = reqwest::Client::new()
                .get("https://discord.com/api/users/@me")
                .header(
                    AUTHORIZATION,
                    format!("{} {}", data.token_type, data.access_token),
                )
                .send()
                .await;
            if let Ok(res) = res {
                let user: UserData = res.json().await.unwrap();
                cookies.add_private(
                    Cookie::build("username", user.clone().username)
                        .same_site(SameSite::Lax)
                        .finish(),
                );
                cookies.add_private(
                    Cookie::build("userid", user.clone().id)
                        .same_site(SameSite::Lax)
                        .finish(),
                );
                return Flash::success(
                    Redirect::to("/"),
                    format!("Welcome, {}!", user.clone().username),
                );
            } else {
                return Flash::error(Redirect::to("/error"), "Error");
            }
        } else {
            return Flash::error(Redirect::to("/error"), "Error");
        }
    }
    return Flash::error(Redirect::to("/error"), "Error");
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("username"));
    cookies.remove_private(Cookie::named("userid"));
    Flash::success(Redirect::to("/"), "Logged out!")
}

#[get("/login")]
pub fn login() -> Result<Redirect, Status> {
    let config = CDNConfig::new().unwrap();
    Ok(Redirect::to(format!(
        "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify",
                config.client_id(),
                config.redirect_uri()
    )))
}

#[get("/error")]
pub fn error() -> Json<Error> {
    Json(Error {
        message: "Error".to_string(),
    })
}
