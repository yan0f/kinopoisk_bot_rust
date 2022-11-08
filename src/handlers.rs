use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

const API_VERSION: &str = "v2.1";
const KINOPOISK_UNOFFICIAL_API: &str = "https://kinopoiskapiunofficial.tech/api";

#[derive(Debug, Serialize, Deserialize)]
pub struct Res {
    films: Vec<Movie>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    #[serde(rename = "filmId")]
    kp_id: u64,
    #[serde(rename = "nameRu")]
    ru_name: Option<String>,
    #[serde(rename = "nameEn")]
    en_name: Option<String>,
    year: String,
    pub description: Option<String>,
    #[serde(rename = "filmLength")]
    duration: Option<String>,
    countries: Vec<Country>,
    kp_rate: Option<String>,
    #[serde(rename = "posterUrlPreview")]
    pub poster_preview_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    country: String,
}
impl Movie {
    pub fn get_kp_url(&self) -> String {
        format!("https://www.kinopoisk.ru/film/{}/", self.kp_id)
    }

    pub fn get_name(&self) -> String {
        self.en_name.clone().unwrap_or(self.ru_name.clone().unwrap_or_default())
    }

    pub fn get_title(&self) -> String {
        let mut title = String::from("");
        match &self.ru_name {
            Some(ru_name) => title.push_str(format!("«{0}» ({1}, {2})", ru_name, self.get_name(), self.year).as_str()),
            None => {
                title.push_str(format!("{}, {}", self.get_name(), self.year).as_str());
                // if self.year {
                //     title.push_str(", {}", movie.year)
                // }
            }
        }
        if let Some(kp_rate) = &self.kp_rate {
            title.push_str(format!(" • {kp_rate}").as_str())
        }
        title
    }

    // fn get_year(&self) -> Option<String> {
    //     if self.year != "null" {
    //         self.year.split("-").nth(0);
    //     }
    //     None
    // }
}

pub async fn search_for_movie(keyword: String) -> Vec<Movie> {
    let client = reqwest::Client::new();

    let api_token = env::var("KINOPOISK_API_TOKEN").expect("$KINOPOISK_API_TOKEN is not set");
    let path = format!("{KINOPOISK_UNOFFICIAL_API}/{API_VERSION}/films/search-by-keyword");
    let response = client
        .get(path)
        .header("X-API-KEY", api_token)
        .query(&[("keyword", keyword)])
        .query(&[("page", "1")])
        .send()
        .await
        .unwrap();
    let res: Res = response.json().await.unwrap();
    res.films
}
