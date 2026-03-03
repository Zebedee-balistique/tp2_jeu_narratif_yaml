use crate::story::{Story};


pub trait GameCommand {
    fn execute(
        &self,
        scenario: &Story,
        state: &mut GameState,
    ) -> Result<CommandOutcome, GameError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameError {
    ExecutionError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutcome {
    hp_lost: u16,
    new_id: String,
    item: Option<String>,
    display_message: String,
}

pub struct GameState {
    pub running: bool,
    pub current_id: String,
    pub current_hp: u16,
    pub items: Vec<String>,
}

pub struct LookCommand {
    
}

impl GameCommand for LookCommand {
    fn execute(&self, scenario: &Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        let ID: String = state.current_id.clone();
        for scene in &scenario.scenes {
            if scene.id == ID {
                return Ok(CommandOutcome { hp_lost: 0, new_id: ID, display_message: scene.text.clone(), item: None });
            }
        }
        return Err(GameError::ExecutionError)
    }
}

pub struct ChooseCommand {
    
}

impl GameCommand for ChooseCommand {
    fn execute(&self, scenario: &Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        let ID: String = state.current_id.clone();
        for scene in &scenario.scenes {
            if scene.id == ID {
                match &scene.choices {
                    Some(choices) => {
                        let mut text = String::new();
                        for choix in choices {
                            text.push_str(&choix.label);
                            text.push_str(", ");
                        }
                        if !text.is_empty() {
                            text.truncate(text.len() - 2);
                        }
                        return Ok(CommandOutcome { hp_lost: 0, new_id: ID, display_message: text, item: None});
                    }
                    None => {return Ok(CommandOutcome {hp_lost:0, new_id: ID, display_message: "No choice".to_string(), item: None});}
                }
            }
        }
        Err(GameError::ExecutionError)
    }
}

pub struct InventoryCommand {
    
}

impl GameCommand for InventoryCommand {
    fn execute(&self, scenario: &Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        if !&state.items.is_empty() {
            let mut text = String::new();
                for item in &state.items {
                            text.push_str(&item);
                            text.push_str(", ");
                        }
                        if !text.is_empty() {
                            text.truncate(text.len() - 2);
                        }
                        return Ok(CommandOutcome { hp_lost: 0, new_id: state.current_id.clone(), display_message: text, item: None});
        }
        Ok(CommandOutcome { hp_lost: 0, new_id: state.current_id.clone(), display_message: "Inventaire vide".to_string(), item: None})
    }
}


pub struct StatusCommand {
    
}

impl GameCommand for StatusCommand {
    fn execute(&self, scenario: &Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        let ID: String = state.current_id.clone();
        for scene in &scenario.scenes {
            if scene.id == ID {
                return Ok(CommandOutcome { hp_lost: 0, new_id: ID, display_message: scene.title.clone(), item: None});
                }
        }
        Err(GameError::ExecutionError)   
    }  
}


pub struct QuitCommand {
}

impl GameCommand for QuitCommand {
    fn execute(&self, scenario: &Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        Ok(CommandOutcome{ hp_lost: u16::MAX, new_id: state.current_id.clone(), display_message: "".to_string(), item: None})
    }
}
