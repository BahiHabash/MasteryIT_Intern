use crate::{
    errors::AppError,
    models::{BookResponse, CreateBookRequest},
    state::SharedState,
};
use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn list_books(
    State(state): State<SharedState>,
) -> Result<(StatusCode, Json<Vec<BookResponse>>), AppError> {
    let books = {
        let state_guard = state
            .lock()
            .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?;
        state_guard.get_all_books()
    };

    let response: Vec<BookResponse> = books.into_iter().map(|b| b.into()).collect();

    Ok((StatusCode::OK, Json(response)))
}

pub async fn create_book(
    State(state): State<SharedState>,
    Json(payload): Json<CreateBookRequest>,
) -> Result<(StatusCode, Json<BookResponse>), AppError> {
    let new_book = {
        let mut state_guard = state
            .lock()
            .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?;
        state_guard
            .add_book(payload)
            .context("Failed to add new book to repository")?
    };

    Ok((StatusCode::CREATED, Json(new_book.into())))
}

pub async fn delete_book_handler(
    State(state): State<SharedState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, AppError> {
    let deleted = {
        let mut state_guard = state
            .lock()
            .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?;
        state_guard.delete_book(id)
    };

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(anyhow::anyhow!("Book with ID {} not found", id).into())
    }
}

pub async fn trigger_panic() -> &'static str {
    panic!("Simulated worker thread panic to demonstrate CatchPanicLayer resilience!");
}
