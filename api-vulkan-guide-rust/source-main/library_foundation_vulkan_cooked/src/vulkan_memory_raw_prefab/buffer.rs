use ::library_foundation_reintroduction::vulkan::VulkanDeviceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanDevicePhysical;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogical;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceSize;
use ::library_foundation_reintroduction::vulkan::VulkanBufferUsageFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanMemoryPropertyFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceMemory;
use ::library_foundation_reintroduction::vulkan::VulkanBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanBufferCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanSharingMode;
use ::library_foundation_reintroduction::vulkan::VulkanMemoryAllocateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanBufferCopy;
use ::library_foundation_reintroduction::vulkan::VulkanFence;
use ::library_foundation_reintroduction::vulkan::VulkanMemoryMapFlagS;

use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCookedOwn;
use crate::error::foundation_vulkan_cooked::ErrorFoundationVulkanCooked;
use crate::vulkan_queue_submit::one_time_launcher::VulkanQueueSubmitOneTimeLauncher;
use crate::vulkan_memory_raw_prefab::self_::VulkanMemoryRawPrefab;


pub struct VulkanMemoryRawPrefabBuffer {}

impl VulkanMemoryRawPrefabBuffer {
    pub fn create(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_buffer_size: VulkanDeviceSize,
        vulkan_buffer_usage: VulkanBufferUsageFlagS)
    -> Result<VulkanBuffer, ErrorFoundationVulkanCooked>
    {
        let vulkan_buffer_create_information =
            VulkanBufferCreateInformation::builder()
            .size(vulkan_buffer_size)
            .usage(vulkan_buffer_usage)
            .sharing_mode(VulkanSharingMode::EXCLUSIVE)
            .build();
        unsafe { vulkan_logical_device.create_buffer(&vulkan_buffer_create_information, None) }
        .or(Err(ErrorFoundationVulkanCookedOwn::VulkanBufferCreateFail.into()))
    }

    pub fn create_with_memory_bound(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_buffer_size: VulkanDeviceSize,
        vulkan_buffer_usage_flag_s: VulkanBufferUsageFlagS,
        vulkan_memory_property_flag_s: VulkanMemoryPropertyFlagS)
    -> Result<(VulkanBuffer, VulkanDeviceMemory), ErrorFoundationVulkanCooked>
    {
        let vulkan_buffer = Self::create(vulkan_logical_device, vulkan_buffer_size, vulkan_buffer_usage_flag_s)?;
        let vulkan_memory_requirement_s =
            unsafe { vulkan_logical_device.get_buffer_memory_requirements(vulkan_buffer) };
        let vulkan_memory_type_index =
            VulkanMemoryRawPrefab::lookup_type_index(
                vulkan_instance, vulkan_physical_device, vulkan_memory_property_flag_s, vulkan_memory_requirement_s)?;
        let vulkan_memory_allocate_information =
            VulkanMemoryAllocateInformation::builder()
            .allocation_size(vulkan_memory_requirement_s.size)
            .memory_type_index(vulkan_memory_type_index.as_raw())
            .build();
        let vulkan_buffer_memory =
            unsafe { vulkan_logical_device.allocate_memory(&vulkan_memory_allocate_information, None) }
            .or(Err(ErrorFoundationVulkanCookedOwn::VulkanMemoryAllocateFail))?;
        unsafe { vulkan_logical_device.bind_buffer_memory(vulkan_buffer, vulkan_buffer_memory, 0) }
        .or(Err(ErrorFoundationVulkanCookedOwn::VulkanBufferMemoryBindFail))?;
        Ok((vulkan_buffer, vulkan_buffer_memory))
    }

    pub fn copy(
        one_time_vulkan_queue_submit_launcher: VulkanQueueSubmitOneTimeLauncher,
        vulkan_fence_o: Option<VulkanFence>,
        source_vulkan_buffer: VulkanBuffer,
        destination_vulkan_buffer: VulkanBuffer,
        vulkan_buffer_size: VulkanDeviceSize)
    -> Result<(), ErrorFoundationVulkanCooked>
    {
        let action = |vulkan_logical_device: &VulkanDeviceLogical, vulkan_command_buffer|
        {
            let vulkan_buffer_copy = VulkanBufferCopy::builder().size(vulkan_buffer_size).build();
            unsafe { vulkan_logical_device.cmd_copy_buffer(
                vulkan_command_buffer, source_vulkan_buffer, destination_vulkan_buffer, &[vulkan_buffer_copy]) };
            Ok(())
        };
        one_time_vulkan_queue_submit_launcher.launch(action, vulkan_fence_o)
    }
}


pub struct VulkanMemoryRawPrefabBufferGraphicMesh {}

impl VulkanMemoryRawPrefabBufferGraphicMesh {
    pub(super) fn allocate<T>(
        vulkan_instance: &VulkanInstance,
        vulkan_physical_device: VulkanDevicePhysical,
        vulkan_logical_device: &VulkanDeviceLogical,
        one_time_vulkan_queue_submit_launcher: VulkanQueueSubmitOneTimeLauncher,
        vulkan_fence_o: Option<VulkanFence>,
        vertex_s: &Vec<T>)
    -> Result<(VulkanBuffer, VulkanDeviceMemory), ErrorFoundationVulkanCooked>
    {
        let vertex_number = vertex_s.len();
        let vertex_s_buffer_size = (size_of::<T>() * vertex_number) as u64;
        //
        let (staging_vulkan_buffer, staging_vulkan_buffer_memory) =
            VulkanMemoryRawPrefabBuffer::create_with_memory_bound(
                &vulkan_instance,
                vulkan_physical_device,
                &vulkan_logical_device,
                vertex_s_buffer_size,
                VulkanBufferUsageFlagS::TRANSFER_SRC,
                VulkanMemoryPropertyFlagS::HOST_COHERENT | VulkanMemoryPropertyFlagS::HOST_VISIBLE)?;
        let staging_vulkan_vertex_s_buffer_memory_ptr =
            unsafe { vulkan_logical_device.map_memory(
                staging_vulkan_buffer_memory, 0, vertex_s_buffer_size, VulkanMemoryMapFlagS::empty()) }
            .or(Err(ErrorFoundationVulkanCookedOwn::VulkanMemoryMapFail))?;
        unsafe { std::ptr::copy_nonoverlapping(
            vertex_s.as_ptr(), staging_vulkan_vertex_s_buffer_memory_ptr.cast(), vertex_number) };
        unsafe { vulkan_logical_device.unmap_memory(staging_vulkan_buffer_memory) };
        //
        let (dedicated_device_vulkan_buffer, dedicated_device_vulkan_buffer_memory) =
            VulkanMemoryRawPrefabBuffer::create_with_memory_bound(
                &vulkan_instance,
                vulkan_physical_device,
                vulkan_logical_device,
                vertex_s_buffer_size,
                VulkanBufferUsageFlagS::TRANSFER_DST | VulkanBufferUsageFlagS::VERTEX_BUFFER,
                VulkanMemoryPropertyFlagS::DEVICE_LOCAL)?;
        VulkanMemoryRawPrefabBuffer::copy(
            one_time_vulkan_queue_submit_launcher, vulkan_fence_o,
            staging_vulkan_buffer, dedicated_device_vulkan_buffer, vertex_s_buffer_size)?;
        unsafe { vulkan_logical_device.destroy_buffer(staging_vulkan_buffer, None) };
        unsafe { vulkan_logical_device.free_memory(staging_vulkan_buffer_memory, None) };
        Ok((dedicated_device_vulkan_buffer, dedicated_device_vulkan_buffer_memory))
    }
}