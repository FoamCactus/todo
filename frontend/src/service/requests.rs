use serde::{Deserialize,Serialize};
use yew::format::Json;
use crate::error::ServiceError as Error;
use yew::callback::Callback;
use yew::format::{Nothing, Text};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use dotenv_codegen::dotenv;

const API_ROOT: &str = dotenv!("API_ROOT");


#[derive(Default, Debug)]
pub struct Requests {}

impl Requests {
    pub fn new() -> Self {
        Self {}
    }

    fn builder<B, T>(
        &mut self,
        method: &str,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static,
        B: Into<Text> + std::fmt::Debug,
    {
        let handler = move |response: Response<Text>| {
            if let (meta, Ok(data)) = response.into_parts() {
                if meta.status.is_success() {
                    let data: Result<T, _> = serde_json::from_str(&data);
                    if let Ok(data) = data {
                        callback.emit(Ok(data))
                    } else {
                        callback.emit(Err(Error::E))
                    }
                } else {
                    callback.emit(Err(Error::E))
                }
            } else {
                callback.emit(Err(Error::E))
            }
        };
        let url = format!("{}{}",API_ROOT,url);
        let builder = Request::builder()
            .method(method)
            .uri(url)
            .header("Content-Type", "application/json");
        let request = builder.body(body).unwrap();
        FetchService::fetch(request, handler.into()).unwrap()
    }

    pub fn get<T>(&mut self, url: String, callback: Callback<Result<T, Error>>) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
    {
        self.builder("GET", url, Nothing, callback)
    }

    pub fn post<B, T>(
        &mut self,
        url: String,
        body: B,
        callback: Callback<Result<T, Error>>,
    ) -> FetchTask
    where
        for<'de> T: Deserialize<'de> + 'static + std::fmt::Debug,
        B: Serialize,
    {
        let body: Text = Json(&body).into();
        self.builder("POST", url, body, callback)
    }
}
