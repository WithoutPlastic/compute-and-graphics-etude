use ::png::DecodingError as FormatPngDecodingError;
use ::libloading::Error as LibraryLoadingError;
use ::window_uniform::WindowUniformErrorOperationSystem;
use ::vulkan::extend::VulkanErrorCode;
use ::tobj::LoadError as ModelFormatObjLoadError;


pub enum TerminationProcessMain {
    Ok,
    InitializationLoggerConsoleFail,
    InitializationWindowUniformFail(WindowUniformErrorOperationSystem),
    InitializationVulkanLibraryLoadingFail(LibraryLoadingError),
    InitializationVulkanEntryCreateFail(Box<dyn std::error::Error + Send + Sync + 'static>),
    InitializationVulkanInstanceCreateFail(VulkanErrorCode),
    InitializationVulkanValidationLayerNotSupport,
    InitializationVulkanEnumeratePhysicalDeviceFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalAllNotQualified,
    InitializationVulkanDeviceLogicalCreateFail(VulkanErrorCode),
    InitializationVulkanSurfaceCreateFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalSurfaceCapabilitySGetFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalSurfaceFormatSGetFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalSurfacePresentModeSGetFail(VulkanErrorCode),
    InitializationVulkanSwapchainCreateFail(VulkanErrorCode),
    InitializationVulkanSwapchainImageSGetFail(VulkanErrorCode),
    InitializationVulkanDevicePhysicalExtensionPropertySEnumerateFail(VulkanErrorCode),
    InitializationVulkanImageViewCreateFail(VulkanErrorCode),
    InitializationVulkanShaderByteCodeAlignmentIncorrect,
    InitializationVulkanShaderModuleCreateFail(VulkanErrorCode),
    InitializationVulkanPipelineLayoutCreateFail(VulkanErrorCode),
    InitializationVulkanRenderPassCreateFail(VulkanErrorCode),
    InitializationVulkanGraphicsPipelineSCreateFail(VulkanErrorCode),
    InitializationVulkanFrameBufferCreateFail(VulkanErrorCode),
    InitializationVulkanCommandPoolCreateFail(VulkanErrorCode),
    InitializationVulkanCommandBufferSAllocateFail(VulkanErrorCode),
    InitializationVulkanCommandBufferBeginFail(VulkanErrorCode),
    InitializationVulkanCommandBufferEndFail(VulkanErrorCode),
    InitializationVulkanSemaphoreCreateFail(VulkanErrorCode),
    InitializationVulkanFenceCreateFail(VulkanErrorCode),
    InitializationVulkanDeviceWaitIdleFail(VulkanErrorCode),
    InitializationVulkanFenceWaitFail(VulkanErrorCode),
    InitializationVulkanAcquireNextImageFail(VulkanErrorCode),
    InitializationVulkanFenceResetFail(VulkanErrorCode),
    InitializationVulkanQueueSubmitFail(VulkanErrorCode),
    InitializationVulkanQueuePresentFail(VulkanErrorCode),
    InitializationVulkanBufferCreateFail(VulkanErrorCode),
    InitializationVulkanMemoryTypeNotSupport,
    InitializationVulkanMemoryAllocateFail(VulkanErrorCode),
    InitializationVulkanMemoryBufferBindFail(VulkanErrorCode),
    InitializationVulkanMemoryMapFail(VulkanErrorCode),
    InitializationVulkanDescriptorSetLayoutCreateFail(VulkanErrorCode),
    InitializationVulkanDescriptorPoolCreateFail(VulkanErrorCode),
    InitializationVulkanDescriptorSetSAllocateFail(VulkanErrorCode),
    InitializationFileOpenFail(String),
    InitializationFormatPngDecodingError(FormatPngDecodingError),
    InitializationVulkanImageCreateFail(VulkanErrorCode),
    InitializationVulkanTextureImageLayoutTransitionNotSupport,
    InitializationVulkanSamplerCreateFail(VulkanErrorCode),
    InitializationVulkanPhysicalDeviceFeatureSamplerAnisotropyNotSupport,
    InitializationVulkanFormatFeatureNotSupport,
    InitializationModelFormatObjLoadingError(ModelFormatObjLoadError),
    InitializationVulkanDevicePhysicalSampledImageFilterLinearNotSupport,
    InitializationVulkanResetCommandPoolFail(VulkanErrorCode),
}

impl TerminationProcessMain {
    fn to_exit_code(self) -> u8 {
        match self {
            Self::Ok => 0u8,
            Self::InitializationLoggerConsoleFail => 1u8,
            Self::InitializationWindowUniformFail(_) => 2u8,
            Self::InitializationVulkanLibraryLoadingFail(_) => 3u8,
            Self::InitializationVulkanEntryCreateFail(_) => 4u8,
            Self::InitializationVulkanInstanceCreateFail(_) => 5u8,
            Self::InitializationVulkanValidationLayerNotSupport => 6u8,
            Self::InitializationVulkanEnumeratePhysicalDeviceFail(_) => 7u8,
            Self::InitializationVulkanDevicePhysicalAllNotQualified => 8u8,
            Self::InitializationVulkanDeviceLogicalCreateFail(_) => 9u8,
            Self::InitializationVulkanSurfaceCreateFail(_) => 10u8,
            Self::InitializationVulkanDevicePhysicalSurfaceCapabilitySGetFail(_) => 11u8,
            Self::InitializationVulkanDevicePhysicalSurfaceFormatSGetFail(_) => 12u8,
            Self::InitializationVulkanDevicePhysicalSurfacePresentModeSGetFail(_) => 13u8,
            Self::InitializationVulkanSwapchainCreateFail(_) => 14u8,
            Self::InitializationVulkanSwapchainImageSGetFail(_) => 15u8,
            Self::InitializationVulkanDevicePhysicalExtensionPropertySEnumerateFail(_) => 16u8,
            Self::InitializationVulkanImageViewCreateFail(_) => 17u8,
            Self::InitializationVulkanShaderByteCodeAlignmentIncorrect => 18u8,
            Self::InitializationVulkanShaderModuleCreateFail(_) => 19u8,
            Self::InitializationVulkanPipelineLayoutCreateFail(_) => 20u8,
            Self::InitializationVulkanRenderPassCreateFail(_) => 21u8,
            Self::InitializationVulkanGraphicsPipelineSCreateFail(_) => 22u8,
            Self::InitializationVulkanFrameBufferCreateFail(_) => 23u8,
            Self::InitializationVulkanCommandPoolCreateFail(_) => 24u8,
            Self::InitializationVulkanCommandBufferSAllocateFail(_) => 25u8,
            Self::InitializationVulkanCommandBufferBeginFail(_) => 26u8,
            Self::InitializationVulkanCommandBufferEndFail(_) => 27u8,
            Self::InitializationVulkanSemaphoreCreateFail(_) => 28u8,
            Self::InitializationVulkanFenceCreateFail(_) => 29u8,
            Self::InitializationVulkanDeviceWaitIdleFail(_) => 30u8,
            Self::InitializationVulkanFenceWaitFail(_) => 31u8,
            Self::InitializationVulkanAcquireNextImageFail(_) => 32u8,
            Self::InitializationVulkanFenceResetFail(_) => 33u8,
            Self::InitializationVulkanQueueSubmitFail(_) => 34u8,
            Self::InitializationVulkanQueuePresentFail(_) => 35u8,
            Self::InitializationVulkanBufferCreateFail(_) => 36u8,
            Self::InitializationVulkanMemoryTypeNotSupport => 37u8,
            Self::InitializationVulkanMemoryAllocateFail(_) => 38u8,
            Self::InitializationVulkanMemoryBufferBindFail(_) => 39u8,
            Self::InitializationVulkanMemoryMapFail(_) => 40u8,
            Self::InitializationVulkanDescriptorSetLayoutCreateFail(_) => 41u8,
            Self::InitializationVulkanDescriptorPoolCreateFail(_) => 42u8,
            Self::InitializationVulkanDescriptorSetSAllocateFail(_) => 43u8,
            Self::InitializationFileOpenFail(_) => 44u8,
            Self::InitializationFormatPngDecodingError(_) => 45u8,
            Self::InitializationVulkanImageCreateFail(_) => 46u8,
            Self::InitializationVulkanTextureImageLayoutTransitionNotSupport => 47u8,
            Self::InitializationVulkanSamplerCreateFail(_) => 48u8,
            Self::InitializationVulkanPhysicalDeviceFeatureSamplerAnisotropyNotSupport => 49u8,
            Self::InitializationVulkanFormatFeatureNotSupport => 50u8,
            Self::InitializationModelFormatObjLoadingError(_) => 51u8,
            Self::InitializationVulkanDevicePhysicalSampledImageFilterLinearNotSupport => 52u8,
            Self::InitializationVulkanResetCommandPoolFail(_) => 53u8,
        }
    }
}

impl std::process::Termination for TerminationProcessMain {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::from(self.to_exit_code())
    }
}

macro_rules! termination_vulkan_error {
    (normal1, $result_identifier:ident, $termination_enum_item:expr) => {
        match $result_identifier {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                Err($termination_enum_item(vulkan_error_code))
            },
            Ok(value) => Ok(value),
        }
    };
    (return1, $result_identifier:ident, $termination_enum_item:expr) => {
        match $result_identifier {
            Err(error) => {
                let vulkan_error_code = VulkanErrorCode::new(error.as_raw());
                return Err($termination_enum_item(vulkan_error_code))
            },
            Ok(value) => value,
        }
    };
}