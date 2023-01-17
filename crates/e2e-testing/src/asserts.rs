use anyhow::Result;
use std::borrow::Borrow;

pub fn assert_status(url: &str, expected: u16) -> Result<()> {
    let resp = req(url)?;
    let status = resp.status();
    let body = resp.text()?;
    assert_eq!(status, expected, "{}", body);
    Ok(())
}

async fn assert_http_request(
    url: &str,
    expected: u16,
    expected_headers: &[(&str, &str)],
    expected_body: &str,
) -> Result<()> {
    let res = req(url)?;

    let status = &res.status();
    assert_eq!(expected, status.as_u16());

    let body = &res.text()?;
    assert_eq!(expected_body, body);

    let headers = res.headers();

    // check the environment variables sent back as headers:
    for (k, v) in expected_headers {
        // assert_eq!(
        //     &headers
        //         .get(k.to_string())
        //         .unwrap_or_else(|| panic!("cannot find header {}", k))
        //         .to_str()?,
        //     v
        // );

        // res.headers()
        //     .get(k)
        //     .unwrap_or_else(|| panic!("cannot find header {}", k))
        //     .as_bytes()

        // assert_eq!(
        //     ,
        //     v
        // );
        // assert_eq!(
        //     &res.headers()
        //         .get(HeaderName::from_bytes(k.as_bytes())?)
        //         .unwrap_or_else(|| panic!("cannot find header {}", k))
        //         .to_str()?,
        //     v
        // );
    }

    // assert_eq!(
    //     res.headers()
    //         .get(HeaderName::from_bytes("spin-path-info".as_bytes())?)
    //         .unwrap_or_else(|| panic!("cannot find spin-path-info header"))
    //         .to_str()?,
    //     expected_path_info
    // );

    Ok(())
}

fn req(url: &str) -> reqwest::Result<reqwest::blocking::Response> {
    println!("{}", url);
    return reqwest::blocking::get(url);
}
