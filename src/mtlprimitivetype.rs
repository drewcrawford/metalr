use objr::bindings::*;
use foundationr::NSUInteger;
objc_enum! {
    #[derive(Debug)]
    pub struct MTLPrimitiveType<NSUInteger>;
    impl MTLPrimitiveType {
        Point = 0,
        Line = 1,
        Strip = 2,
        Triangle = 3,
        TriangleStrip = 4
    }
}

