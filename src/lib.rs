use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f139fdae-d598-45ac-8225-97e2a3f056e0"]
pub struct OutlineAndTextureMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub thickness : f32,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
}
impl Material2d for OutlineAndTextureMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline_and_texture_material.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f139fdae-d598-45ac-8225-97e2a3f011e0"]
pub struct OutlineAndTextureMaterialAnimate {
    #[uniform(0)]
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,
}
impl Material2d for OutlineAndTextureMaterialAnimate {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline_and_texture_material_animate.wgsl".into()
    }
}