use macroquad::miniquad::BlendState;
use macroquad::miniquad::BlendValue;
use macroquad::prelude::load_material;
use macroquad::prelude::Material;
use macroquad::prelude::MaterialParams;
use macroquad::prelude::PipelineParams;
use macroquad::prelude::ShaderSource;
use macroquad::prelude::UniformDesc;
use macroquad::prelude::UniformType;
use crate::miniquad::Equation;
use crate::miniquad::BlendFactor;

pub fn get_noise() -> Material {
    let vertex_shader = include_str!("../shaders/noise_vertex.glsl");
    let fragment_shader = include_str!("../shaders/noise_frag.glsl");

    let alpha_blend = BlendState::new(Equation::Add, BlendFactor::Value(BlendValue::SourceAlpha), BlendFactor::OneMinusValue(BlendValue::SourceAlpha));
    let color_blend = BlendState::new(Equation::Add, BlendFactor::Value(BlendValue::SourceAlpha), BlendFactor::OneMinusValue(BlendValue::SourceAlpha));

    let pipeline_params = PipelineParams {
        color_blend: Some(color_blend),
        alpha_blend: Some(alpha_blend),
        ..Default::default()  // Keep the default values for other pipeline parameters
    };



    // Add the uniform descriptions (remove Texture0)
    return load_material(
        ShaderSource::Glsl {
            vertex: vertex_shader,
            fragment: fragment_shader,
        },
        MaterialParams {
            pipeline_params: pipeline_params,
            uniforms: vec![
                UniformDesc {
                    name: "time".to_string(),
                    uniform_type: UniformType::Float1,
                    array_count: 1,
                },
            ],
            ..Default::default() // No textures anymore
        },
    )
    .unwrap();
}