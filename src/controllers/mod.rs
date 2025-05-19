use std::sync::Arc;

use axum::{Json, extract::State};

use mistralrs::ChatCompletionChunkResponse;
use mistralrs_server_core::{
    chat_completion::{
        ChatCompletionResponder, OnDoneCallback, create_chat_streamer, create_response_channel,
        handle_error, parse_request, process_non_streaming_chat_response, send_request,
    },
    openai::ChatCompletionRequest,
};

use crate::AppState;

#[utoipa::path(
  post,
  tag = "Custom",
  path = "/chat",
  request_body = ChatCompletionRequest,
  responses((status = 200, description = "Chat completions"))
)]
pub async fn custom_chat(
    State(state): State<Arc<AppState>>,
    Json(oai_request): Json<ChatCompletionRequest>,
) -> ChatCompletionResponder {
    let mistral_state = state.mistral_state.clone();
    let (tx, mut rx) = create_response_channel();

    let (request, is_streaming) = match parse_request(oai_request, mistral_state.clone(), tx).await
    {
        Ok(x) => x,
        Err(e) => return handle_error(mistral_state, e.into()),
    };

    dbg!(request.clone());

    if let Err(e) = send_request(&mistral_state, request).await {
        return handle_error(mistral_state, e.into());
    }

    if is_streaming {
        let db_fn = state.db_create;
        let on_done: OnDoneCallback = Box::new(move |chunks: &[ChatCompletionChunkResponse]| {
            dbg!(chunks);
            (db_fn)();
        });

        let streamer = create_chat_streamer(rx, mistral_state.clone(), Some(on_done));

        ChatCompletionResponder::Sse(streamer)
    } else {
        let response = process_non_streaming_chat_response(&mut rx, mistral_state.clone()).await;

        match &response {
            ChatCompletionResponder::Json(json_response) => {
                dbg!(json_response);
            }
            _ => {
                //
            }
        }

        response
    }
}
