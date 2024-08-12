use crate::{config_file::UserConfig, tides::TideStamp};
use crate::tides::parse_tides;

use serde::{Deserialize, Serialize};
use serde_json;
use reqwest::header;

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionToken {
    access_token: String,
    refresh_token: String,
    expires_in: i32,
    token_type: String
}

fn get_login_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();

    headers.insert(header::CONTENT_TYPE, "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert(header::ACCEPT, "application/json".parse().unwrap());
    headers.insert(header::ACCEPT_LANGUAGE, "en-US,en;q=0.9".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "credentials: same-origin".parse().unwrap());
    headers.insert(header::ORIGIN, "https://www.surfline.com".parse().unwrap());
    headers.insert(header::REFERER, "https://www.surfline.com/".parse().unwrap());

    return headers;
}

fn format_login_request_body(user: &UserConfig) -> String {
    let body = format!(
        "grant_type=password&username={}&password={}",
        &user.email,
        &user.password
    );

    return body;
}

fn make_login_request(url: &str, user: &UserConfig) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let mut headers = get_login_headers();
    headers.insert(header::AUTHORIZATION, user.auth_header.parse().unwrap());


    return client
        .post(url)
        .body(format_login_request_body(&user))
        .headers(headers)
        .send()
}

fn parse_session_token(response_body: &String) -> Result<SessionToken, serde_json::Error> {
    return serde_json::from_str(&response_body);
}

pub fn get_session_token(user: &UserConfig) -> Result<SessionToken, String> {
    let url = "https://services.surfline.com/trusted/token?isShortLived=false";
    let response = match make_login_request(url, user) {
        Ok(res) => res,
        Err(error) => {
            println!("ERR: {:?}", error);
            return Err(error.to_string())
        }
    };

    match response.status() {
        reqwest::StatusCode::OK => {
            let body = response.text();
            if body.is_err() {
                return Err(format!("Unexpected token response body: {}", body.unwrap_err()))
            }
            parse_session_token(&body.unwrap()).map_err(|err| err.to_string())
        },
        _ => Err(format!("Response code: {}", response.status()))
    }
}

fn make_request(url: &String) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    // let headers 
    return client
        .get(url)
        .send();
}


// pub fn get_tide_data(session_token: &SessionToken, spot_id: &String) -> Result<TideStamp, String> {
pub fn get_tide_data(session_token: &SessionToken, spot_id: &String) {
    // For now, just console log it.
    // We can add formatting later.
    let mut url: String = String::from("https://services.surfline.com/kbyg/spots/forecasts/tides?cacheEnabled=true&units%5BtideHeight%5D=FT");

    url.push_str(format!("&spotId={}", spot_id).as_str());
    url.push_str(format!("&accessToken={}", session_token.access_token).as_str());

    let response = match make_request(&url) {
        Ok(res) => res,
        // Err(error) => return Err(error.to_string())
        Err(error) => panic!("Oh NNO")
    };
    // match response
    let tide_data = match response.status() {
        reqwest::StatusCode::OK => {
            let body = response.text();
            if body.is_err() {
                // return Err(format!("Error getting tides"))
                println!("Error getting tides");
            }
            parse_tides(&body.unwrap()) //.map_err(|err| err.to_string())
        },
        _ => {
            println!("Response code: {}", response.status());
            vec!()
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_session_token_error_invalid_body() {
        let body = String::from("invalid");
        assert!(parse_session_token(&body).is_err());
    }

    #[test]
    fn test_parse_session_token_success() {
        let body = String::from("
            {\"access_token\":\"token\",\"refresh_token\":\"token\",\"expires_in\":2592000,\"token_type\":\"Bearer\"}
        ");
        let token = parse_session_token(&body).unwrap();
        assert_eq!(token.access_token, "token");
    }
}