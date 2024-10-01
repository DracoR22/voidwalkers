use bevy::prelude::*;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
pub struct GameFpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
pub struct GameFPSText;

pub fn write_game_fps(commands: &mut Commands) -> Entity {
    let style = TextStyle::default();

    let text_fps = commands.spawn((
        GameFPSText,
        TextBundle {
            text: Text::from_sections([
                TextSection {
                    value: "fps: ".into(),
                    style: style.clone(),
                },
                TextSection {
                    value: " N/A".into(),
                    style: style.clone(),
                },
            ]),
            ..Default::default()
        },
    )).id();

   return text_fps
}

pub fn update_game_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<GameFPSText>>,) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.sections[1].value = format!("{value:>4.0}");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            text.sections[1].style.color = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::rgb(
                    (1.0 - (value - 60.0) / (120.0 - 60.0)) as f32,
                    1.0,
                    0.0,
                )
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::rgb(
                    1.0,
                    ((value - 30.0) / (60.0 - 30.0)) as f32,
                    0.0,
                )
            } else {
                // Below 30 FPS, use red color
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            // display "N/A" if we can't get a FPS measurement
            // add an extra space to preserve alignment
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}