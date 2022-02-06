use objr::bindings::*;
use super::MTLPixelFormat;
use foundationr::NSUInteger;

objc_class!{
    pub struct MTLTextureDescriptor {
        @class(MTLTextureDescriptor)
    }
}
objc_selector_group! {
    pub(crate) trait MTLTextureDescriptorSelectors {
        @selector("setWidth:")
        @selector("setHeight:")
        @selector("setPixelFormat:")
        @selector("setResourceOptions:")
        @selector("setUsage:")
    }
    impl MTLTextureDescriptorSelectors for Sel {}
}
objc_enum! {
    pub struct MTLCPUCacheMode<NSUInteger>;
    impl MTLCPUCacheMode {
        DefaultCache = 0,
        WriteCombined = 1
    }
}
objc_enum! {
    pub struct MTLStorageMode<NSUInteger>;
    impl MTLStorageMode {
        Shared  = 0,
        Managed = 1,
        Private = 2,
        Memoryless = 3
    }
}

objc_enum! {
    pub struct MTLHazardTrackingMode<NSUInteger>;
    impl MTLHazardTrackingMode {
        Default = 0,
        Untracked = 1,
        Tracked = 2
    }
}
objc_enum! {
    pub struct MTLTextureUsage<NSUInteger>;
    impl MTLTextureUsage {
        Unknown = 0,
        ShaderRead      = 0x0001,
        ShaderWrite     = 0x0002,
        RenderTarget    = 0x0004,
        PixelFormatView = 0x0010
    }
}
#[repr(transparent)]
#[derive(Debug,Copy,Clone)]
pub struct MTLResourceOptions(NSUInteger);
impl MTLResourceOptions {
    pub const fn with_options(cache_mode: MTLCPUCacheMode, storage_mode: MTLStorageMode, tracking_mode: MTLHazardTrackingMode) -> Self {
        let mut options = 0;
        options = options | cache_mode.field();
        options = options | (storage_mode.field() << 4);
        options = options | (tracking_mode.field() << 8);
        Self(options)
    }
}
unsafe impl Arguable for MTLResourceOptions {}
impl MTLTextureDescriptor {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongMutCell<MTLTextureDescriptor> {
        unsafe{ Self::class().alloc_init(pool).assume_mut() }
    }
    pub fn set_width(&mut self, width: NSUInteger,pool: &ActiveAutoreleasePool) {
        unsafe{
            Self::perform_primitive(self, Sel::setWidth_(), pool, (width,))
        }
    }
    pub fn set_height(&mut self, height: NSUInteger,pool: &ActiveAutoreleasePool) {
        unsafe{ Self::perform_primitive(self, Sel::setHeight_(), pool, (height,)) }
    }

    pub fn set_pixel_format(&mut self, format: &MTLPixelFormat,pool: &ActiveAutoreleasePool) {
        unsafe { Self::perform_primitive(self, Sel::setPixelFormat_(), pool, (format.field(),))}
    }
    pub fn set_resource_options(&mut self, options: MTLResourceOptions, pool: &ActiveAutoreleasePool) {
        unsafe { Self::perform_primitive(self, Sel::setResourceOptions_(), pool, (options,))}
    }
    pub fn set_usage(&mut self, usage: &MTLTextureUsage, pool: &ActiveAutoreleasePool) {
        unsafe { Self::perform_primitive(self, Sel::setUsage_(),pool,(usage.field(),)) }
    }
}

#[test]
fn configure() {
    objr::bindings::autoreleasepool(|pool| {
        let mut descriptor = MTLTextureDescriptor::new(pool);
        descriptor.set_width(500,pool);
        descriptor.set_height(1000,pool);
        descriptor.set_pixel_format(&MTLPixelFormat::R8Unorm,pool);
        descriptor.set_usage(&MTLTextureUsage::ShaderRead,pool);
        let description_strong = descriptor.description(pool);
        let description = description_strong.to_str(pool);
        println!("{}",description );
        assert!(description.contains("width = 500"));
        assert!(description.contains("height = 1000"));
        assert!(description.contains("pixelFormat = MTLPixelFormatR8Unorm"));
        assert!(description.contains("usage = MTLTextureUsageShaderRead"));
    });

}