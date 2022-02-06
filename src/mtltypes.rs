use foundationr::NSUInteger;
use objr::bindings::Arguable;

#[repr(C)]
#[derive(Debug)]
pub struct MTLOrigin {
    pub x: NSUInteger,
    pub y: NSUInteger,
    pub z: NSUInteger
}
unsafe impl Arguable for MTLOrigin {}
impl MTLOrigin {
    pub const ZERO: MTLOrigin = MTLOrigin { x: 0, y: 0, z: 0};
}



#[repr(C)]
#[derive(Debug)]
pub struct MTLSize {
    pub width: NSUInteger,
    pub height: NSUInteger,
    pub depth: NSUInteger
}
unsafe impl Arguable for MTLSize {}

#[repr(C)]
#[derive(Debug)]
pub struct MTLRegion {
    pub origin: MTLOrigin,
    pub size: MTLSize,
}
unsafe impl Arguable for MTLRegion {}