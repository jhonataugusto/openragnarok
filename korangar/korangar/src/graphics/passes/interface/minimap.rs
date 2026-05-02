use std::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};
use wgpu::util::StagingBelt;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource,
    BindingType, BlendState, BufferBindingType, BufferUsages, ColorTargetState, ColorWrites, CommandEncoder, Device, FragmentState,
    MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, Queue, RenderPass, RenderPipeline, RenderPipelineDescriptor,
    ShaderStages, TextureSampleType, TextureView, TextureViewDimension, VertexState,
};

use crate::graphics::passes::{
    BindGroupCount, ColorAttachmentCount, DepthAttachmentCount, Drawer, InterfaceRenderPassContext, RenderPassContext,
};
use crate::graphics::shader_compiler::ShaderCompiler;
use crate::graphics::{Buffer, Capabilities, GlobalContext, MinimapInstruction, Prepare, RenderInstruction};

const DRAWER_NAME: &str = "interface minimap";

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct MinimapUniforms {
    tint: [f32; 4],
    screen_position: [f32; 2],
    screen_size: [f32; 2],
    player_uv: [f32; 2],
    map_size_tiles: [f32; 2],
    visible_tiles: f32,
    rotation_radians: f32,
    _padding: [f32; 2],
}

pub(crate) struct InterfaceMinimapDrawer {
    uniforms_buffer: Buffer<MinimapUniforms>,
    bind_group_layout: BindGroupLayout,
    bind_group: BindGroup,
    pipeline: RenderPipeline,
    uniforms: Option<MinimapUniforms>,
}

impl Drawer<{ BindGroupCount::One }, { ColorAttachmentCount::One }, { DepthAttachmentCount::None }> for InterfaceMinimapDrawer {
    type Context = InterfaceRenderPassContext;
    type DrawData<'data> = Option<&'data MinimapInstruction>;

    fn new(
        _capabilities: &Capabilities,
        device: &Device,
        _queue: &Queue,
        shader_compiler: &ShaderCompiler,
        global_context: &GlobalContext,
        render_pass_context: &Self::Context,
    ) -> Self {
        let shader_module = shader_compiler.create_shader_module("interface", "minimap");

        let uniforms_buffer = Buffer::with_capacity(
            device,
            format!("{DRAWER_NAME} uniforms"),
            BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            size_of::<MinimapUniforms>() as _,
        );

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some(DRAWER_NAME),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(size_of::<MinimapUniforms>() as _),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = Self::create_bind_group(
            device,
            &bind_group_layout,
            &uniforms_buffer,
            global_context.solid_pixel_texture.get_texture_view(),
        );

        let pass_bind_group_layouts = Self::Context::bind_group_layout(device);
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some(DRAWER_NAME),
            bind_group_layouts: &[Some(pass_bind_group_layouts[0]), Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(DRAWER_NAME),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: render_pass_context.color_attachment_formats()[0],
                    blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: ColorWrites::default(),
                })],
            }),
            primitive: Default::default(),
            multisample: MultisampleState::default(),
            depth_stencil: None,
            cache: None,
            multiview_mask: None,
        });

        Self {
            uniforms_buffer,
            bind_group_layout,
            bind_group,
            pipeline,
            uniforms: None,
        }
    }

    fn draw(&mut self, pass: &mut RenderPass<'_>, draw_data: Self::DrawData<'_>) {
        if draw_data.is_none() || self.uniforms.is_none() {
            return;
        }

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(1, &self.bind_group, &[]);
        pass.draw(0..6, 0..1);
    }
}

impl Prepare for InterfaceMinimapDrawer {
    fn prepare(&mut self, device: &Device, instructions: &RenderInstruction) {
        let Some(instruction) = instructions.minimap else {
            self.uniforms = None;
            return;
        };

        self.uniforms = Some(MinimapUniforms {
            tint: instruction.tint.components_linear(),
            screen_position: instruction.screen_position.into(),
            screen_size: instruction.screen_size.into(),
            player_uv: instruction.player_uv.into(),
            map_size_tiles: instruction.map_size_tiles.into(),
            visible_tiles: instruction.visible_tiles,
            rotation_radians: instruction.rotation_radians,
            _padding: Default::default(),
        });

        self.bind_group = Self::create_bind_group(
            device,
            &self.bind_group_layout,
            &self.uniforms_buffer,
            instruction.texture.get_texture_view(),
        );
    }

    fn upload(&mut self, device: &Device, staging_belt: &mut StagingBelt, command_encoder: &mut CommandEncoder) {
        if let Some(uniforms) = self.uniforms {
            self.uniforms_buffer.write(device, staging_belt, command_encoder, &[uniforms]);
        }
    }
}

impl InterfaceMinimapDrawer {
    fn create_bind_group(
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        uniforms_buffer: &Buffer<MinimapUniforms>,
        minimap_texture: &TextureView,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some(DRAWER_NAME),
            layout: bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: uniforms_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(minimap_texture),
                },
            ],
        })
    }
}
