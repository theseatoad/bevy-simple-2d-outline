//! 2D outline materials written in wgsl for the bevy game engine.
use bevy::prelude::*;
use bevy::{
    reflect::{TypePath},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
/// Renders only the outline of the given texture.
/// # Example
///```
///fn main() {
///   App::new()
///        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
///        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
///        .add_plugin(Material2dPlugin::<OutlineMaterial>::default())
///        .add_startup_system(setup)
///        .run();
/// }
///
///fn setup(
///    mut commands: Commands,
///    mut meshes: ResMut<Assets<Mesh>>,
///    mut materials: ResMut<Assets<OutlineMaterial>>,
///    asset_server: Res<AssetServer>,
///) {
///    commands.spawn(MaterialMesh2dBundle {
///        mesh: meshes.add(Rectangle::default()).into(),
///        transform: Transform::default().with_scale(Vec3::splat(128.)),
///        material: materials.add(OutlineMaterial {
///            color: Color::BLUE,
///            thickness : 0.01,
///            texture: asset_server.load("textures/sprite_seatoad.png"),
///        }),
///        ..default()
///    });
///
///    commands.spawn(Camera2dBundle::default());
///}
///```
#[derive(AsBindGroup, TypePath, Asset, Debug, Clone)]
pub struct OutlineMaterial {
    /// The color of the outline.
    #[uniform(0)]
    pub color: Color,
    /// The thickness of the outline. Preferred values between 0.01 and 0.005.
    #[uniform(0)]
    pub thickness: f32,
    /// The texture to outline.
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for OutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline_material.wgsl".into()
    }
}
/// Renders an outline and the texture.
/// # Example
///```
///fn main() {
///   App::new()
///        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
///        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
///        .add_plugin(Material2dPlugin::<OutlineAndTextureMaterial>::default())
///        .add_startup_system(setup)
///        .run();
/// }
///
///fn setup(
///    mut commands: Commands,
///    mut meshes: ResMut<Assets<Mesh>>,
///    mut materials: ResMut<Assets<OutlineAndTextureMaterial>>,
///    asset_server: Res<AssetServer>,
///) {
///    commands.spawn(MaterialMesh2dBundle {
///        mesh: meshes.add(Rectangle::default()).into(),
///        transform: Transform::default().with_scale(Vec3::splat(128.)),
///        material: materials.add(OutlineAndTextureMaterial {
///            color: Color::BLUE,
///            thickness : 0.01,
///            texture: asset_server.load("textures/sprite_seatoad.png"),
///        }),
///        ..default()
///    });
///
///    commands.spawn(Camera2dBundle::default());
///}
///```
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct OutlineAndTextureMaterial {
    /// The color of the outline.
    #[uniform(0)]
    pub color: Color,
    /// The thickness of the outline. Preferred values between 0.01 and 0.005.
    #[uniform(0)]
    pub thickness: f32,
    /// The texture to outline.
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}
impl Material2d for OutlineAndTextureMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline_and_texture_material.wgsl".into()
    }
}
/// Renders only an animated rainbow outline.
/// # Example
///```
/// fn main() {
///       App::new()
///          .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
///         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
///         .add_plugin(Material2dPlugin::<RainbowOutlineMaterial>::default())
///         .add_startup_system(setup)
///         .run();
/// }
///
/// fn setup(
///     mut commands: Commands,
///     mut meshes: ResMut<Assets<Mesh>>,
///     mut materials: ResMut<Assets<RainbowOutlineMaterial>>,
///     asset_server: Res<AssetServer>,
/// ) {
///     commands.spawn(MaterialMesh2dBundle {
///         mesh: meshes.add(Rectangle::default()).into(),
///         transform: Transform::default().with_scale(Vec3::splat(128.)),
///         material: materials.add(RainbowOutlineMaterial {
///            thickness : 0.01,
///            frequency : 1.25,
///            texture: asset_server.load("textures/sprite_seatoad.png"),
///        }),
///        ..default()
///    });
///
///     commands.spawn(Camera2dBundle::default());
/// }
///```
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct RainbowOutlineMaterial {
    /// The thickness of the outline. Preferred values between 0.01 and 0.005.
    #[uniform(0)]
    pub thickness: f32,
    /// Frequency at which the colors of the rainbow are iterated over.
    #[uniform(0)]
    pub frequency: f32,
    /// The texture to outline.
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}
impl Material2d for RainbowOutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/rainbow_outline_material.wgsl".into()
    }
}
/// Renders an animated rainbow outline and the texture.
/// # Example
///```
/// fn main() {
///       App::new()
///          .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
///         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
///         .add_plugin(Material2dPlugin::<RainbowOutlineAndTextureMaterial>::default())
///         .add_startup_system(setup)
///         .run();
/// }
///
/// fn setup(
///     mut commands: Commands,
///     mut meshes: ResMut<Assets<Mesh>>,
///     mut materials: ResMut<Assets<RainbowOutlineAndTextureMaterial>>,
///     asset_server: Res<AssetServer>,
/// ) {
///     commands.spawn(MaterialMesh2dBundle {
///         mesh: meshes.add(Rectangle::default()).into(),
///         transform: Transform::default().with_scale(Vec3::splat(128.)),
///         material: materials.add(RainbowOutlineAndTextureMaterial {
///            thickness : 0.01,
///            frequency : 1.25,
///            texture: asset_server.load("textures/sprite_seatoad.png"),
///        }),
///        ..default()
///    });
///
///     commands.spawn(Camera2dBundle::default());
/// }
///```
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct RainbowOutlineAndTextureMaterial {
    /// The thickness of the outline. Preferred values between 0.01 and 0.005.
    #[uniform(0)]
    pub thickness: f32,
    /// Frequency at which the colors of the rainbow are iterated over.
    #[uniform(0)]
    pub frequency: f32,
    /// The texture to outline.
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}
impl Material2d for RainbowOutlineAndTextureMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/rainbow_outline_and_texture_material.wgsl".into()
    }
}
