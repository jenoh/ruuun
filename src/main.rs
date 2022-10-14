use actix_session::{storage::RedisActorSessionStore, Session, SessionMiddleware};
use actix_web::{
    dev::ServiceRequest, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder, Result,
};
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
            .route("/hello", web::get().to(manual_hello))
            .service(get_code)
            .service(get_webhook)
            .service(post_webhook)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/")]
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

    services::auth::subscriptions(
        services::config::get_strava_client_id(),
        services::config::get_strava_client_secret(),
        services::config::get_url_webhook(),
        "toto".to_string(),
    )
    .await?;
    Ok(HttpResponse::Ok().body("OK"))
}
#[post("/webhook")]
pub async fn post_webhook(
    _req: HttpRequest,
    json: web::Json<model::FormWebhook::FormWebhook>,
) -> Result<HttpResponse, Error> {
    info!("Event received, type: {}", json.object_type);

    Ok(HttpResponse::Ok().body("OK"))
}
#[get("/webhook")]
pub async fn get_webhook(_req: HttpRequest) -> Result<impl Responder> {
    let params =
        serde_urlencoded::from_str::<model::ParamsHub::ParamsHub>(_req.query_string()).unwrap();
    let response_challenge = model::ResponseChallenge::ResponseChallenge {
        challenge: params.challenge.to_string(),
    };
    Ok(web::Json(response_challenge))
}
