#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(material_color_texture, material_color_sampler, mesh.uv);
    let final_color = material_color * tex_color;
    
    // Discard pixels below a certain alpha threshold
    if (final_color.a < 0.01) {
        discard;
    }
    
    return final_color;
}