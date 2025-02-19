use crate::error::{FfmpegError, FfmpegErrorCode};
use crate::ffi::*;
use crate::rational::Rational;
use crate::smart_object::{SmartObject, SmartPtr};
use crate::utils::{check_i64, or_nopts};
use crate::{AVPictureType, AVPixelFormat};

/// A frame. Thin wrapper around [`AVFrame`].
pub struct GenericFrame(SmartPtr<AVFrame>);

impl Clone for GenericFrame {
    fn clone(&self) -> Self {
        // Safety: `av_frame_clone` is safe to call.
        let clone = unsafe { av_frame_clone(self.0.as_ptr()) };

        // Safety: The pointer here is valid.
        unsafe { Self::wrap(clone).expect("failed to clone frame") }
    }
}

/// Safety: `GenericFrame` is safe to send between threads.
unsafe impl Send for GenericFrame {}

/// Safety: `GenericFrame` is safe to share between threads.
unsafe impl Sync for GenericFrame {}

/// A video frame. Thin wrapper around [`GenericFrame`]. Like a frame but has specific video properties.
#[derive(Clone)]
pub struct VideoFrame(GenericFrame);

/// An audio frame. Thin wrapper around [`GenericFrame`]. Like a frame but has specific audio properties.
#[derive(Clone)]
pub struct AudioFrame(GenericFrame);

impl GenericFrame {
    /// Creates a new frame.
    pub(crate) fn new() -> Result<Self, FfmpegError> {
        // Safety: `av_frame_alloc` is safe to call.
        let frame = unsafe { av_frame_alloc() };

        // Safety: The pointer here is valid.
        unsafe { Self::wrap(frame).ok_or(FfmpegError::Alloc) }
    }

    /// Wraps a pointer to an `AVFrame`.
    ///
    /// # Safety
    /// `ptr` must be a valid pointer to an `AVFrame`.
    pub(crate) unsafe fn wrap(ptr: *mut AVFrame) -> Option<Self> {
        SmartPtr::wrap_non_null(ptr, |ptr| av_frame_free(ptr)).map(Self)
    }

    /// Allocates a buffer for the frame.
    ///
    /// # Safety
    /// This function is unsafe because the caller must ensure the frame has not been allocated yet.
    /// Also the frame must be properly initialized after the allocation as the data is not zeroed out.
    /// Therefore reading from the frame after allocation will result in reading uninitialized data.
    pub(crate) unsafe fn alloc_frame_buffer(&mut self, alignment: Option<i32>) -> Result<(), FfmpegError> {
        // Safety: `self.as_mut_ptr()` is assumed to provide a valid mutable pointer to an
        // `AVFrame` structure. The `av_frame_get_buffer` function from FFMPEG allocates
        // and attaches a buffer to the `AVFrame` if it doesn't already exist.
        // It is the caller's responsibility to ensure that `self` is properly initialized
        // and represents a valid `AVFrame` instance.
        FfmpegErrorCode(unsafe { av_frame_get_buffer(self.as_mut_ptr(), alignment.unwrap_or(0)) }).result()?;
        Ok(())
    }

    /// Returns a pointer to the frame.
    pub(crate) const fn as_ptr(&self) -> *const AVFrame {
        self.0.as_ptr()
    }

    /// Returns a mutable pointer to the frame.
    pub(crate) const fn as_mut_ptr(&mut self) -> *mut AVFrame {
        self.0.as_mut_ptr()
    }

    /// Make this frame a video frame.
    pub(crate) const fn video(self) -> VideoFrame {
        VideoFrame(self)
    }

    /// Make this frame an audio frame.
    pub(crate) const fn audio(self) -> AudioFrame {
        AudioFrame(self)
    }

    /// Returns the presentation timestamp of the frame.
    pub(crate) const fn pts(&self) -> Option<i64> {
        check_i64(self.0.as_deref_except().pts)
    }

    /// Sets the presentation timestamp of the frame.
    pub(crate) const fn set_pts(&mut self, pts: Option<i64>) {
        self.0.as_deref_mut_except().pts = or_nopts(pts);
        self.0.as_deref_mut_except().best_effort_timestamp = or_nopts(pts);
    }

    /// Returns the duration of the frame.
    pub(crate) const fn duration(&self) -> Option<i64> {
        check_i64(self.0.as_deref_except().duration)
    }

    /// Sets the duration of the frame.
    pub(crate) const fn set_duration(&mut self, duration: Option<i64>) {
        self.0.as_deref_mut_except().duration = or_nopts(duration);
    }

    /// Returns the best effort timestamp of the frame.
    pub(crate) const fn best_effort_timestamp(&self) -> Option<i64> {
        check_i64(self.0.as_deref_except().best_effort_timestamp)
    }

    /// Returns the decoding timestamp of the frame.
    pub(crate) const fn dts(&self) -> Option<i64> {
        check_i64(self.0.as_deref_except().pkt_dts)
    }

    /// Sets the decoding timestamp of the frame.
    pub(crate) const fn set_dts(&mut self, dts: Option<i64>) {
        self.0.as_deref_mut_except().pkt_dts = or_nopts(dts);
    }

    /// Returns the time base of the frame.
    pub(crate) fn time_base(&self) -> Rational {
        self.0.as_deref_except().time_base.into()
    }

    /// Sets the time base of the frame.
    pub(crate) fn set_time_base(&mut self, time_base: impl Into<Rational>) {
        self.0.as_deref_mut_except().time_base = time_base.into().into();
    }

    /// Returns the format of the frame.
    pub(crate) const fn format(&self) -> i32 {
        self.0.as_deref_except().format
    }

    /// Sets the format of the frame.
    pub(crate) const fn set_format(&mut self, format: i32) {
        self.0.as_deref_mut_except().format = format;
    }

    /// Returns true if the frame is an audio frame.
    pub(crate) const fn is_audio(&self) -> bool {
        self.0.as_deref_except().ch_layout.nb_channels != 0
    }

    /// Returns true if the frame is a video frame.
    pub(crate) const fn is_video(&self) -> bool {
        self.0.as_deref_except().width != 0
    }

    /// Returns the linesize of the frame.
    pub(crate) const fn linesize(&self, index: usize) -> Option<i32> {
        if index >= self.0.as_deref_except().linesize.len() {
            return None;
        }
        Some(self.0.as_deref_except().linesize[index])
    }
}

impl std::fmt::Debug for GenericFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericFrame")
            .field("pts", &self.pts())
            .field("dts", &self.dts())
            .field("duration", &self.duration())
            .field("best_effort_timestamp", &self.best_effort_timestamp())
            .field("time_base", &self.time_base())
            .field("format", &self.format())
            .field("is_audio", &self.is_audio())
            .field("is_video", &self.is_video())
            .finish()
    }
}

impl VideoFrame {
    /// Creates a new video frame.
    pub fn new() -> Result<Self, FfmpegError> {
        let frame = GenericFrame::new()?;
        Ok(VideoFrame(frame))
    }

    /// Returns the width of the frame.
    pub const fn width(&self) -> usize {
        self.0 .0.as_deref_except().width as usize
    }

    /// Returns the height of the frame.
    pub const fn height(&self) -> usize {
        self.0 .0.as_deref_except().height as usize
    }

    /// Returns the sample aspect ratio of the frame.
    pub fn sample_aspect_ratio(&self) -> Rational {
        self.0 .0.as_deref_except().sample_aspect_ratio.into()
    }

    /// Sets the sample aspect ratio of the frame.
    pub fn set_sample_aspect_ratio(&mut self, sample_aspect_ratio: impl Into<Rational>) {
        self.0 .0.as_deref_mut_except().sample_aspect_ratio = sample_aspect_ratio.into().into();
    }

    /// Sets the width of the frame.
    pub const fn set_width(&mut self, width: usize) {
        self.0 .0.as_deref_mut_except().width = width as i32;
    }

    /// Sets the height of the frame.
    pub const fn set_height(&mut self, height: usize) {
        self.0 .0.as_deref_mut_except().height = height as i32;
    }

    /// Returns true if the frame is a keyframe.
    pub const fn is_keyframe(&self) -> bool {
        self.0 .0.as_deref_except().key_frame != 0
    }

    /// Returns the picture type of the frame.
    pub const fn pict_type(&self) -> AVPictureType {
        AVPictureType(self.0 .0.as_deref_except().pict_type as i32)
    }

    /// Sets the picture type of the frame.
    pub const fn set_pict_type(&mut self, pict_type: AVPictureType) {
        self.0 .0.as_deref_mut_except().pict_type = pict_type.0 as u32;
    }

    /// Returns the data of the frame. By specifying the index of the plane.
    pub fn data(&self, index: usize) -> Option<&[u8]> {
        let line = self.linesize(index)? as usize;
        let height = self.height();
        let raw = *self.0 .0.as_deref_except().data.get(index)?;

        // Safety: The pointer here is valid & has the sizeof the `line * height`.
        unsafe { Some(std::slice::from_raw_parts(raw, line * height)) }
    }

    /// Returns the data of the frame. By specifying the index of the plane.
    pub fn data_mut(&mut self, index: usize) -> Option<&mut [u8]> {
        let line = self.linesize(index)? as usize;
        let height = self.height();
        let raw = *self.0 .0.as_deref_mut_except().data.get(index)?;

        // Safety: The pointer here is valid & has the sizeof the `line * height`.
        unsafe { Some(std::slice::from_raw_parts_mut(raw, line * height)) }
    }

    /// Get the pixel format of the frame.
    pub const fn format(&self) -> AVPixelFormat {
        AVPixelFormat(self.0 .0.as_deref_except().format)
    }
}

impl std::fmt::Debug for VideoFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VideoFrame")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("sample_aspect_ratio", &self.sample_aspect_ratio())
            .field("pts", &self.pts())
            .field("dts", &self.dts())
            .field("duration", &self.duration())
            .field("best_effort_timestamp", &self.best_effort_timestamp())
            .field("time_base", &self.time_base())
            .field("format", &self.format())
            .field("is_audio", &self.is_audio())
            .field("is_video", &self.is_video())
            .field("is_keyframe", &self.is_keyframe())
            .finish()
    }
}

impl std::ops::Deref for VideoFrame {
    type Target = GenericFrame;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for VideoFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A thin wrapper around `AVChannelLayout` to make it easier to use.
pub struct AudioChannelLayout(SmartObject<AVChannelLayout>);

impl Default for AudioChannelLayout {
    fn default() -> Self {
        // Safety: this is a c-struct and those are safe to zero out.
        let zeroed_layout = unsafe { std::mem::zeroed() };

        Self(SmartObject::new(zeroed_layout, Self::destructor))
    }
}

impl AudioChannelLayout {
    #[doc(hidden)]
    fn destructor(ptr: &mut AVChannelLayout) {
        // Safety: `av_channel_layout_uninit` is safe to call.
        unsafe { av_channel_layout_uninit(ptr) };
    }

    /// Creates a new `AudioChannelLayout` instance.
    pub fn new(channels: i32) -> Result<Self, FfmpegError> {
        let mut layout = Self::default();

        // Safety: `av_channel_layout_default` is safe to call.
        unsafe { av_channel_layout_default(layout.0.as_mut(), channels) };

        layout.validate()?;

        Ok(layout)
    }

    /// Validates the channel layout.
    pub fn validate(&self) -> Result<(), FfmpegError> {
        // Safety: `av_channel_layout_check` is safe to call
        if unsafe { av_channel_layout_check(self.0.as_ref()) } == 0 {
            return Err(FfmpegError::Arguments("invalid channel layout"));
        }

        Ok(())
    }

    /// Wraps an `AVChannelLayout` automatically calling `av_channel_layout_uninit` on drop.
    ///
    /// # Safety
    /// Requires that the layout can be safely deallocated with `av_channel_layout_uninit`
    pub unsafe fn wrap(layout: AVChannelLayout) -> Self {
        Self(SmartObject::new(layout, Self::destructor))
    }

    /// Returns the number of channels in the layout.
    pub fn channel_count(&self) -> i32 {
        self.0.as_ref().nb_channels
    }

    /// Consumes the `AudioChannelLayout` and returns the inner `AVChannelLayout`.
    /// The caller is responsible for calling `av_channel_layout_uninit` on the returned value.
    pub fn into_inner(self) -> AVChannelLayout {
        self.0.into_inner()
    }

    pub(crate) fn apply(mut self, layout: &mut AVChannelLayout) {
        std::mem::swap(layout, self.0.as_mut());
    }
}

impl AudioFrame {
    /// Sets channel layout to default with a channel count of `channel_count`.
    pub fn set_channel_layout_default(&mut self, channel_count: usize) -> Result<(), FfmpegError> {
        let layout = AudioChannelLayout::new(channel_count as i32)?;

        // Safety: Our pointer is valid.
        let av_frame = unsafe { self.as_mut_ptr().as_mut() }.ok_or(FfmpegError::Alloc)?;

        layout.apply(&mut av_frame.ch_layout);

        Ok(())
    }

    /// Sets channel layout to a custom layout. Note that the channel count
    /// is defined by the given `crate::ffi::AVChannelLayout`.
    pub fn set_channel_layout_custom(&mut self, custom_layout: AudioChannelLayout) -> Result<(), FfmpegError> {
        custom_layout.validate()?;

        // Safety: Our pointer is valid.
        let av_frame = unsafe { self.as_mut_ptr().as_mut() }.ok_or(FfmpegError::Alloc)?;

        custom_layout.apply(&mut av_frame.ch_layout);

        Ok(())
    }

    /// Returns the channel layout of the frame.
    pub const fn channel_layout(&self) -> AVChannelLayout {
        self.0 .0.as_deref_except().ch_layout
    }

    /// Returns the channel count of the frame.
    pub const fn channel_count(&self) -> usize {
        self.0 .0.as_deref_except().ch_layout.nb_channels as usize
    }

    /// Returns the number of samples in the frame.
    pub const fn nb_samples(&self) -> i32 {
        self.0 .0.as_deref_except().nb_samples
    }

    /// Sets the number of samples in the frame.
    pub const fn set_nb_samples(&mut self, nb_samples: usize) {
        self.0 .0.as_deref_mut_except().nb_samples = nb_samples as i32;
    }

    /// Returns the sample rate of the frame.
    pub const fn sample_rate(&self) -> i32 {
        self.0 .0.as_deref_except().sample_rate
    }

    /// Sets the sample rate of the frame.
    pub const fn set_sample_rate(&mut self, sample_rate: usize) {
        self.0 .0.as_deref_mut_except().sample_rate = sample_rate as i32;
    }
}

impl std::fmt::Debug for AudioFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioFrame")
            .field("channel_count", &self.channel_count())
            .field("nb_samples", &self.nb_samples())
            .field("sample_rate", &self.sample_rate())
            .field("pts", &self.pts())
            .field("dts", &self.dts())
            .field("duration", &self.duration())
            .field("best_effort_timestamp", &self.best_effort_timestamp())
            .field("time_base", &self.time_base())
            .field("format", &self.format())
            .field("is_audio", &self.is_audio())
            .field("is_video", &self.is_video())
            .finish()
    }
}

impl std::ops::Deref for AudioFrame {
    type Target = GenericFrame;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for AudioFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use insta::assert_debug_snapshot;
    use rand::Rng;
    use rusty_ffmpeg::ffi::AVRational;

    use crate::ffi::av_frame_get_buffer;
    use crate::frame::{AudioChannelLayout, GenericFrame, VideoFrame};
    use crate::rational::Rational;
    use crate::{AVChannelOrder, AVPictureType, AVPixelFormat, AVSampleFormat};

    #[test]
    fn test_frame_clone() {
        let mut frame = VideoFrame::new().expect("Failed to create frame");
        frame.set_format(AVPixelFormat::Yuv420p.into());

        // Safety: Our pointer is valid.
        frame.set_width(16);
        frame.set_height(16);

        // Safety: `av_frame_get_buffer` is safe to call.
        unsafe { frame.alloc_frame_buffer(Some(32)) }.expect("Failed to allocate frame buffer");

        frame.set_pts(Some(12));
        frame.set_dts(Some(34));
        frame.set_duration(Some(5));
        frame.set_time_base(Rational::static_new::<1, 30>());
        frame.set_format(AVPixelFormat::Yuv420p.into());

        let cloned_frame = frame.clone();

        assert_eq!(
            format!("{:?}", frame),
            format!("{:?}", cloned_frame),
            "Cloned frame should be equal to the original frame."
        );
    }

    #[test]
    fn test_audio_conversion() {
        let frame = GenericFrame::new().expect("Failed to create frame");
        let mut audio_frame = frame.audio();

        assert!(audio_frame.set_channel_layout_default(2).is_ok());
        assert!(audio_frame.is_audio(), "The frame should be identified as audio.");
        assert!(!audio_frame.is_video(), "The frame should not be identified as video.");
    }

    #[test]
    fn test_set_format() {
        let mut frame = GenericFrame::new().expect("Failed to create frame");
        frame.set_format(AVPixelFormat::Yuv420p.into());
        assert_eq!(
            frame.format(),
            AVPixelFormat::Yuv420p.0,
            "The format should match the set value."
        );

        frame.set_format(AVPixelFormat::Rgb24.into());
        assert_eq!(
            frame.format(),
            AVPixelFormat::Rgb24.0,
            "The format should match the updated value."
        );
    }

    #[test]
    fn test_linesize() {
        let mut frame = VideoFrame::new().expect("Failed to create frame");
        frame.set_format(AVPixelFormat::Yuv420p.into());
        frame.set_width(1920);
        frame.set_height(1080);

        // Safety: `av_frame_get_buffer` is safe to call.
        unsafe { frame.alloc_frame_buffer(Some(32)) }.expect("Failed to allocate frame buffer");

        assert!(
            frame.linesize(0).unwrap_or(0) > 0,
            "Linesize should be greater than zero for valid index."
        );

        assert!(
            frame.linesize(100).is_none(),
            "Linesize at an invalid index should return None."
        );
    }

    #[test]
    fn test_frame_debug() {
        let mut frame = GenericFrame::new().expect("Failed to create frame");
        frame.set_pts(Some(12345));
        frame.set_dts(Some(67890));
        frame.set_duration(Some(1000));
        frame.set_time_base(Rational::static_new::<1, 30>());
        frame.set_format(AVPixelFormat::Yuv420p.into());

        assert_debug_snapshot!(frame, @r"
        GenericFrame {
            pts: Some(
                12345,
            ),
            dts: Some(
                67890,
            ),
            duration: Some(
                1000,
            ),
            best_effort_timestamp: Some(
                12345,
            ),
            time_base: Rational {
                numerator: 1,
                denominator: 30,
            },
            format: 0,
            is_audio: false,
            is_video: false,
        }
        ");
    }

    #[test]
    fn test_sample_aspect_ratio() {
        let frame = GenericFrame::new().expect("Failed to create frame");
        let mut video_frame = frame.video();
        let sample_aspect_ratio = Rational::static_new::<16, 9>();
        video_frame.set_sample_aspect_ratio(sample_aspect_ratio);

        assert_eq!(
            video_frame.sample_aspect_ratio(),
            sample_aspect_ratio,
            "Sample aspect ratio should match the set value."
        );
    }

    #[test]
    fn test_pict_type() {
        let frame = GenericFrame::new().expect("Failed to create frame");
        let mut video_frame = frame.video();
        video_frame.set_pict_type(AVPictureType::Intra);

        assert_eq!(
            video_frame.pict_type(),
            AVPictureType::Intra,
            "Picture type should match the set value."
        );
    }

    #[test]
    fn test_data_allocation_and_access() {
        let mut frame = GenericFrame::new().expect("Failed to create frame");
        frame.set_format(AVPixelFormat::Yuv420p.into());
        let mut video_frame = frame.video();
        video_frame.set_width(16);
        video_frame.set_height(16);

        // Safety: Our pointer is valid.
        let av_frame = unsafe { video_frame.as_mut_ptr().as_mut() }.expect("Failed to get mutable pointer");

        // Safety: `av_frame_get_buffer` is safe to call.
        unsafe {
            assert!(av_frame_get_buffer(av_frame, 32) >= 0, "Failed to allocate buffer for frame.");
        }

        // randomize y-plane (data[0])
        let linesize = av_frame.linesize[0] as usize; // bytes per row
        let height = av_frame.height as usize; // total rows
        let data_ptr = av_frame.data[0]; // pointer to the Y-plane data

        let randomized_data = if !data_ptr.is_null() {
            // Safety: `std::slice::from_raw_parts_mut` is safe to call.
            let data_slice = unsafe { std::slice::from_raw_parts_mut(data_ptr, linesize * height) };
            let mut rng = rand::rng();
            rng.fill(data_slice);
            data_slice.to_vec()
        } else {
            panic!("Failed to get valid data pointer for Y-plane.");
        };

        if let Some(data) = video_frame.data(0) {
            assert_eq!(data, randomized_data.as_slice(), "Data does not match randomized content.");
        } else {
            panic!("Data at index 0 should not be None.");
        }
    }

    #[test]
    fn test_video_frame_debug() {
        let mut frame = GenericFrame::new().expect("Failed to create frame");
        frame.set_pts(Some(12345));
        frame.set_dts(Some(67890));
        frame.set_duration(Some(1000));
        frame.set_time_base(AVRational { num: 1, den: 30 });
        frame.set_format(AVPixelFormat::Yuv420p.into());
        let mut video_frame = frame.video();
        video_frame.set_width(1920);
        video_frame.set_height(1080);
        video_frame.set_sample_aspect_ratio(AVRational { num: 16, den: 9 });

        assert_debug_snapshot!(video_frame, @r"
        VideoFrame {
            width: 1920,
            height: 1080,
            sample_aspect_ratio: Rational {
                numerator: 16,
                denominator: 9,
            },
            pts: Some(
                12345,
            ),
            dts: Some(
                67890,
            ),
            duration: Some(
                1000,
            ),
            best_effort_timestamp: Some(
                12345,
            ),
            time_base: Rational {
                numerator: 1,
                denominator: 30,
            },
            format: AVPixelFormat::Yuv420p,
            is_audio: false,
            is_video: true,
            is_keyframe: false,
        }
        ");
    }

    #[test]
    fn test_set_channel_layout_default_invalid_count_error() {
        let frame = GenericFrame::new().expect("Failed to create frame");
        let mut audio_frame = frame.audio();

        assert!(
            audio_frame.set_channel_layout_default(usize::MAX).is_err(),
            "Expected error for invalid channel count."
        );
    }

    #[test]
    fn test_set_channel_layout_custom_invalid_layout_error() {
        let frame = GenericFrame::new().expect("Failed to create frame");
        let mut audio_frame = frame.audio();
        // Safety: This is safe to be deallocated by the layout destructor.
        let custom_layout = unsafe {
            AudioChannelLayout::wrap(crate::ffi::AVChannelLayout {
                order: AVChannelOrder::Native.into(),
                nb_channels: -1,
                u: crate::ffi::AVChannelLayout__bindgen_ty_1 { mask: 2 },
                opaque: std::ptr::null_mut(),
            })
        };

        assert!(
            audio_frame.set_channel_layout_custom(custom_layout).is_err(),
            "Expected error for invalid custom channel layout"
        );
    }

    #[test]
    fn test_set_channel_layout_custom() {
        let frame = GenericFrame::new().expect("Failed to create frame");
        let mut audio_frame = frame.audio();
        // Safety: This is safe to be deallocated by the layout destructor.
        let custom_layout = unsafe {
            AudioChannelLayout::wrap(crate::ffi::AVChannelLayout {
                order: AVChannelOrder::Native.into(),
                nb_channels: 2,
                u: crate::ffi::AVChannelLayout__bindgen_ty_1 { mask: 3 },
                opaque: std::ptr::null_mut(),
            })
        };

        assert!(
            audio_frame.set_channel_layout_custom(custom_layout).is_ok(),
            "Failed to set custom channel layout"
        );

        let layout = audio_frame.channel_layout();
        assert_eq!(layout.nb_channels, 2, "Expected channel layout to have 2 channels (stereo).");
        assert_eq!(
            // Safety: this should be a mask not a pointer.
            unsafe { layout.u.mask },
            3,
            "Expected channel mask to match AV_CH_LAYOUT_STEREO."
        );
        assert_eq!(
            AVChannelOrder(layout.order),
            AVChannelOrder::Native,
            "Expected channel order to be AV_CHANNEL_ORDER_NATIVE."
        );
    }

    #[test]
    fn test_alloc_frame_buffer() {
        let cases = [(None, true), (Some(0), true), (Some(32), true), (Some(-1), false)];

        for alignment in cases {
            let mut frame = GenericFrame::new().expect("Failed to create frame");
            frame.set_format(AVSampleFormat::S16.into());
            let mut audio_frame = frame.audio();
            audio_frame.set_nb_samples(1024);
            audio_frame.set_sample_rate(44100);

            assert!(
                audio_frame.set_channel_layout_default(2).is_ok(),
                "Failed to set default channel layout"
            );

            assert_eq!(
                // Safety: `audio_frame` is a valid pointer. And we dont attempt to read from the frame until after the allocation.
                unsafe { audio_frame.alloc_frame_buffer(alignment.0).is_ok() },
                alignment.1,
                "Failed to allocate buffer with alignment {:?}",
                alignment
            );
        }
    }

    #[test]
    fn test_alloc_frame_buffer_error() {
        let cases = [None, Some(0), Some(32), Some(-1)];

        for alignment in cases {
            let mut frame = GenericFrame::new().expect("Failed to create frame");
            frame.set_format(AVSampleFormat::S16.into());
            let mut audio_frame = frame.audio();
            audio_frame.set_nb_samples(1024);

            assert!(
                // Safety: `audio_frame` is a valid pointer. And we dont attempt to read from the frame until after the allocation.
                unsafe { audio_frame.alloc_frame_buffer(alignment).is_err() },
                "Should fail to allocate buffer with invalid frame and alignment {:?}",
                alignment
            );
        }
    }

    #[test]
    fn test_nb_samples() {
        let mut frame = GenericFrame::new().expect("Failed to create frame");
        frame.set_format(AVSampleFormat::S16.into());
        let mut audio_frame = frame.audio();
        audio_frame.set_nb_samples(1024);

        assert_eq!(
            audio_frame.nb_samples(),
            1024,
            "The number of samples should match the set value."
        );
    }

    #[test]
    fn test_sample_rate() {
        let mut frame = GenericFrame::new().expect("Failed to create frame");
        frame.set_format(AVSampleFormat::S16.into());
        let mut audio_frame = frame.audio();
        audio_frame.set_sample_rate(44100);

        assert_eq!(
            audio_frame.sample_rate(),
            44100,
            "The sample rate should match the set value."
        );
    }

    #[test]
    fn test_audio_frame_debug() {
        let mut frame = GenericFrame::new().expect("Failed to create frame");
        frame.set_format(AVSampleFormat::S16.into());
        let mut audio_frame = frame.audio();
        audio_frame.set_nb_samples(1024);
        audio_frame.set_sample_rate(44100);
        audio_frame.set_pts(Some(12345));
        audio_frame.set_dts(Some(67890));
        audio_frame.set_duration(Some(512));
        audio_frame.set_time_base(AVRational { num: 1, den: 44100 });

        assert!(
            audio_frame.set_channel_layout_default(2).is_ok(),
            "Failed to set default channel layout"
        );
        assert_debug_snapshot!(audio_frame, @r"
        AudioFrame {
            channel_count: 2,
            nb_samples: 1024,
            sample_rate: 44100,
            pts: Some(
                12345,
            ),
            dts: Some(
                67890,
            ),
            duration: Some(
                512,
            ),
            best_effort_timestamp: Some(
                12345,
            ),
            time_base: Rational {
                numerator: 1,
                denominator: 44100,
            },
            format: 1,
            is_audio: true,
            is_video: false,
        }
        ");
    }
}
