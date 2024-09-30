#[allow(dead_code)]
pub async fn headers() -> reqwest::header::HeaderMap {
    let mut header = reqwest::header::HeaderMap::new();
    header.insert(
        "User-Agent",
        reqwest::header::HeaderValue::from_static("HTTPie/3.2.3"),
    );
    header.insert(
        "Accept",
        reqwest::header::HeaderValue::from_static("application/json, text/plain, */*"),
    );
    header.insert(
        "Accept-Language",
        reqwest::header::HeaderValue::from_static("en-US,en;q=0.9"),
    );
    header.insert(
        "Referer",
        reqwest::header::HeaderValue::from_static("https://comick.fun/"),
    );
    header.insert(
        "Origin",
        reqwest::header::HeaderValue::from_static("https://comick.fun"),
    );
    header.insert(
        "Sec-Fetch-Dest",
        reqwest::header::HeaderValue::from_static("empty"),
    );
    header.insert(
        "Sec-Fetch-Mode",
        reqwest::header::HeaderValue::from_static("cors"),
    );
    header.insert(
        "Sec-Fetch-Site",
        reqwest::header::HeaderValue::from_static("same-site"),
    );

    return header;
}


