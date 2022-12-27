use bevy::{
    asset::{AssetServer, Assets},
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    utils::default,
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f139fdae-d598-45ac-8225-97e2a3f056e0"]
pub struct OutlineAndTextureMaterial {
    #[uniform(0)]
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
}
impl Material2d for OutlineAndTextureMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline_and_texture_material.wgsl".into()
    }
}