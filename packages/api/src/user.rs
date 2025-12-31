#[cfg(feature = "server")]
use dioxus::fullstack::{Cookie, TypedHeader};
use dioxus::fullstack::{Form, SetCookie, SetHeader};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserLoginForm {
    username: String,
    password: String,
}

#[post("/api/login")]
async fn login(form: Form<UserLoginForm>) -> Result<SetHeader<SetCookie>> {
    // Verify the username and password. In a real application, you'd check these against a database.
    if form.0.username == "admin" && form.0.password == "password" {
        return Ok(SetHeader::new(format!(
            "auth-demo={};",
            "&*THIS_SESSION_ID"
        ))?);
    }

    HttpError::unauthorized("Invalid username or password")?
}
