use bevy::prelude::*;
use bevy::render::*;
use bevy::render::settings::*;
use bevy::prelude::{Plugin, Window};
use bevy::window::WindowResolution;

pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

pub struct CustomPlugin;

impl Plugin for CustomPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends:Some(Backends::DX12),
                                ..default()
               			}),
                        synchronous_pipeline_compilation: true,
        })
        .set(WindowPlugin{
            primary_window: Some(Window{
                title: "Snake Game".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
}