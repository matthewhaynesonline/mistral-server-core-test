```sh
curl -X 'POST' \
  'http://localhost:3000/api/mistral/v1/chat/completions' \
  -H 'accept: */*' \
  -H 'Content-Type: application/json' \
  -d '{
  "model": "microsoft/Phi-3.5-mini-instruct",
  "stream": true,
  "messages": [{
    "role": "system",
    "content": "You are a helpful bot that ends everything with Yea BOI!."
  }, {
    "role": "user",
    "content": "hi!"
  }]
}'
```
