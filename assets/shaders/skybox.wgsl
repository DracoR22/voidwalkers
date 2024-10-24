struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,  // Output clip-space position
    @location(0) v_pos: vec3<f32>,               // Pass through the vertex position to the fragment shader
};

@vertex
fn vertex_main(
    @location(0) in_pos: vec3<f32>               // Input vertex position
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in_pos, 1.0);  // Set the output clip-space position
    out.v_pos = in_pos;                          // Pass the position to the fragment shader
    return out;
}

@group(0) @binding(0) var u_cube_map: texture_cube<f32>;
@group(0) @binding(1) var u_sampler: sampler;

@fragment
fn fragment(
    v_in: VertexOutput
) -> @location(0) vec4<f32> {
    let direction = normalize(v_in.v_pos);        // Normalize for cubemap sampling
    return textureSample(u_cube_map, u_sampler, direction);  // Sample cubemap
}
