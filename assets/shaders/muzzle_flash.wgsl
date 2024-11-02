#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(material_color_texture, material_color_sampler, mesh.uv);

    let center = vec2(0.5, 0.5);
    let dist_from_center = distance(mesh.uv, center);

    let brightness = dot(tex_color.rgb, vec3(0.299, 0.587, 0.114));

    var final_alpha = tex_color.a;
    if (dist_from_center < 0.3) { 
        // Make the center brighter but keep the original alpha
        let flash_alpha = smoothstep(0.0, 0.3, brightness); 
        final_alpha = flash_alpha * material_color.a;
    }

    let final_color = vec4(material_color.rgb * tex_color.rgb, final_alpha);

    if (final_color.a < 0.01) {
        discard;
    }

    return final_color;
}
