use objr::bindings::*;

objc_instance! {
    pub struct MTLRenderPipelineState;
}
///Reasonably confident based on common API use
unsafe impl Send for MTLRenderPipelineState {}