use axum::{Json, extract::State};

use mistralrs_server_core::{
    SharedMistralState,
    chat_completion::{
        ChatCompletionResponder, StreamerChunks, create_chat_streamer, create_response_channel,
        handle_error, parse_request, process_non_streaming_chat_response, send_request,
    },
    openai::ChatCompletionRequest,
};

#[utoipa::path(
  post,
  tag = "Custom",
  path = "/chat",
  request_body = ChatCompletionRequest,
  responses((status = 200, description = "Chat completions"))
)]
pub async fn custom_chat(
    State(state): State<SharedMistralState>,
    Json(oai_request): Json<ChatCompletionRequest>,
) -> ChatCompletionResponder {
    let (tx, mut rx) = create_response_channel();

    let (request, is_streaming) = match parse_request(oai_request, state.clone(), tx).await {
        Ok(x) => x,
        Err(e) => return handle_error(state, e.into()),
    };

    dbg!(request.clone());
    dbg!(is_streaming);

    if let Err(e) = send_request(&state, request).await {
        return handle_error(state, e.into());
    }

    if is_streaming {
        // For streaming, we need to wrap the streamer to capture all chunks
        let base_streamer = create_chat_streamer(rx, state.clone(), Some(stream_complete));

        // Wrap the streamer to capture the full response
        // let capturing_streamer = ResponseCapturingStreamer::new(base_streamer, state.clone());

        ChatCompletionResponder::Sse(base_streamer)
    } else {
        // For non-streaming, we can capture the full response at once
        let response = process_non_streaming_chat_response(&mut rx, state.clone()).await;

        // Update the record with the response
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

pub fn stream_complete(chunks: StreamerChunks) {
    dbg!(chunks);
}
