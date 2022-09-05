use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanRenderPass;
use ::vulkan::VulkanExtentD2;
use ::vulkan::VulkanPipeline;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanFrameBuffer;
use ::vulkan::VulkanCommandPool;
use ::vulkan::VulkanCommandBufferAllocateInformation;
use ::vulkan::VulkanCommandBufferLevel;
use ::vulkan::VulkanCommandBufferBeginInformation;
use ::vulkan::VulkanClearValue;
use ::vulkan::VulkanClearColorValue;
use ::vulkan::VulkanRectangleD2;
use ::vulkan::VulkanOffsetD2;
use ::vulkan::VulkanRenderPassBeginInformation;
use ::vulkan::VulkanSubpassContents;
use ::vulkan::VulkanPipelineBindPoint;
use ::vulkan::VulkanCommandBuffer;

use crate::termination::TerminationProcessMain;


pub struct ApplicationVulkanCommandBuffer {}

impl ApplicationVulkanCommandBuffer {
    pub unsafe fn create_all(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_command_pool: VulkanCommandPool,
        vulkan_frame_buffer_s: &Vec<VulkanFrameBuffer>,
        vulkan_extent: VulkanExtentD2,
        vulkan_render_pass: VulkanRenderPass,
        vulkan_pipeline: VulkanPipeline)
     -> Result<Vec<VulkanCommandBuffer>, TerminationProcessMain>
    {
        let vulkan_command_buffer_allocate_information =
            VulkanCommandBufferAllocateInformation::builder()
            .command_pool(vulkan_command_pool)
            .level(VulkanCommandBufferLevel::PRIMARY)
            .command_buffer_count(vulkan_frame_buffer_s.len() as u32);
        let allocate_vulkan_command_buffer_s_result =
            vulkan_logical_device.allocate_command_buffers(&vulkan_command_buffer_allocate_information);
        let vulkan_command_buffer_s =
            match allocate_vulkan_command_buffer_s_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanCommandBufferSAllocateFail(vulkan_error_code));
                },
                Ok(buffer_s) => buffer_s,
            };
        for (index, vulkan_command_buffer) in vulkan_command_buffer_s.iter().enumerate() {
            let vulkan_command_buffer_begin_information = VulkanCommandBufferBeginInformation::builder();
            let begin_vulkan_command_buffer_result =
                vulkan_logical_device.begin_command_buffer(
                    *vulkan_command_buffer, &vulkan_command_buffer_begin_information);
            match begin_vulkan_command_buffer_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanCommandBufferBeginFail(vulkan_error_code));
                },
                Ok(()) => (),
            };
            let vulkan_render_area =
                VulkanRectangleD2::builder()
                .offset(VulkanOffsetD2::default())
                .extent(vulkan_extent);
            let vulkan_clear_value =
                VulkanClearValue { color: VulkanClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] } };
            let vulkan_clear_value_s = &[vulkan_clear_value];
            let vulkan_render_pass_begin_information =
                VulkanRenderPassBeginInformation::builder()
                .render_pass(vulkan_render_pass)
                .framebuffer(vulkan_frame_buffer_s[index])
                .render_area(vulkan_render_area)
                .clear_values(vulkan_clear_value_s);
            vulkan_logical_device.cmd_begin_render_pass(
                *vulkan_command_buffer, &vulkan_render_pass_begin_information, VulkanSubpassContents::INLINE);
            vulkan_logical_device.cmd_bind_pipeline(
                *vulkan_command_buffer, VulkanPipelineBindPoint::GRAPHICS, vulkan_pipeline);
            vulkan_logical_device.cmd_draw(*vulkan_command_buffer, 3, 1, 0, 0);
            vulkan_logical_device.cmd_end_render_pass(*vulkan_command_buffer);
            let end_vulkan_command_buffer_result =
                vulkan_logical_device.end_command_buffer(*vulkan_command_buffer);
            match end_vulkan_command_buffer_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanCommandBufferEndFail(vulkan_error_code));
                },
                Ok(()) => (),
            };
        };
        Ok(vulkan_command_buffer_s)
    }
}