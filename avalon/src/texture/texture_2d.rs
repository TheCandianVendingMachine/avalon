use nalgebra_glm::IVec2;
use crate::texture::{ data::Data, GpuTexture2d, CpuTexture2d, Component, gpu, cpu };

#[derive(Clone)]
pub struct Texture2d {
    gpu: Option<GpuTexture2d>,
    cpu: Option<CpuTexture2d>,
    dimensions: IVec2,
}

impl Texture2d {
    pub fn new(dimensions: IVec2) -> Texture2dBuilder {
        Texture2dBuilder::new(dimensions)
    }

    pub fn dimensions(&self) -> IVec2 {
        self.dimensions
    }

    pub fn cpu_to_gpu(&mut self) {
        if let Some(gpu) = &mut self.gpu {
            if let None = self.cpu {
                gpu.bind().clear(0);
            } else {

            }
        } else {
            let arguments = gpu::texture_2d::Arguments {
                dimensions: self.dimensions,
                internal_components: Component::RGBA,
                internal_size: gpu::SizedComponent::RGBA8,
                mipmap_type: gpu::Mipmap::None,
                data: None
            };
            self.gpu = Some(gpu::Texture2d::generate(arguments));
        }
    }

    pub fn gpu_to_cpu(&mut self) {
        todo!();
        if let Some(cpu) = &mut self.cpu {

        }
    }

    pub fn gpu(&self) -> Option<&GpuTexture2d> {
        self.gpu.as_ref()
    }

    pub fn cpu(&self) -> Option<&CpuTexture2d> {
        self.cpu.as_ref()
    }

    pub fn free_gpu(&mut self) {
        self.gpu = None;
    }

    pub fn free_cpu(&mut self) {
        self.cpu = None;
    }
}

pub struct Texture2dBuilder {
    data: Option<Data>,
    generate_cpu: bool,
    pub(super) gpu_texture_arguments: Option<gpu::texture_2d::Arguments>,
    pub(super) dimensions: IVec2,
    pub(super) components: Component,
}

impl Texture2dBuilder {
    fn new(dimensions: IVec2) -> Texture2dBuilder {
        Texture2dBuilder {
            data: None,
            gpu_texture_arguments: None,
            generate_cpu: false,
            dimensions,
            components: Component::RGBA,
        }
    }

    pub fn gpu(self) -> gpu::texture_2d::TextureBuilder2d {
        gpu::texture_2d::TextureBuilder2d::new(self)
    }

    pub fn cpu(mut self) -> Texture2dBuilder {
        self.generate_cpu = true;
        self
    }

    pub fn components(mut self, components: Component) -> Texture2dBuilder {
        self.components = components;
        self
    }

    pub fn data(mut self, data: Data) -> Texture2dBuilder {
        self.data = Some(data);
        self
    }

    pub fn build(mut self) -> Texture2d {
        let gpu = if let Some(gpu) = &mut self.gpu_texture_arguments {
            gpu.internal_components = self.components;
            Some(gpu::Texture2d::generate(gpu.clone()))
        } else {
            None
        };

        let cpu = if self.generate_cpu {
            if let Some(data) = self.data {
                Some(cpu::Texture2d::generate(self.dimensions, data))
            } else {
                let count = self.dimensions.x * self.dimensions.y;
                Some(cpu::Texture2d::generate(self.dimensions, Data::empty_u8(self.components, count as usize)))
            }
        } else {
            None
        };

        Texture2d {
            dimensions: self.dimensions,
            gpu,
            cpu
        }
    }
}

