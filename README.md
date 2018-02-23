# Timeliner

Simple Timeline

## Dependencies

tokio-minihttp, lazy_static, serde_json, tokio_signal, websocket

## Models

Post: String author, Date time, String text

Comment: String author, Date time, String text

## APIs

```http
GET / -> num_posts
GET /<nth>/ -> Post
GET /<nth>/views -> num_views
GET /<nth>/comments/ -> [Comment
GET /<nth>/comments/len -> num_comments


GET /pop/ (basic auth) -> new_len
POST /<author>/ body=<text> (basic auth) -> new_len
```

## Extra

数据库不使用，采用每次启动读取数据每次退出序列化数据的方式 🌚

WebSocket 在文章被回复的时候发送文章索引。

Using: `timelinerd <password>`

SIGQUIT: output data and exit

SIGUSR1: output data

SIGUSR2: clear anti-spamming log
