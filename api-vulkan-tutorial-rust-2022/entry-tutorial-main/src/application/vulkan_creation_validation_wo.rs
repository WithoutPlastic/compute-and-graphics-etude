use std::time::Instant;

use ::window_uniform::prelude::*;
use ::vulkan::VULKAN_LIBRARY_FILE_NAME;
use ::vulkan::extend::VulkanErrorCode;
use ::vulkan::VulkanLibraryLoader;
use ::vulkan::VulkanInstanceCreateInformation;
use ::vulkan::VulkanWindow;
use ::vulkan::prelude::version1_2::*;
use ::vulkan::VulkanExtensionName;
use ::vulkan::VulkanCommandBuffer;
use ::vulkan::VulkanCommandBufferLevel;
use ::vulkan::VulkanCommandBufferAllocateInformation;

use crate::termination::TerminationProcessMain;
use crate::lib::window_viewport::WINDOW_VIEWPORT_LOGICAL_NUMBER_MAX;
use crate::lib::d3_model_mesh::D3ModelMesh;
use crate::lib::window_viewport::WindowViewportLogicalNumber;
use crate::data::d3_model_resource::DataD3ModelResource;
use crate::data::d3_model_mesh_tutorial_simple::DataD3ModelMeshTutorialSimple;
use crate::data::d3_model_mesh_tutorial_format_obj::DataD3ModelMeshTutorialFormatObj;
use crate::data::d3_model_texture_tutorial_simple::DataD3ModelTextureTutorialSimple;
use crate::data::d3_model_texture_tutorial_format_obj::DataD3ModelTextureTutorialFormatObj;
use crate::application::main::Application;
use crate::application::vulkan_share::ApplicationVulkanShare;
use crate::application::vulkan_device_physical::ApplicationVulkanDevicePhysical;
use crate::application::vulkan_device_logical::ApplicationVulkanDeviceLogical;
use crate::application::vulkan_swapchain::ApplicationVulkanSwapchain;
use crate::application::vulkan_swapchain_image_view::ApplicationSwapchainImageView;
use crate::application::vulkan_pipeline::ApplicationVulkanPipeline;
use crate::application::vulkan_render_pass::ApplicationVulkanRenderPass;
use crate::application::vulkan_frame_buffer::ApplicationVulkanFrameBuffer;
use crate::application::vulkan_command_pool::ApplicationVulkanCommandPool;
use crate::application::vulkan_command_buffer::ApplicationVulkanCommandBufferSwapchainImage;
use crate::application::vulkan_synchronization::ApplicationVulkanSynchronization;
use crate::application::vulkan_vertex::ApplicationVulkanVertexBuffer;
use crate::application::vulkan_vertex_index::ApplicationVulkanVertexIndexBuffer;
use crate::application::vulkan_transform_d3_descriptor::ApplicationVulkanTransformD3Descriptor;
use crate::application::vulkan_transform_d3_buffer::ApplicationVulkanTransformD3Buffer;
use crate::application::vulkan_descriptor::ApplicationVulkanDescriptorPool;
use crate::application::vulkan_descriptor::ApplicationVulkanDescriptorSet;
use crate::application::vulkan_texture_image::ApplicationVulkanTextureImage;
use crate::application::vulkan_descriptor::ApplicationVulkanDescriptorSetLayout;
use crate::application::vulkan_depth::ApplicationVulkanDepth;
use crate::application::vulkan_mipmap::ApplicationVulkanMipmap;
use crate::application::vulkan_anti_aliasing_multisampling::ApplicationVulkanAntiAliasingMultiSampling;


pub struct ApplicationVulkanCreationValidationWo {}

impl ApplicationVulkanCreationValidationWo {
    pub unsafe fn create(
        window: &WindowUniformWindow,
        vulkan_extension_s: &[VulkanExtensionName],
        d3_model_resource_name: DataD3ModelResource)
     -> Result<Application, TerminationProcessMain>
    {
        let (d3_model_mesh, texture_image_data, texture_image_information) =
            match d3_model_resource_name {
                DataD3ModelResource::TutorialSimple(resource_name) => {
                    let d3_model_mesh =
                        DataD3ModelMeshTutorialSimple::load(resource_name)?;
                    let (texture_image_data, texture_image_information) =
                        DataD3ModelTextureTutorialSimple::load(resource_name)?;
                    (D3ModelMesh::TutorialSimple(d3_model_mesh), texture_image_data, texture_image_information)
                },
                DataD3ModelResource::TutorialFormatObj(resource_name) => {
                    let d3_model_mesh =
                        DataD3ModelMeshTutorialFormatObj::load(resource_name)?;
                    let (texture_image_data, texture_image_information) =
                        DataD3ModelTextureTutorialFormatObj::load(resource_name)?;
                    (D3ModelMesh::TutorialFormatObj(d3_model_mesh), texture_image_data, texture_image_information)
                },
            };
        let vulkan_mip_level =
            ApplicationVulkanMipmap::calculate_level_max(texture_image_information.width, texture_image_information.height);

        let vulkan_library_loader =
            match VulkanLibraryLoader::new(VULKAN_LIBRARY_FILE_NAME) {
                Err(error) => return Err(TerminationProcessMain::InitializationVulkanLibraryLoadingFail(error)),
                Ok(loader) => loader,
            };
        let vulkan_entry =
            match VulkanEntry::new(vulkan_library_loader) {
                Err(error) => return Err(TerminationProcessMain::InitializationVulkanEntryCreateFail(error)),
                Ok(entry) => entry,
            };
        let vulkan_instance =
            match Self::create_vulkan_instance(window, &vulkan_entry) {
                Err(error) => return Err(error),
                Ok(instance) => instance,
            };
        let create_vulkan_surface_result = VulkanWindow::create_surface(&vulkan_instance, window);
        let vulkan_surface = termination_vulkan_error!(return1,
            create_vulkan_surface_result, TerminationProcessMain::InitializationVulkanSurfaceCreateFail);
        let (vulkan_physical_device,
             vulkan_graphic_queue_family_index,
             vulkan_surface_queue_family_index) =
            match ApplicationVulkanDevicePhysical::pick(&vulkan_instance, vulkan_surface, vulkan_extension_s) {
                Err(error) => return Err(error),
                Ok(device_and_queue_index) => device_and_queue_index,
            };
        let create_vulkan_logical_device_result =
            ApplicationVulkanDeviceLogical::create(
                &vulkan_instance,
                vulkan_physical_device,
                vulkan_extension_s,
                vulkan_graphic_queue_family_index,
                vulkan_surface_queue_family_index);
        let (vulkan_logical_device, vulkan_graphic_queue, vulkan_present_queue) =
            match create_vulkan_logical_device_result {
                Err(error) => return Err(error),
                Ok(device_and_queue) => device_and_queue,
            };
        let vulkan_anti_aliasing_multisampling_number =
            ApplicationVulkanAntiAliasingMultiSampling::get_sample_count_max(&vulkan_instance, vulkan_physical_device);
        let (vulkan_surface_format, vulkan_extent, vulkan_swapchain, vulkan_image_s) =
            ApplicationVulkanSwapchain::create(
                window, &vulkan_instance, vulkan_surface, &vulkan_logical_device,
                vulkan_physical_device, vulkan_graphic_queue_family_index, vulkan_surface_queue_family_index
            )?;
        let vulkan_image_view_s =
            ApplicationSwapchainImageView::create_all(
                &vulkan_logical_device, vulkan_surface_format, &vulkan_image_s)?;
        let vulkan_render_pass =
            ApplicationVulkanRenderPass::create(
                &vulkan_instance, vulkan_physical_device, &vulkan_logical_device,
                vulkan_surface_format, vulkan_anti_aliasing_multisampling_number)?;
        let vulkan_3d_transform_descriptor_set_layout_binding =
            ApplicationVulkanTransformD3Descriptor::create_set_layout_binding()?;
        let vulkan_texture_sampler_transform_descriptor_set_layout_binding =
            ApplicationVulkanTextureImage::create_sampler_descriptor_set_layout_binding()?;
        let vulkan_descriptor_set_layout =
            ApplicationVulkanDescriptorSetLayout::create(
                &vulkan_logical_device,
                vulkan_3d_transform_descriptor_set_layout_binding,
                vulkan_texture_sampler_transform_descriptor_set_layout_binding)?;
        let (vulkan_pipeline, vulkan_pipeline_layout) =
            ApplicationVulkanPipeline::create_layout(
                &vulkan_logical_device, vulkan_extent, vulkan_render_pass,
                vulkan_descriptor_set_layout, vulkan_anti_aliasing_multisampling_number)?;
        let vulkan_main_command_pool =
            ApplicationVulkanCommandPool::create_main(&vulkan_logical_device, vulkan_graphic_queue_family_index)?;
        let vulkan_swapchain_image_command_pool_s =
            ApplicationVulkanCommandPool::create_swapchain_image_all(
                &vulkan_logical_device, &vulkan_image_s, vulkan_graphic_queue_family_index)?;
        let (vulkan_anti_aliasing_multisampling_image,
             vulkan_anti_aliasing_multisampling_image_memory,
             vulkan_anti_aliasing_multisampling_image_view) =
            ApplicationVulkanAntiAliasingMultiSampling::get_image_memory_view(
                &vulkan_instance, vulkan_physical_device, &vulkan_logical_device,
                vulkan_extent, vulkan_surface_format,
                vulkan_anti_aliasing_multisampling_number)?;
        let (vulkan_depth_image, vulkan_depth_image_memory, vulkan_depth_image_view) =
            ApplicationVulkanDepth::create_image_memory_view(
                &vulkan_instance, vulkan_physical_device, &vulkan_logical_device,
                vulkan_extent, vulkan_anti_aliasing_multisampling_number)?;
        let vulkan_frame_buffer_s =
            ApplicationVulkanFrameBuffer::create_all(
                &vulkan_logical_device, &vulkan_image_view_s,
                vulkan_depth_image_view, vulkan_anti_aliasing_multisampling_image_view,
                vulkan_render_pass, vulkan_extent)?;
        let (vulkan_texture_image, vulkan_texture_image_memory) =
            ApplicationVulkanTextureImage::create_buffer_with_memory(
                &vulkan_instance, vulkan_physical_device, &vulkan_logical_device,
                vulkan_main_command_pool, vulkan_graphic_queue,
                texture_image_data, texture_image_information, vulkan_mip_level)?;
        let vulkan_texture_image_view =
            ApplicationVulkanTextureImage::create_view(&vulkan_logical_device, vulkan_texture_image, vulkan_mip_level)?;
        let vulkan_texture_sampler =
            ApplicationVulkanTextureImage::create_sampler(&vulkan_logical_device, vulkan_mip_level)?;
        let (vulkan_vertex_buffer, vulkan_vertex_buffer_memory) =
            ApplicationVulkanVertexBuffer::create(
                &vulkan_instance, vulkan_physical_device, &vulkan_logical_device,
                vulkan_main_command_pool, vulkan_graphic_queue, &d3_model_mesh)?;
        let (vulkan_vertex_index_buffer, vulkan_vertex_index_buffer_memory) =
            ApplicationVulkanVertexIndexBuffer::create(
                &vulkan_instance, vulkan_physical_device, &vulkan_logical_device,
                vulkan_main_command_pool, vulkan_graphic_queue, &d3_model_mesh)?;
        let (vulkan_main_3d_transform_buffer_s, vulkan_main_3d_transform_buffer_memory_s) =
            ApplicationVulkanTransformD3Buffer::create_main_all(
                &vulkan_instance, vulkan_physical_device, &vulkan_logical_device, &vulkan_image_s)?;
        let vulkan_descriptor_pool =
            ApplicationVulkanDescriptorPool::create(&vulkan_logical_device, &vulkan_image_s)?;
        let vulkan_descriptor_set_s =
            ApplicationVulkanDescriptorSet::create_all(
                &vulkan_logical_device, &vulkan_image_s,
                vulkan_descriptor_set_layout, &vulkan_main_3d_transform_buffer_s, vulkan_descriptor_pool,
                vulkan_texture_image_view, vulkan_texture_sampler)?;
        let vulkan_swapchain_image_command_buffer_s =
            ApplicationVulkanCommandBufferSwapchainImage::create_blank_all(
                &vulkan_logical_device, &vulkan_image_s, &vulkan_swapchain_image_command_pool_s)?;
        let (vulkan_image_available_semaphore_s, vulkan_render_finished_semaphore_s,
             vulkan_slide_in_flight_fence_s, vulkan_image_in_flight_fence_s) =
            ApplicationVulkanSynchronization::create_all(&vulkan_logical_device, &vulkan_image_s)?;
        let mut vulkan_swapchain_image_logical_viewport_command_buffer_tt: Vec<Vec<VulkanCommandBuffer>> = Vec::new();
        vulkan_swapchain_image_logical_viewport_command_buffer_tt.reserve_exact(vulkan_image_s.len());
        for image_index in 0..vulkan_image_s.len() {
            let vulkan_swapchain_image_command_pool =
                vulkan_swapchain_image_command_pool_s[image_index];
            let vulkan_logical_viewport_command_buffer_allocate_information =
                VulkanCommandBufferAllocateInformation::builder()
                .command_pool(vulkan_swapchain_image_command_pool)
                .level(VulkanCommandBufferLevel::SECONDARY)
                .command_buffer_count(1);
            let mut vulkan_logical_viewport_command_buffer_s: Vec<VulkanCommandBuffer> = Vec::new();
            vulkan_logical_viewport_command_buffer_s.reserve_exact(WINDOW_VIEWPORT_LOGICAL_NUMBER_MAX);
            for _viewport_index in 0..WINDOW_VIEWPORT_LOGICAL_NUMBER_MAX {
                let allocate_vulkan_command_buffer_result =
                    vulkan_logical_device.allocate_command_buffers(
                        &vulkan_logical_viewport_command_buffer_allocate_information);
                let vulkan_logical_viewport_command_buffer = termination_vulkan_error!(return1,
                    allocate_vulkan_command_buffer_result, TerminationProcessMain::InitializationVulkanCommandBufferSAllocateFail)[0];
                vulkan_logical_viewport_command_buffer_s.push(vulkan_logical_viewport_command_buffer);
            }
            vulkan_swapchain_image_logical_viewport_command_buffer_tt.push(vulkan_logical_viewport_command_buffer_s);
        }
        Ok(Application {
            instant_start: Instant::now(),
            signal_window_resized: false,
            vulkan_entry: vulkan_entry,
            vulkan_instance: vulkan_instance,
            vulkan_debug_messenger: None,
            vulkan_device_physical: vulkan_physical_device,
            vulkan_device_logical: vulkan_logical_device,
            vulkan_queue_family_index_graphic: vulkan_graphic_queue_family_index,
            vulkan_queue_graphic: vulkan_graphic_queue,
            vulkan_surface: vulkan_surface,
            vulkan_queue_family_index_present: vulkan_surface_queue_family_index,
            vulkan_queue_present: vulkan_present_queue,
            vulkan_swapchain_format: vulkan_surface_format,
            vulkan_swapchain_extent: vulkan_extent,
            vulkan_swapchain: vulkan_swapchain,
            vulkan_swapchain_image_s: vulkan_image_s,
            vulkan_swapchain_image_view_s: vulkan_image_view_s,
            vulkan_render_pass: vulkan_render_pass,
            vulkan_pipeline_layout: vulkan_pipeline_layout,
            vulkan_pipeline: vulkan_pipeline,
            vulkan_frame_buffer_s: vulkan_frame_buffer_s,
            vulkan_command_pool_main: vulkan_main_command_pool,
            vulkan_command_pool_swapchain_image_s: vulkan_swapchain_image_command_pool_s,
            vulkan_command_buffer_swapchain_image_s: vulkan_swapchain_image_command_buffer_s,
            vulkan_command_buffer_swapchain_image_logical_viewport_tt: vulkan_swapchain_image_logical_viewport_command_buffer_tt,
            vulkan_semaphore_s_image_available: vulkan_image_available_semaphore_s,
            vulkan_semaphore_s_render_finished: vulkan_render_finished_semaphore_s,
            vulkan_fence_s_in_flight_slide: vulkan_slide_in_flight_fence_s,
            vulkan_fence_s_in_flight_image: vulkan_image_in_flight_fence_s,
            vulkan_frame_index_current: 0,
            vulkan_vertex_buffer: vulkan_vertex_buffer,
            vulkan_vertex_buffer_memory: vulkan_vertex_buffer_memory,
            vulkan_vertex_index_buffer: vulkan_vertex_index_buffer,
            vulkan_vertex_index_buffer_memory: vulkan_vertex_index_buffer_memory,
            vulkan_transform_d3_main_buffer_s: vulkan_main_3d_transform_buffer_s,
            vulkan_transform_d3_main_buffer_memory_s: vulkan_main_3d_transform_buffer_memory_s,
            vulkan_descriptor_set_layout: vulkan_descriptor_set_layout,
            vulkan_descriptor_pool: vulkan_descriptor_pool,
            vulkan_descriptor_set_s: vulkan_descriptor_set_s,
            vulkan_texture_image: vulkan_texture_image,
            vulkan_texture_image_memory: vulkan_texture_image_memory,
            vulkan_texture_image_view: vulkan_texture_image_view,
            vulkan_texture_sampler: vulkan_texture_sampler,
            vulkan_depth_image: vulkan_depth_image,
            vulkan_depth_image_memory: vulkan_depth_image_memory,
            vulkan_depth_image_view: vulkan_depth_image_view,
            vulkan_mip_level: vulkan_mip_level,
            vulkan_anti_aliasing_multisampling_number: vulkan_anti_aliasing_multisampling_number,
            vulkan_anti_aliasing_multisampling_image: vulkan_anti_aliasing_multisampling_image,
            vulkan_anti_aliasing_multisampling_image_memory: vulkan_anti_aliasing_multisampling_image_memory,
            vulkan_anti_aliasing_multisampling_image_view: vulkan_anti_aliasing_multisampling_image_view,
            window_viewport_logical_number: WindowViewportLogicalNumber::new(1),
            d3_model_mesh: d3_model_mesh,
        })
    }

    unsafe fn create_vulkan_instance(
        window: &WindowUniformWindow, vulkan_entry: &VulkanEntry) -> Result<VulkanInstance, TerminationProcessMain> {
        let vulkan_application_information =
            ApplicationVulkanShare::create_vulkan_instance_application_information();
        let vulkan_application_extension_s =
            ApplicationVulkanShare::create_vulkan_instance_application_extension_s(window);
        let vulkan_instance_create_information =
            VulkanInstanceCreateInformation::builder()
            .application_info(&vulkan_application_information)
            .enabled_extension_names(&vulkan_application_extension_s);
        let create_vulkan_instance_result =
            vulkan_entry.create_instance(&vulkan_instance_create_information, None);
        let vulkan_instance = termination_vulkan_error!(return1,
            create_vulkan_instance_result, TerminationProcessMain::InitializationVulkanInstanceCreateFail);
        Ok(vulkan_instance)
    }
}