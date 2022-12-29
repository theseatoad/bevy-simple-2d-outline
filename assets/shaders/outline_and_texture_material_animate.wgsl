#import bevy_pbr::mesh_types
// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
 #import bevy_pbr::mesh_view_bindings
struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
    @builtin(position) coord : vec4<f32>
) -> @location(0) vec4<f32> {    
    var outline : f32 = textureSample(base_color_texture, base_color_sampler,uv + vec2<f32>(0.01,0.0)).a;
    outline += textureSample(base_color_texture, base_color_sampler,uv + vec2<f32>(-0.01,0.0)).a;
    outline += textureSample(base_color_texture, base_color_sampler,uv + vec2<f32>(0.0,0.01)).a;
    outline += textureSample(base_color_texture, base_color_sampler,uv + vec2<f32>(0.0,-0.01)).a;
    outline += textureSample(base_color_texture, base_color_sampler,uv + vec2<f32>(0.01,-0.01)).a;
    outline += textureSample(base_color_texture, base_color_sampler,uv + vec2<f32>(-0.01,0.01)).a;
    outline += textureSample(base_color_texture, base_color_sampler,uv + vec2<f32>(0.01,0.01)).a;
    outline += textureSample(base_color_texture, base_color_sampler,uv + vec2<f32>(-0.01,-0.01)).a;
    outline = min(outline, 1.0);
    let t_2 = cos(globals.time);
    var color : vec4<f32> = textureSample(base_color_texture, base_color_sampler,uv);
    return mix(color, material.color, outline - color.a);
}