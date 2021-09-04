use objr::bindings::*;
use super::MTLTexture;
use foundationr::NSUInteger;

objc_enum! {
    pub struct MTLLoadAction<NSUInteger>;
    impl MTLLoadAction {
        DontCare = 0,
        Load = 1,
        Clear = 2
    }
}
objc_enum! {
    pub struct MTLStoreAction<NSUInteger>;
    impl MTLStoreAction {
        DontCare = 0,
        Store = 1,
        MultisampleResolve = 2,
        StoreAndMultisampleResolve = 3,
        Unknown = 4,
        CustomSampleDepthStore = 5
    }
}

objc_class! {
    ///This is declared following the objc header premise.
    ///However, in practice it is an abstract class!  Methods cannot
    /// be used on it unless it is a subclass of something else.
    ///
    /// For this reason, functions are marked unsafe.
    pub struct MTLRenderPassAttachmentDescriptor {
        @class(MTLRenderPassAttachmentDescriptor)
    }
}
objc_selector_group! {
    trait MTLRenderPassAttachmentDescriptorSelectors {
        @selector("texture")
        @selector("setTexture:")
        @selector("setLoadAction:")
        @selector("setStoreAction:")
    }
    impl MTLRenderPassAttachmentDescriptorSelectors for Sel {}
}

///Trait for conforming concrete subtypes
///Unsafe because this can only be implemented by a concrete subtype.
pub unsafe trait MTLRenderPassAttachmentDescriptorTrait: ObjcInstance {}
#[allow(non_snake_case)]
pub trait MTLRenderPassAttachmentDescriptorImpl {
    fn texture(&self,pool: &ActiveAutoreleasePool) -> Option<StrongCell<MTLTexture>>;
    fn setTexture(&mut self, pool: &ActiveAutoreleasePool, texture: &MTLTexture);
    fn setLoadAction(&mut self, pool: &ActiveAutoreleasePool, action: MTLLoadAction);
    fn setStoreAction(&mut self, pool: &ActiveAutoreleasePool, action: MTLStoreAction);
}

impl<T: MTLRenderPassAttachmentDescriptorTrait> MTLRenderPassAttachmentDescriptorImpl for T {
    fn texture(&self, pool: &ActiveAutoreleasePool) -> Option<StrongCell<MTLTexture>> {
        unsafe{
            let ptr = Self::perform_autorelease_to_retain(self.assume_nonmut_perform(), Sel::texture(), pool, ());
            MTLTexture::nullable(ptr).assume_retained()
        }
    }
    fn setTexture(&mut self, pool: &ActiveAutoreleasePool, texture: &MTLTexture) {
        unsafe {
            Self::perform_primitive(self, Sel::setTexture_(), pool, (texture,))
        }
    }

    fn setLoadAction(&mut self, pool: &ActiveAutoreleasePool, action: MTLLoadAction) {
        unsafe{ Self::perform_primitive(self,Sel::setLoadAction_(), pool, (action.field(),)) }
    }
    fn setStoreAction(&mut self, pool: &ActiveAutoreleasePool, action: MTLStoreAction) {
        unsafe{ Self::perform_primitive(self,Sel::setStoreAction_(), pool, (action.field(),)) }
    }
}



#[test] fn init_descriptor() {
    autoreleasepool(|pool| {
        let p = MTLRenderPassAttachmentDescriptor::class().alloc_init(pool);
        println!("{}",p);
    })
}