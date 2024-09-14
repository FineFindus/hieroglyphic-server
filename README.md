# Hieroglyphic Server

Data contribution server for [Hieroglyphic](https://github.com/FineFindus/Hieroglyphic).
The server allows users to (optionally) upload and contribute stroke data for symbol recognition, which helps improve the machine learning model behind Hieroglyphic's functionality.

### Architecture

The server is built on [shuttle.rs](https://shuttle.rs/), a managed Rust backend provider, the contributed data is stored in a PostgreSQL database.
