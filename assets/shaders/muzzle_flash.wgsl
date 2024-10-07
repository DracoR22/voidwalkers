#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the texture color and alpha
    let tex_color = textureSample(material_color_texture, material_color_sampler, mesh.uv);

    // Calculate the distance from the center of the texture (normalized UV coordinates)
    let center = vec2(0.5, 0.5);
    let dist_from_center = distance(mesh.uv, center);

    // Calculate brightness (grayscale approximation)
    let brightness = dot(tex_color.rgb, vec3(0.299, 0.587, 0.114));

    // Apply the effect only to the center region
    var final_alpha = tex_color.a;
    if (dist_from_center < 0.3) { // Only modify within a radius of 0.3 from the center
        // Make the center brighter but keep the original alpha
        let flash_alpha = smoothstep(0.0, 0.3, brightness); // Adjust alpha based on brightness
        final_alpha = flash_alpha * material_color.a;
    }

    // Combine material color with texture color, apply the adjusted alpha
    let final_color = vec4(material_color.rgb * tex_color.rgb, final_alpha);

    // Discard fully transparent pixels
    if (final_color.a < 0.01) {
        discard;
    }

    return final_color;
}
