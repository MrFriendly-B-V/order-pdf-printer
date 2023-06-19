mod error;
pub mod footer;
pub mod header;
pub mod invoice;
mod jvm;
pub mod packing_slip;
mod render_target;

pub use error::*;
pub use jvm::*;
pub use render_target::*;

/// Something which can be rendered to a PDF target
pub trait PdfRenderable {
    /// Render self to the target
    fn render<'a>(&self, target: &RenderTarget<'a>, env: &mut jni::JNIEnv<'a>)
        -> Result<(), Error>;
}
