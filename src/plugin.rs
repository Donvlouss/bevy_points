use bevy::{prelude::{MaterialPlugin, Plugin,}, asset::embedded_asset};

use crate::prelude::PointsMaterial;

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        embedded_asset!(app, "src\\", "shaders\\points.wgsl");
        app.add_plugins(MaterialPlugin::<PointsMaterial>::default());
    }
}
