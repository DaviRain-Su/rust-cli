use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
pub struct HttpServeState {
    pub path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Serve {:?} on addr {}", path, addr);
    let state = HttpServeState { path: path.clone() };
    let dr_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_gzip()
        .precompressed_zstd();

    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", dr_service)
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, Response) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Read file: {:?}", p);
    if !p.exists() {
        let body = Body::from(format!("File {} Not found", p.display()));
        (StatusCode::NOT_FOUND, Response::new(body))
    } else {
        // TODO: test p is directory.
        // if it is a directory, list files/subdirectories.
        // as <li><a href="/path/to/file">file name</a></li>
        // <html><body><ul>...</ul></body></html>
        //
        if p.is_dir() {
            let mut data = String::new();
            let mut entries = tokio::fs::read_dir(p).await.unwrap();
            while let Some(entry) = entries.next_entry().await.unwrap() {
                // Here, `entry` is a `DirEntry`.
                let path = entry.path();
                let path = path.strip_prefix(&state.path).unwrap();
                let path = path.to_str().unwrap();
                data.push_str(&format!("<li><a href=\"{}\">{}</a></li>\n", path, path));
            }
            // generate better html template
            let html = format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                <title>Home</title>
                </head>
                <body>
                <h1>Directory list</h1>
                <ul>
                {}
                </ul>
                </body>
                </html>
                "#,
                data
            );
            (StatusCode::OK, Response::new(Body::from(html)))
        } else {
            match tokio::fs::read_to_string(p).await {
                Ok(data) => {
                    info!("Read {} Bytes", data.len());
                    (StatusCode::OK, Response::new(Body::from(data)))
                }
                Err(e) => {
                    warn!("Error reading file: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Response::new(Body::from(format!("Error: {}", e))),
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, _data) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        // assert!(data.body() contains("[package]"));
    }
}
