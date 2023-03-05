use std::mem::MaybeUninit;
use std::ptr;

use ffmpeg_sys_next::{av_init_packet, av_packet_unref, AVPacket};

use super::WrapperType;

pub struct Packet(AVPacket);

impl Packet {
    pub fn new() -> Self {
        let mut pkt = unsafe {
            let mut pkt: MaybeUninit<AVPacket> = MaybeUninit::uninit();
            av_init_packet(pkt.as_mut_ptr());
            pkt.assume_init()
        };

        pkt.data = ptr::null_mut();
        pkt.size = 0;

        Packet(pkt)
    }

    pub fn unref(mut self) {
        unsafe { av_packet_unref(&mut self.0) }
    }
}

impl WrapperType for Packet {
    type OUT = AVPacket;

    fn get_inner(&self) -> *const Self::OUT {
        &self.0 as *const AVPacket
    }

    fn get_inner_mut(&mut self) -> *mut Self::OUT {
        &mut self.0 as *mut AVPacket
    }
}

// impl Drop for Packet {
//     fn drop(&mut self) {
//         let mut pkt = (&mut self.0) as *mut AVPacket;
//         let pkt = (&mut pkt) as *mut *mut AVPacket;
//         unsafe { av_packet_free(pkt) }
//     }
// }
