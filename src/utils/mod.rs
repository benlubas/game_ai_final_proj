use rlbot_lib::rlbot::{PlayerInput, RenderMessage, ControllerState};

pub mod rl_match;
pub mod arena;
pub mod math;
pub mod render;

pub struct ActionTickResult {
    pub controller: ControllerState,
    pub render: Vec<RenderMessage>,
}

impl ActionTickResult {
    pub fn from(input: ControllerState) -> ActionTickResult {
        ActionTickResult {
            controller: input,
            render: vec![],
        }
    }
}

pub struct AgentTickResult {
    pub input: PlayerInput,
    pub render: Vec<RenderMessage>,
}

impl AgentTickResult {
    pub fn from(input: PlayerInput) -> AgentTickResult {
        AgentTickResult {
            input,
            render: vec![],
        }
    }
}
