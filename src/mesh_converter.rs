use macroquad::prelude::*;
use gltf::{self, mesh::util::ReadTexCoords};

pub async fn convert_to_macroquad_mesh(gltf_mesh: gltf::mesh::Mesh<'_>, buffers: Vec<gltf::buffer::Data>, texture_data: &[u8], scale: f32, world_position: Vec3) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    for primitive in gltf_mesh.primitives() {
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
        let positions = reader.read_positions();
        let tex_coords = reader.read_tex_coords(0);
                
        // Access material and its color properties
        let material_color = primitive.material().pbr_metallic_roughness().base_color_factor();
        let vertex_color = Color::new(material_color[0], material_color[1], material_color[2], material_color[3]);
        
        if let Some(positions) = positions {
            // Handle texture data
            let uvs = match tex_coords {
                Some(ReadTexCoords::U8(iter)) => {
                    iter.map(|uv| Vec2::new(uv[0] as f32 / 255.0, uv[1] as f32 / 255.0)).collect::<Vec<_>>()
                },
                Some(ReadTexCoords::U16(iter)) => {
                    iter.map(|uv| Vec2::new(uv[0] as f32 / 65535.0, uv[1] as f32 / 65535.0)).collect::<Vec<_>>()
                },
                Some(ReadTexCoords::F32(iter)) => {
                    iter.map(|uv| Vec2::new(uv[0], uv[1])).collect::<Vec<_>>()
                },
                None => Vec::new(),
            };

            // Fill the macroquad vertex with the gltf vertex information
            for (i, position) in positions.enumerate() {
                let uv = if i < uvs.len() {
                    uvs[i]
                } else {
                    Vec2::new(0.0, 0.0) // Default to (0,0) if UV is missing or mismatched
                };

                vertices.push(Vertex::new(
                    position[0] * scale + world_position[0],
                    position[1] * scale + world_position[1],
                    position[2] * scale + world_position[2],
                    uv.x,
                    uv.y,
                    vertex_color,
                ));
            }
        }

        if let Some(index_data) = reader.read_indices() {
            for index in index_data.into_u32() {
                indices.push(index as u16);
            }
        }
    }

    if texture_data.is_empty() {
        return Mesh {
            vertices,
            indices,
            texture: None,
        };
    }
    
    let texture = Texture2D::from_file_with_format(texture_data, None);
    Mesh {
        vertices,
        indices,
        texture: Some(texture),
    }
}
