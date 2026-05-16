use zenjpeg::encoder::OptimizationPreset::HybridMaxCompression;
use zenjpeg::encoder::{
    ChromaSubsampling, EncoderConfig, Limits, PixelLayout, Quality, Unstoppable,
};

#[link(wasm_import_module = "js")]
unsafe extern "C" {
    fn println(ptr: usize, len: usize);
}

#[inline(always)]
fn log(s: &str) {
    unsafe { println(s.as_ptr() as usize, s.len()) }
}

/// # Safety
/// We assume the pointer points array of the correct `len`.
#[unsafe(no_mangle)]
pub unsafe fn encode_rgba(
    ptr: *const u8,
    len: usize,
    width: usize,
    height: usize,
    quality: u8,
) -> Box<[u8; 8]> {
    let rgba = unsafe { std::slice::from_raw_parts(ptr, len) };
    let mut limits = Limits::default();
    limits.max_pixels = Some(64_000_000);
    limits.max_memory = Some(1_073_741_824);
    let config = EncoderConfig::ycbcr(
        Quality::ApproxSsim2(quality as f32),
        ChromaSubsampling::None,
    )
    // Not sure why it's #[doc(hidden)]
    .optimization(HybridMaxCompression);
    let config = config.request().limits(limits);
    let mut encoder = config
        .encode_from_bytes(width as u32, height as u32, PixelLayout::Rgba8Srgb)
        .map_err(|e| log(format!("{e:?}").as_str()))
        .unwrap();
    encoder
        .push_packed(rgba, Unstoppable)
        .map_err(|e| log(format!("{e:?}").as_str()))
        .unwrap();
    let data = encoder
        .finish()
        .map_err(|e| log(format!("{e:?}").as_str()))
        .unwrap()
        .into_boxed_slice();
    let len = data.len() as u32;
    let ptr = Box::into_raw(data) as *mut u8 as u32;
    let mut ptr_and_len = Vec::with_capacity(8);
    ptr_and_len.extend_from_slice(&ptr.to_le_bytes());
    ptr_and_len.extend_from_slice(&len.to_le_bytes());
    unsafe { Box::from_raw(Box::into_raw(ptr_and_len.into_boxed_slice()) as *mut [u8; 8]) }
}

#[unsafe(no_mangle)]
pub fn malloc(len: usize) -> *mut u8 {
    let mut vec = Vec::<u8>::with_capacity(len);
    let ptr = vec.as_mut_ptr();
    core::mem::forget(vec);
    ptr
}

/// # Safety
/// We assume the pointer points to an array of the correct `len`.
#[unsafe(no_mangle)]
pub unsafe fn free(ptr: *mut u8, len: usize) {
    unsafe {
        Vec::from_raw_parts(ptr, 0, len);
    }
}
