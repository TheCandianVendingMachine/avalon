use nalgebra_glm::IVec2;
use crate::debug::GpuAnnotation;
use crate::shader::{ self, Source, Program };
use crate::texture::GpuTexture2d;
use crate::texture::gpu::{ self, ManagedTexture, UniqueTexture, Arguments2d, Mipmap };

pub struct Rescaler {
    downsample: Program,
    upscale: Program
}

impl Rescaler {
    pub fn new() -> Rescaler {
        Rescaler {
            downsample: Program::new()
                .compute(shader::Compute::load_from_path("assets/shaders/downsample.comp").unwrap())
                .build()
                .unwrap(),
            upscale: Program::new()
                .compute(shader::Compute::load_from_path("assets/shaders/upscale.comp").unwrap())
                .build()
                .unwrap(),
        }
    }

    pub fn downsample_halving(&self, original: &GpuTexture2d, halve_count: u32) -> ManagedTexture<GpuTexture2d> {
        let _annotation = GpuAnnotation::push("Halve-Downsample");
        let mut size = original.dimensions();
        let mut temp_downsampled = GpuTexture2d::generate_many::<2>(Arguments2d {
            dimensions: size,
            internal_components: original.internal_components,
            internal_size: original.internal_size,
            mipmap_type: Mipmap::None,
            data: None
        }).map(|t| Some(t.as_managed()));

        {
            let mut bind = self.downsample.activate();
            if halve_count == 0 {
                let downsampled = temp_downsampled[1].as_ref().unwrap();
                bind.uniform("originalSize").unwrap().set_ivec2(size);
                bind.uniform("newSize").unwrap().set_ivec2(size);
                bind.sampler("original", original).unwrap();
                bind.image("downsampled", downsampled, gpu::Access::Write(0)).unwrap();

                let (dispatch_x, dispatch_y, dispatch_z) = self.downsample.dispatch_counts(
                    size.x as usize,
                    size.y as usize,
                    1
                );
                bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
                bind.barrier();
            } else {
                for halve in 1..=halve_count {
                    let original = if halve == 1 {
                        original
                    } else {
                        temp_downsampled[(halve % 2) as usize].as_ref().unwrap()
                    };
                    let downsampled = temp_downsampled[((halve + 1) % 2) as usize].as_ref();
                    size /= 2;
                    bind.uniform("originalSize").unwrap().set_ivec2(size * 2);
                    bind.uniform("newSize").unwrap().set_ivec2(size);
                    bind.sampler("original", original).unwrap();
                    bind.image("downsampled", downsampled.unwrap(), gpu::Access::Write(0)).unwrap();

                    let (dispatch_x, dispatch_y, dispatch_z) = self.downsample.dispatch_counts(
                        size.x as usize,
                        size.y as usize,
                        1
                    );
                    bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
                    bind.barrier();
                }
            }
        }

        let mut downsampled = temp_downsampled[((halve_count + 1) % 2) as usize].take();
        downsampled.take().unwrap()
    }

    pub fn upscale_doubling(&self, original: &GpuTexture2d, double_count: u32) -> ManagedTexture<GpuTexture2d> {
        let _annotation = GpuAnnotation::push("Upscale Doubling");
        let mut size = original.dimensions();
        let mut temp_upscaled = GpuTexture2d::generate_many::<2>(Arguments2d {
            dimensions: size * 2_i32.pow(double_count),
            internal_components: original.internal_components,
            internal_size: original.internal_size,
            mipmap_type: Mipmap::None,
            data: None
        }).map(|t| Some(t.as_managed()));

        {
            let mut bind = self.upscale.activate();
            if double_count == 0 {
                let upscaled = temp_upscaled[1].as_ref().unwrap();
                bind.uniform("originalSize").unwrap().set_ivec2(size);
                bind.uniform("newSize").unwrap().set_ivec2(size);
                bind.sampler("original", original).unwrap();
                bind.image("upscaled", upscaled, gpu::Access::Write(0)).unwrap();

                let (dispatch_x, dispatch_y, dispatch_z) = self.upscale.dispatch_counts(
                    size.x as usize,
                    size.y as usize,
                    1
                );
                bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
                bind.barrier();
            } else {
                for double in 1..=double_count {
                    let original = if double == 1 {
                        original
                    } else {
                        temp_upscaled[(double % 2) as usize].as_ref().unwrap()
                    };
                    let upscaled = temp_upscaled[((double + 1) % 2) as usize].as_ref().unwrap();
                    size *= 2;
                    bind.uniform("originalSize").unwrap().set_ivec2(size / 2);
                    bind.uniform("newSize").unwrap().set_ivec2(size);
                    bind.sampler("original", original).unwrap();
                    bind.image("upscaled", upscaled, gpu::Access::Write(0)).unwrap();

                    let (dispatch_x, dispatch_y, dispatch_z) = self.upscale.dispatch_counts(
                        size.x as usize,
                        size.y as usize,
                        1
                    );
                    bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);
                    bind.barrier();
                }
            }
        }

        let mut upscaled = temp_upscaled[((double_count + 1) % 2) as usize].take();
        upscaled.take().unwrap()
    }

    pub fn downsample(&self, original: &GpuTexture2d, desired_size: IVec2) -> ManagedTexture<GpuTexture2d> {
        let _annotation = GpuAnnotation::push("Downsample To Specific");
        let downsampled = GpuTexture2d::generate(Arguments2d {
            dimensions: desired_size,
            internal_components: original.internal_components,
            internal_size: original.internal_size,
            mipmap_type: Mipmap::None,
            data: None
        });

        let original_size = original.dimensions();
        let halves_to_near =
            (original_size.x / desired_size.x).ilog2().min(
            (original_size.y / desired_size.y).ilog2()
        );

        let near_texture = self.downsample_halving(original, halves_to_near).as_managed();

        let mut bind = self.downsample.activate();
        bind.uniform("originalSize").unwrap().set_ivec2(original_size / 2_i32.pow(halves_to_near));
        bind.uniform("newSize").unwrap().set_ivec2(desired_size);
        bind.sampler("original", &near_texture).unwrap();
        bind.image("downsampled", &downsampled, gpu::Access::Write(0)).unwrap();

        let (dispatch_x, dispatch_y, dispatch_z) = self.downsample.dispatch_counts(
            desired_size.x as usize,
            desired_size.y as usize,
            1
        );
        bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);

        downsampled.into()
    }

    pub fn upscale(&self, original: &GpuTexture2d, desired_size: IVec2) -> ManagedTexture<GpuTexture2d> {
        let _annotation = GpuAnnotation::push("Upscale To Specific");
        let upscaled = GpuTexture2d::generate(Arguments2d {
            dimensions: desired_size,
            internal_components: original.internal_components,
            internal_size: original.internal_size,
            mipmap_type: Mipmap::None,
            data: None
        });

        let original_size = original.dimensions();
        let doubles_to_near =
            (desired_size.x / original_size.x).ilog2().min(
            (desired_size.y / original_size.y).ilog2()
        );

        let near_texture = self.upscale_doubling(original, doubles_to_near).as_managed();

        let mut bind = self.upscale.activate();
        bind.uniform("originalSize").unwrap().set_ivec2(near_texture.dimensions());
        bind.uniform("newSize").unwrap().set_ivec2(desired_size);
        bind.sampler("original", &near_texture).unwrap();
        bind.image("upscaled", &upscaled, gpu::Access::Write(0)).unwrap();

        let (dispatch_x, dispatch_y, dispatch_z) = self.upscale.dispatch_counts(
            desired_size.x as usize,
            desired_size.y as usize,
            1
        );
        bind.dispatch_compute(dispatch_x as u32, dispatch_y as u32, dispatch_z as u32);

        upscaled.into()
    }
}
