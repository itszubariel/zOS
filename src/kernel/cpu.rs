pub fn cpuid(eax: u32) -> (u32, u32, u32, u32) {
    let eax_out: u32;
    let ebx_out: u32;
    let ecx_out: u32;
    let edx_out: u32;
    unsafe {
        core::arch::asm!(
            "push rbx",
            "cpuid",
            "mov {ebx:e}, ebx",
            "pop rbx",
            inout("eax") eax => eax_out,
            ebx = out(reg) ebx_out,
            out("ecx") ecx_out,
            out("edx") edx_out,
        );
    }
    (eax_out, ebx_out, ecx_out, edx_out)
}

pub fn get_cpu_model() -> [u8; 48] {
    let mut model = [0u8; 48];
    // CPUID EAX=0x80000002, 0x80000003, 0x80000004 for brand string
    for i in 0..3 {
        let (eax, ebx, ecx, edx) = cpuid(0x80000002 + (i as u32));
        let bytes = [
            (eax & 0xff) as u8,
            ((eax >> 8) & 0xff) as u8,
            ((eax >> 16) & 0xff) as u8,
            ((eax >> 24) & 0xff) as u8,
            (ebx & 0xff) as u8,
            ((ebx >> 8) & 0xff) as u8,
            ((ebx >> 16) & 0xff) as u8,
            ((ebx >> 24) & 0xff) as u8,
            (ecx & 0xff) as u8,
            ((ecx >> 8) & 0xff) as u8,
            ((ecx >> 16) & 0xff) as u8,
            ((ecx >> 24) & 0xff) as u8,
            (edx & 0xff) as u8,
            ((edx >> 8) & 0xff) as u8,
            ((edx >> 16) & 0xff) as u8,
            ((edx >> 24) & 0xff) as u8,
        ];
        for j in 0..16 {
            model[i * 16 + j] = bytes[j];
        }
    }
    model
}
