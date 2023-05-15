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

/// A screen capturer.
///
/// Can capture video frames with reasonable performance for
/// screenshooting, recording, streaming, etc.
#[cfg(windows)]
pub struct Capturer {
    dxgi_manager: dxgcap::DXGIManager,
    width: usize,
    height: usize,
    image: Option<Vec<Bgr8>>,
}

/// A screen capturer.
///
/// Can capture video frames with reasonable performance for
/// screenshooting, recording, streaming, etc.
#[cfg(not(windows))]
pub struct Capturer {
    x11_capturer: x11cap::Capturer,
    pub image: Option<x11cap::Image>,
}

impl Capturer {
    /// Construct a new capturer for a given capture source, e.g. a display.
    #[cfg(windows)]
    pub fn new(capture_src: usize) -> Result<Capturer, String> {
        Capturer::new_with_timeout(capture_src, Duration::from_millis(200))
    }

    /// Windows only, does nothing on other platforms. Construct a new capturer for a given capture source, e.g. a display, with a given timeout.
    #[cfg(windows)]
    pub fn new_with_timeout(capture_src: usize, timeout: Duration) -> Result<Capturer, String> {
        (timeout.as_secs() as u32)
            .checked_mul(1000)
            .and_then(|ms| ms.checked_add(timeout.subsec_millis()))
            .ok_or(
                "Failed to convert the given duration to a legal u32 millisecond value due to \
                    integer overflow.",
            )
            .and_then(|timeout| {
                dxgcap::DXGIManager::new(timeout).map(|mut mgr| {
                    mgr.set_capture_source_index(capture_src);
                    Capturer {
                        dxgi_manager: mgr,
                        width: 0,
                        height: 0,
                        image: None,
                    }
                })
            })
            .map_err(|err| err.to_owned())
    }

    /// Construct a new capturer for a given capture source, e.g. a display.
    #[cfg(not(windows))]
    pub fn new(capture_src: usize) -> Result<Capturer, String> {
        x11cap::Capturer::new(x11cap::CaptureSource::Monitor(capture_src))
            .map(|c| Capturer {
                x11_capturer: c,
                image: None,
            })
            .map_err(|()| "Failed to initialize capturer".to_string())
    }

    /// Windows only, does nothing on other platforms. Construct a new capturer for a given capture source, e.g. a display, with a given timeout.
    #[cfg(not(windows))]
    pub fn new_with_timeout(_capture_src: usize, _timeout: Duration) -> Result<Capturer, String> {
        Err("Windows only method. Does nothing on other platforms.".to_string())
    }

    /// Returns the width and height of the area to capture
    #[cfg(windows)]
    pub fn geometry(&self) -> (u32, u32) {
        let (w, h) = self.dxgi_manager.geometry();
        (w as u32, h as u32)
    }

    /// Returns the width and height of the area to capture
    #[cfg(not(windows))]
    pub fn geometry(&self) -> (u32, u32) {
        let geo = self.x11_capturer.get_geometry();
        (geo.width, geo.height)
    }

    /// Returns the horizontal and vertical offset of the capture source
    /// from the primary display.
    #[cfg(not(windows))]
    pub fn position(&self) -> (i32, i32) {
        let geo = self.x11_capturer.get_geometry();
        (geo.x, geo.y)
    }

    /// Capture screen and return an owned `Vec` of the image color data
    ///
    /// On Windows there's no performance difference between doing
    /// `self.capture_frame` and `self.capture_store_frame(); self.get_stored_frame()`
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

    /// Capture screen and return an owned `Vec` of the image color data in bgr format
    #[cfg(windows)]
    pub fn capture_frame_components(&mut self) -> Result<Vec<u8>, CaptureError> {
        use dxgcap::CaptureError::*;

        match self.dxgi_manager.capture_frame_components() {
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

    /// Capture screen and store in `self` for later retreival
    #[cfg(windows)]
    pub fn capture_store_frame(&mut self) -> Result<(), CaptureError> {
        use dxgcap::CaptureError::*;

        match self.dxgi_manager.capture_frame() {
            Ok((data, (w, h))) => {
                self.image = Some(data);
                self.width = w;
                self.height = h;
                Ok(())
            }
            Err(AccessDenied) => Err(CaptureError::AccessDenied),
            Err(AccessLost) => Err(CaptureError::AccessLost),
            Err(RefreshFailure) => Err(CaptureError::RefreshFailure),
            Err(Timeout) => Err(CaptureError::Timeout),
            Err(Fail(e)) => Err(CaptureError::Fail(e.to_string())),
        }
    }

    /// Capture screen and return an owned `Vec` of the image color data
    ///
    /// Worse performance than `self.capture_store_frame(); self.get_stored_frame()`
    /// due to an extra `.to_vec()` call.
    #[cfg(not(windows))]
    pub fn capture_frame(&mut self) -> Result<Vec<Bgr8>, CaptureError> {
        self.capture_store_frame()
            .map(|_| self.get_stored_frame().unwrap().to_vec())
    }

    /// Capture screen and store in `self` for later retreival
    ///
    /// Performs no unnecessary allocations or copies, and is as such faster than
    /// `Self::capture_frame`.
    ///
    /// Recommended over `Self::capture_frame` unless an owned `Vec` is required.
    #[cfg(not(windows))]
    pub fn capture_store_frame(&mut self) -> Result<(), CaptureError> {
        match self.x11_capturer.capture_frame() {
            Ok(image) => {
                self.image = Some(image);
                Ok(())
            }
            Err(x11cap::CaptureError::Fail(e)) => Err(CaptureError::Fail(e.to_string())),
        }
    }

    /// Get the last frame stored in `self` by `Self::capture_store_frame`,
    /// if one has ever been stored.
    pub fn get_stored_frame(&self) -> Option<&[Bgr8]> {
        self.image.as_ref().map(|img| img.as_slice())
    }
}

#[cfg(all(test, windows))]
mod captrs_tests_windows {
    use super::*;

    #[test]
    fn test_capture_components() {
        let mut capturer = Capturer::new(0).unwrap();

        let (w, h) = capturer.geometry();

        let frame = capturer.capture_frame_components().unwrap();

        // check that the capture is the correct size
        // should be width * height * $ (RGBA)
        assert_eq!((w * h * 4) as usize, frame.len())
    }

    #[test]
    fn test_capture() {
        let mut capturer = Capturer::new(0).unwrap();

        let (w, h) = capturer.geometry();

        let frame = capturer.capture_frame().unwrap();

        // check that the capture is the correct size
        // should be width * height * $ (RGBA)
        assert_eq!((w * h) as usize, frame.len())
    }
}

#[cfg(all(test, not(windows)))]
mod captrs_tests_not_windows {
    use super::*;

    #[test]
    fn test_capture() {
        let mut capturer = Capturer::new(0).unwrap();

        let (w, h) = capturer.geometry();

        let frame = capturer.capture_frame().unwrap();

        // check that the capture is the correct size
        // should be width * height * $ (RGBA)
        assert_eq!((w * h) as usize, frame.len())
    }
}