use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use crate::story::{Story, NarratifError};
use crate::game::{GameCommand, LookCommand, ChooseCommand, InventoryCommand, StatusCommand, QuitCommand};


pub mod story;
pub mod game;

pub enum ParseError {
    EmptyInput, 
    UnknownCommand,
}

impl Story {

    pub fn new() -> Story {
        Self {
            start_scene: "".to_string(),
            initial_hp: 0,
            scenes: Vec::new(),
        }
    }

    pub fn load_story(path: &str) -> Result<Self, NarratifError>{
        let file = File::open(path)?;
        
        let reader = BufReader::new(file);
        let story = serde_yaml::from_reader(reader)?;
        Ok(story)
    }

    fn scene_check(&self, scene: &str) -> bool {
        for scenet in &self.scenes {
            if scenet.title == scene {
                return true;
            }
        }
        return false;
    }

    pub fn story_check(&self) -> bool {
        // Vérification de la première scène
        if !&self.scene_check(&self.start_scene) {
            return false;
        }

        // Vérification de l'unicité

        let mut scene_evaluees = HashSet::new();

        for scene in &self.scenes {
            if scene_evaluees.contains(&scene.id){
                return false;
            }
            scene_evaluees.insert(scene.id.clone());
        }

        // Vérification des choix

        for scene in &self.scenes {
            match &scene.choices {
                Some(choices) => {
                    for choix in choices {
                        if !scene_evaluees.contains(&choix.next){
                            return false;
                        }
                    }
                
                }
                None => {}
            }
        }

        return true;
    }
}

pub fn parse_command(line: &str) -> Result<Box<dyn GameCommand>, ParseError> {
    
    let input: Vec<&str> = line.trim().split_whitespace().collect();

    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    match input[0] {
        "look" => {Ok(Box::new(LookCommand {}))},
        "choices" => {Ok(Box::new(ChooseCommand {}))},
        "inventory" => {Ok(Box::new(InventoryCommand {}))},
        "status" => {Ok(Box::new(StatusCommand {}))},
        "quit" => {Ok(Box::new(QuitCommand {}))},
        _ => Err(ParseError::UnknownCommand),
    }
}