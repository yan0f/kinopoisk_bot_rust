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
    #[serde(rename = "type")]
    film_type: String,
    year: String,
    pub(crate) description: Option<String>,
    #[serde(rename = "filmLength")]
    duration: Option<String>,
    countries: Vec<Country>,
    genres: Vec<Genre>,
    kp_rate: Option<String>,
    #[serde(rename = "ratingVoteCount")]
    rating_vote_count: u64,
    #[serde(rename = "posterUrl")]
    poster_url: String,
    #[serde(rename = "posterUrlPreview")]
    pub(crate) poster_preview_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    genre: String,
}

impl Movie {
    pub(crate) fn get_kp_url(&self) -> String {
        format!("https://www.kinopoisk.ru/film/{}/", self.kp_id)
    }

    pub(crate) fn get_name(&self) -> String {
        self.en_name
            .clone()
            .unwrap_or(self.ru_name.clone().unwrap_or_default())
    }
    pub(crate) fn get_title(&self) -> String {
        todo!();
    }
    // def get_result_article_title(movie: Movie) -> str:
    //     title = ''
    //     if movie.ru_name:
    //         title += f'«{movie.ru_name}» ({movie.name}, {movie.year})'
    //     else:
    //         title += f'{movie.name}'
    //         if movie.year:
    //             title += f', {movie.year}'
    //     if movie.kp_rate:
    //         title += f' • {movie.kp_rate}'
    //     return title
    pub(crate) fn get_year(&self) -> Option<String> {
        if self.year != "null" {
            self.year.split("-").nth(0);
        }
        None
    }
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
    // if !response.status().is_success() {
    //     panic!("")
    // }
    res.films
    //    if request.status_code != 200:
    //         raise Exception(request_json['error'])
}
