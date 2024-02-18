//! Based on the shader_material.rs example at https://github.com/bevyengine/bevy/blob/main/examples/shader/shader_material.rs
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use bevy::{
    asset::{AssetServer, Assets},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    utils::default,
};
use bevy_simple_2d_outline::{RainbowOutlineAndTextureMaterial};

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            Material2dPlugin::<RainbowOutlineAndTextureMaterial>::default()
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RainbowOutlineAndTextureMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(500.)),
        material: materials.add(RainbowOutlineAndTextureMaterial {
            thickness : 0.01,
            frequency : 1.0,
            texture: asset_server.load("textures/sprite_seatoad.png"),
        }),
        ..default()
    });

    commands.spawn(Camera2dBundle::default());
}