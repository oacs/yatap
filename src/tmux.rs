use std::path::PathBuf;

use tmux_interface::{
    Error, NewSession, Sessions, SwitchClient, TargetSession, TmuxOutput, SESSION_ALL,
};

pub fn attach_or_create_tmux_session(dir: PathBuf) -> Result<TmuxOutput, Error> {
    let session_name = dir.file_name().unwrap().to_str().unwrap().to_string();
    let target_session = TargetSession::Raw(&session_name).to_string();
    let sessions = Sessions::get(SESSION_ALL).unwrap();
    let session = sessions
        .into_iter()
        .find(|s| s.name == Some(target_session.to_string()));
    if let Some(_) = session {
        return SwitchClient::new().target_session(target_session).output();
    } else {
        NewSession::new()
            .detached()
            .session_name(session_name.clone())
            .start_directory(dir.to_str().unwrap().to_string())
            .output()
            .unwrap();
        SwitchClient::new().target_session(target_session).output()
    }
}
