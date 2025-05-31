use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use mistralrs::{AutoDeviceMapParams, ModelDType, ModelSelected};
use mistralrs_server_core::{
    mistralrs_for_server_builder::MistralRsForServerBuilder,
    mistralrs_server_router_builder::MistralRsServerRouterBuilder, openapi_doc::get_openapi_doc,
    types::SharedMistralState,
};

pub mod controllers;
use controllers::custom_chat;

#[derive(OpenApi)]
#[openapi(
    paths(root, controllers::custom_chat),
    tags(
        (name = "hello", description = "Hello world endpoints")
    ),
    info(
        title = "Hello World API",
        version = "1.0.0",
        description = "A simple API that responds with a greeting"
    )
)]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    pub mistral_state: SharedMistralState,
    pub db_create: fn(),
}

#[tokio::main]
async fn main() {
    let plain_model_id = String::from("meta-llama/Llama-3.2-1B-Instruct");
    let tokenizer_json = None;
    let arch = None;
    let organization = None;
    let write_uqff = None;
    let from_uqff = None;
    let imatrix = None;
    let calibration_file = None;
    let hf_cache_path = None;

    // let quantized_model_id = String::from("bartowski/Llama-3.2-1B-Instruct-GGUF");
    // let quantized_filename = String::from("Llama-3.2-1B-Instruct-Q4_K_M.gguf");

    let dtype = ModelDType::Auto;
    let topology = None;
    let max_seq_len = AutoDeviceMapParams::DEFAULT_MAX_SEQ_LEN;
    let max_batch_size = AutoDeviceMapParams::DEFAULT_MAX_BATCH_SIZE;

    // let model = ModelSelected::GGUF {
    //     tok_model_id: None,
    //     quantized_model_id,
    //     quantized_filename,
    //     dtype,
    //     topology,
    //     max_seq_len,
    //     max_batch_size,
    // };

    let model = ModelSelected::Plain {
        model_id: plain_model_id,
        tokenizer_json,
        arch,
        dtype,
        topology,
        organization,
        write_uqff,
        from_uqff,
        imatrix,
        calibration_file,
        max_seq_len,
        max_batch_size,
        hf_cache_path,
    };

    // let args = Args {
    //     model,
    //     ..Args::default()
    // };

    // Use ISQ instead of GGUF
    // https://github.com/EricLBuehler/mistral.rs/issues/1383

    let shared_mistralrs = MistralRsForServerBuilder::new()
        .with_model(model)
        .with_in_situ_quant("8".to_string())
        .with_paged_attn(true)
        .build()
        .await
        .unwrap();

    let mistral_base_path = "/api/mistral";

    let mistral_routes = MistralRsServerRouterBuilder::new()
        .with_mistralrs(shared_mistralrs.clone())
        .with_include_swagger_routes(false)
        .with_base_path(mistral_base_path)
        .build()
        .await
        .unwrap();

    let mistral_doc = get_openapi_doc(Some(mistral_base_path));
    let mut api_docs = ApiDoc::openapi();
    api_docs.merge(mistral_doc);

    let app_state = Arc::new(AppState {
        mistral_state: shared_mistralrs,
        db_create: mock_db_call,
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/chat", post(custom_chat))
        .with_state(app_state.clone())
        .nest(mistral_base_path, mistral_routes)
        .merge(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", api_docs));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Listening on 0.0.0.0:3000");
}

#[utoipa::path(
    get,
    path = "/",
    tag = "hello",
    responses(
        (status = 200, description = "Successful response with greeting message", body = String)
    )
)]
async fn root() -> &'static str {
    "Hello, World!"
}

pub fn mock_db_call() {
    println!("Saving to DB");
}
