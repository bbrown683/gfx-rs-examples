use std::os::raw::c_void;
use ash::extensions::{ext::DebugReport, khr::Surface, khr::XlibSurface};
use ash::version::{EntryV1_0, InstanceV1_0};
use ash::vk;
use winit::platform::unix::WindowExtUnix;
use winit::window::Window;

pub fn create_surface<E: EntryV1_0, I: InstanceV1_0>(entry : &E, instance : &I, window : &Window)
    -> vk::SurfaceKHR {
    let xlib_display = window.xlib_display().unwrap();
    let xlib_window = window.xlib_window().unwrap();
    let xlib_create_info = vk::XlibSurfaceCreateInfoKHR::builder()
        .dpy(xlib_display as *mut vk::Display)
        .window(xlib_window as vk::Window)
        .build();

    let xlib_surface_loader = XlibSurface::new(entry, instance);
    unsafe {
        xlib_surface_loader.create_xlib_surface(&xlib_create_info, None)
            .expect("Failed to create surface")
    }
}

pub fn get_required_instance_extensions() -> Vec<*const i8> {
    vec![Surface::name().as_ptr(), XlibSurface::name().as_ptr(), DebugReport::name().as_ptr()] as Vec<*const i8>
}
