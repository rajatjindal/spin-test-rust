use anyhow::{Context, Result};
use hyper::header::HeaderName;
use hyper::{Body, Client, Response};

pub async fn assert_status(url: &str, expected: u16) -> Result<()> {
    let res = req(url).await?;
    let status = res.status();
    let body = hyper::body::to_bytes(res.into_body())
        .await
        .expect("read body");
    assert_eq!(status, expected, "{}", String::from_utf8_lossy(&body));

    Ok(())
}

async fn assert_http_request(
    url: &str,
    expected: u16,
    expected_env_as_headers: &[(&str, &str)],
    expected_path_info: &str,
) -> Result<()> {
    let res = req(url).await?;
    assert_eq!(res.status(), expected);

    // check the environment variables sent back as headers:
    for (k, v) in expected_env_as_headers {
        assert_eq!(
            &res.headers()
                .get(HeaderName::from_bytes(k.as_bytes())?)
                .unwrap_or_else(|| panic!("cannot find header {}", k))
                .to_str()?,
            v
        );
    }

    assert_eq!(
        res.headers()
            .get(HeaderName::from_bytes("spin-path-info".as_bytes())?)
            .unwrap_or_else(|| panic!("cannot find spin-path-info header"))
            .to_str()?,
        expected_path_info
    );

    Ok(())
}

async fn req(url: &str) -> Result<Response<Body>> {
    let c = Client::new();
    Ok(c.get(url.parse().with_context(|| "cannot parse URL")?)
        .await?)
}
