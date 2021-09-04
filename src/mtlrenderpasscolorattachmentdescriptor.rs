use objr::bindings::*;
use super::MTLRenderPassAttachmentDescriptorTrait;
objc_class! {
    pub struct MTLRenderPassColorAttachmentDescriptor {
        @class(MTLRenderPassColorAttachmentDescriptor)
    }
}

impl MTLRenderPassColorAttachmentDescriptor {
    pub fn new(pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        Self::class().alloc_init(pool)
    }
}

///Safe because this is a subclass
unsafe impl MTLRenderPassAttachmentDescriptorTrait for MTLRenderPassColorAttachmentDescriptor {}
#[test] fn descriptor_texture() {
    autoreleasepool(|pool| {
        use crate::*;
        let mut p = unsafe {
            MTLRenderPassColorAttachmentDescriptor::class().alloc_init(pool).assume_mut()
        };
        println!("{}",p);
        let deref: &mut MTLRenderPassColorAttachmentDescriptor = &mut p;

        assert!(deref.texture(pool).is_none());

        let mut device = super::MTLDevice::default().unwrap();
        let texture_descriptor = super::MTLTextureDescriptor::new(pool);
        let texture = device.newTextureWithDescriptor(&texture_descriptor, pool).unwrap();
        deref.setTexture(&texture,pool);
        assert!(deref.texture(pool).is_some());
        deref.setLoadAction( MTLLoadAction::Load,pool);
        deref.setStoreAction( MTLStoreAction::Store,pool);
    })
}
