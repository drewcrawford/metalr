/**
# Drew's fast Rust Metal bindings

Provides select Rust bindings for Apple [Metal](https://developer.apple.com/metal/) framework.  May be compared to alternative crates [metal](https://crates.io/crates/metal), [objrs_frameworks_metal](https://crates.io/crates/objrs_frameworks_metal),
[metal-rs](https://crates.io/crates/metal-rs), and [metal-sys](https://crates.io/crates/metal-sys).

Part of the [objr expanded universe](https://github.com/drewcrawford/objr#objr-expanded-universe), distinctive features of this library:

* Fast.  This crate is *significantly* faster than other crates.  If you're struggling to get 60fps or ProMotion/adaptive refresh rate speeds, this
    is the solution for you.
    * The full set of optimization is far too many to list, but the big idea is to either do what native ObjC/Swift applications do, or do something faster.
    * Compile-time selectors.  Most Rust crates do a runtime lookup for ObjC methods, which involves acquiring a lock and other yucky stuff, either on the first call or every call.  Instead, we do what real ObjC compilers do, which is way faster.  For more details, see [objr](https://github.com/drewcrawford/objr)
    * Smart pointers that provide global ARC inference.  Like ARC, you don't need to write manual retain/release calls.  Unlike ARC, the compiler
      usually doesn't need to write them either, meaning lower runtime memory management cost than even native code.  For more details, see [objr](https://github.com/drewcrawford/objr).
    * Runtime autorelease eliding, which keeps your objects out of autoreleasepools.  For more details, see [objr](https://github.com/drewcrawford/objr).
    * Pointer packing for optional types so they fit in a `u64`.  For more details, see [objr](https://github.com/drewcrawford/objr)
    * Stack-allocated blocks.  For more details, see [blocksr](https://github.com/drewcrawford/blocksr)
* Safe APIs.  Where possible APIs are designed with safe abstractions to provide familiar guarantees to Rust developers
* Async bindings out of the box for shader compilation and similar tasks
* Low-level.  These bindings assume familiarity with bound APIs and are not documented separately.  For details on how they work, see the native documentation.
* Free for noncommercial or "small commercial" use.

# Implementation status
Generally it's implemented enough that you can write basic high-performance rendering applications.

The following APIs are at least partially implemented.  These implementations are incomplete but contain common functions or "the ones I use".

The [objr](https://github.com/drewcrawford/objr) macro system makes it very ergonomic to add new bindings for specific missing features
or new Metal APIs.

## Objects, etc.
* MTLDevice
* MTLCommandBuffer
* MTLCommandQueue
* MTLBuffer

## Render passes
* MTLRenderPassAttachmentDescriptor
* MTLRenderPassColorAttachmentDescriptor
* MTLRenderPassColorAttachmentDescriptorArray
* MTLRenderPassDescriptor

## Render pipelines
* MTLRenderPipelineDescriptor
* MTLRenderPipelineState
* MTLRenderPipelineColorAttachmentDescriptor
* MTLRenderPipelineColorAttachmentDescriptorArray

## Encoders
* MTLBlitCommandEncoder
* MTLRendercommandEncoder

## Textures
* MTLTexture
* MTLTextureDescriptor

## Types
* MTLPixelFormat
* MTLPrimitiveType
* MTLOrigin
* MTLSize

## Protocols
* MTLDrawable

## Planned

Not yet implemented:
* Compute
* iOS

## Out of scope

* Compiling shaders at build-time. You can compile at runtime with [this API](https://developer.apple.com/documentation/metal/mtldevice/1433431-makelibrary) (see tests), or write a `build.rs` script to build your shaders.
  Stay tuned for future crates!

*/
mod mtltexture;
mod mtldevice;
mod mtlcommandqueue;
mod mtllibrary;
mod mtlrenderpipelinedescriptor;
mod mtlrenderpipelinestate;
mod mtlcommandbuffer;
mod mtlfunction;
mod mtlrenderpipelinecolorattachmentdescriptorarray;
mod mtlrenderpassdescriptor;
mod mtlrendercommandencoder;
mod mtldrawable;
mod mtlrenderpipelinecolorattachmentdescriptor;
mod mtlrenderpasscolorattachmentdescriptorarray;
mod mtlprimitivetype;
mod mtlpixelformat;
mod mtltexturedescriptor;
mod mtlrenderpassattachmentdescriptor;
mod mtlrenderpasscolorattachmentdescriptor;
mod mtlfunctionconstantvalues;
mod mtlbuffer;
mod mtlblitcommandencoder;
mod mtltypes;

pub use mtltexture::MTLTexture;
pub use mtltexturedescriptor::{MTLTextureDescriptor,MTLHazardTrackingMode, MTLStorageMode, MTLCPUCacheMode, MTLResourceOptions,MTLTextureUsage};
pub use mtlrenderpipelinedescriptor::MTLRenderPipelineDescriptor;
pub use mtlrenderpipelinestate::MTLRenderPipelineState;
pub use mtlfunction::MTLFunction;
pub use mtlrenderpipelinecolorattachmentdescriptorarray::MTLRenderPipelineColorAttachmentDescriptorArray;
pub use mtlrenderpassdescriptor::MTLRenderPassDescriptor;
pub use mtlrendercommandencoder::MTLRenderCommandEncoder;
pub use mtldrawable::MTLDrawable;
pub use mtlrenderpipelinecolorattachmentdescriptor::MTLRenderPipelineColorAttachmentDescriptor;
pub use mtlrenderpasscolorattachmentdescriptorarray::MTLRenderPassColorAttachmentDescriptorArray;
pub use mtlprimitivetype::MTLPrimitiveType;
pub use mtlpixelformat::MTLPixelFormat;
pub use mtlrenderpasscolorattachmentdescriptor::MTLRenderPassColorAttachmentDescriptor;
pub use mtldevice::MTLDevice;
pub use mtlrenderpassattachmentdescriptor::{MTLLoadAction,MTLStoreAction,MTLRenderPassAttachmentDescriptorTrait,MTLRenderPassAttachmentDescriptorImpl};
pub use mtlcommandqueue::MTLCommandQueue;
pub use mtlcommandbuffer::MTLCommandBuffer;
pub use mtllibrary::MTLLibrary;
pub use mtlfunctionconstantvalues::MTLFunctionConstantValues;
pub use mtlbuffer::MTLBuffer;
pub use mtltypes::{MTLOrigin, MTLSize};
pub use mtlblitcommandencoder::MTLBlitCommandEncoder;