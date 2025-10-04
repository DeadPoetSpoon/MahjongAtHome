use std::fmt::Display;

use rocket::serde::{Serialize, json::Json};

pub type ApiResult<T> = Result<Json<ApiResultMsg<T>>, ()>;

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum ApiResultCode {
    Success = 1,
    Failure = 2,
}

#[derive(Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ApiResultMsg<T> {
    code: ApiResultCode,
    message: Option<String>,
    data: Option<T>,
}

impl<T> From<&mut ApiResultMsg<T>> for ApiResult<T>
where
    T: Clone,
{
    fn from(msg: &mut ApiResultMsg<T>) -> Self {
        Ok(Json(msg.clone()))
    }
}

impl<T> From<&ApiResultMsg<T>> for ApiResult<T>
where
    T: Clone,
{
    fn from(msg: &ApiResultMsg<T>) -> Self {
        Ok(Json(msg.clone()))
    }
}

impl<T> From<ApiResultMsg<T>> for ApiResult<T> {
    fn from(msg: ApiResultMsg<T>) -> Self {
        Ok(Json(msg))
    }
}

impl<T> Default for ApiResultMsg<T> {
    fn default() -> Self {
        ApiResultMsg::new(ApiResultCode::Success, None, None)
    }
}

impl<T> ApiResultMsg<T> {
    pub fn new(code: ApiResultCode, message: Option<String>, data: Option<T>) -> Self {
        ApiResultMsg {
            code,
            message,
            data,
        }
    }

    pub fn new_success_with_data(data: T) -> Self {
        ApiResultMsg::new(ApiResultCode::Success, None, Some(data))
    }

    pub fn new_success_with_optipn_data(data: Option<T>) -> Self {
        ApiResultMsg::new(ApiResultCode::Success, None, data)
    }

    pub fn new_failure_with_msg<D>(message: D) -> Self
    where
        D: Into<String>,
    {
        ApiResultMsg::new(ApiResultCode::Failure, Some(message.into()), None)
    }

    pub fn new_success() -> Self {
        ApiResultMsg::new(ApiResultCode::Success, None, None)
    }
    pub fn new_failure() -> Self {
        ApiResultMsg::new(ApiResultCode::Failure, None, None)
    }

    pub fn success(&mut self) -> &mut Self {
        self.code = ApiResultCode::Success;
        self
    }

    pub fn failure(&mut self) -> &mut Self {
        self.code = ApiResultCode::Failure;
        self
    }

    pub fn with_msg(&mut self, message: String) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn with_data(&mut self, date: T) -> &mut Self {
        self.data = Some(date);
        self
    }

    pub fn with_option_date(&mut self, date: Option<T>) -> &mut Self {
        self.data = date;
        self
    }
}

impl<T, E> From<Result<T, E>> for ApiResultMsg<T>
where
    E: Display,
{
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(data) => ApiResultMsg::new(ApiResultCode::Success, None, Some(data)),
            Err(err) => ApiResultMsg::new(ApiResultCode::Failure, Some(format!("{}", err)), None),
        }
    }
}
