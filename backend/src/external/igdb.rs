use chrono::DateTime;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    models::{item::MediaType, search::SearchCandidate},
};

#[derive(Debug, Deserialize)]
pub struct IgdbToken {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

pub async fn fetch_igdb_token(
    client: &Client,
    igdb_client_id: &str,
    igdb_client_secret: &str,
) -> Result<String, AppError> {
    let response = client
        .post("https://id.twitch.tv/oauth2/token")
        .query(&[
            ("client_id", igdb_client_id),
            ("client_secret", igdb_client_secret),
            ("grant_type", "client_credentials"),
        ])
        .send()
        .await?
        .json::<IgdbToken>()
        .await?;

    Ok(response.access_token)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IgdbGame {
    pub id: i64,
    pub name: String,
    pub first_release_date: i64,
    pub cover: IgdbGameCover,
    pub game_modes: Vec<IgdbGameMode>,
    pub genres: Vec<IgdbGameGenre>,
    pub platforms: Vec<IgdbGamePlatform>,
    pub summary: String,
}

impl From<IgdbGame> for SearchCandidate {
    fn from(api: IgdbGame) -> Self {
        let metadata = serde_json::to_value(&api).expect("failed to serialize metadata");

        SearchCandidate {
            external_id: api.id.to_string(),
            title: api.name,
            media_type: MediaType::Game,
            year: DateTime::from_timestamp_secs(api.first_release_date)
                .map(|dt| dt.format("%Y-%m-%d").to_string()),
            description: Some(api.summary),
            poster_url: Some(api.cover.url),
            metadata,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IgdbGameCover {
    pub id: i64,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IgdbGameMode {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IgdbGameGenre {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IgdbGamePlatform {
    pub id: i64,
    pub name: String,
}

pub async fn search_games(
    client: &Client,
    access_token: &str,
    client_id: &str,
    query: &str,
) -> Result<Vec<SearchCandidate>, AppError> {
    let response = client
        .post("https://api.igdb.com/v4/games")
        .header("Authorization", format!("Bearer {access_token}"))
        .header("Client-ID", client_id)
        .body(format!("search \"{query}\"; fields id,name,summary,first_release_date,cover.url,genres.name,game_modes.name,platforms.name; limit 10;"))
        .send()
        .await?;

    if !response.status().is_success() {
        let body = response.text().await?;
        return Err(AppError::ExternalApi(format!("IGDB error: {body}")));
    }

    let data = response.json::<Vec<IgdbGame>>().await?;

    Ok(data.into_iter().map(Into::into).collect())
}

// these are in seconds
#[derive(Debug, Deserialize, Serialize)]
pub struct IgdbGameTimeToComplete {
    pub id: i64,
    pub hastily: i64,
    pub normally: i64,
    pub completely: i64,
}

pub async fn fetch_game_completion_time(
    client: &Client,
    access_token: &str,
    client_id: &str,
    id: &str,
) -> Result<Option<IgdbGameTimeToComplete>, AppError> {
    let response = client
        .post("https://api.igdb.com/v4/game_time_to_beats")
        .header("Authorization", format!("Bearer {access_token}"))
        .header("Client-ID", client_id)
        .body(format!(
            "where game_id = {id}; fields id,hastily,normally,completely;"
        ))
        .send()
        .await?;

    if !response.status().is_success() {
        let body = response.text().await?;
        return Err(AppError::ExternalApi(format!("IGDB error: {body}")));
    }

    let data = response.json::<Vec<IgdbGameTimeToComplete>>().await?;

    Ok(data.into_iter().next())
}
