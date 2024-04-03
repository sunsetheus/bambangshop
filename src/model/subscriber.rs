use rocket::serde::{Deseialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshop::REQUEST_CLIENT;
use create::model::notification::Notification;

#[derive(Debug, Clone, Deseialize, Serialize)]
#[derive(serde(create = "rocket::serde"))]
pub struct Subscriber {
    pub url: String,
    pub name: String
}