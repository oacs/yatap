use anyhow::anyhow;
use anyhow::Result;
use std::{env, path::PathBuf};

use tmux_interface::{
    AttachSession, NewSession, Sessions, StartServer, SwitchClient, TargetSession, TmuxOutput,
    SESSION_ALL,
};

pub fn attach_or_create_tmux_session(dir: PathBuf) -> Result<TmuxOutput> {
    StartServer::new()
        .output()
        .map_err(|e| anyhow!("Failed to create tmux server: {}", e))?;
    let session_name = dir
        .file_name()
        .ok_or_else(|| anyhow!("Failed to get file name from directory"))?
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert file name to string"))?
        .to_string();
    let target_session = TargetSession::Raw(&session_name).to_string();
    let session = Sessions::get(SESSION_ALL)
        .map(|s| {
            s.into_iter()
                .map(|s| s.name)
                .filter_map(|mut s| s.take())
                .find(|s| *s == target_session)
                .take()
        })
        .unwrap_or(None);

    if session.is_none() {
        NewSession::new()
            .detached()
            .session_name(session_name.clone())
            .start_directory(dir.to_str().unwrap().to_string())
            .shell_command("nvim .")
            .output()
            .map_err(|e| anyhow!("Failed to create new tmux session: {}", e))?;
    }

    if env::var("TMUX").is_err() {
        AttachSession::new()
            .target_session(&session_name)
            .output()?;
    }

    SwitchClient::new()
        .target_session(&target_session)
        .output()
        .map_err(|e| anyhow!("Failed to switch to client: {}", e))
}
