use std::{env, fs::File, io::Write, path::PathBuf};

use tmux_interface::{
    AttachSession, Error, NewSession, Sessions, StartServer, SwitchClient, TargetSession,
    TmuxOutput, SESSION_ALL,
};

pub fn attach_or_create_tmux_session(dir: PathBuf) -> Result<TmuxOutput, Error> {
    let mut output = File::create("log")?;
    StartServer::new().output()?;
    let session_name = dir.file_name().unwrap().to_str().unwrap().to_string();
    writeln!(output, "session_name: {}", session_name)?;
    let target_session = TargetSession::Raw(&session_name).to_string();
    let sessions = Sessions::get(SESSION_ALL)?;
    let session = sessions
        .into_iter()
        .find(|s| s.name == Some(target_session.to_string()));
    if session.is_none() {
        NewSession::new()
            .detached()
            .session_name(session_name.clone())
            .start_directory(dir.to_str().unwrap().to_string())
            .shell_command("nvim .")
            .output()
            .unwrap();
    }

    if env::var("TMUX").is_err() {
        AttachSession::new().target_session(session_name).output()?;
    }
    SwitchClient::new().target_session(target_session).output()
}
