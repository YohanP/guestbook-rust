# Project: Guestbook Migration PHP to Rust

## Context
We are rewriting the original "Kubernetes Guestbook" application from PHP to Rust.
- Original Source (PHP): https://github.com/kubernetes/examples/tree/master/web/guestbook/php-redis
- Reference Docker Image: gcr.io/google-samples/gb-frontend:v5

## Target Stack (Rust)
- Web Framework: Axum
- Database: Redis (via `redis` crate)
- Templates: Tera (for HTML rendering)
- Environment Variables: `REDISHOST` (defaults to 'redis-leader')

## Objective
Produce a high-performance, memory-safe, and cloud-native microservice ready for ArgoCD deployment.
