use std::process::Stdio;
use std::time::Duration;
use tokio::process::Child;
use tokio::signal;
use tokio::time::interval;
use tokio::io::AsyncReadExt;
use tokio::pin;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //init logger
    tracing_subscriber::fmt::init();

    info!("audio-inhibit-daemon starting...");

    // Poll every 1 second
    let mut ticker = interval(Duration::from_secs(1));

    // Handle to systemd-inhibit child
    let mut inhibitor: Option<Child> = None;

    let ctrl_c = signal::ctrl_c();
    pin!(ctrl_c);

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                match is_audio_playing().await {
                    Ok(true) => {
                        if inhibitor.is_none() {
                            info!("Audio detected - activating sleep inhibitor");
                            match spawn_inhibitor_process().await {
                                Ok(child) => inhibitor = Some(child),
                                Err(e) => error!("Failed to spawn inhibitor: {}", e),
                            }
                        }
                    }
                    Ok(false) => {
                        if inhibitor.is_some() {
                            info!("No audio - releasing inhibitor");
                            if let Err(e) = kill_inhibitor(&mut inhibitor).await {
                                error!("Error killing sleep inhibitor: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error checking audio: {}", e);
                    }
                }
            }
            _ = &mut ctrl_c => {
                info!("received Ctrl-C - cleaning up and exiting");
                if let Err(e) = kill_inhibitor(&mut inhibitor).await {
                    error!("Error killing inhibitor during shutdown: {}", e);
                }
                break;
            }
        }
    }

    info!("audio-inhibit-daemon stopped");
    Ok(())
}

async fn is_audio_playing() -> anyhow::Result<bool> {
    let mut cmd = tokio::process::Command::new("pactl");
    cmd.arg("list").arg("sink-inputs");
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::null());

    let mut child = cmd.spawn()?;
    let mut stdout = child.stdout.take().expect("stdout piped");
    let mut buf = String::new();
    stdout.read_to_string(&mut buf).await.ok();
    let _ = child.wait().await;

    // parse lines for "Corked: no" to detect active audio
    let active = buf.lines().any(|line| line.trim().eq_ignore_ascii_case("corked: no"));

    Ok(active)
}

async fn spawn_inhibitor_process() -> anyhow::Result<Child> {
    // run cmd: systemd-inhibit --what=sleep --why="Audio playing" --mode=block sleep infinity
    let mut cmd = tokio::process::Command::new("systemd-inhibit");
    cmd.arg("--what=sleep")
        .arg("--why=Audio playing")
        .arg("--mode=block")
        .arg("sleep")
        .arg("infinity")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let child = cmd.spawn()?;
    Ok(child)
}

async fn kill_inhibitor(inhibitor: &mut Option<Child>) -> anyhow::Result<()> {
    if let Some(child) = inhibitor.as_mut() {
        // kill the inhibitor process
        match child.kill().await {
            Ok(_) => {
                *inhibitor = None;
                Ok(())
            }
            Err(e) => {
                error!("Error killing child: {}", e);
                *inhibitor = None;
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}
