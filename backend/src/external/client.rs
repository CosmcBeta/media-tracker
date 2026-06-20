use reqwest::Client;

#[derive(Debug, Clone)]
pub struct ApiClient {
    pub client: Client,
    pub tmdb_access_token: String,
    pub igdb_client_id: String,
    pub igdb_access_token: String,
}

impl ApiClient {
    pub fn new(
        client: Client,
        tmdb_access_token: String,
        igdb_client_id: String,
        igdb_access_token: String,
    ) -> ApiClient {
        ApiClient {
            client,
            tmdb_access_token,
            igdb_client_id,
            igdb_access_token,
        }
    }
}
