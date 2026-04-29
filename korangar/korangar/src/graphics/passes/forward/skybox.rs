use std::sync::Arc;

use wgpu::{
    BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Device, FragmentState, MultisampleState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPass, RenderPipeline, RenderPipelineDescriptor,
    StencilState, VertexState,
};

use crate::graphics::passes::{
    BindGroupCount, ColorAttachmentCount, DepthAttachmentCount, Drawer, ForwardRenderPassContext, RenderPassContext,
};
use crate::graphics::shader_compiler::ShaderCompiler;
use crate::graphics::{Capabilities, GlobalContext, SkyboxInstruction, Texture};

const DRAWER_NAME: &str = "forward skybox";
const SKYDOME_VERTEX_COUNT: u32 = 64 * 24 * 6;

pub(crate) struct ForwardSkyboxDrawer {
    skybox_texture: Arc<Texture>,
    pipeline: RenderPipeline,
}

impl Drawer<{ BindGroupCount::Two }, { ColorAttachmentCount::Three }, { DepthAttachmentCount::One }> for ForwardSkyboxDrawer {
    type Context = ForwardRenderPassContext;
    type DrawData<'data> = SkyboxInstruction;

    fn new(
        _capabilities: &Capabilities,
        device: &Device,
        _queue: &Queue,
        shader_compiler: &ShaderCompiler,
        global_context: &GlobalContext,
        render_pass_context: &Self::Context,
    ) -> Self {
        let shader_module = shader_compiler.create_shader_module("forward", "skybox");
        let pass_bind_group_layouts = Self::Context::bind_group_layout(device);

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some(DRAWER_NAME),
            bind_group_layouts: &[
                Some(pass_bind_group_layouts[0]),
                Some(pass_bind_group_layouts[1]),
                Some(Texture::bind_group_layout(device)),
            ],
            immediate_size: 0,
        });

        let color_attachment_formats = render_pass_context.color_attachment_formats();

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
                targets: &[
                    Some(ColorTargetState {
                        format: color_attachment_formats[0],
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    }),
                    Some(ColorTargetState {
                        format: color_attachment_formats[1],
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::empty(),
                    }),
                    Some(ColorTargetState {
                        format: color_attachment_formats[2],
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::empty(),
                    }),
                ],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: Some(DepthStencilState {
                format: render_pass_context.depth_attachment_output_format()[0],
                depth_write_enabled: Some(false),
                depth_compare: Some(CompareFunction::Always),
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState {
                count: global_context.msaa.sample_count(),
                ..Default::default()
            },
            cache: None,
            multiview_mask: None,
        });

        Self {
            skybox_texture: global_context.skybox_texture.clone(),
            pipeline,
        }
    }

    fn draw(&mut self, pass: &mut RenderPass<'_>, instruction: Self::DrawData<'_>) {
        if !instruction.enabled {
            return;
        }

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(2, self.skybox_texture.get_bind_group(), &[]);
        pass.draw(0..SKYDOME_VERTEX_COUNT, 0..1);
    }
}
