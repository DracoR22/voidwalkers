use bevy::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum GlockAnimationsList {
    IDLE,
    WALK,
    SHOOT,
    RELOADFAST,
    RELOADFULL
}

impl Default for GlockAnimationsList {
    fn default() -> Self {
        Self::IDLE
    }
}

impl From<&KeyCode> for GlockAnimationsList {
    fn from(key_code: &KeyCode) -> Self {
        match key_code {
            KeyCode::KeyW => GlockAnimationsList::WALK,
            KeyCode::KeyA => GlockAnimationsList::WALK,
            KeyCode::KeyS => GlockAnimationsList::WALK,
            KeyCode::KeyD => GlockAnimationsList::WALK,
            KeyCode::KeyR => GlockAnimationsList::RELOADFULL,

            _ => GlockAnimationsList::IDLE,
        }
    }
}

pub fn setup_glock_animations(mut commands: Commands) {

}