use std::sync::{Arc, PoisonError, RwLockReadGuard};
use thiserror::Error;
use wgpu::{
    BindGroup, BindingResource, Device, Queue, Sampler, TextureView, TextureViewDescriptor,
};

use crate::{
    log_warning,
    prelude::AppError,
    scene::texture::texture::Texture,
    shader::{reflection::ShaderReflection, shader::Shader},
};

pub struct Material {
    pub shader: Arc<Shader>,             // Reference to the shader
    pub bind_group: Option<BindGroup>,   // Bind group to use for rendering
    pub textures: Vec<Arc<Texture>>,     // Textures used in the material
    pub texture_views: Vec<TextureView>, // Hold TextureViews so they don't go out of scope
    pub sampler: Sampler,                // Sampler used for the textures
    pub shininess: f32,                  // For specular highlights, etc.
}

impl Material {
    pub fn new(
        device: &Device,
        shader: Arc<Shader>,
        textures: Vec<Arc<Texture>>,
        sampler: Sampler,
        shader_reflection: ShaderReflection,
        shininess: f32,
    ) -> Result<Material, AppError> {
        let bind_group_layout =
            shader_reflection.create_bind_group_layout(device, "Material Bind Group Layout");

        // Create the texture views and store them in a vector to maintain their lifetime
        let texture_views: Vec<TextureView> = textures
            .iter()
            .map(|texture| {
                texture
                    .texture
                    .create_view(&TextureViewDescriptor::default())
            })
            .collect();

        // Create the binding resources using the `texture_views` instead of the temporary references
        let binding_resources: Vec<BindingResource> = texture_views
            .iter()
            .map(|view| BindingResource::TextureView(view))
            .chain(std::iter::once(BindingResource::Sampler(&sampler)))
            .collect();

        let bind_group = shader_reflection.create_bind_group(
            device,
            &bind_group_layout,
            "Material Bind Group",
            &binding_resources,
        );

        Ok(Material {
            shader,
            bind_group: Some(bind_group),
            textures,
            texture_views, // Store the texture views so they are kept alive
            sampler,
            shininess, // Default value, can be set differently per material
        })
    }
}
struct Recipe {
    diffuse: Option<Arc<Texture>>,
    normal: Option<Arc<Texture>>,
    specular: Option<Arc<Texture>>,
}

impl Default for Recipe {
    fn default() -> Self {
        Self {
            diffuse: Default::default(),
            normal: Default::default(),
            specular: Default::default(),
        }
    }
}

#[derive(Error, Debug)]
pub enum ResourceFactoryError {
    #[error("Failed to lock the RwLock: {0}")]
    LockError(String),
}

impl<T> From<PoisonError<RwLockReadGuard<'_, T>>> for ResourceFactoryError {
    fn from(err: PoisonError<RwLockReadGuard<'_, T>>) -> Self {
        ResourceFactoryError::LockError(err.to_string())
    }
}

pub struct MaterialFactory {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    in_progress: Option<Recipe>,
    default_texture: Arc<Texture>, // Store the default texture here
}

impl MaterialFactory {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Self {
        let default_texture = Arc::new(Texture::no_texture(&device, &queue));

        Self {
            device,
            queue,
            in_progress: None,
            default_texture,
        }
    }

    /// Access the `Device`
    pub fn device(&self) -> Arc<Device> {
        Arc::clone(&self.device)
    }

    /// Access the `Queue`
    pub fn queue(&self) -> Arc<Queue> {
        Arc::clone(&self.queue)
    }

    fn set_in_progress(&mut self) {
        self.in_progress = Some(Recipe::default());
    }

    fn log_overwrite_warning(&self) {
        if self.in_progress.is_some() {
            log_warning!("Start requested while in_progress is not none");
        }
    }

    fn start(&mut self) -> &mut MaterialFactory {
        self.log_overwrite_warning();
        self.set_in_progress();
        assert!(self.in_progress.is_some());
        self
    }

    pub fn new_material(&mut self) {
        if self.in_progress.is_none() {
            self.in_progress = Some(Recipe::default())
        }
    }

    pub fn create_material(
        &mut self,
        shader: Arc<Shader>,
        shader_reflection: ShaderReflection,
        diffuse_texture: Option<Arc<Texture>>,
        normal_texture: Option<Arc<Texture>>,
        specular_texture: Option<Arc<Texture>>,
        sampler: Sampler,
        shininess: f32,
    ) -> Result<Material, AppError> {
        let textures = vec![
            diffuse_texture.unwrap_or_else(|| Arc::clone(&self.default_texture)),
            normal_texture.unwrap_or_else(|| Arc::clone(&self.default_texture)),
            specular_texture.unwrap_or_else(|| Arc::clone(&self.default_texture)),
        ];

        let material = Material::new(
            &self.device,
            shader,
            textures,
            sampler,
            shader_reflection,
            shininess,
        )?;

        Ok(material)
    }
}
