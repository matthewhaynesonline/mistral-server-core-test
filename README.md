## This has been merged into Mistral.rs. See: https://ericlbuehler.github.io/mistral.rs/mistralrs_server_core/

```sh
curl -X 'POST' \
  'http://localhost:3000/api/mistral/v1/chat/completions' \
  -H 'accept: */*' \
  -H 'Content-Type: application/json' \
  -d '{
  "model": "placeholder",
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
  "model": "placeholder",
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
  "model": "placeholder",
  "stream": true,
  "messages": [{
    "role": "user",
    "content": "hi!"
  }]
}'

curl -X 'POST' \
  'http://localhost:3000/chat' \
  -H 'accept: */*' \
  -H 'Content-Type: application/json' \
  -d '{
  "model": "placeholder",
  "messages": [{
    "role": "user",
    "content": "hi!"
  }]
}'

curl -X 'POST' \
  'http://localhost:3000/chat' \
  -H 'accept: */*' \
  -H 'Content-Type: application/json' \
  -d '{
  "model": "placeholder",
  "stream": true,
  "messages": [{
    "role": "user",
    "content": "hi!"
  }]
}'
```
