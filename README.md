# Keycloak Rust OIDC Example

## Get Started
``` bash
docker compose up -d
```

Got to http://localhost:8080/ and login with `admin:admin`. Afterwards:
1. Go to clients
2. Create client
3. Client ID -> `test-client`
4. Turn on `Client authentication`
5. Set `Root URL` -> `http://localhost:3000/`
6. Set `Valid redirect URIs` -> `http://localhost:3000/auth/callback`
7. Save
8. Go to the `Credentials` tab -> Copy `Client Secret`

Now create a `.env` file and replace `<YOUR-CLIENT-SECRET>`:
```
CLIENT_ID = "test-client"
CLIENT_SECRET = "<YOUR-CLIENT-SECRET>"
ISSUER_URL = "http://localhost:8080/realms/master"
REDIRECT_URI = "http://localhost:3000/auth/callback"
```

Now you can run the application and go to -> `http://localhost:3000/login`
``` bash
cargo run
```
