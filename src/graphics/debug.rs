use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use ash::vk;

pub unsafe extern "system" fn debug_callback(
    flags : vk::DebugReportFlagsEXT,
    object_type : vk::DebugReportObjectTypeEXT,
    object : u64,
    location : usize,
    message_code : i32,
    p_layer_prefix : *const c_char,
    p_message : *const c_char,
    p_user_data : *mut c_void,
) -> u32 {
    match flags {
        vk::DebugReportFlagsEXT::ERROR => error!("{:?}", CStr::from_ptr(p_message)),
        vk::DebugReportFlagsEXT::WARNING | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING => warn!("{:?}", CStr::from_ptr(p_message)),
        vk::DebugReportFlagsEXT::DEBUG => debug!("{:?}", CStr::from_ptr(p_message)),
        vk::DebugReportFlagsEXT::INFORMATION => info!("{:?}", CStr::from_ptr(p_message)),
        _ => trace!("{:?}", CStr::from_ptr(p_message)),
    }
    vk::FALSE
}