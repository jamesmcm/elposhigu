use actix_web::{web, App, HttpResponse, HttpServer};
use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use serde_derive::Deserialize;
use tokio::io::AsyncReadExt;
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use uuid::Uuid;

const BUCKET_NAME: &str = "rustpaste-pastes";

lazy_static::lazy_static! {
    static ref TRACING: () = {
        let filter = if std::env::var("TEST_LOG").is_ok() { "debug" } else { "info" };
        let subscriber = get_subscriber("test".into(), filter.into());
        init_subscriber(subscriber);
    };
}

#[derive(Deserialize)]
struct FormData {
    body: String,
}

#[tracing::instrument(
    name = "Creating new paste",
    skip(form, s3_client),
    fields(
        request_id=%Uuid::new_v4(),
    )
)]
async fn create(
    form: web::Form<FormData>,
    s3_client: web::Data<S3Client>,
) -> Result<HttpResponse, HttpResponse> {
    let uuid = Uuid::new_v4();
    let uuid_str = base_62::encode(uuid.as_bytes());

    let paste_id = upload_paste(&s3_client, &uuid_str, &form.body).await;

    match paste_id {
        Ok(id) => Ok(HttpResponse::Ok().body(id)),
        Err(e) => Err(HttpResponse::InternalServerError().body(format!("{:?}", e))),
    }
}

#[tracing::instrument(name = "Uploading paste", skip(s3_client))]
async fn upload_paste(s3_client: &S3Client, paste_id: &str, body: &str) -> anyhow::Result<String> {
    s3_client
        .put_object(PutObjectRequest {
            bucket: String::from(BUCKET_NAME),
            key: paste_id.to_string(),
            body: Some(body.to_string().into_bytes().into()),
            ..Default::default()
        })
        .await?;
    Ok(paste_id.to_string())
}

#[tracing::instrument(
    name = "Reading paste",
    fields(
        request_id=%Uuid::new_v4(),
    ),
    skip(s3_client)
)]
async fn read_paste(
    info: web::Path<Info>,
    s3_client: web::Data<S3Client>,
) -> Result<HttpResponse, HttpResponse> {
    let paste_str = download_paste(s3_client.as_ref(), info.paste_id.clone()).await;
    match paste_str {
        Ok(paste) => Ok(HttpResponse::Ok().body(paste)),
        Err(e) => Err(HttpResponse::InternalServerError().body(format!("{:?}", e))),
    }
}

#[derive(Deserialize, Debug)]
struct Info {
    paste_id: String,
}

#[tracing::instrument(name = "Downloading paste", skip(s3_client))]
async fn download_paste(s3_client: &S3Client, paste_id: String) -> anyhow::Result<String> {
    let obj = s3_client
        .get_object(GetObjectRequest {
            bucket: String::from(BUCKET_NAME),
            key: paste_id,
            ..Default::default()
        })
        .await?;
    tracing::info!("Object details: {:?}", &obj);
    let mut bytes = obj.body.unwrap().into_async_read();
    let mut buffer: Vec<u8> = Vec::with_capacity(1024);
    bytes.read_to_end(&mut buffer).await?;
    let output = String::from_utf8(buffer)?;
    tracing::info!("buffer: {:?}", &output);
    Ok(output)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    lazy_static::initialize(&TRACING);
    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger)
            .route("/create", web::post().to(create))
            .route("/{paste_id}", web::get().to(read_paste))
            .app_data(web::Data::new(S3Client::new(Region::EuWest1)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
