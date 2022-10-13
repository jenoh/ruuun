use actix_session::{storage::RedisActorSessionStore, Session, SessionMiddleware};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use log::info;
use std::env;

mod model;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Get env vars
    let strava_client_id: String = env::var("STRAVA_CLIENT_ID").unwrap();
    let strava_client_secret: String = env::var("STRAVA_CLIENT_SECRET").unwrap();

    info!(
        "Get env vars: {}, {}",
        stringify!(strava_client_id),
        stringify!(strava_client_secret)
    );

    // let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;

    // match con.get("strava_access_token") {
    //     Ok::<String, _>(p) => {
    //         info!(
    //             "There is already a strava_access_token in redis {}",
    //             p.to_string()
    //         )
    //     }
    //     Err(_) => {
    //         info!("No strava_access_token in redis");

    //         let athlete: model::ResponseOauthToken::ResponseOauthToken =
    //             services::auth::oauth_token(strava_client_id, strava_client_secret, strava_code)
    //                 .await?;

    //         con.set("strava_access_token", athlete.access_token)?;
    //         con.set("strava_refresh_token", athlete.refresh_token)?;
    //     }
    // }
    // Generate a random 32 byte key. Note that it is important to use a unique
    // private key for every project. Anyone with access to the key can generate
    // authentication cookies for any user!
    let private_key = actix_web::cookie::Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
                    private_key.clone(),
                )
                .build(),
            )
            .route("/", web::get().to(manual_hello))
            .service(get_code)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/code")]
pub async fn get_code(_req: HttpRequest, session: Session) -> Result<HttpResponse, Error> {
    let params = web::Query::<model::Params::Params>::from_query(_req.query_string()).unwrap();
    info!("Receiving the code:  {}", params.code);

    session.insert("code", params.code.clone())?;
    let code: Option<String> = session.get("code").unwrap_or(None);
    info!("Store the code in redis");

    let athlete: model::ResponseOauthToken::ResponseOauthToken = services::auth::oauth_token(
        services::config::get_strava_client_id(),
        services::config::get_strava_client_secret(),
        code.unwrap(),
    )
    .await?;

    session.insert("access_token", athlete.access_token)?;
    session.insert("refresh_token", athlete.refresh_token)?;
    info!("Store access and refresh token in redis");

    Ok(HttpResponse::Ok().body("OK"))
}
