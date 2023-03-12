mod codec;
mod codec_context;
mod format_context;
mod frame;
mod packet;
mod scale_context;
mod stream;

pub use codec::*;
pub use codec_context::*;
pub use format_context::*;
pub use frame::*;
pub use packet::*;
pub use scale_context::*;
pub use stream::*;

pub trait WrapperType {
    type OUT;

    fn get_inner(&self) -> *const Self::OUT;
    fn get_inner_mut(&mut self) -> *mut Self::OUT;
}
