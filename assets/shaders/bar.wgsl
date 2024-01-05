//// The vertex input structure
//struct VertexInput {
//    @location(0) position: vec4<f32>
//    // Include other vertex attributes if necessary
//};
//
//// The vertex output structure
//struct VertexOutput {
//    @builtin(position) position: vec4<f32>
//    // Include other vertex attributes if necessary
//};
//
//@vertex
//fn vertex(input: VertexInput) -> VertexOutput {
//    var output: VertexOutput;
//
//    output.position = input.position + vec4<f32>(0.0, 1.0, 0.0, 0.0);
//    return output;
//}

#import bevy_pbr::{
    mesh_view_bindings::globals,
    mesh_bindings::mesh,
    mesh_functions as mesh_functions,
    view_transformations::position_world_to_clip,
    forward_io::{Vertex,VertexOutput},
}
#import bevy_render::instance_index::get_instance_index

const COLOR_MULTIPLIER: vec4<f32> = vec4<f32>(1.0, 0.0, 0.0, 0.5);

struct FragInput {
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

//@fragment
//fn fragment(
//    in: FragInput,
//) -> @location(0) vec4<f32> {
//    return COLOR_MULTIPLIER;
//}

@vertex
fn vertex(
    vertex: Vertex
) -> VertexOutput {

    var out: VertexOutput;
    var model = mesh_functions::get_model_matrix(vertex.instance_index);
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.world_position = mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.position = position_world_to_clip(out.world_position.xyz);
    return out;
}