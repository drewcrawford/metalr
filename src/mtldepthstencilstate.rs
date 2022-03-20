use objr::bindings::*;
objc_instance! {
    pub struct MTLDepthStencilState;
}
///Reasonably confident based on common API use
unsafe impl Send for MTLDepthStencilState {}
