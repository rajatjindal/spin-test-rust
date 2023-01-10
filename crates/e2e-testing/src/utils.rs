use anyhow::Result;
use std::{
    collections::HashMap,
    ffi::OsStr,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    process::{self, Child, Command, Output},
    time::Duration,
};
use tokio::{net::TcpStream, time::sleep};

pub fn run<S: Into<String> + AsRef<OsStr>>(
    args: Vec<S>,
    dir: Option<S>,
    envs: Option<HashMap<&str, &str>>,
) -> Result<Output> {
    let mut cmd = Command::new(get_os_process());
    cmd.stdout(process::Stdio::piped());
    cmd.stderr(process::Stdio::piped());

    if let Some(dir) = dir {
        cmd.current_dir(dir.into());
    };

    cmd.arg("-c");
    cmd.arg(
        args.into_iter()
            .map(Into::into)
            .collect::<Vec<String>>()
            .join(" "),
    );
    if let Some(envs) = envs {
        for (k, v) in envs {
            cmd.env(k, v);
        }
    }

    let output = cmd.output()?;
    let code = output.status.code().expect("should have status code");
    if code != 0 {
        println!("{:#?}", std::str::from_utf8(&output.stderr)?);
        println!("{:#?}", std::str::from_utf8(&output.stdout)?);
        panic!("command `{:?}` exited with code {}", cmd, code);
    }

    Ok(output)
}

fn get_os_process() -> String {
    if cfg!(target_os = "windows") {
        String::from("powershell.exe")
    } else {
        String::from("/bin/bash")
    }
}

pub fn get_random_port() -> Result<u16> {
    Ok(
        TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))?
            .local_addr()?
            .port(),
    )
}

pub async fn wait_tcp(url: &str, process: &mut Child, target: &str) -> Result<()> {
    let mut wait_count = 0;
    loop {
        if wait_count >= 240 {
            panic!(
                "Ran out of retries waiting for {} to start on URL {}",
                target, url
            );
        }

        if let Ok(Some(_)) = process.try_wait() {
            panic!(
                "Process exited before starting to serve {} to start on URL {}",
                target, url
            );
        }

        match TcpStream::connect(&url).await {
            Ok(_) => break,
            Err(e) => {
                println!("connect {} error {}, retry {}", &url, e, wait_count);
                wait_count += 1;
                sleep(Duration::from_secs(1)).await;
            }
        }
    }

    Ok(())
}
