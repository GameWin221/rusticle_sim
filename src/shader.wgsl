struct PushConstants {
    proj_view: mat4x4<f32>,
};

struct ColorsBuffer {
    colors: array<vec4<f32>, 32>,
};

@group(0) @binding(0)
var<uniform> colors: ColorsBuffer;
var<push_constant> pc: PushConstants;

struct VertexInput {
    @location(0) vpos: vec2<f32>
};

struct InstanceInput {
    @location(1) position: vec2<f32>,
    @location(2) radius: f32,
    @location(3) color_id: u32,
};

struct VertexOutput {
    @builtin(position) fpos: vec4<f32>,

    @location(0) fuv: vec2<f32>,
    @location(1) fcolor: vec3<f32>
};

@vertex
fn vs_main(vert: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    out.fpos = pc.proj_view * vec4<f32>(vert.vpos * instance.radius + instance.position, 0.0, 1.0);
    out.fuv = vert.vpos*2.0;
    out.fcolor = colors.colors[instance.color_id].xyz;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let circle = smoothstep(1.0, 0.0, length(in.fuv));
    
    return vec4<f32>(in.fcolor, circle);
}