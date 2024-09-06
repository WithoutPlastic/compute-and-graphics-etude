use std::collections::HashSet;

use ::library_foundation_reintroduction::vulkan::VulkanBuilderHas;
use ::library_foundation_reintroduction::vulkan::VulkanApplicationInformation;
use ::library_foundation_reintroduction::vulkan::VulkanLayerName;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionName;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessageTypeFlagS;
use ::library_foundation_reintroduction::vulkan::VulkanExtensionDebugUtilityMessageSeverityFlagS;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApi;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionEngine;
use ::library_foundation_reintroduction::vulkan::version::VulkanVersionApplication;
use ::library_foundation_reintroduction::vulkan::engine::VulkanEngineName;
use ::library_foundation_reintroduction::vulkan::application::VulkanApplicationName;

use crate::vulkan_device_physical::feature::VulkanDevicePhysicalFeatureStandardName;
use crate::vulkan_requirement::version::VulkanRequirementVersionApiLeast;


#[derive(Debug, Clone)]
pub struct ConfigVulkanBase<'t> {
    pub version_api_least_requirement: VulkanRequirementVersionApiLeast,
    pub engine_name: VulkanEngineName<'t>,
    pub engine_version: VulkanVersionEngine,
    pub application_name: VulkanApplicationName<'t>,
    pub application_version: VulkanVersionApplication,
    pub instance_layer_validation_enable: bool,
    pub instance_layer_name_s_required: HashSet<VulkanLayerName>,
    pub instance_layer_name_s_optional: HashSet<VulkanLayerName>,
    pub instance_extension_window_name_s: HashSet<VulkanLayerName>,
    pub instance_extension_name_s_required: HashSet<VulkanExtensionName>,
    pub instance_extension_name_s_optional: HashSet<VulkanExtensionName>,
    //pub device_physical_layer_name_s_required: HashSet<VulkanLayerName>,
    //pub device_physical_layer_name_s_optional: HashSet<VulkanLayerName>,
    pub device_physical_extension_name_s_required: HashSet<VulkanExtensionName>,
    pub device_physical_extension_name_s_optional: HashSet<VulkanExtensionName>,
    pub device_physical_feature_name_s_required: HashSet<VulkanDevicePhysicalFeatureStandardName>,
    pub device_physical_feature_name_s_optional: HashSet<VulkanDevicePhysicalFeatureStandardName>,
    pub extension_debug_utility_message_type_flag_s: VulkanExtensionDebugUtilityMessageTypeFlagS,
    pub extension_debug_utility_message_severity_flag_s: VulkanExtensionDebugUtilityMessageSeverityFlagS,
}

impl<'t> ConfigVulkanBase<'t> {
    pub fn create_vulkan_application_information(
        &self, negotiated_api_version: &VulkanVersionApi)
    -> VulkanApplicationInformation
    {
        VulkanApplicationInformation::builder()
        .application_name(self.application_name.as_ref_byte_s_with_nul())
        .application_version(self.application_version.as_raw())
        .engine_name(self.engine_name.as_ref_byte_s_with_nul())
        .engine_version(self.engine_version.as_raw())
        .api_version(negotiated_api_version.as_raw())
        .build()
    }
}