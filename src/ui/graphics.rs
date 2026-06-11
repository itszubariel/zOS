pub struct Framebuffer {
    pub address: *mut u32,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
}

static mut FRAMEBUFFER: Option<Framebuffer> = None;

pub unsafe fn init(fb_tag: &crate::kernel::multiboot::FramebufferTag) {
    unsafe {
        FRAMEBUFFER = Some(Framebuffer {
            address: fb_tag.address as *mut u32,
            width: fb_tag.width as usize,
            height: fb_tag.height as usize,
            pitch: (fb_tag.pitch / 4) as usize,
        });
    }
}

pub fn draw_pixel(x: usize, y: usize, color: u32) {
    unsafe {
        let fb_ref = &raw const FRAMEBUFFER;
        if let Some(fb) = &*fb_ref {
            if x < fb.width && y < fb.height {
                *fb.address.add(y * fb.pitch + x) = color;
            }
        }
    }
}

pub fn draw_rect(x: usize, y: usize, width: usize, height: usize, color: u32) {
    for i in x..x + width {
        for j in y..y + height {
            draw_pixel(i, j, color);
        }
    }
}
