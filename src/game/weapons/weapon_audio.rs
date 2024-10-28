use bevy::prelude::*;

use crate::game::weapons::weapon_audio::WeaponAudioList::DryFire;
use bevy_kira_audio::AudioSource;
use std::ops::Index;

use super::WeaponAudios;
pub fn setup_weapon_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(WeaponAudios(vec![
        // general
        asset_server.load("audio/dry_fire.ogg"),
        asset_server.load("audio/bullet-casing-bounce.ogg"),

        // glock
        asset_server.load("audio/glock-fire.ogg"),
        asset_server.load("audio/glock-reload-empty.ogg"),

        // ak74
        asset_server.load("audio/ak74-fire.ogg"),
        asset_server.load("audio/ak74-reload-empty.ogg"),
     ]));
}

#[derive(PartialEq, Clone, Copy)]
pub enum WeaponAudioList {
    DryFire,
    BulletCasing,
    GlockFire,
    GlockReloadFull,
    Ak74Fire,
    Ak74RealoadFull
}

impl Default for WeaponAudioList {
    fn default() -> Self {
        self::DryFire
    }
}

impl Index<WeaponAudioList> for WeaponAudios {
    type Output = Handle<AudioSource>;

    fn index(&self, index: WeaponAudioList) -> &Self::Output {
        &self.0[index as usize]
    }
}
