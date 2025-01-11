use nalgebra_glm::IVec3;
use crate::texture::{ data::Data, GpuTexture3d, CpuTexture3d, Component, gpu, cpu };

#[derive(Clone)]
pub struct Texture3d {
    gpu: Option<GpuTexture3d>,
    cpu: Option<CpuTexture3d>,
    dimensions: IVec3,
}

impl Texture3d {
    pub fn new(dimensions: IVec3) -> Texture3dBuilder {
        Texture3dBuilder::new(dimensions)
    }

    pub fn dimensions(&self) -> IVec3 {
        self.dimensions
    }

    pub fn cpu_to_gpu(&mut self) {
        if let Some(gpu) = &mut self.gpu {
            if let None = self.cpu {
                gpu.bind().clear(0);
            } else {

            }
        } else {
            let arguments = gpu::texture_3d::Arguments {
                dimensions: self.dimensions,
                internal_components: Component::RGBA,
                internal_size: gpu::SizedComponent::RGBA8,
                mipmap_type: gpu::Mipmap::None,
                data: None
            };
            self.gpu = Some(gpu::Texture3d::generate(arguments));
        }
    }

    pub fn gpu_to_cpu(&mut self) {
        todo!();
        if let Some(cpu) = &mut self.cpu {

        }
    }

    pub fn gpu(&self) -> Option<&GpuTexture3d> {
        self.gpu.as_ref()
    }

    pub fn cpu(&self) -> Option<&CpuTexture3d> {
        self.cpu.as_ref()
    }

    pub fn free_gpu(&mut self) {
        self.gpu = None;
    }

    pub fn free_cpu(&mut self) {
        self.cpu = None;
    }
}

pub struct Texture3dBuilder {
    data: Option<Data>,
    generate_cpu: bool,
    pub(super) gpu_texture_arguments: Option<gpu::texture_3d::Arguments>,
    pub(super) dimensions: IVec3,
    pub(super) components: Component,
}

impl Texture3dBuilder {
    fn new(dimensions: IVec3) -> Texture3dBuilder {
        Texture3dBuilder {
            data: None,
            gpu_texture_arguments: None,
            generate_cpu: false,
            dimensions,
            components: Component::RGBA,
        }
    }

    pub fn gpu(self) -> gpu::texture_3d::TextureBuilder3d {
        gpu::texture_3d::TextureBuilder3d::new(self)
    }

    pub fn cpu(mut self) -> Texture3dBuilder {
        self.generate_cpu = true;
        self
    }

    pub fn components(mut self, components: Component) -> Texture3dBuilder {
        self.components = components;
        self
    }

    pub fn data(mut self, data: Data) -> Texture3dBuilder {
        self.data = Some(data);
        self
    }

    pub fn build(mut self) -> Texture3d {
        let gpu = if let Some(gpu) = &mut self.gpu_texture_arguments {
            gpu.internal_components = self.components;
            Some(gpu::Texture3d::generate(gpu.clone()))
        } else {
            None
        };

        let cpu = if self.generate_cpu {
            if let Some(data) = self.data {
                Some(cpu::Texture3d::generate(self.dimensions, data))
            } else {
                let count = self.dimensions.x * self.dimensions.y * self.dimensions.z;
                Some(cpu::Texture3d::generate(self.dimensions, Data::empty_u8(self.components, count as usize)))
            }
        } else {
            None
        };

        Texture3d {
            dimensions: self.dimensions,
            gpu,
            cpu
        }
    }
}

