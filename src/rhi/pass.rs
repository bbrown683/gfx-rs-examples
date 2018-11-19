use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;
use ash::version::DeviceV1_0;
use ash::vk;
use super::Device;

pub struct RenderPass {
    device : Rc<RefCell<Device>>,
    render_pass : vk::RenderPass,
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            self.device.borrow().get_ash_device().destroy_render_pass(self.render_pass, None);
        }
    }
}

impl RenderPass {
    pub fn new(device : Rc<RefCell<Device>>) -> Self {
        let color_attachment = vk::AttachmentDescription::builder()
            .format(vk::Format::B8G8R8A8_SRGB)
            .samples(vk::SampleCountFlags::TYPE_1)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .build();

        let depth_attachment = vk::AttachmentDescription::builder()
            .format(vk::Format::D32_SFLOAT)
            .samples(vk::SampleCountFlags::TYPE_1)
            .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .build();

        let attachments = vec![color_attachment, depth_attachment];

        // TODO: create depth stencil reference.
        let color_reference = vec![vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build()];

        let depth_reference = vk::AttachmentReference::builder()
            .attachment(1)
            .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .build();

        // TODO: add depth stencil attachment to subpass.
        let subpass = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(color_reference.as_slice())
            .depth_stencil_attachment(&depth_reference)
            .build();

        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(attachments.as_slice())
            .subpasses(&[subpass])
            .build();

        let render_pass = unsafe {
             device
                 .borrow()
                 .get_ash_device()
                 .create_render_pass(&render_pass_info, None)
                 .expect("Failed to create render pass")
        };
        Self { device, render_pass }
    }

    pub fn get_render_pass_raw(&self) -> vk::RenderPass {
        self.render_pass
    }
}
