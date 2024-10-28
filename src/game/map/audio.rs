use bevy::prelude::*;

use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioSource;

use std::ops::Index;

use crate::game::map::audio::MapAudioList::DoorOpen;

#[derive(Resource, Default)]
pub struct MapAudios(pub Vec<Handle<AudioSource>>);

pub fn setup_map_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(MapAudios(vec![
        asset_server.load("audio/Door_Open.ogg"),
    ]));
}

#[derive(PartialEq, Clone, Copy)]
pub enum MapAudioList {
    DoorOpen
}

impl Default for MapAudioList {
    fn default() -> Self {
        self::DoorOpen
    }
}

impl Index<MapAudioList> for MapAudios {
    type Output = Handle<AudioSource>;

    fn index(&self, index: MapAudioList) -> &Self::Output {
        &self.0[index as usize]
    }
}

pub fn play_map_audio() {
    
}
