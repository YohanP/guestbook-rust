# Guestbook Rust Migration

A rewrite of the classic [Kubernetes PHP Guestbook](https://github.com/kubernetes/examples/tree/master/web/guestbook/php-redis) using Rust and Axum.
Built with the assistance of the **Gemini-CLI AI** agent.

## Overview

This project is a modern migration of the original PHP-based Guestbook application. While the original used a single Redis key with comma-separated values, this Rust implementation leverages Redis lists.
It is a simple and convenient project to perform demo, test or anything you have mind.

### Key Changes
- **Language:** Migrated from PHP to **Rust**.
- **Web Framework:** Replaced Apache/PHP with **Axum** (built on Tokio).
- **Template Engine:** Replaced AngularJS frontend with **Tera** (server-side rendering).
- **Data Structure:** Improved from a single CSV string to a **Redis LIST** (`RPUSH`/`LRANGE`).
- **Containerization:** Modern multi-stage Docker builds for minimal runtime footprints.

## Prerequisites

To run this project, you need:
- [Docker](https://www.docker.com/get-started) and [Docker Compose](https://docs.docker.com/compose/install/)
- [Rust](https://www.rust-lang.org/tools/install) (optional, for local development without Docker)

## Running Locally

The easiest way to get the application running is using Docker Compose:

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd guestbook-rust
   ```

2. **Start the services:**
   ```bash
   docker-compose up --build
   ```

3. **Access the Guestbook:**
   Open your browser and navigate to `http://localhost:3000`.

## Project Structure

- `src/main.rs`: Axum web server logic and Redis integration.
- `templates/`: HTML templates for the Tera engine.
- `Dockerfile`: Multi-stage build (Rust Builder -> Debian Runtime).
- `docker-compose.yml`: Local orchestration for the App and Redis services.

## Configuration

The application uses the following environment variables:
- `REDISHOST`: The hostname of the Redis instance (defaults to `redis-leader`).