use foundationr::NSUInteger;
use objr::bindings::*;
use crate::MTLPixelFormat;

objc_class! {
    pub struct MTLRenderPipelineColorAttachmentDescriptor {
        @class(MTLRenderPipelineColorAttachmentDescriptor)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("isBlendingEnabled")
        @selector("setBlendingEnabled:")
        @selector("alphaBlendOperation")
        @selector("setAlphaBlendOperation:")
        @selector("rgbBlendOperation")
        @selector("setRgbBlendOperation:")
        @selector("destinationAlphaBlendFactor")
        @selector("setDestinationAlphaBlendFactor:")
        @selector("destinationRGBBlendFactor")
        @selector("setDestinationRGBBlendFactor:")
        @selector("sourceAlphaBlendFactor")
        @selector("setSourceAlphaBlendFactor:")
        @selector("sourceRGBBlendFactor")
        @selector("setSourceRGBBlendFactor:")
    }
    impl Selectors for Sel {

    }
}
objc_enum! {
    #[derive(PartialEq,Debug)]
    pub struct MTLBlendOperation<NSUInteger>;
    impl MTLBlendOperation{
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4
    }
}

objc_enum! {
    #[derive(PartialEq,Debug)]
    pub struct MTLBlendFactor<NSUInteger>;
    impl MTLBlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 2,
    OneMinusSourceColor = 3,
    SourceAlpha = 4,
    OneMinusSourceAlpha = 5,
    DestinationColor = 6,
    OneMinusDestinationColor = 7,
    DestinationAlpha = 8,
    OneMinusDestinationAlpha = 9,
    SourceAlphaSaturated = 10,
    BlendColor = 11,
    OneMinusBlendColor = 12,
    BlendAlpha = 13,
    OneMinusBlendAlpha = 14,
    Source1Color = 15,
    OneMinusSource1Color  = 16,
    Source1Alpha  = 17,
    OneMinusSource1Alpha = 18
    }
}
#[allow(non_snake_case)]
impl MTLRenderPipelineColorAttachmentDescriptor {
    pub fn set_pixelFormat(&mut self, pixelFormat: MTLPixelFormat,pool: &ActiveAutoreleasePool) {
        unsafe {
            use crate::mtltexturedescriptor::MTLTextureDescriptorSelectors;
            Self::perform_primitive(self, Sel::setPixelFormat_(), pool, (pixelFormat.field(),))
        }
    }
    pub fn isBlendingEnabled(&self, pool: &ActiveAutoreleasePool) -> bool {
        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(),Sel::isBlendingEnabled(), pool, ())
        }
    }
    pub fn setBlendingEnabled(&mut self, value: bool, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self,Sel::setBlendingEnabled_(), pool, (value,))

        }
    }
    pub fn alphaBlendOperation(&self, pool: &ActiveAutoreleasePool) -> MTLBlendOperation {
        unsafe {
            MTLBlendOperation(Self::perform_primitive(self.assume_nonmut_perform(),Sel::alphaBlendOperation(),pool,()))
        }
    }
    pub fn setAlphaBlendOperation(&mut self, value: MTLBlendOperation, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setAlphaBlendOperation_(), pool, (value.field(),))
        }
    }
    pub fn rgbBlendOperation(&self, pool: &ActiveAutoreleasePool) -> MTLBlendOperation {
        unsafe {
            MTLBlendOperation(Self::perform_primitive(self.assume_nonmut_perform(),Sel::rgbBlendOperation(),pool,()))
        }
    }
    pub fn setRgbBlendOperation(&mut self, value: MTLBlendOperation, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setRgbBlendOperation_(), pool, (value.field(),))
        }
    }

    pub fn destinationAlphaBlendFactor(&self, pool: &ActiveAutoreleasePool) -> MTLBlendFactor {
        unsafe {
            MTLBlendFactor(Self::perform_primitive(self.assume_nonmut_perform(),Sel::destinationAlphaBlendFactor(),pool,()))
        }
    }
    pub fn setDestinationAlphaBlendFactor(&mut self, value: MTLBlendFactor, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setDestinationAlphaBlendFactor_(), pool, (value.field(),))
        }
    }
    pub fn destinationRGBBlendFactor(&self, pool: &ActiveAutoreleasePool) -> MTLBlendFactor {
        unsafe {
            MTLBlendFactor(Self::perform_primitive(self.assume_nonmut_perform(),Sel::destinationRGBBlendFactor(),pool,()))
        }
    }
    pub fn setDestinationRGBBlendFactor(&mut self, value: MTLBlendFactor, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setDestinationRGBBlendFactor_(), pool, (value.field(),))
        }
    }
    pub fn sourceAlphaBlendFactor(&self, pool: &ActiveAutoreleasePool) -> MTLBlendFactor {
        unsafe {
            MTLBlendFactor(Self::perform_primitive(self.assume_nonmut_perform(),Sel::sourceAlphaBlendFactor(),pool,()))
        }
    }
    pub fn setSourceAlphaBlendFactor(&mut self, value: MTLBlendFactor, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setSourceAlphaBlendFactor_(), pool, (value.field(),))
        }
    }
    pub fn sourceRGBBlendFactor(&self, pool: &ActiveAutoreleasePool) -> MTLBlendFactor {
        unsafe {
            MTLBlendFactor(Self::perform_primitive(self.assume_nonmut_perform(),Sel::sourceRGBBlendFactor(),pool,()))
        }
    }
    pub fn setSourceRGBBlendFactor(&mut self, value: MTLBlendFactor, pool: &ActiveAutoreleasePool) {
        unsafe {
            Self::perform_primitive(self, Sel::setSourceRGBBlendFactor_(), pool, (value.field(),))
        }
    }
}

#[test] fn smoke_test() {
    autoreleasepool(|pool| {
        let mut d = unsafe{ MTLRenderPipelineColorAttachmentDescriptor::class().alloc_init(pool).assume_mut() };
        d.set_pixelFormat( MTLPixelFormat::A8Unorm,pool);
        let description = d.description(pool).to_str(pool).to_owned();
        assert!(description.contains("MTLPixelFormatA8Unorm"));

        assert!(!d.isBlendingEnabled(pool));
        d.setBlendingEnabled(true,pool);
        assert!(d.isBlendingEnabled(pool));

        assert_eq!(d.alphaBlendOperation(pool), MTLBlendOperation::Add);
        d.setAlphaBlendOperation(MTLBlendOperation::Max,pool);
        assert_eq!(d.alphaBlendOperation(pool), MTLBlendOperation::Max);

        assert_eq!(d.rgbBlendOperation(pool), MTLBlendOperation::Add);
        d.setRgbBlendOperation(MTLBlendOperation::Max, pool);
        assert_eq!(d.rgbBlendOperation(pool), MTLBlendOperation::Max);

        assert_eq!(d.destinationAlphaBlendFactor(pool), MTLBlendFactor::Zero);
        d.setDestinationAlphaBlendFactor(MTLBlendFactor::BlendAlpha, pool);
        assert_eq!(d.destinationAlphaBlendFactor(pool), MTLBlendFactor::BlendAlpha);

        assert_eq!(d.destinationRGBBlendFactor(pool), MTLBlendFactor::Zero);
        d.setDestinationRGBBlendFactor(MTLBlendFactor::BlendAlpha, pool);
        assert_eq!(d.destinationRGBBlendFactor(pool), MTLBlendFactor::BlendAlpha);

        assert_eq!(d.sourceAlphaBlendFactor(pool), MTLBlendFactor::One);
        d.setSourceAlphaBlendFactor(MTLBlendFactor::BlendAlpha, pool);
        assert_eq!(d.sourceAlphaBlendFactor(pool), MTLBlendFactor::BlendAlpha);
    })
}