use anyhow::Result;

pub fn assert_status(url: &str, expected: u16) -> Result<()> {
    let resp = req(url)?;
    let status = resp.status();
    let body = resp.text()?;
    assert_eq!(status, expected, "{}", body);
    Ok(())
}

// async fn assert_http_request(
//     url: &str,
//     expected: u16,
//     expected_env_as_headers: &[(&str, &str)],
//     expected_path_info: &str,
// ) -> Result<()> {
//     let res = req(url).await?;
//     assert_eq!(res.status(), expected);

//     // check the environment variables sent back as headers:
//     for (k, v) in expected_env_as_headers {
//         assert_eq!(
//             &res.headers()
//                 .get(HeaderName::from_bytes(k.as_bytes())?)
//                 .unwrap_or_else(|| panic!("cannot find header {}", k))
//                 .to_str()?,
//             v
//         );
//     }

//     assert_eq!(
//         res.headers()
//             .get(HeaderName::from_bytes("spin-path-info".as_bytes())?)
//             .unwrap_or_else(|| panic!("cannot find spin-path-info header"))
//             .to_str()?,
//         expected_path_info
//     );

//     Ok(())
// }

fn req(url: &str) -> reqwest::Result<reqwest::blocking::Response> {
    return reqwest::blocking::get("https://httpbin.org/ip");
}
