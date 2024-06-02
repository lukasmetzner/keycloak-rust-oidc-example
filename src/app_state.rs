#[derive(Clone, Debug)]
pub struct EnvVariables {
    pub client_id: String,
    pub client_secret: String,
    pub issuer_url: String,
    pub redirect_uri: String,
}

impl EnvVariables {
    fn from_env() -> Self {
        dotenv::dotenv().ok();

        let client_id = dotenv::var("CLIENT_ID").expect("CLIENT_ID not set");
        let client_secret = dotenv::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
        let issuer_url = dotenv::var("ISSUER_URL").expect("ISSUER_URL not set");
        let redirect_uri = dotenv::var("REDIRECT_URI").expect("REDIRECT_URI not set");

        Self {
            client_id,
            client_secret,
            issuer_url,
            redirect_uri,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub env: EnvVariables,
}

impl AppState {
    pub fn from_env() -> Self {
        let env = EnvVariables::from_env();
        Self { env }
    }
}
