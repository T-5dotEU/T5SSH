use crate::pty::PtyHandle;
use portable_pty::MasterPty;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum SessionState {
    Creating,
    Connecting,
    Connected,
    Closed,
}

pub struct Session {
    pub id: String,
    pub state: SessionState,
    pub pty_handle: PtyHandle,
    pub master: Box<dyn MasterPty + Send>,
    pub label: String,
}

#[derive(Clone, Serialize)]
pub struct SessionInfo {
    pub id: String,
    pub state: SessionState,
    pub label: String,
}

impl Session {
    pub fn info(&self) -> SessionInfo {
        SessionInfo {
            id: self.id.clone(),
            state: self.state.clone(),
            label: self.label.clone(),
        }
    }
}

pub struct SessionManager {
    pub sessions: Arc<Mutex<HashMap<String, Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert(&self, session: Session) {
        self.sessions
            .lock()
            .unwrap()
            .insert(session.id.clone(), session);
    }

    pub fn remove(&self, id: &str) -> Option<Session> {
        self.sessions.lock().unwrap().remove(id)
    }

    pub fn list(&self) -> Vec<SessionInfo> {
        self.sessions
            .lock()
            .unwrap()
            .values()
            .map(|s| s.info())
            .collect()
    }

    pub fn drain_all(&self) -> Vec<Session> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.drain().map(|(_, s)| s).collect()
    }
}
