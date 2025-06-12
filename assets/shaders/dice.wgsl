#import bevy_pbr::{
  mesh_functions,
  view_transformations::position_world_to_clip,
}

@group(2) @binding(0) var texture0: texture_2d<f32>;
@group(2) @binding(1) var sampler0: sampler;
@group(2) @binding(2) var texture1: texture_2d<f32>;
@group(2) @binding(3) var sampler1: sampler;
@group(2) @binding(4) var texture2: texture_2d<f32>;
@group(2) @binding(5) var sampler2: sampler;
@group(2) @binding(6) var texture3: texture_2d<f32>;
@group(2) @binding(7) var sampler3: sampler;
@group(2) @binding(8) var texture4: texture_2d<f32>;
@group(2) @binding(9) var sampler4: sampler;
@group(2) @binding(10) var texture5: texture_2d<f32>;
@group(2) @binding(11) var sampler5: sampler;

struct VertexInput {
  @builtin(instance_index) instance_index: u32,
  @location(0) position: vec3<f32>,
  @location(1) normals: vec3<f32>,
  @location(2) uv: vec2<f32>,
  @location(3) face_index: u32,
};

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(2) uv: vec2<f32>,
  @location(3) @interpolate(flat) face_index: u32,
};

fn sample_texture(index: u32, uv: vec2<f32>) -> vec4<f32> {
  switch index {
    case 0u: { return textureSample(texture0, sampler0, uv); }
    case 1u: { return textureSample(texture1, sampler1, uv); }
    case 2u: { return textureSample(texture2, sampler2, uv); }
    case 3u: { return textureSample(texture3, sampler3, uv); }
    case 4u: { return textureSample(texture4, sampler4, uv); }
    case 5u: { return textureSample(texture5, sampler5, uv); }
    default: { return vec4<f32>(1.0, 0.0, 1.0, 1.0); } // Magenta = error
  }
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
  var vertex_output: VertexOutput;
  let position = vec4<f32>(
      in.position,
      1.0
  );
    
  var world_from_local = mesh_functions::get_world_from_local(in.instance_index);
  var world_position = mesh_functions::mesh_position_local_to_world(world_from_local, position);
  vertex_output.position = position_world_to_clip(world_position.xyz);
  vertex_output.uv = in.uv;
  vertex_output.face_index = in.face_index;
  return vertex_output;
}

@fragment
fn fragment(
  in: VertexOutput,
  @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
  let color = sample_texture(in.face_index, in.uv);
  return color;
}
