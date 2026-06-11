#[repr(C, packed)]
pub struct MultibootHeader {
    total_size: u32,
    reserved: u32,
}

#[repr(C, packed)]
pub struct Tag {
    tag_type: u32,
    size: u32,
}

#[repr(C, packed)]
pub struct FramebufferTag {
    tag_type: u32,
    size: u32,
    pub address: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub fb_type: u8,
    reserved: u16,
}

pub unsafe fn find_framebuffer_tag(addr: u64) -> Option<&'static FramebufferTag> {
    let header = unsafe { &*(addr as *const MultibootHeader) };
    let mut current = (addr + 8) as *const Tag;
    let end = addr + (header.total_size as u64);

    while (current as u64) < end {
        if (unsafe { (*current).tag_type }) == 8 {
            return Some(unsafe { &*(current as *const FramebufferTag) });
        }
        current = ((current as u64) +
            ((((unsafe { (*current).size }) + 7) & !7) as u64)) as *const Tag;
    }
    None
}
