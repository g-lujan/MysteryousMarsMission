use gltf::{buffer::Data, Document};
use macroquad::prelude::*;
use serde::Deserialize;
use std::fs;
use std::{collections::HashMap, io::Read};
use std::error::Error;

use crate::{aabb::AABB, mesh_converter::convert_to_macroquad_mesh};

pub struct StaticActor {
    pub mesh: Mesh,
    pub boundary: Option<AABB>,
}

#[derive(Deserialize)]
struct ActorData {
    pub gltf_path: String,      
    pub texture_path: String,   
    pub position: [f32; 3],     
    pub scale: f32,             
}


// use to load many static actors of the same mesh
pub async fn load_static_actors(file_path: &str) -> Result<Vec<StaticActor>, Box<dyn Error>> {
    let mut file = std::fs::File::open(file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let actor_data: Vec<ActorData> = serde_json::from_str(&data)?;
    let mut gltf_cache: HashMap<String, (Document, Vec<Data>)> = HashMap::new();
    let mut actors: Vec<StaticActor> = Vec::new();
    for data in actor_data {
        let (gltf, buffers) = if let Some(cached) = gltf_cache.get(&data.gltf_path) {
            cached.clone()
        } else {
            let (gltf, buffers, _) = gltf::import(&data.gltf_path).expect("Failed to load GLTF");
            let mesh_and_buffers = (gltf.clone(), buffers.clone());
            gltf_cache.insert(data.gltf_path.clone(), mesh_and_buffers); 
            (gltf, buffers)
        };
        let gltf_mesh = gltf.meshes().next().expect("No meshes found");
        let texture_data = fs::read(&data.texture_path)?;
        let mesh = convert_to_macroquad_mesh(gltf_mesh, buffers, &texture_data, data.scale, Vec3::new(data.position[0], data.position[1], data.position[2])).await;
        actors.push(StaticActor { mesh, boundary: None });
    }

    Ok(actors)
}