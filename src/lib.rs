//! Cross-platform screen capture. Uses DXGI desktop-duplication on Windows,
//! and X11 (xlib, XGetImage) on *nix

#[cfg(windows)]
extern crate dxgcap;
#[cfg(not(windows))]
extern crate x11cap;

use std::time::Duration;

/// Color represented by additive channels: Blue (b), Green (g), Red (r), and Alpha (a)
#[cfg(windows)]
pub type Bgr8 = dxgcap::BGRA8;
/// Color represented by additive channels: Blue (b), Green (g), and Red (r).
///
/// A fourth field of padding makes this struct 4 bytes.
#[cfg(not(windows))]
pub type Bgr8 = x11cap::Bgr8;

#[derive(Clone, Debug)]
pub enum CaptureError {
    /// Could not duplicate output, access denied. Might be in protected fullscreen.
    #[cfg(windows)]
    AccessDenied,
    /// Access to the duplicated output was lost. Likely, mode was changed e.g. window => full
    AccessLost,
    /// Error when trying to refresh outputs after some failure.
    #[cfg(windows)]
    RefreshFailure,
    /// Aquisition of next frame timed out.
    Timeout,
    /// General/Unexpected failure
    Fail(String),
}

#[cfg(windows)]
pub struct Capturer {
    dxgi_manager: dxgcap::DXGIManager,
    width: usize,
    height: usize,
}

#[cfg(not(windows))]
pub struct Capturer {
    x11_capturer: x11cap::Capturer,
}

impl Capturer {
    #[cfg(windows)]
    pub fn new(capture_src: usize) -> Result<Capturer, String> {
        Capturer::new_with_timeout(capture_src, Duration::from_millis(200))
    }

    #[cfg(windows)]
    pub fn new_with_timeout(capture_src: usize, timeout: Duration) -> Result<Capturer, String> {
        (timeout.as_secs() as u32)
            .checked_mul(1000)
            .and_then(|ms| ms.checked_add(timeout.subsec_nanos() / 1_000_000))
            .ok_or("Failed to convert the given duration to a legal u32 millisecond value due to integer overflow.".to_owned())
            .and_then(|timeout| {
                dxgcap::DXGIManager::new(timeout)
                    .map(|mut mgr| {
                        mgr.set_capture_source_index(capture_src);
                        Capturer {
                            dxgi_manager: mgr,
                            width: 0,
                            height: 0,
                        }
                    })
                    .map_err(|err| err.to_owned())
            })
    }

    #[cfg(not(windows))]
    pub fn new(capture_src: usize) -> Result<Capturer, String> {
        x11cap::Capturer::new(x11cap::CaptureSource::Monitor(capture_src))
            .map(|c| Capturer { x11_capturer: c })
            .map_err(|()| "Failed to initialize capturer".to_string())
    }

    #[cfg(windows)]
    pub fn geometry(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    #[cfg(not(windows))]
    pub fn geometry(&self) -> (u32, u32) {
        let geo = self.x11_capturer.get_geometry();
        (geo.width, geo.height)
    }

    #[cfg(windows)]
    pub fn capture_frame(&mut self) -> Result<Vec<Bgr8>, CaptureError> {
        use dxgcap::CaptureError::*;

        match self.dxgi_manager.capture_frame() {
            Ok((data, (w, h))) => {
                self.width = w;
                self.height = h;
                Ok(data)
            }
            Err(AccessDenied) => Err(CaptureError::AccessDenied),
            Err(AccessLost) => Err(CaptureError::AccessLost),
            Err(RefreshFailure) => Err(CaptureError::RefreshFailure),
            Err(Timeout) => Err(CaptureError::Timeout),
            Err(Fail(e)) => Err(CaptureError::Fail(e.to_string())),
        }
    }

    #[cfg(not(windows))]
    pub fn capture_frame(&mut self) -> Result<Vec<Bgr8>, CaptureError> {
        self.x11_capturer
            .capture_frame()
            .map_err(|x11cap::CaptureError::Fail(e)| CaptureError::Fail(e.to_string()))
    }
}
