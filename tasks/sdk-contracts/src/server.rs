use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{Context, Result, ensure};
use serde_json::Value;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task::JoinHandle,
};

use crate::{Response, model::CapturedRequest};

pub struct Server {
    pub url: String,
    requests: Arc<Mutex<Vec<CapturedRequest>>>,
    task: JoinHandle<Result<()>>,
    expected: usize,
}

impl Server {
    pub async fn start(responses: Vec<Response>) -> Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let url = format!("http://{}", listener.local_addr()?);
        let requests = Arc::new(Mutex::new(Vec::new()));
        let captured = requests.clone();
        let expected = responses.len();
        let task = tokio::spawn(async move {
            // Read requests in fixture order, but let delayed replies overlap.
            let mut replies = tokio::task::JoinSet::new();
            for response in responses {
                let (mut stream, _) = listener.accept().await?;
                let request = read_request(&mut stream).await?;
                captured.lock().unwrap().push(request);
                replies.spawn(async move {
                    tokio::time::sleep(Duration::from_millis(response.delay_ms)).await;
                    let body = if response.status == 204 {
                        String::new()
                    } else {
                        response.body.to_string()
                    };
                    let mut message = format!(
                        "HTTP/1.1 {} Fixture\r\nContent-Type: application/json\r\nContent-Length: \
                         {}\r\nConnection: close\r\n",
                        response.status,
                        body.len()
                    );
                    for (key, value) in response.headers {
                        message.push_str(&format!("{key}: {value}\r\n"));
                    }
                    message.push_str("\r\n");
                    message.push_str(&body);
                    // A timed-out client is allowed to close before the delayed
                    // reply.
                    let _ = stream.write_all(message.as_bytes()).await;
                });
            }
            while let Some(result) = replies.join_next().await {
                result?;
            }
            Ok(())
        });
        Ok(Self { url, requests, task, expected })
    }

    pub async fn finish(mut self) -> Result<Vec<CapturedRequest>> {
        let result = tokio::time::timeout(Duration::from_secs(3), &mut self.task).await;
        if result.is_err() {
            self.task.abort();
        }
        result.context("Fixture server did not receive all expected requests")???;
        let requests = self.requests.lock().unwrap();
        ensure!(requests.len() == self.expected, "Unexpected request count");
        Ok(requests.clone())
    }
}

async fn read_request(stream: &mut TcpStream) -> Result<CapturedRequest> {
    let mut bytes = Vec::new();
    let header_end = loop {
        if let Some(index) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
            break index + 4;
        }
        let mut chunk = [0; 4096];
        let n = stream.read(&mut chunk).await?;
        ensure!(n > 0, "Connection closed before request headers");
        bytes.extend_from_slice(&chunk[..n]);
        ensure!(bytes.len() < 1024 * 1024, "Fixture request too large");
    };
    let head = std::str::from_utf8(&bytes[..header_end])?;
    let mut lines = head.split("\r\n");
    let mut first = lines.next().unwrap().split_whitespace();
    let method = first.next().context("Missing method")?.to_owned();
    let target = first.next().context("Missing request target")?.to_owned();
    let headers: BTreeMap<String, String> = lines
        .filter_map(|line| line.split_once(':'))
        .map(|(key, value)| (key.to_ascii_lowercase(), value.trim().to_owned()))
        .collect();
    let length: usize =
        headers.get("content-length").map(|value| value.parse()).transpose()?.unwrap_or(0);
    ensure!(length < 1024 * 1024, "Fixture request body too large");
    while bytes.len() < header_end + length {
        let mut chunk = [0; 4096];
        let n = stream.read(&mut chunk).await?;
        ensure!(n > 0, "Connection closed before request body");
        bytes.extend_from_slice(&chunk[..n]);
    }
    canonical(&method, &target, &headers, &bytes[header_end..header_end + length])
}

fn canonical(
    method: &str,
    target: &str,
    headers: &BTreeMap<String, String>,
    body: &[u8],
) -> Result<CapturedRequest> {
    let url = url::Url::parse(&format!("http://localhost{target}"))?;
    // Split before decoding: an encoded slash must remain inside its path
    // segment.
    let path: Vec<String> = url
        .path()
        .split('/')
        .map(|part| {
            percent_encoding::percent_decode_str(part).decode_utf8().map(|s| s.into_owned())
        })
        .collect::<Result<_, _>>()?;
    let mut query: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (key, value) in url.query_pairs() {
        query.entry(key.into_owned()).or_default().push(value.into_owned());
    }
    let mut selected = BTreeMap::new();
    for key in ["authorization", "polar-version", "polar-organization"] {
        if let Some(value) = headers.get(key) {
            selected.insert(key.to_owned(), value.clone());
        }
    }
    if !body.is_empty()
        && let Some(value) = headers.get("content-type")
    {
        selected.insert("content-type".into(), value.clone());
    }
    Ok(CapturedRequest {
        method: method.to_owned(),
        path,
        query,
        headers: selected,
        body: if body.is_empty() { Value::Null } else { serde_json::from_slice(body)? },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn query_order_is_ignored_but_repeated_value_order_is_not() {
        let headers = BTreeMap::new();
        let a = canonical("GET", "/v1/products?sorting=a&sorting=b&q=a+b", &headers, b"").unwrap();
        let b =
            canonical("GET", "/v1/products?q=a%20b&sorting=a&sorting=b", &headers, b"").unwrap();
        let reversed =
            canonical("GET", "/v1/products?q=a+b&sorting=b&sorting=a", &headers, b"").unwrap();
        assert_eq!(a, b);
        assert_ne!(a, reversed);
    }
    #[test]
    fn escaped_slash_cannot_equal_path_separator() {
        let headers = BTreeMap::new();
        assert_ne!(
            canonical("GET", "/external/a%2Fb", &headers, b"").unwrap(),
            canonical("GET", "/external/a/b", &headers, b"").unwrap()
        );
    }
    #[test]
    fn body_omission_is_distinct_from_explicit_null() {
        let headers = BTreeMap::new();
        assert_ne!(
            canonical("PATCH", "/", &headers, br#"{}"#).unwrap(),
            canonical("PATCH", "/", &headers, br#"{"name":null}"#).unwrap()
        );
    }
}
