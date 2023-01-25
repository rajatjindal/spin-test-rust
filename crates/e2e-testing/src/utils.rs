use anyhow::Result;
use std::path::PathBuf;
use std::str;
use std::{
    collections::HashMap,
    ffi::OsStr,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    process::{self, Child, Command, Output},
    time::Duration,
};
use tokio::sync::mpsc::error::SendTimeoutError::Timeout;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::time::timeout;
use tokio::{net::TcpStream, time::sleep};

pub fn testcases_base_dir() -> String {
    let basedir: PathBuf = [env!("CARGO_MANIFEST_DIR"), "..", "..", "tests", "testcases"]
        .iter()
        .collect();

    basedir.to_str().unwrap().to_string()
}

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

pub async fn wait_tcp2(url: &str, process: &mut tokio::process::Child, target: &str) -> Result<()> {
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

pub fn run_async<S: Into<String> + AsRef<OsStr>>(
    args: Vec<S>,
    dir: Option<S>,
    envs: Option<HashMap<&str, &str>>,
) -> tokio::process::Child {
    let mut cmd = TokioCommand::new(get_os_process());
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

    return cmd.spawn().expect("failed to spawn command");
}

pub async fn get_output(mut child: tokio::process::Child) -> Result<Vec<String>> {
    // pub async fn get_output(mut child: tokio::process::Child) -> Result<String> {
    print!("inside get_output");
    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    print!("after let reader");
    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    tokio::spawn(async move {
        print!("inside tokio spawn");
        let status = child
            .wait()
            .await
            .expect("child process encountered an error");

        print!("inside tokio::spawn after wait");
        println!("child status was: {}", status);
    });

    print!("after tokio spawn");

    // let inner_reader = reader.into_inner();
    // let logs = match str::from_utf8(inner_reader.buffer()) {
    //     Ok(logs) => logs,
    //     Err(error) => panic!("problem fetching deploy logs for app {:?}", error),
    // };

    let mut output = vec![];

    let nl = reader.next_line();

    match timeout(Duration::from_secs(5), nl).await? {
        Err(_) => (),
        Ok(line) => println!("Line: {}", line.unwrap()),
    }

    // if let Err(_) = timeout(
    //     Duration::from_millis(5000),
    //     while let Some(line) = reader.next_line().await? {
    //         print!("inside while loop");
    //         output.push(line.to_string());
    //         println!("Line: {}", line);
    //     },
    // ) {}

    print!("before returning");
    Ok(output)
    // Ok(logs.to_string())
}
