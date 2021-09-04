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
    }
    impl MTLTextureDescriptorSelectors for Sel {}
}
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
}

#[test]
fn configure() {
    objr::bindings::autoreleasepool(|pool| {
        let mut descriptor = MTLTextureDescriptor::new(pool);
        descriptor.set_width(500,pool);
        descriptor.set_height(1000,pool);
        descriptor.set_pixel_format(&MTLPixelFormat::R8Unorm,pool);
        let description_strong = descriptor.description(pool);
        let description = description_strong.to_str(pool);
        println!("{}",description );
        assert!(description.contains("width = 500"));
        assert!(description.contains("height = 1000"));
        assert!(description.contains("pixelFormat = MTLPixelFormatR8Unorm"));
    });

}