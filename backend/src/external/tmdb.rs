use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::models::{item::MediaType, search::SearchCandidate};

#[derive(Debug, Deserialize, Serialize)]
pub struct TmdbShow {
    pub id: i64,
    pub name: String,
    pub overview: Option<String>,
    pub first_air_date: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<i64>,
    pub origin_country: Vec<String>,
    pub original_language: Option<String>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<i64>,
    pub popularity: Option<f64>,
}

impl From<TmdbShow> for SearchCandidate {
    fn from(api: TmdbShow) -> Self {
        let metadata = serde_json::to_string(&api).expect("failed to serialize metadata");

        SearchCandidate {
            external_id: api.id,
            title: api.name,
            media_type: MediaType::Show,
            year: api.first_air_date,
            description: api.overview,
            poster_url: api.poster_path,
            metadata,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TmdbShowSearchResponse {
    pub results: Vec<TmdbShow>,
    pub total_results: i64,
    pub total_pages: i64,
}

pub async fn search_shows(
    client: &Client,
    access_token: &str,
    query: &str,
) -> Result<Vec<SearchCandidate>, reqwest::Error> {
    let response = client
        .get("https://api.themoviedb.org/3/search/tv")
        .bearer_auth(access_token)
        .query(&[("query", query)])
        .send()
        .await?
        .json::<TmdbShowSearchResponse>()
        .await?;

    Ok(response.results.into_iter().map(Into::into).collect())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TmdbMovie {
    pub id: i64,
    pub title: String,
    pub overview: Option<String>,
    pub release_date: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<i64>,
    pub original_language: Option<String>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<i64>,
    pub popularity: Option<f64>,
}

impl From<TmdbMovie> for SearchCandidate {
    fn from(api: TmdbMovie) -> Self {
        let metadata = serde_json::to_string(&api).expect("failed to serialize metadata");

        SearchCandidate {
            external_id: api.id,
            title: api.title,
            media_type: MediaType::Movie,
            year: api.release_date,
            description: api.overview,
            poster_url: api.poster_path,
            metadata,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TmdbMovieSearchResponse {
    pub results: Vec<TmdbMovie>,
    pub total_results: i64,
    pub total_pages: i64,
}

pub async fn search_movies(
    client: &Client,
    access_token: &str,
    query: &str,
) -> Result<Vec<SearchCandidate>, reqwest::Error> {
    let response = client
        .get("https://api.themoviedb.org/3/search/movie")
        .bearer_auth(access_token)
        .query(&[("query", query)])
        .send()
        .await?
        .json::<TmdbMovieSearchResponse>()
        .await?;

    Ok(response.results.into_iter().map(Into::into).collect())
}
