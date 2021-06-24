use crate::io;
#[cfg(all(libc, target_os = "linux"))]
use crate::libc::conv::ret_ssize_t;
use bitflags::bitflags;

#[cfg(libc)]
bitflags! {
    /// `GRND_*`
    pub struct GetRandomFlags: u32 {
        /// GRND_RANDOM
        const RANDOM = libc::GRND_RANDOM;
        /// GRND_NONBLOCK
        const NONBLOCK = libc::GRND_NONBLOCK;
    }
}

#[cfg(linux_raw)]
bitflags! {
    /// `GRND_*`
    pub struct GetRandomFlags: u32 {
        /// GRND_RANDOM
        const RANDOM = linux_raw_sys::v5_4::general::GRND_RANDOM;
        /// GRND_NONBLOCK
        const NONBLOCK = linux_raw_sys::v5_4::general::GRND_NONBLOCK;
    }
}

/// `getrandom(buf.as_mut_ptr(), buf.len(), flags)`
#[inline]
pub fn getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    _getrandom(buf, flags)
}

#[cfg(libc)]
fn _getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    let nread = unsafe {
        ret_ssize_t(libc::getrandom(
            buf.as_mut_ptr().cast::<_>(),
            buf.len(),
            flags.bits(),
        ))?
    };
    Ok(nread as usize)
}

#[cfg(linux_raw)]
#[inline]
fn _getrandom(buf: &mut [u8], flags: GetRandomFlags) -> io::Result<usize> {
    crate::linux_raw::getrandom(buf, flags.bits())
}
