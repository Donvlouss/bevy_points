use bevy::{
    asset::embedded_asset,
    prelude::{MaterialPlugin, Plugin,}
};

use crate::prelude::PointsMaterial;

pub struct PointsPlugin;

impl Plugin for PointsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[cfg(all(target_family = "windows"))]
        embedded_asset!(app, "src\\", "shaders\\points.wgsl");
        #[cfg(any(not(target_family = "windows")))]
        embedded_asset!(app, "src/", "shaders/points.wgsl");
        app.add_plugins(MaterialPlugin::<PointsMaterial>::default());
    }
}
