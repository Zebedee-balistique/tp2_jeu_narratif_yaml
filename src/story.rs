use serde::Deserialize;
use std::io::{self};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NarratifError {
    CannotLoad,
}

impl From<io::Error> for NarratifError {
    fn from(_err: io::Error) -> Self {
        NarratifError::CannotLoad
    }
}

impl From<serde_yaml::Error> for NarratifError {
    fn from(_err: serde_yaml::Error) -> Self {
        NarratifError::CannotLoad
    }
}

#[derive(Debug, Deserialize)]
pub struct Choix {
    pub label: String,
    pub next: String,
    pub required_item: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Scene {
    pub id: String,
    pub title: String,
    pub text: String,
    pub hp_delta: Option<i16>,
    pub found_item: Option<String>,
    pub choices: Option<Vec<Choix>>,
    pub ending: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Story {
    pub start_scene: String,
    pub initial_hp: u16,
    pub scenes: Vec<Scene>,
}