use itertools::Itertools;

use crate::core::cache::CacheId;
use crate::core::{error::AppError, files::FileSystem};
use crate::ecs::components::material::model::Material;
use crate::ecs::components::mesh::model::Mesh;
use crate::ecs::components::model::model::ModelVertex;
use crate::graphics::binding::BindGroupLayouts;
use crate::graphics::model::VertexType;
use crate::graphics::pipelines::manager::PipelineManager;
use crate::graphics::pipelines::setup::create_render_pipeline;
use crate::graphics::shaders::manager::ShaderManager;
use crate::graphics::textures::Texture;
use crate::graphics::PrimitiveTopology;
use crate::log_error;
use std::io::{BufReader, Cursor};

use super::components::instance::model::InstanceRaw;
use super::components::material::manager::MaterialManager;
use super::components::mesh::manager::MeshManager;
use super::components::model::manager::ModelManager;
use super::components::model::model::ModelRaw;
use super::components::transform::TransformManager;
use super::systems::render::BufferManager;
use super::traits::{Cache, Vertex};

pub fn setup_pipeline_manager(
    device: &wgpu::Device,
    depth_format: Option<wgpu::TextureFormat>,
    bind_group_layouts: &BindGroupLayouts,
    hdr_format: wgpu::TextureFormat,
) -> Result<PipelineManager, AppError> {
    let mut pipeline_manager = PipelineManager::new();

    let normal_shader = wgpu::ShaderModuleDescriptor {
        label: Some("Normal Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../assets/shaders/normal.wgsl").into()),
    };

    let light_shader = wgpu::ShaderModuleDescriptor {
        label: Some("Light Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../assets/shaders/light.wgsl").into()),
    };

    for topology in [
        PrimitiveTopology::PointList,
        PrimitiveTopology::LineList,
        PrimitiveTopology::LineStrip,
        PrimitiveTopology::TriangleList,
        PrimitiveTopology::TriangleStrip,
    ] {
        let topology_label = topology.label();

        let normal_pipeline_id = CacheId::from(format!("{}_pipeline", topology_label).as_str());
        pipeline_manager.get_or_create(normal_pipeline_id.value(), || {
            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Normal Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &bind_group_layouts.texture_bind_group_layout,
                        &bind_group_layouts.camera_bind_group_layout,
                        &bind_group_layouts.light_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

            Ok(create_render_pipeline(
                &device,
                &render_pipeline_layout,
                hdr_format,
                depth_format,
                &[ModelVertex::desc(), InstanceRaw::desc()],
                topology.to_wgpu_topology(),
                normal_shader.clone(),
            ))
        })?;

        let light_pipeline_id =
            CacheId::from(format!("{}_light_pipeline", topology_label).as_str());
        pipeline_manager.get_or_create(light_pipeline_id.value(), || {
            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Light Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &bind_group_layouts.camera_bind_group_layout,
                        &bind_group_layouts.light_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

            Ok(create_render_pipeline(
                &device,
                &render_pipeline_layout,
                hdr_format,
                depth_format,
                &[ModelVertex::desc()],
                topology.to_wgpu_topology(),
                light_shader.clone(),
            ))
        })?;
    }

    let sky_pipeline_id = CacheId::from("sky_pipeline");
    pipeline_manager.get_or_create(sky_pipeline_id.value(), || {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Sky Pipeline Layout"),
            bind_group_layouts: &[
                &bind_group_layouts.camera_bind_group_layout,
                &bind_group_layouts.environment_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let shader = wgpu::include_wgsl!("../assets/shaders/sky.wgsl");
        Ok(create_render_pipeline(
            &device,
            &layout,
            hdr_format,
            depth_format,
            &[],
            wgpu::PrimitiveTopology::TriangleList,
            shader,
        ))
    })?;

    Ok(pipeline_manager)
}
pub struct ResourceManager {
    pub model_manager: ModelManager,
    pub mesh_manager: MeshManager,
    pub material_manager: MaterialManager,
    pub buffer_manager: BufferManager,
    pub transform_manager: TransformManager,
    pub pipeline_manager: PipelineManager,
    pub shader_manager: ShaderManager,
}
impl ResourceManager {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layouts: &BindGroupLayouts,
        hdr_format: wgpu::TextureFormat,
    ) -> Result<Self, AppError> {
        let pipeline_manager = match setup_pipeline_manager(
            device,
            Some(Texture::DEPTH_FORMAT),
            &bind_group_layouts,
            hdr_format,
        ) {
            Ok(pipelines) => pipelines,
            Err(e) => {
                log_error!("Failed to setup pipeline manager: {:?}", e);
                return Err(e);
            }
        };
        Ok(Self {
            pipeline_manager,
            material_manager: MaterialManager::new(),
            mesh_manager: MeshManager::new(),
            model_manager: ModelManager::new(),
            transform_manager: TransformManager::new(),
            buffer_manager: BufferManager::new(),
            shader_manager: ShaderManager::new(),
        })
    }
}
pub async fn load_texture(
    file_name: &str,
    is_normal_map: bool,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<Texture, AppError> {
    let data = FileSystem::load_binary(file_name)?;
    Texture::from_bytes(device, queue, &data, file_name, is_normal_map)
}
pub async fn load_model<'a>(
    file_name: &str,
    cache_id: &u64,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    buffer_manager: &'a mut BufferManager,
) -> Result<ModelRaw, AppError> {
    let obj_text = FileSystem::load_string(file_name)?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);
    let (models, obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            let mat_text =
                FileSystem::load_string(&p).expect("load_model::FileSystem::load_string");
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
    .await?;

    let mut materials = Vec::new();
    for m in obj_materials? {
        let diffuse_texture = load_texture(&m.diffuse_texture, false, device, queue).await?;
        let normal_texture = load_texture(&m.normal_texture, true, device, queue).await?;
        materials.push(Material::new(
            device,
            &m.name,
            diffuse_texture,
            normal_texture,
            layout,
        )?);
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let mut vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| {
                    VertexType::Modeled(ModelVertex {
                        position: [
                            m.mesh.positions[i * 3],
                            m.mesh.positions[i * 3 + 1],
                            m.mesh.positions[i * 3 + 2],
                        ],
                        tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                        normal: [
                            m.mesh.normals[i * 3],
                            m.mesh.normals[i * 3 + 1],
                            m.mesh.normals[i * 3 + 2],
                        ],
                        tangent: [0.0; 3],
                        bitangent: [0.0; 3],
                    })
                })
                .collect::<Vec<_>>();

            let indices = m.mesh.indices.iter().map(|&i| i as u16).collect_vec();

            calculate_tangents_and_bitangents(&mut vertices, &indices);

            buffer_manager.create_vertex_buffer(device, &vertices, *cache_id);

            buffer_manager.create_index_buffer(device, &indices, *cache_id);

            Mesh {
                name: file_name.to_string(),
                num_elements: indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();

    Ok(ModelRaw { meshes, materials })
}
fn calculate_tangents_and_bitangents(vertices: &mut Vec<VertexType>, indices: &[u16]) {
    let mut triangles_included = vec![0; vertices.len()];

    for c in indices.chunks(3) {
        let v0 = match &vertices[c[0] as usize] {
            VertexType::Modeled(v) => v,
            _ => continue,
        };
        let v1 = match &vertices[c[1] as usize] {
            VertexType::Modeled(v) => v,
            _ => continue,
        };
        let v2 = match &vertices[c[2] as usize] {
            VertexType::Modeled(v) => v,
            _ => continue,
        };

        let pos0: cgmath::Vector3<_> = v0.position.into();
        let pos1: cgmath::Vector3<_> = v1.position.into();
        let pos2: cgmath::Vector3<_> = v2.position.into();

        let uv0: cgmath::Vector2<_> = v0.tex_coords.into();
        let uv1: cgmath::Vector2<_> = v1.tex_coords.into();
        let uv2: cgmath::Vector2<_> = v2.tex_coords.into();

        let delta_pos1 = pos1 - pos0;
        let delta_pos2 = pos2 - pos0;

        let delta_uv1 = uv1 - uv0;
        let delta_uv2 = uv2 - uv0;

        let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
        let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
        let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * -r;

        if let VertexType::Modeled(v) = &mut vertices[c[0] as usize] {
            v.tangent = (cgmath::Vector3::from(v.tangent) + tangent).into();
            v.bitangent = (cgmath::Vector3::from(v.bitangent) + bitangent).into();
        }
        if let VertexType::Modeled(v) = &mut vertices[c[1] as usize] {
            v.tangent = (cgmath::Vector3::from(v.tangent) + tangent).into();
            v.bitangent = (cgmath::Vector3::from(v.bitangent) + bitangent).into();
        }
        if let VertexType::Modeled(v) = &mut vertices[c[2] as usize] {
            v.tangent = (cgmath::Vector3::from(v.tangent) + tangent).into();
            v.bitangent = (cgmath::Vector3::from(v.bitangent) + bitangent).into();
        }

        triangles_included[c[0] as usize] += 1;
        triangles_included[c[1] as usize] += 1;
        triangles_included[c[2] as usize] += 1;
    }

    for (i, count) in triangles_included.into_iter().enumerate() {
        if count > 0 {
            if let VertexType::Modeled(v) = &mut vertices[i] {
                let denom = 1.0 / count as f32;
                v.tangent = (cgmath::Vector3::from(v.tangent) * denom).into();
                v.bitangent = (cgmath::Vector3::from(v.bitangent) * denom).into();
            }
        }
    }
}

// pub async fn load_model(
//     file_name: &str,
//     device: &wgpu::Device,
//     queue: &wgpu::Queue,
//     layout: &wgpu::BindGroupLayout,
// ) -> Result<Model, AppError> {
//     log_info!("Loading model from file: {:?}", file_name);
//     let obj_text = FileSystem::load_string(file_name)?;
//     log_info!("obj_text");
//     let obj_cursor = Cursor::new(obj_text);
//     log_info!("obj_cursor");
//     let mut obj_reader = BufReader::new(obj_cursor);
//     log_info!("obj_reader");
//     let (models, obj_materials) = tobj::load_obj_buf_async(
//         &mut obj_reader,
//         &tobj::LoadOptions {
//             triangulate: true,
//             single_index: true,
//             ..Default::default()
//         },
//         |p| async move {
//             let mat_text =
//                 FileSystem::load_string(&p).expect("load_model::FileSystem::load_string");
//             tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
//         },
//     )
//     .await?;

//     let mut materials = Vec::new();
//     for m in obj_materials? {
//         log_info!("Loading m.diffuse_texture: {:?}", m.diffuse_texture);
//         log_info!("Loading m.normal_texture: {:?}", m.normal_texture);
//         let diffuse_texture = load_texture(&m.diffuse_texture, false, device, queue).await?;
//         let normal_texture = load_texture(&m.normal_texture, true, device, queue).await?;

//         materials.push(Material::new(
//             device,
//             &m.name,
//             diffuse_texture,
//             normal_texture,
//             layout,
//         ));
//     }

//     let meshes = models
//         .into_iter()
//         .map(|m| {
//             let mut vertices = (0..m.mesh.positions.len() / 3)
//                 .map(|i| ModelVertex {
//                     position: [
//                         m.mesh.positions[i * 3],
//                         m.mesh.positions[i * 3 + 1],
//                         m.mesh.positions[i * 3 + 2],
//                     ],
//                     tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
//                     normal: [
//                         m.mesh.normals[i * 3],
//                         m.mesh.normals[i * 3 + 1],
//                         m.mesh.normals[i * 3 + 2],
//                     ],
//                     // We'll calculate these later
//                     tangent: [0.0; 3],
//                     bitangent: [0.0; 3],
//                 })
//                 .collect::<Vec<_>>();

//             let indices = &m.mesh.indices;
//             let mut triangles_included = vec![0; vertices.len()];
//             for c in indices.chunks(3) {
//                 let v0 = vertices[c[0] as usize];
//                 let v1 = vertices[c[1] as usize];
//                 let v2 = vertices[c[2] as usize];

//                 let pos0: cgmath::Vector3<_> = v0.position.into();
//                 let pos1: cgmath::Vector3<_> = v1.position.into();
//                 let pos2: cgmath::Vector3<_> = v2.position.into();

//                 let uv0: cgmath::Vector2<_> = v0.tex_coords.into();
//                 let uv1: cgmath::Vector2<_> = v1.tex_coords.into();
//                 let uv2: cgmath::Vector2<_> = v2.tex_coords.into();

//                 let delta_pos1 = pos1 - pos0;
//                 let delta_pos2 = pos2 - pos0;

//                 let delta_uv1 = uv1 - uv0;
//                 let delta_uv2 = uv2 - uv0;

//                 let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
//                 let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;

//                 let bitangent = (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * -r;

//                 vertices[c[0] as usize].tangent =
//                     (tangent + cgmath::Vector3::from(vertices[c[0] as usize].tangent)).into();
//                 vertices[c[1] as usize].tangent =
//                     (tangent + cgmath::Vector3::from(vertices[c[1] as usize].tangent)).into();
//                 vertices[c[2] as usize].tangent =
//                     (tangent + cgmath::Vector3::from(vertices[c[2] as usize].tangent)).into();
//                 vertices[c[0] as usize].bitangent =
//                     (bitangent + cgmath::Vector3::from(vertices[c[0] as usize].bitangent)).into();
//                 vertices[c[1] as usize].bitangent =
//                     (bitangent + cgmath::Vector3::from(vertices[c[1] as usize].bitangent)).into();
//                 vertices[c[2] as usize].bitangent =
//                     (bitangent + cgmath::Vector3::from(vertices[c[2] as usize].bitangent)).into();

//                 triangles_included[c[0] as usize] += 1;
//                 triangles_included[c[1] as usize] += 1;
//                 triangles_included[c[2] as usize] += 1;
//             }

//             for (i, n) in triangles_included.into_iter().enumerate() {
//                 let denom = 1.0 / n as f32;
//                 let v = &mut vertices[i];
//                 v.tangent = (cgmath::Vector3::from(v.tangent) * denom).into();
//                 v.bitangent = (cgmath::Vector3::from(v.bitangent) * denom).into();
//             }
//             let vertex_buffer = BufferFactory::create_buffer(
//                 device,
//                 &vertices,
//                 wgpu::BufferUsages::VERTEX,
//                 &format!("{:?} Vertex Buffer", file_name),
//             );

//             let index_buffer = BufferFactory::create_buffer(
//                 device,
//                 &m.mesh.indices,
//                 wgpu::BufferUsages::INDEX,
//                 &format!("{:?} Index Buffer", file_name),
//             );

//             Mesh {
//                 name: file_name.to_string(),
//                 vertex_buffer,
//                 index_buffer,
//                 num_elements: m.mesh.indices.len() as u32,
//                 material: m.mesh.material_id.unwrap_or(0),
//             }
//         })
//         .collect::<Vec<_>>();

//     Ok(Model { meshes, materials })
// }
