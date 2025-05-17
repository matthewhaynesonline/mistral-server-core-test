```sh
curl -X 'POST' \
  'http://localhost:3000/api/mistral/v1/chat/completions' \
  -H 'accept: */*' \
  -H 'Content-Type: application/json' \
  -d '{
  "model": "bartowski/Llama-3.2-1B-Instruct-GGUF",
  "messages": [{
    "role": "system",
    "content": "You are a helpful bot that ends everything with Yea BOI!."
  }, {
    "role": "user",
    "content": "hi!"
  }]
}'

curl -X 'POST' \
  'http://localhost:3000/api/mistral/v1/chat/completions' \
  -H 'accept: */*' \
  -H 'Content-Type: application/json' \
  -d '{
  "model": "bartowski/Llama-3.2-1B-Instruct-GGUF",
  "messages": [{
    "role": "user",
    "content": "hi!"
  }]
}'

curl -X 'POST' \
  'http://localhost:3000/api/mistral/v1/chat/completions' \
  -H 'accept: */*' \
  -H 'Content-Type: application/json' \
  -d '{
  "model": "bartowski/Llama-3.2-1B-Instruct-GGUF",
  "stream": true,
  "messages": [{
    "role": "user",
    "content": "hi!"
  }]
}'
```
