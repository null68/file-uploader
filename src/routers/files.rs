use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket::serde::json::Json;
use serde::Serialize;

use crate::utils::config::CDNConfig;
use crate::utils::id::generate_id;
use crate::utils::store::create_user_dir;
use crate::utils::store::get_user_files;
// TODO: Add unauthorized page
// TODO: Add file upload page
#[derive(FromForm)]
pub struct Upload<'r> {
    userid: String,
    key: String,
    file: TempFile<'r>,
}
#[derive(Serialize)]
pub struct Success {
    url: String,
}
#[derive(Serialize)]
pub struct Error {
    message: String,
}

#[get("/")]
pub fn index(cookies: &CookieJar<'_>) -> Result<RawHtml<String>, Status> {
    let config = CDNConfig::new().unwrap();
    let username = cookies.get_private("username");
    let userid = cookies.get_private("userid");
    if username.is_none() || userid.is_none() {
        return Err(Status::Unauthorized);
    }
    let username = username.unwrap();
    let username = username.value();
    let userid = userid.unwrap();
    let userid = userid.value();
    let users = config.users();
    let mut authorized = false;
    for user in users {
        if user.user_id == userid {
            authorized = true;
        }
    }
    if !authorized {
        return Err(Status::Forbidden);
    }
    let files = get_user_files(userid);
    let mut html = String::from(format!("<a href=\"/logout\">Logout</a><br><h1>{}'s files</h1>", username));
    if files.is_empty() {
        html.push_str("<p>No files found</p>");
    } else {
        html.push_str("<ul>");
        for file in files {
            html.push_str(
                format!("<li><a href=\"{}/{}\">{}</a></li>", userid, file, file).as_str(),
            );
        }
        html.push_str("</ul>");
    }
    Ok(RawHtml(html))
}

#[post("/upload", data = "<upload>", format = "multipart/form-data")]
pub async fn upload(mut upload: Form<Upload<'_>>) -> Result<Json<Success>, Json<Error>> {
    let config = CDNConfig::new().unwrap();
    if upload.userid.is_empty() || upload.key.is_empty() {
        return Err(Json(Error {
            message: String::from("Missing userid or key"),
        }));
    } 
    let users = config.users();
    let mut authorized = false;
    for user in users {
        if user.user_id == upload.userid && user.user_key == upload.key {
            authorized = true;
        }
    }
    if !authorized {
        return Err(Json(Error {
            message: String::from("Unauthorized"),
        }));
    }
    let userid = upload.userid.clone();
    create_user_dir(&userid);
    let filename = generate_id();
    let extension = {
        let mut extension = String::from(".");
        extension.push_str(
            upload
                .file
                .content_type()
                .unwrap()
                .0
                .to_string()
                .split("/")
                .last()
                .unwrap(),
        );
        extension
    };
    upload
        .file
        .persist_to(format!("files/{}/{}{}", userid, filename, extension))
        .await
        .unwrap();
    Ok(Json(Success {
        url: format!(
            "{}/{}/{}{}",
            config.uri(),
            userid,
            filename,
            extension
        ),
    }))
}
