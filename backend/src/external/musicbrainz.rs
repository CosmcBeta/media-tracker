use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    external::USER_AGENT,
    models::{item::MediaType, search::SearchCandidate},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MusicBrainzReleaseGroup {
    pub id: String,
    pub title: String,
    pub first_release_date: Option<String>,
    pub primary_type: Option<String>,
    pub artist_credit: Vec<MusicBrainzArtistCredit>,
    pub releases: Vec<MusicBrainzRelease>,
    #[serde(default)]
    pub tags: Vec<MusicBrainzTag>,
}

impl From<MusicBrainzReleaseGroup> for SearchCandidate {
    fn from(api: MusicBrainzReleaseGroup) -> Self {
        let metadata = serde_json::to_value(&api).expect("failed to serialize metadata");
        let poster_url = Some(format!(
            "https://coverartarchive.org/release-group/{}/front",
            &api.id
        ));

        SearchCandidate {
            external_id: api.id,
            title: api.title,
            media_type: MediaType::Album,
            year: api.first_release_date,
            description: None,
            poster_url,
            metadata,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MusicBrainzArtistCredit {
    pub name: Option<String>,
    pub artist: MusicBrainzArtist,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MusicBrainzArtistRef {
    pub id: String,
    pub name: String,
    pub sort_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MusicBrainzRelease {
    pub id: String,
    pub title: String,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MusicBrainzTag {
    pub count: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MusicBrainzReleaseGroupSearchResponse {
    pub release_groups: Vec<MusicBrainzReleaseGroup>,
    pub count: i64,
    pub offset: i64,
    pub created: String,
}

pub async fn search_release_groups(
    client: &Client,
    query: &str,
) -> Result<Vec<SearchCandidate>, AppError> {
    let response = client
        .get("https://musicbrainz.org/ws/2/release-group/")
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .query(&[("query", query), ("fmt", "json")])
        .send()
        .await?;

    if !response.status().is_success() {
        let body = response.text().await?;
        return Err(AppError::ExternalApi(format!("MusicBrainz error: {body}")));
    }

    let data = response
        .json::<MusicBrainzReleaseGroupSearchResponse>()
        .await?;

    Ok(data.release_groups.into_iter().map(Into::into).collect())
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MusicBrainzArtist {
    pub id: String,
    pub name: String,
    pub sort_name: String,
    #[serde(rename = "type")]
    pub artist_type: Option<String>,
    pub gender: Option<String>,
    pub country: Option<String>,
    pub life_span: Option<MusicBrainzLifeSpan>,
    #[serde(default)]
    pub tags: Vec<MusicBrainzTag>,
}

impl From<MusicBrainzArtist> for SearchCandidate {
    fn from(api: MusicBrainzArtist) -> Self {
        let metadata = serde_json::to_value(&api).expect("failed to serialize metadata");

        SearchCandidate {
            external_id: api.id,
            title: api.name,
            media_type: MediaType::Artist,
            year: api.life_span.and_then(|ls| ls.begin),
            description: None,
            poster_url: None, // will add from either spotify/last.fm
            metadata,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MusicBrainzLifeSpan {
    pub begin: Option<String>,
    pub end: Option<String>,
    pub ended: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MusicBrainzArtistSearchResponse {
    pub artists: Vec<MusicBrainzArtist>,
    pub count: i64,
    pub offset: i64,
    pub created: String,
}

pub async fn search_artists(
    client: &Client,
    query: &str,
) -> Result<Vec<SearchCandidate>, AppError> {
    let response = client
        .get("https://musicbrainz.org/ws/2/artist/")
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .query(&[("query", query), ("fmt", "json")])
        .send()
        .await?;

    if !response.status().is_success() {
        let body = response.text().await?;
        return Err(AppError::ExternalApi(format!("MusicBrainz error: {body}")));
    }

    let data = response.json::<MusicBrainzArtistSearchResponse>().await?;

    Ok(data.artists.into_iter().map(Into::into).collect())
}
