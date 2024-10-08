use ::vulkan::extend::VulkanErrorCode;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanExtentD2;
use ::vulkan::VulkanShaderModule;
use ::vulkan::VulkanShaderModuleCreateInformation;
use ::vulkan::VulkanPipelineShaderStageCreateInformation;
use ::vulkan::VulkanShaderStageFlagS;
use ::vulkan::VulkanPipelineVertexInputStateCreateInformation;
use ::vulkan::VulkanPipelineInputAssemblyStateCreateInformation;
use ::vulkan::VulkanPrimitiveTopology;
use ::vulkan::VulkanViewport;
use ::vulkan::VulkanRectangleD2;
use ::vulkan::VulkanOffsetD2;
use ::vulkan::VulkanPipelineViewportStateCreateInformation;
use ::vulkan::VulkanPipelineRasterizationStateCreateInformation;
use ::vulkan::VulkanPolygonMode;
use ::vulkan::VulkanCullModeFlagS;
use ::vulkan::VulkanFrontFace;
use ::vulkan::VulkanPipelineMultisampleStateCreateInformation;
use ::vulkan::VulkanPipelineColorBlendStateCreateInformation;
use ::vulkan::VulkanPipelineLayoutCreateInformation;
use ::vulkan::VulkanColorComponentFlagS;
use ::vulkan::VulkanLogicOperation;
use ::vulkan::VulkanSampleCountFlagS;
use ::vulkan::VulkanPipelineColorBlendAttachmentState;
use ::vulkan::VulkanPipelineLayout;
use ::vulkan::VulkanGraphicsPipelineCreateInformation;
use ::vulkan::VulkanRenderPass;
use ::vulkan::VulkanPipelineCache;
use ::vulkan::VulkanPipeline;
use ::vulkan::VulkanDescriptorSetLayout;
use ::vulkan::VulkanPipelineDepthStencilStateCreateInformation;
use ::vulkan::VulkanCompareOperation;
use ::vulkan::VulkanBlendFactor;
use ::vulkan::VulkanBlendOperation;
use ::vulkan::VulkanPushConstantRange;

use crate::termination::TerminationProcessMain;
use crate::data::d3_model_vertex::DataD3ModelVulkanVertexInput;


pub struct ApplicationVulkanPipeline {}

impl ApplicationVulkanPipeline {
    pub unsafe fn create_layout(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_2d_extent: VulkanExtentD2,
        vulkan_render_pass: VulkanRenderPass,
        vulkan_descriptor_set_layout: VulkanDescriptorSetLayout,
        vulkan_anti_aliasing_multisampling_number: VulkanSampleCountFlagS)
     -> Result<(VulkanPipeline, VulkanPipelineLayout), TerminationProcessMain>
    {
        let vulkan_vertex_shader_bytecode_data = include_bytes!("../../shader/vert.spv");
        let vulkan_fragment_shader_bytecode_data = include_bytes!("../../shader/frag.spv");
        let vulkan_vertex_shader_module =
            Self::create_shader_module(vulkan_logical_device, vulkan_vertex_shader_bytecode_data)?;
        let vulkan_fragment_shader_module =
            Self::create_shader_module(vulkan_logical_device, vulkan_fragment_shader_bytecode_data)?;
        let vulkan_vertex_shader_stage_create_information =
            VulkanPipelineShaderStageCreateInformation::builder()
            .stage(VulkanShaderStageFlagS::VERTEX)
            .module(vulkan_vertex_shader_module)
            .name(b"main\0");
        let vulkan_fragment_shader_stage_create_infomation =
            VulkanPipelineShaderStageCreateInformation::builder()
            .stage(VulkanShaderStageFlagS::FRAGMENT)
            .module(vulkan_fragment_shader_module)
            .name(b"main\0");
        let vulkan_vertex_input_binding_description = &[DataD3ModelVulkanVertexInput::get_binding_descrption()];
        let (vulkan_vertex_input_position_attribute_description,
             vulkan_vertex_input_color_attribute_description,
             vulkan_vertex_input_texture_coordinate) =
            DataD3ModelVulkanVertexInput::get_attribute_description_s();
        let vulkan_vertex_input_attribute_description_s =
            &[vulkan_vertex_input_position_attribute_description, vulkan_vertex_input_color_attribute_description,
              vulkan_vertex_input_texture_coordinate];
        let vulkan_vertex_input_state_create_infomation =
            VulkanPipelineVertexInputStateCreateInformation::builder()
            .vertex_binding_descriptions(vulkan_vertex_input_binding_description)
            .vertex_attribute_descriptions(vulkan_vertex_input_attribute_description_s);
        let vulkan_input_assembly_state_create_information =
            VulkanPipelineInputAssemblyStateCreateInformation::builder()
            .topology(VulkanPrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);
        let vulkan_viewport =
            VulkanViewport::builder()
            .x(0.0)
            .y(0.0)
            .width(vulkan_2d_extent.width as f32)
            .height(vulkan_2d_extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0);
        let vulkan_scissor =
            VulkanRectangleD2::builder()
            .offset(VulkanOffsetD2 { x: 0, y: 0 })
            .extent(vulkan_2d_extent);
        let vulkan_viewport_s = &[vulkan_viewport];
        let vulkan_scissor_s = &[vulkan_scissor];
        let vulkan_viewport_state_create_information =
            VulkanPipelineViewportStateCreateInformation::builder()
            .viewports(vulkan_viewport_s)
            .scissors(vulkan_scissor_s);
        let vulkan_rasterization_state_create_information =
            VulkanPipelineRasterizationStateCreateInformation::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(VulkanPolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(VulkanCullModeFlagS::BACK)
            .front_face(VulkanFrontFace::COUNTER_CLOCKWISE)
            .depth_bias_enable(false);
        let vulkan_pipeline_multisample_state_create_information =
            VulkanPipelineMultisampleStateCreateInformation::builder()
            .sample_shading_enable(true)
            .min_sample_shading(0.2)
            .rasterization_samples(vulkan_anti_aliasing_multisampling_number);
        let vulkan_depth_stencil_state =
            VulkanPipelineDepthStencilStateCreateInformation::builder()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(VulkanCompareOperation::LESS)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(false);
        let vulkan_color_blend_attachment_state =
            VulkanPipelineColorBlendAttachmentState::builder()
            .color_write_mask(VulkanColorComponentFlagS::all())
            .blend_enable(true)
            .src_color_blend_factor(VulkanBlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(VulkanBlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(VulkanBlendOperation::ADD)
            .src_alpha_blend_factor(VulkanBlendFactor::ONE)
            .dst_alpha_blend_factor(VulkanBlendFactor::ZERO)
            .alpha_blend_op(VulkanBlendOperation::ADD);
        let vulkan_color_blend_attachment_state_s = &[vulkan_color_blend_attachment_state];
        let vulkan_color_blend_state_create_information =
            VulkanPipelineColorBlendStateCreateInformation::builder()
            .logic_op_enable(false)
            .logic_op(VulkanLogicOperation::COPY)
            .attachments(vulkan_color_blend_attachment_state_s)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);
        let vulkan_vertex_push_constant_range =
            VulkanPushConstantRange::builder()
            .stage_flags(VulkanShaderStageFlagS::VERTEX)
            .offset(0)
            .size(64);
        let vulkan_fragment_push_constant_range =
            VulkanPushConstantRange::builder()
            .stage_flags(VulkanShaderStageFlagS::FRAGMENT)
            .offset(64)
            .size(4);
        let vulkan_descriptor_set_layout_s = &[vulkan_descriptor_set_layout];
        let vulkan_push_constant_range_s =
            &[vulkan_vertex_push_constant_range, vulkan_fragment_push_constant_range];
        let vulkan_pipeline_layout_create_infomation =
            VulkanPipelineLayoutCreateInformation::builder()
            .set_layouts(vulkan_descriptor_set_layout_s)
            .push_constant_ranges(vulkan_push_constant_range_s);
        let create_vulkan_pipeline_layout_result =
            vulkan_logical_device.create_pipeline_layout(&vulkan_pipeline_layout_create_infomation, None);
        let vulkan_pipeline_layout = termination_vulkan_error!(return1,
            create_vulkan_pipeline_layout_result, TerminationProcessMain::InitializationVulkanPipelineLayoutCreateFail);
        let vulkan_stage_s =
            &[vulkan_vertex_shader_stage_create_information, vulkan_fragment_shader_stage_create_infomation];
        let vulkan_graphics_pipeline_create_information =
            VulkanGraphicsPipelineCreateInformation::builder()
            .stages(vulkan_stage_s)
            .vertex_input_state(&vulkan_vertex_input_state_create_infomation)
            .input_assembly_state(&vulkan_input_assembly_state_create_information)
            .viewport_state(&vulkan_viewport_state_create_information)
            .rasterization_state(&vulkan_rasterization_state_create_information)
            .multisample_state(&vulkan_pipeline_multisample_state_create_information)
            .depth_stencil_state(&vulkan_depth_stencil_state)
            .color_blend_state(&vulkan_color_blend_state_create_information)
            .layout(vulkan_pipeline_layout)
            .render_pass(vulkan_render_pass)
            .subpass(0);
        let create_vulkan_graphics_pipeline_result =
            vulkan_logical_device.create_graphics_pipelines(
                VulkanPipelineCache::null(),
                &[vulkan_graphics_pipeline_create_information],
                None);
        let (vulkan_graphics_pipeline, _) = termination_vulkan_error!(return1,
            create_vulkan_graphics_pipeline_result,
            TerminationProcessMain::InitializationVulkanGraphicsPipelineSCreateFail);
        vulkan_logical_device.destroy_shader_module(vulkan_vertex_shader_module, None);
        vulkan_logical_device.destroy_shader_module(vulkan_fragment_shader_module, None);
        Ok((vulkan_graphics_pipeline, vulkan_pipeline_layout))
    }

    unsafe fn create_shader_module(
        vulkan_logical_device: &VulkanDeviceLogical, byte_code_data: &[u8])
     -> Result<VulkanShaderModule, TerminationProcessMain>
    {
        let bytecode_byte_s = Vec::<u8>::from(byte_code_data);
        let (align_prefix, byte_s, align_suffix) = bytecode_byte_s.align_to::<u32>();
        if !align_prefix.is_empty() || !align_suffix.is_empty() {
            return Err(TerminationProcessMain::InitializationVulkanShaderByteCodeAlignmentIncorrect);
        }
        let vulkan_shader_module_create_information =
            VulkanShaderModuleCreateInformation::builder()
            .code_size(bytecode_byte_s.len())
            .code(byte_s);
        let create_vulkan_shader_module_result =
            vulkan_logical_device.create_shader_module(&vulkan_shader_module_create_information, None);
        termination_vulkan_error!(normal1,
            create_vulkan_shader_module_result, TerminationProcessMain::InitializationVulkanShaderModuleCreateFail)
    }
}