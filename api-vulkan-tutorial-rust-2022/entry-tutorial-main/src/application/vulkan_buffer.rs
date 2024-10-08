use ::vulkan::prelude::version1_2::*;
use ::vulkan::extend::VulkanErrorCode;
use ::vulkan::VulkanDevicePhysical;
use ::vulkan::VulkanInstance;
use ::vulkan::VulkanSharingMode;
use ::vulkan::VulkanBufferUsageFlagS;
use ::vulkan::VulkanBufferCreateInformation;
use ::vulkan::VulkanMemoryAllocateInformation;
use ::vulkan::VulkanMemoryPropertyFlagS;
use ::vulkan::VulkanBuffer;
use ::vulkan::VulkanDeviceMemory;
use ::vulkan::VulkanDeviceSize;
use ::vulkan::VulkanCommandBufferAllocateInformation;
use ::vulkan::VulkanCommandBufferLevel;
use ::vulkan::VulkanCommandPool;
use ::vulkan::VulkanCommandBufferUsageFlagS;
use ::vulkan::VulkanCommandBufferBeginInformation;
use ::vulkan::VulkanBufferCopy;
use ::vulkan::VulkanSubmitInformation;
use ::vulkan::VulkanQueue;
use ::vulkan::VulkanFence;

use crate::termination::TerminationProcessMain;
use crate::application::vulkan_memory::ApplicationVulkanMemory;


pub struct ApplicationVulkanBuffer {}

impl ApplicationVulkanBuffer {
    pub unsafe fn create_with_memory(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_buffer_size: VulkanDeviceSize,
        vulkan_buffer_usage: VulkanBufferUsageFlagS,
        vulkan_memory_property_flag_s: VulkanMemoryPropertyFlagS)
     -> Result<(VulkanBuffer, VulkanDeviceMemory), TerminationProcessMain>
    {
        let vulkan_buffer_create_information =
            VulkanBufferCreateInformation::builder()
            .size(vulkan_buffer_size)
            .usage(vulkan_buffer_usage)
            .sharing_mode(VulkanSharingMode::EXCLUSIVE);
        let create_vulkan_buffer_result =
            vulkan_logical_device.create_buffer(&vulkan_buffer_create_information, None);
        let vulkan_buffer = termination_vulkan_error!(return1,
            create_vulkan_buffer_result, TerminationProcessMain::InitializationVulkanBufferCreateFail);
        let vulkan_memory_requirement_s =
            vulkan_logical_device.get_buffer_memory_requirements(vulkan_buffer);
        let vulkan_memory_type_index =
            ApplicationVulkanMemory::get_type_index(
                vulkan_instance, vulkan_physical_device, vulkan_memory_property_flag_s, vulkan_memory_requirement_s)?;
        let vulkan_memory_allocate_infomation =
            VulkanMemoryAllocateInformation::builder()
            .allocation_size(vulkan_memory_requirement_s.size)
            .memory_type_index(vulkan_memory_type_index.as_raw());
        let allocate_vulkan_buffer_memory_result =
            vulkan_logical_device.allocate_memory(&vulkan_memory_allocate_infomation, None);
        let vulkan_buffer_memory = termination_vulkan_error!(return1,
            allocate_vulkan_buffer_memory_result, TerminationProcessMain::InitializationVulkanMemoryAllocateFail);
        let bind_buffer_memory_result =
            vulkan_logical_device.bind_buffer_memory(vulkan_buffer, vulkan_buffer_memory, 0);
        termination_vulkan_error!(return1,
            bind_buffer_memory_result, TerminationProcessMain::InitializationVulkanMemoryBufferBindFail);
        Ok((vulkan_buffer, vulkan_buffer_memory))
    }

    pub unsafe fn copy(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_command_pool: VulkanCommandPool,
        vulkan_graphic_queue: VulkanQueue,
        vulkan_souruce_buffer: VulkanBuffer,
        vulkan_destination_buffer: VulkanBuffer,
        vulkan_buffer_size: VulkanDeviceSize)
     -> Result<(), TerminationProcessMain>
    {
        let vulkan_command_buffer_allocate_infomation =
            VulkanCommandBufferAllocateInformation::builder()
            .level(VulkanCommandBufferLevel::PRIMARY)
            .command_pool(vulkan_command_pool)
            .command_buffer_count(1);
        let allocate_vulkan_command_buffer_s_result =
            vulkan_logical_device.allocate_command_buffers(&vulkan_command_buffer_allocate_infomation);
        let vulkan_command_buffer = termination_vulkan_error!(return1,
            allocate_vulkan_command_buffer_s_result,
            TerminationProcessMain::InitializationVulkanCommandBufferSAllocateFail)[0];
        let vulkan_command_buffer_begin_information =
            VulkanCommandBufferBeginInformation::builder()
            .flags(VulkanCommandBufferUsageFlagS::ONE_TIME_SUBMIT);
        let begin_vulkan_command_buffer_result =
            vulkan_logical_device.begin_command_buffer(
                vulkan_command_buffer, &vulkan_command_buffer_begin_information);
        termination_vulkan_error!(return1,
            begin_vulkan_command_buffer_result, TerminationProcessMain::InitializationVulkanCommandBufferBeginFail);
        let vulkan_buffer_copy = VulkanBufferCopy::builder().size(vulkan_buffer_size);
        vulkan_logical_device.cmd_copy_buffer(
            vulkan_command_buffer, vulkan_souruce_buffer, vulkan_destination_buffer, &[vulkan_buffer_copy]);
        let end_vulkan_command_buffer_result = vulkan_logical_device.end_command_buffer(vulkan_command_buffer);
        termination_vulkan_error!(return1,
            end_vulkan_command_buffer_result, TerminationProcessMain::InitializationVulkanCommandBufferEndFail);
        let vulkan_command_buffer_s = &[vulkan_command_buffer];
        let vulkan_submit_information =
            VulkanSubmitInformation::builder().command_buffers(vulkan_command_buffer_s);
        let submit_vulkan_queue_result =
            vulkan_logical_device.queue_submit(
                vulkan_graphic_queue, &[vulkan_submit_information], VulkanFence::null());
        termination_vulkan_error!(return1,
            submit_vulkan_queue_result, TerminationProcessMain::InitializationVulkanQueueSubmitFail);
        let wait_vulkan_queue_idle_result = vulkan_logical_device.queue_wait_idle(vulkan_graphic_queue);
        termination_vulkan_error!(return1,
            wait_vulkan_queue_idle_result, TerminationProcessMain::InitializationVulkanDeviceWaitIdleFail);
        vulkan_logical_device.free_command_buffers(vulkan_command_pool, vulkan_command_buffer_s);
        Ok(())
    }
}