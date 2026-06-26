//! HTTP handlers. Each renders a minijinja template with structured context.

use crate::content::{SiteConfig, WritingMeta};
use crate::markdown;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
};
use minijinja::{context, Environment, Value};
use std::{path::PathBuf, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<SiteConfig>,
    pub writings: Arc<Vec<WritingMeta>>,
    pub env: Arc<Environment<'static>>,
    pub root: Arc<PathBuf>,
    /// Repo root holding the canonical `essays/` and `assets/` (the writing data).
    pub writings_root: Arc<PathBuf>,
}

fn render(state: &AppState, tpl: &str, ctx: Value) -> Response {
    match state.env.get_template(tpl).and_then(|t| t.render(ctx)) {
        Ok(body) => Html(body).into_response(),
        Err(err) => {
            tracing::error!("template {tpl} failed: {err:#}");
            (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response()
        }
    }
}

fn not_found(state: &AppState) -> Response {
    let body = state
        .env
        .get_template("notfound.html")
        .and_then(|t| t.render(context! { cfg => &*state.cfg, page => "" }))
        .unwrap_or_else(|_| "<h1>404 — Not found</h1>".to_string());
    (StatusCode::NOT_FOUND, Html(body)).into_response()
}

pub async fn home(State(state): State<AppState>) -> Response {
    let featured: Vec<&WritingMeta> = state.writings.iter().filter(|w| w.featured).collect();
    render(
        &state,
        "home.html",
        context! {
            cfg => &*state.cfg,
            featured => featured,
            page => "home",
        },
    )
}

pub async fn writings_index(State(state): State<AppState>) -> Response {
    render(
        &state,
        "writings.html",
        context! {
            cfg => &*state.cfg,
            writings => &*state.writings,
            page => "writings",
        },
    )
}

pub async fn essay(State(state): State<AppState>, Path(slug): Path<String>) -> Response {
    let Some(idx) = state.writings.iter().position(|w| w.slug == slug) else {
        return not_found(&state);
    };
    let meta = &state.writings[idx];

    let path = state
        .writings_root
        .join("essays")
        .join(format!("{slug}.md"));
    let md = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return not_found(&state),
    };

    let rewrite = markdown::RewriteCtx {
        published: state.writings.iter().map(|w| w.slug.clone()).collect(),
        repo_base: format!("{}/blob/main", state.cfg.repo_url),
    };
    let body = markdown::strip_toc(&markdown::strip_first_h1(&md));
    let rendered = markdown::render(&body, &rewrite);
    let prev = idx.checked_sub(1).map(|i| &state.writings[i]);
    let next = state.writings.get(idx + 1);

    render(
        &state,
        "essay.html",
        context! {
            cfg => &*state.cfg,
            meta => meta,
            content => rendered.html,
            toc => rendered.toc,
            prev => prev,
            next => next,
            page => "writings",
        },
    )
}

pub async fn consulting(State(state): State<AppState>) -> Response {
    render(
        &state,
        "consulting.html",
        context! {
            cfg => &*state.cfg,
            page => "consulting",
        },
    )
}

pub async fn resume(State(state): State<AppState>) -> Response {
    let path = state.root.join("Buddy_Williams_Resume.pdf");
    match std::fs::read(&path) {
        Ok(bytes) => Response::builder()
            .header(header::CONTENT_TYPE, "application/pdf")
            .header(
                header::CONTENT_DISPOSITION,
                "inline; filename=\"Buddy_Williams_Resume.pdf\"",
            )
            .body(Body::from(bytes))
            .unwrap(),
        Err(_) => not_found(&state),
    }
}
