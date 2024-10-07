// use std::sync::{Arc, RwLock};

// use crate::pipeline::cache::PipelineCache;

// use super::SceneDescription;

// const TEXTURED_PIPELINE_CACHE_KEY: u64 = 1;
// const UNTEXTURED_PIPELINE_CACHE_KEY: u64 = 0;

// fn rotate(object: &mut Object, delta_time: f64) {
//     object.rotation[1] += 1.0 * delta_time as f32;
// }

// fn scale(object: &mut Object, delta_time: f64) {
//     object.scale[0] += 0.1 * delta_time as f32;
// }

// pub struct Scene {
//     objects: Vec<Object>,
//     camera_position: Vec3,
//     update_strategies: Vec<Box<dyn Fn(&mut Object, f64) + Send + Sync>>,
// }

// impl Scene {
//     pub fn from_description(description: SceneDescription) -> Self {
//         let objects = description
//             .objects
//             .into_iter()
//             .map(|desc| {
//                 Object::new(
//                     desc.position,
//                     desc.rotation,
//                     desc.scale,
//                     desc.mesh,
//                     desc.pipeline_cache_key,
//                 )
//             })
//             .collect();

//         let update_strategies: Vec<Box<dyn Fn(&mut Object, f64) + Send + Sync>> =
//             vec![Box::new(rotate)];

//         Scene {
//             objects,
//             camera_position: description.camera_position,
//             update_strategies,
//         }
//     }

//     pub fn update_scene_objects(&mut self, delta_time: f64) {
//         for object in &mut self.objects {
//             object.update(delta_time, &self.update_strategies);
//         }
//     }

//     pub fn render_scene_objects(
//         &mut self,
//         device: &wgpu::Device,
//         pipeline_cache: &Arc<RwLock<PipelineCache>>,
//         encoder: &mut wgpu::CommandEncoder,
//         output_view: &wgpu::TextureView,
//         render_context: &RenderContext,
//     ) {
//         for object in &mut self.objects {
//             let is_textured = object.mesh.is_textured();
//             let pipeline_cache_key = if is_textured {
//                 TEXTURED_PIPELINE_CACHE_KEY
//             } else {
//                 UNTEXTURED_PIPELINE_CACHE_KEY
//             };

//             // let pipeline = pipeline_cache
//             //     .write()
//             //     .unwrap()
//             //     .get_or_create_pipeline(
//             //         device,
//             //         pipeline_cache_key,
//             //         &render_context.g_bg_layout,
//             //         &render_context.m_bg_layout,
//             //         Some(&render_context.t_bg_layout),
//             //         &render_context.vs,
//             //         &render_context.fs,
//             //         render_context.swf,
//             //     )
//             //     .unwrap();

//             // let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//             //     label: Some("Render Pass"),
//             //     color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//             //         view: output_view,
//             //         resolve_target: None,
//             //         ops: wgpu::Operations {
//             //             load: wgpu::LoadOp::Clear(wgpu::Color {
//             //                 r: 0.1,
//             //                 g: 0.2,
//             //                 b: 0.3,
//             //                 a: 1.0,
//             //             }),
//             //             store: wgpu::StoreOp::Store,
//             //         },
//             //     })],
//             //     depth_stencil_attachment: None,
//             //     timestamp_writes: None,
//             //     occlusion_query_set: None,
//             // });

//             // object.render_object(&mut render_pass, &pipeline, &render_context.g_bg);
//         }
//     }
// }

// fn new_update_strategy(
//     strategy: impl Fn(&mut Object, f32) + Send + Sync + 'static,
// ) -> Box<dyn Fn(&mut Object, f32) + Send + Sync> {
//     Box::new(strategy)
// }
