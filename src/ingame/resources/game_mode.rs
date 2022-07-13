use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameMode {
    ClassicMode,
    SurvivalMode,
}