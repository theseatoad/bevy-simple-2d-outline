struct OutlineAndTextureMaterial {
    color: vec4<f32>,
    thickness : f32
};
@group(1) @binding(0)
var<uniform> material: OutlineAndTextureMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

fn get_sample(
    probe: vec2<f32>
) -> f32 {
    return textureSample(base_color_texture, base_color_sampler, probe).a;
}

#import bevy_pbr::forward_io::VertexOutput
@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    var uv = in.uv;
    var outline : f32 = get_sample(uv + vec2<f32>(material.thickness,0.0));
    outline += get_sample(uv + vec2<f32>(-material.thickness,0.0));
    outline += get_sample(uv + vec2<f32>(0.0,material.thickness));
    outline += get_sample(uv + vec2<f32>(0.0,-material.thickness));
    outline += get_sample(uv + vec2<f32>(material.thickness,-material.thickness));
    outline += get_sample(uv + vec2<f32>(-material.thickness,material.thickness));
    outline += get_sample(uv + vec2<f32>(material.thickness,material.thickness));
    outline += get_sample(uv + vec2<f32>(-material.thickness,-material.thickness));
    outline = min(outline, 1.0);
    var color : vec4<f32> = textureSample(base_color_texture, base_color_sampler,uv);
    return mix(color, material.color, outline - color.a);
}