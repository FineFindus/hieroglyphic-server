# Hieroglyphic Server

Data contribution server for [Hieroglyphic](https://github.com/FineFindus/Hieroglyphic).
The server allows users to (optionally) upload and contribute stroke data for symbol recognition, which helps improve the machine learning model behind Hieroglyphic's functionality.

### Architecture

The server uses [`axum`](https://github.com/tokio-rs/axum), the contributed data is stored in a MongoDB instance.
