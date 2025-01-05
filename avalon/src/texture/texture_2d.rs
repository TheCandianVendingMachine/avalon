use nalgebra_glm::IVec2;
use crate::texture::{ GpuTexture2d, CpuTexture, Component, gpu };

pub struct Texture2d {
    gpu: Option<GpuTexture2d>,
    cpu: Option<CpuTexture>,
    dimensions: IVec2,
    components: Component,
}

impl Texture2d {
    pub fn new(dimensions: IVec2) -> Texture2dBuilder {
        Texture2dBuilder::new(dimensions)
    }

    pub fn dimensions(&self) -> IVec2 {
        self.dimensions
    }

    pub fn generate_gpu(&mut self) -> &GpuTexture2d {
        if let None = self.gpu {

        }
        self.gpu().unwrap()
    }

    pub fn generate_cpu(&mut self) -> &CpuTexture {
        if let None = self.cpu {

        }
        self.cpu().unwrap()
    }

    pub fn gpu(&self) -> Option<&GpuTexture2d> {
        self.gpu.as_ref()
    }

    pub fn cpu(&self) -> Option<&CpuTexture> {
        self.cpu.as_ref()
    }
}

pub struct Texture2dBuilder {
    pub(super) gpu_texture_arguments: Option<gpu::Arguments>,
    pub(super) dimensions: IVec2,
    pub(super) components: Component,
}

impl Texture2dBuilder {
    fn new(dimensions: IVec2) -> Texture2dBuilder {
        Texture2dBuilder {
            gpu_texture_arguments: None,
            dimensions,
            components: Component::RGBA,
        }
    }

    pub fn gpu(self) -> gpu::TextureBuilder2d {
        gpu::TextureBuilder2d::new(self)
    }

    pub fn components(mut self, components: Component) -> Texture2dBuilder {
        self.components = components;
        self
    }

    pub fn build(mut self) -> Texture2d {
        let gpu = if let Some(gpu) = &mut self.gpu_texture_arguments {
            gpu.internal_components = self.components;
            Some(gpu::Texture2d::generate(*gpu))
        } else {
            None
        };

        Texture2d {
            dimensions: self.dimensions,
            components: self.components,
            gpu,
            cpu
        }
    }
}

