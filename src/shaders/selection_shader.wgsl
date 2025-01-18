// vertex shader

//THIS IS FOR LOGARITHMIC Z FIGHTING THROBLES
   //gl_Position.z = log(gl_Position.w*C + 1)/log(far*C + 1);
    //gl_Position.z *= gl_Position.w;
const C:f32=0.001;
const FAR:f32=20000.0;

const PI:f32= 3.14159265358979323846;

const vx:vec3<f32>=vec3<f32>(1.0,0.0,0.0);
const vy:vec3<f32>=vec3<f32>(0.0,1.0,0.0);
const vz:vec3<f32>=vec3<f32>(0.0,0.0,1.0);
const def_color =vec4<f32>(0.1,0.1,0.1,1.0);

const dorn_id:i32=200;
const r_arrow_x_id:i32=201;

struct VertexInput {
    @location(0) position: vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) id: i32,
    @location(3) tex_coords: vec2<f32>,
};

struct Camera {
    mvp : mat4x4<f32>,
    n_matrix : mat4x4<f32>,
    forward_dir:vec3<f32>,
};
@binding(0) @group(0) var<uniform> camera : Camera;

struct CameraUniforms {
    light_position : vec4<f32>,
    eye_position : vec4<f32>,
    resolution : vec4<f32>
};
@binding(1) @group(0) var<uniform> camera_uniforms : CameraUniforms;

struct LightUniforms {
    color : vec4<f32>,
    specular_color : vec4<f32>,
    ambient_intensity: f32,
    diffuse_intensity :f32,
    specular_intensity: f32,
    specular_shininess: f32
};
@binding(2) @group(0) var<uniform> light_uniformsArray: array<LightUniforms, 140>;

@binding(3) @group(0) var<uniform> vertex_meta_data0 : array<vec4<i32>, 256>;

@binding(4) @group(0) var<uniform> dorn_scale : mat4x4<f32>;

@binding(5) @group(0) var<uniform> dorn_translate : mat4x4<f32>;






struct Output {
    @builtin(position) position : vec4<f32>,
    @location(0) @interpolate(flat) id : vec4<i32>,
};


@vertex
fn vs_main(@builtin(vertex_index) vertex_index : u32,in:VertexInput) -> Output {

    var output: Output;

    switch in.id {
        case dorn_id: {
            output.position = camera.mvp *dorn_translate * dorn_scale * in.position;
        }

        default: {
            output.position = camera.mvp  * in.position;
        }
      }

    //
    output.position.z = log(output.position.w*C + 1)/log(FAR*C + 1);
    output.position.z *= output.position.w;

    output.id =vec4<i32> (in.id,0,0,0);
    //
    return output;
}

@fragment
fn fs_main(in:Output) ->  @location(0) vec4<i32> {
   return in.id;
   //return vec4<i32>(5,4,3,2);
}

