use ::library_foundation_reintroduction::window_uniform::WindowUniformWindow;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogicalCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceVersion1_0;
use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanEntry;
use ::library_foundation_reintroduction::vulkan::VulkanInstance;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessengerCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanInstanceCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanWindow;
use ::library_foundation_reintroduction::vulkan::VulkanSurfaceKhr;
use ::library_foundation_reintroduction::vulkan::VulkanFormat;
use ::library_foundation_reintroduction::vulkan::VulkanDeviceLogical;
use ::library_foundation_reintroduction::vulkan::VulkanRenderPass;
use ::library_foundation_reintroduction::vulkan::VulkanAttachmentDescription;
use ::library_foundation_reintroduction::vulkan::VulkanAttachmentReference;
use ::library_foundation_reintroduction::vulkan::VulkanAttachmentLoadOperation;
use ::library_foundation_reintroduction::vulkan::VulkanAttachmentStoreOperation;
use ::library_foundation_reintroduction::vulkan::VulkanImageLayout;
use ::library_foundation_reintroduction::vulkan::VulkanSampleCountFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanSubpassDescription;
use ::library_foundation_reintroduction::vulkan::VulkanSubpassDependency;
use ::library_foundation_reintroduction::vulkan::VulkanPipelineBindPoint;
use ::library_foundation_reintroduction::vulkan::VulkanAccessFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanPipelineStageFlagS;
use ::library_foundation_reintroduction::vulkan::VULKAN_SUBPASS_EXTERNAL;
use ::library_foundation_reintroduction::vulkan::VulkanImageView;
use ::library_foundation_reintroduction::vulkan::VulkanExtentD2;
use ::library_foundation_reintroduction::vulkan::VulkanFrameBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanFrameBufferCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanRenderPassCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanCommandPool;
use ::library_foundation_reintroduction::vulkan::VulkanCommandPoolCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanCommandPoolCreateFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBufferAllocateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBufferLevel;
use ::library_foundation_reintroduction::vulkan::VulkanCommandBuffer;
use ::library_foundation_reintroduction::vulkan::VulkanFence;
use ::library_foundation_reintroduction::vulkan::VulkanFenceCreateInformation;
use ::library_foundation_reintroduction::vulkan::VulkanFenceCreateFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanSemaphore;
use ::library_foundation_reintroduction::vulkan::VulkanSemaphoreCreateInformation;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApi;
use ::library_foundation_reintroduction::vulkan::queue::VulkanQueueFamilyIndexGraphic;
use ::library_foundation_vulkan_cooked::vulkan_requirement::instance::VulkanRequirementInstance;
use ::library_foundation_vulkan_cooked::initialization::window::InitializationWindowUniform;

use crate::error::foundation_application_guide::ErrorFoundationApplicationGuideOwn;
use crate::error::foundation_application_guide::ErrorFoundationApplicationGuide;
use crate::application_v1_1_c1::config::ApplicationConfig;
use crate::application_v1_1_c1::vulkan_debug::ApplicationVulkanDebug;
use crate::application_v1_1_c1::self_::Application;


pub struct ApplicationInitialization {}

impl ApplicationInitialization {
    fn initialize_vulkan_instance(config: &ApplicationConfig, vulkan_entry: &VulkanEntry)
    -> Result<VulkanInstance, ErrorFoundationApplicationGuide>
    {
        let api_version: VulkanVersionApi =
            vulkan_entry
            .version()
            .map_err(|_| ErrorFoundationApplicationGuideOwn::VulkanInstanceVersionApiQueryFail)?.into();
        config.vulkan.version_api_least_requirement.fulfill_instance(&api_version)?;
        let vulkan_application_information =
            config.vulkan.create_vulkan_application_information(&api_version);
        let vulkan_instance_layer_name_s =
            VulkanRequirementInstance::fulfill_layer_name_s(
                vulkan_entry,
                &config.vulkan.instance_layer_name_s_required,
                &config.vulkan.instance_layer_name_s_optional)?;
        let vulkan_instance_extension_name_s =
            VulkanRequirementInstance::fulfill_extension_name_s(
                vulkan_entry,
                &config.vulkan.instance_extension_window_name_s,
                &config.vulkan.instance_extension_name_s_required,
                &config.vulkan.instance_extension_name_s_optional)?;
        let mut vulkan_debug_messager_create_information =
            VulkanExtensionDebugUtilityMessengerCreateInformation::builder()
            .message_severity(config.vulkan.extension_debug_utility_message_severity_flag_s)
            .message_type(config.vulkan.extension_debug_utility_message_type_flag_s)
            .user_callback(Some(ApplicationVulkanDebug::callback))
            .build();
        //
        let vulkan_instance_create_information =
            VulkanInstanceCreateInformation::builder()
            .application_info(&vulkan_application_information)
            .enabled_layer_names(vulkan_instance_layer_name_s.iter().map(|n| n.as_ptr()).collect::<Vec<_>>().as_ref())
            .enabled_extension_names(vulkan_instance_extension_name_s.iter().map(|n| n.as_ptr()).collect::<Vec<_>>().as_ref())
            .push_next(&mut vulkan_debug_messager_create_information)
            .build();
        match unsafe { vulkan_entry.create_instance(&vulkan_instance_create_information, None) } {
            Err(_e) => Err(ErrorFoundationApplicationGuideOwn::VulkanInstanceCreateFail)?,
            Ok(i) => Ok(i),
        }
    }

    fn initialize_vulkan_surface(vulkan_instance: &VulkanInstance, window: &WindowUniformWindow)
    -> Result<VulkanSurfaceKhr, ErrorFoundationApplicationGuide>
    {
        match unsafe { VulkanWindow::create_surface(&vulkan_instance, window, window) } {
            Err(_e) => Err(ErrorFoundationApplicationGuideOwn::VulkanSurfaceCreateFail)?,
            Ok(s) => Ok(s),
        }
    }

    fn initialize_render_pass(vulkan_logical_device: &VulkanDeviceLogical, vulkan_swapchain_format: VulkanFormat)
    -> Result<VulkanRenderPass, ErrorFoundationApplicationGuide>
    {
        let vulkan_color_attachment =
            VulkanAttachmentDescription::builder()
            .format(vulkan_swapchain_format)
            .samples(VulkanSampleCountFlagS::_1)
            .load_op(VulkanAttachmentLoadOperation::CLEAR)
            .store_op(VulkanAttachmentStoreOperation::STORE)
            .stencil_load_op(VulkanAttachmentLoadOperation::DONT_CARE)
            .stencil_store_op(VulkanAttachmentStoreOperation::DONT_CARE)
            .initial_layout(VulkanImageLayout::UNDEFINED)
            .final_layout(VulkanImageLayout::PRESENT_SRC_KHR)
            .build();
        let vulkan_color_attachment_reference =
            VulkanAttachmentReference::builder()
            .attachment(0)
            .layout(VulkanImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();
        let vulkan_attachment_s = &[vulkan_color_attachment];
        let vulkan_color_attachment_reference_s = &[vulkan_color_attachment_reference];
        //
        let vulkan_subpass_description =
            VulkanSubpassDescription::builder()
            .pipeline_bind_point(VulkanPipelineBindPoint::GRAPHICS)
            .color_attachments(vulkan_color_attachment_reference_s)
            .build();
        let vulkan_subpass_dependency =
            VulkanSubpassDependency::builder()
            .src_subpass(VULKAN_SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(VulkanPipelineStageFlagS::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(VulkanAccessFlagS::empty())
            .dst_stage_mask(VulkanPipelineStageFlagS::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(VulkanAccessFlagS::COLOR_ATTACHMENT_WRITE)
            .build();
        let vulkan_subpass_description_s = &[vulkan_subpass_description];
        let vulkan_subpass_dependency_s = &[vulkan_subpass_dependency];
        //
        let vulkan_render_pass_create_information =
            VulkanRenderPassCreateInformation::builder()
            .attachments(vulkan_attachment_s)
            .subpasses(vulkan_subpass_description_s)
            .dependencies(vulkan_subpass_dependency_s)
            .build();
        unsafe { vulkan_logical_device.create_render_pass(&vulkan_render_pass_create_information, None) }
        .or(Err(ErrorFoundationApplicationGuideOwn::VulkanRenderPassCreateFail.into()))
    }

    fn initialize_frame_buffer_s(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_image_view_s: &[VulkanImageView],
        vulkan_render_pass: VulkanRenderPass,
        vulkan_extent: VulkanExtentD2)
    -> Result<Vec<VulkanFrameBuffer>, ErrorFoundationApplicationGuide>
    {
        vulkan_image_view_s
        .iter()
        .try_fold(Vec::new(), |mut result_frame_buffer_s, iv| {
            let vulkan_frame_buffer_create_information =
                VulkanFrameBufferCreateInformation::builder()
                .render_pass(vulkan_render_pass)
                .attachments(&[*iv])
                .width(vulkan_extent.width)
                .height(vulkan_extent.height)
                .layers(1)
                .build();
            unsafe { vulkan_logical_device.create_framebuffer(&vulkan_frame_buffer_create_information, None) }
            .map(|fb| { result_frame_buffer_s.push(fb); result_frame_buffer_s })
        })
        .or(Err(ErrorFoundationApplicationGuideOwn::VulkanFrameBufferCreateFail.into()))
    }

    fn initialize_command_pool_and_buffer_main(
        vulkan_logical_device: &VulkanDeviceLogical,
        vulkan_graphic_queue_family_index: VulkanQueueFamilyIndexGraphic)
    -> Result<(VulkanCommandPool, VulkanCommandBuffer), ErrorFoundationApplicationGuide>
    {
        let vulkan_command_pool_create_information =
            VulkanCommandPoolCreateInformation::builder()
            .flags(VulkanCommandPoolCreateFlagS::RESET_COMMAND_BUFFER)
            .queue_family_index(vulkan_graphic_queue_family_index.as_raw())
            .build();
        let main_vulkan_command_pool =
            unsafe { vulkan_logical_device.create_command_pool(&vulkan_command_pool_create_information, None) }
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanCommandPoolCreateFail))?;
        let vulkan_command_buffer_allocate_information =
            VulkanCommandBufferAllocateInformation::builder()
            .command_pool(main_vulkan_command_pool)
            .level(VulkanCommandBufferLevel::PRIMARY)
            .command_buffer_count(1)
            .build();
        let vulkan_command_buffer_s =
            unsafe { vulkan_logical_device.allocate_command_buffers(&vulkan_command_buffer_allocate_information) }
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanCommandBufferAllocateFail))?;
        assert!(vulkan_command_buffer_s.len() == 1);
        let main_vulkan_command_buffer = vulkan_command_buffer_s[0];
        Ok((main_vulkan_command_pool, main_vulkan_command_buffer))
    }

    fn initialize_synchronization_primitive_set(vulkan_logical_device: &VulkanDeviceLogical)
    -> Result<(VulkanFence, VulkanSemaphore, VulkanSemaphore), ErrorFoundationApplicationGuide>
    {
        let vulkan_fence_create_information =
            VulkanFenceCreateInformation::builder().flags(VulkanFenceCreateFlagS::SIGNALED).build();
        let frame_rendering_finished_vulkan_fence =
            unsafe { vulkan_logical_device.create_fence(&vulkan_fence_create_information, None) }
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanFenceCreateFail))?;
        let vulkan_semaphore_create_information = VulkanSemaphoreCreateInformation::builder().build();
        let image_available_vulkan_semaphore =
            unsafe { vulkan_logical_device.create_semaphore(&vulkan_semaphore_create_information, None) }
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanSemaphoreCreateFail))?;
        let render_finished_vulkan_semaphore =
            unsafe { vulkan_logical_device.create_semaphore(&vulkan_semaphore_create_information, None) }
            .or(Err(ErrorFoundationApplicationGuideOwn::VulkanSemaphoreCreateFail))?;
        Ok((frame_rendering_finished_vulkan_fence, image_available_vulkan_semaphore, render_finished_vulkan_semaphore))
    }

    //
    pub fn initialize<'t>(config: ApplicationConfig<'t>)
    -> Result<Application<'t>, ErrorFoundationApplicationGuide>
    {
        todo!()
    }
}