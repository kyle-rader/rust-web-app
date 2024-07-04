use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorLobby {
    #[error("Lobby not found")]
    NotFound,

    #[error("Internal server error")]
    Internal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Visibility {
    Public,
    Friends,
    Private,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lobby {
    pub id: i32,
    pub name: String,
    pub visibility: Visibility,
    pub created_at: u64,
}

#[derive(Debug, Deserialize)]
pub struct LobbyForCreate {
    pub name: String,
    pub visibility: Visibility,
}

// region: Lobby Controller
#[derive(Debug, Clone)]
pub struct LobbyController {
    lobbies: Arc<Mutex<Vec<Option<Lobby>>>>,
}

impl LobbyController {
    pub async fn new() -> Result<Self, ErrorLobby> {
        let lobbies = Arc::default();
        Ok(Self { lobbies })
    }

    pub async fn create_lobby(
        &self,
        LobbyForCreate { name, visibility }: LobbyForCreate,
    ) -> Result<Lobby, ErrorLobby> {
        let mut lobbies = self.lobbies.lock().map_err(|_| ErrorLobby::Internal)?;
        let id = lobbies.len() as i32;
        let lobby = Lobby {
            id,
            name,
            visibility,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|_| {
                    eprintln!(
                        "Ah! Time Travel! The system time appears to be before the Unix epoch!"
                    );
                    ErrorLobby::Internal
                })?
                .as_secs(),
        };

        lobbies.push(Some(lobby.clone()));
        Ok(lobby)
    }

    pub async fn get_lobby(&self, id: i32) -> anyhow::Result<Lobby> {
        let lobbies = self.lobbies.lock().unwrap();
        lobbies[id as usize]
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Lobby not found"))
    }

    pub async fn get_lobbies(&self) -> anyhow::Result<Vec<Lobby>> {
        let lobbies = self.lobbies.lock().unwrap();
        Ok(lobbies.iter().filter_map(|l| l.clone()).collect())
    }

    pub async fn delete_lobby(&self, id: i32) -> anyhow::Result<()> {
        let mut lobbies = self.lobbies.lock().unwrap();
        lobbies[id as usize]
            .take()
            .ok_or_else(|| anyhow::anyhow!("Lobby not found"))?;
        Ok(())
    }
}
