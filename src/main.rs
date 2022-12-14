// use actix_session::{storage::RedisActorSessionStore, Session, SessionMiddleware};
use actix_web::{
    dev::ServiceRequest, get, post, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Responder, Result,
};
use log::info;
use r2d2_redis::redis::Commands;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
// use redis::Commands;
use std::env;
mod model;
mod services;

pub type R2D2Pool = r2d2::Pool<RedisConnectionManager>;
pub type R2D2Con = r2d2::PooledConnection<RedisConnectionManager>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Get env vars
    let strava_client_id: String = env::var("STRAVA_CLIENT_ID").unwrap();
    let strava_client_secret: String = env::var("STRAVA_CLIENT_SECRET").unwrap();
    let strava_verify_token: String = env::var("VERIFY_TOKEN").unwrap();
    let redis_url: String = env::var("REDIS_URL").unwrap();
    let port: u16 = env::var("RUUUN_PORT").unwrap().parse().unwrap();

    let manager = RedisConnectionManager::new(redis_url).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    info!(
        "Get env vars: {}, {}, {}",
        stringify!(strava_client_id),
        stringify!(strava_client_secret),
        stringify!(strava_verify_token)
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/hello", web::get().to(manual_hello))
            .service(get_code)
            .service(get_webhook)
            .service(post_webhook)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/")]
pub async fn get_code(
    _req: HttpRequest,
    pool: web::Data<r2d2::Pool<RedisConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let params = web::Query::<model::Params::Params>::from_query(_req.query_string()).unwrap();
    info!("Receiving the code:  {}", params.code);

    let mut con_result = pool.get().unwrap();
    let _: String = con_result.set("code", params.code.clone()).unwrap();
    info!("Store the code in redis");

    let athlete: model::ResponseOauthToken::ResponseOauthToken = services::auth::oauth_token(
        services::config::get_strava_client_id(),
        services::config::get_strava_client_secret(),
        params.code.clone(),
    )
    .await?;

    let _: String = con_result
        .set("access_token", athlete.access_token)
        .unwrap();
    let _: String = con_result
        .set("refresh_token", athlete.refresh_token)
        .unwrap();
    info!("Store access and refresh token in redis");

    services::auth::subscriptions(
        services::config::get_strava_client_id(),
        services::config::get_strava_client_secret(),
        services::config::get_url_webhook(),
        services::config::get_verify_token(),
    )
    .await?;
    Ok(HttpResponse::Ok().body("OK"))
}
#[post("/webhook")]
pub async fn post_webhook(
    _req: HttpRequest,
    json: web::Json<model::FormWebhook::FormWebhook>,
    pool: web::Data<r2d2::Pool<RedisConnectionManager>>,
) -> Result<HttpResponse, Error> {
    info!(
        "Event received, type: {}, id: {}",
        json.object_type, json.object_id
    );

    let mut con_result = pool.get().unwrap();
    let token: String = con_result.get("access_token").unwrap();

    if json.object_type == "activity" {
        let activity: model::ResponseActivity::ResponseActivity = services::api::get_activity(
            json.object_id.to_string().clone(),
            format!("Bearer {}", token.clone()),
        )
        .await?;
        services::api::put_activity(
            json.object_id.to_string().clone(),
            format!("Bearer {}", token.clone()),
            activity.distance.clone(),
            // activity.moving_time.clone(),
        )
        .await?;
    }

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
