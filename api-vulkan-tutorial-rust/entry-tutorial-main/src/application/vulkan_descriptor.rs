use std::mem::size_of;

use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanErrorCode;
use ::vulkan::VulkanBuffer;
use ::vulkan::VulkanImage;
use ::vulkan::VulkanDescriptorPool;
use ::vulkan::VulkanDescriptorPoolSize;
use ::vulkan::VulkanDescriptorPoolCreateInformation;
use ::vulkan::VulkanDescriptorType;
use ::vulkan::VulkanDescriptorSetLayout;
use ::vulkan::VulkanDescriptorBufferInformation;
use ::vulkan::VulkanDescriptorSetAllocateInformation;
use ::vulkan::VulkanDescriptorSet;
use ::vulkan::VulkanWriteDescriptorSet;
use ::vulkan::VulkanCopyDescriptorSet;

use crate::termination::TerminationProcessMain;
use crate::lib::transform_d3_model_view_projection::TransformD3ModelViewProjection;


pub struct ApplicationVulkanDescriptorPool {}

impl ApplicationVulkanDescriptorPool {
    pub unsafe fn create(vulkan_logical_device: &VulkanDeviceLogical, vulkan_swapchain_image_s: &Vec<VulkanImage>)
     -> Result<VulkanDescriptorPool, TerminationProcessMain>
    {
        let vulkan_descriptor_pool_size =
            VulkanDescriptorPoolSize::builder()
            .type_(VulkanDescriptorType::UNIFORM_BUFFER)
            .descriptor_count(vulkan_swapchain_image_s.len() as u32);
        let vulkan_descriptor_pool_size_s = &[vulkan_descriptor_pool_size];
        let vulkan_descriptor_pool_create_information =
            VulkanDescriptorPoolCreateInformation::builder()
            .pool_sizes(vulkan_descriptor_pool_size_s)
            .max_sets(vulkan_swapchain_image_s.len() as u32);
        let create_vulkan_descriptor_pool_result =
            vulkan_logical_device.create_descriptor_pool(&vulkan_descriptor_pool_create_information, None);
        match create_vulkan_descriptor_pool_result {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err(TerminationProcessMain::InitializationVulkanDescriptorPoolCreateFail(vulkan_error_code));
            },
            Ok(pool) => Ok(pool),
        }
    }
}

pub struct ApplicationVulkanDescriptorSet {}

impl ApplicationVulkanDescriptorSet {
    pub unsafe fn create_all(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_swapchain_image_s: &Vec<VulkanImage>,
        vulkan_descriptor_set_layout: VulkanDescriptorSetLayout,
        vulkan_main_3d_transform_buffer_s: &Vec<VulkanBuffer>,
        vulkan_descriptor_pool: VulkanDescriptorPool,
    )
     -> Result<Vec<VulkanDescriptorSet>, TerminationProcessMain>
    {
        let vulkan_descriptor_set_layout_s =
            vec![vulkan_descriptor_set_layout; vulkan_swapchain_image_s.len()];
        let vulkan_descriptor_set_allocate_information =
            VulkanDescriptorSetAllocateInformation::builder()
            .descriptor_pool(vulkan_descriptor_pool)
            .set_layouts(&vulkan_descriptor_set_layout_s);
        let allocate_vulkan_descriptor_set_s_result =
            vulkan_logical_device.allocate_descriptor_sets(&vulkan_descriptor_set_allocate_information);
        let vulkan_descriptor_set_s =
            match allocate_vulkan_descriptor_set_s_result {
                Err(error) => {
                    let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                    return Err(TerminationProcessMain::InitializationVulkanDescriptorSetSAllocateFail(vulkan_error_code));
                },
                Ok(set_s) => set_s,
            };
        for index in 0..vulkan_swapchain_image_s.len() {
            let vulkan_descriptor_buffer_information =
                VulkanDescriptorBufferInformation::builder()
                .buffer(vulkan_main_3d_transform_buffer_s[index])
                .offset(0)
                .range(size_of::<TransformD3ModelViewProjection>() as u64);
            let vulkan_descriptor_buffer_information_s = &[vulkan_descriptor_buffer_information];
            let vulkan_write_descriptor_set =
                VulkanWriteDescriptorSet::builder()
                .dst_set(vulkan_descriptor_set_s[index])
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(VulkanDescriptorType::UNIFORM_BUFFER)
                .buffer_info(vulkan_descriptor_buffer_information_s);
            vulkan_logical_device.update_descriptor_sets(
                &[vulkan_write_descriptor_set],
                &[] as &[VulkanCopyDescriptorSet]);
        }
        Ok(vulkan_descriptor_set_s)
    }
}