use anyhow::anyhow;
use anyhow::Result;
use std::{env, path::PathBuf};
use tmux_interface::Session;
use tmux_interface::TmuxCommand;

use tmux_interface::{
    AttachSession, NewSession, Sessions, StartServer, SwitchClient, TargetSession, TmuxOutput,
    SESSION_ALL,
};

/**
 * This function attaches to an existing tmux session if it exists with the same name as the name provided directory.
 * If a session does not exist, a new one is created with that name and launched with "nvim ."
 * If we are not on a tmux session, the function attaches to the newly created or existing session.
 *
 * @param dir A PathBuf representing the directory to attach or create the tmux session in
 *
 * @returns A Result containing TmuxOutput if the function succeeds, otherwise an error is returned
 *
 * Errors:
 *  - Failed to create tmux server: if creating a new tmux server fails
 *  - Failed to get file name from directory: if the file name cannot be extracted from the provided directory
 *  - Failed to convert file name to string: if the file name cannot be converted to a string
 *  - Failed to create new tmux session: if creating a new tmux session fails
 *  - Failed to switch to client: if switching to the client fails
 */
pub fn attach_or_create_tmux_session(dir: PathBuf) -> Result<TmuxOutput> {
    // Will start server if it is off
    StartServer::new()
        .output()
        .map_err(|e| anyhow!("Failed to create tmux server: {}", e))?;

    // Get the name of the session to be created or attached to
    let session_name = dir
        .file_name()
        .ok_or_else(|| anyhow!("Failed to get file name from directory"))?
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert file name to string"))?
        .to_string();

    // Check if a session with the same name already exists
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

    // If no session exists with the same name, create a new one
    if session.is_none() {
        NewSession::new()
            .detached()
            .session_name(&session_name)
            .start_directory(dir.to_str().unwrap().to_string())
            .output()
            .map_err(|e| anyhow!("Failed to create new tmux session: {}", e))?;

        let tmux = TmuxCommand::new();
        tmux.send_keys()
            .target_pane(&target_session)
            .key("nvim .")
            .key("C-m")
            .output()?;
    }

    // If we are not on a tmux session, the function attaches to the newly created or existing session.
    if env::var("TMUX").is_err() {
        AttachSession::new()
            .target_session(&session_name)
            .output()?;
    }

    // Switch to the client for the target session
    SwitchClient::new()
        .target_session(&target_session)
        .output()
        .map_err(|e| anyhow!("Failed to switch to client: {}", e))
}
