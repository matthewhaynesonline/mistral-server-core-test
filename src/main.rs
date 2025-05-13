use axum::{
    Router,
    routing::{get, post},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use mistralrs::{AutoDeviceMapParams, ModelDType, ModelSelected, TokenSource};
use mistralrs_server_core::{
    Args, bootstrap_mistralrs, bootstrap_mistralrs_router_from_state, get_openapi_doc,
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

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();

    let quantized_model_id = String::from("bartowski/Llama-3.2-1B-Instruct-GGUF");
    let quantized_filename = String::from("Llama-3.2-1B-Instruct-Q4_K_M.gguf");
    let dtype = ModelDType::Auto;
    let topology = None;
    let max_seq_len = AutoDeviceMapParams::DEFAULT_MAX_SEQ_LEN;
    let max_batch_size = AutoDeviceMapParams::DEFAULT_MAX_BATCH_SIZE;

    let model = ModelSelected::GGUF {
        tok_model_id: None,
        quantized_model_id,
        quantized_filename,
        dtype,
        topology,
        max_seq_len,
        max_batch_size,
    };

    let args = Args {
        serve_ip: None,
        seed: None,
        port: None,
        log: None,
        truncate_sequence: true,
        model,
        max_seqs: 16,
        no_kv_cache: false,
        chat_template: None,
        jinja_explicit: None,
        token_source: TokenSource::CacheToken,
        interactive_mode: false,
        prefix_cache_n: 16,
        num_device_layers: None,
        in_situ_quant: None,
        paged_attn_gpu_mem: None,
        paged_attn_gpu_mem_usage: None,
        paged_ctxt_len: None,
        paged_attn_block_size: None,
        no_paged_attn: false,
        paged_attn: true,
        throughput_log: true,
        prompt_chunksize: None,
        cpu: true,
        enable_search: false,
        search_bert_model: None,
        enable_thinking: false,
    };

    let shared_mistralrs = bootstrap_mistralrs(args).await.unwrap();

    let mistral_base_path = "/api/mistral";

    let mistral_routes = bootstrap_mistralrs_router_from_state(
        shared_mistralrs.clone(),
        false,
        Some(mistral_base_path),
    )
    .await
    .unwrap();

    let mistral_doc = get_openapi_doc(Some(mistral_base_path));
    let mut api_docs = ApiDoc::openapi();
    api_docs.merge(mistral_doc);

    let app = Router::new()
        .route("/", get(root))
        .route("/chat", post(custom_chat))
        .with_state(shared_mistralrs.clone())
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
