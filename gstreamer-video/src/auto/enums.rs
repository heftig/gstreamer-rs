// This file was generated by gir (https://github.com/gtk-rs/gir @ d1e0127)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_ffi;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VideoColorMatrix {
    Unknown,
    Rgb,
    Fcc,
    Bt709,
    Bt601,
    Smpte240m,
    Bt2020,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for VideoColorMatrix {
    type GlibType = ffi::GstVideoColorMatrix;

    fn to_glib(&self) -> ffi::GstVideoColorMatrix {
        match *self {
            VideoColorMatrix::Unknown => ffi::GST_VIDEO_COLOR_MATRIX_UNKNOWN,
            VideoColorMatrix::Rgb => ffi::GST_VIDEO_COLOR_MATRIX_RGB,
            VideoColorMatrix::Fcc => ffi::GST_VIDEO_COLOR_MATRIX_FCC,
            VideoColorMatrix::Bt709 => ffi::GST_VIDEO_COLOR_MATRIX_BT709,
            VideoColorMatrix::Bt601 => ffi::GST_VIDEO_COLOR_MATRIX_BT601,
            VideoColorMatrix::Smpte240m => ffi::GST_VIDEO_COLOR_MATRIX_SMPTE240M,
            VideoColorMatrix::Bt2020 => ffi::GST_VIDEO_COLOR_MATRIX_BT2020,
            VideoColorMatrix::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoColorMatrix> for VideoColorMatrix {
    fn from_glib(value: ffi::GstVideoColorMatrix) -> Self {
        skip_assert_initialized!();
        match value {
            0 => VideoColorMatrix::Unknown,
            1 => VideoColorMatrix::Rgb,
            2 => VideoColorMatrix::Fcc,
            3 => VideoColorMatrix::Bt709,
            4 => VideoColorMatrix::Bt601,
            5 => VideoColorMatrix::Smpte240m,
            6 => VideoColorMatrix::Bt2020,
            value => VideoColorMatrix::__Unknown(value),
        }
    }
}

impl StaticType for VideoColorMatrix {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_color_matrix_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoColorMatrix {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoColorMatrix {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for VideoColorMatrix {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VideoColorPrimaries {
    Unknown,
    Bt709,
    Bt470m,
    Bt470bg,
    Smpte170m,
    Smpte240m,
    Film,
    Bt2020,
    Adobergb,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for VideoColorPrimaries {
    type GlibType = ffi::GstVideoColorPrimaries;

    fn to_glib(&self) -> ffi::GstVideoColorPrimaries {
        match *self {
            VideoColorPrimaries::Unknown => ffi::GST_VIDEO_COLOR_PRIMARIES_UNKNOWN,
            VideoColorPrimaries::Bt709 => ffi::GST_VIDEO_COLOR_PRIMARIES_BT709,
            VideoColorPrimaries::Bt470m => ffi::GST_VIDEO_COLOR_PRIMARIES_BT470M,
            VideoColorPrimaries::Bt470bg => ffi::GST_VIDEO_COLOR_PRIMARIES_BT470BG,
            VideoColorPrimaries::Smpte170m => ffi::GST_VIDEO_COLOR_PRIMARIES_SMPTE170M,
            VideoColorPrimaries::Smpte240m => ffi::GST_VIDEO_COLOR_PRIMARIES_SMPTE240M,
            VideoColorPrimaries::Film => ffi::GST_VIDEO_COLOR_PRIMARIES_FILM,
            VideoColorPrimaries::Bt2020 => ffi::GST_VIDEO_COLOR_PRIMARIES_BT2020,
            VideoColorPrimaries::Adobergb => ffi::GST_VIDEO_COLOR_PRIMARIES_ADOBERGB,
            VideoColorPrimaries::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoColorPrimaries> for VideoColorPrimaries {
    fn from_glib(value: ffi::GstVideoColorPrimaries) -> Self {
        skip_assert_initialized!();
        match value {
            0 => VideoColorPrimaries::Unknown,
            1 => VideoColorPrimaries::Bt709,
            2 => VideoColorPrimaries::Bt470m,
            3 => VideoColorPrimaries::Bt470bg,
            4 => VideoColorPrimaries::Smpte170m,
            5 => VideoColorPrimaries::Smpte240m,
            6 => VideoColorPrimaries::Film,
            7 => VideoColorPrimaries::Bt2020,
            8 => VideoColorPrimaries::Adobergb,
            value => VideoColorPrimaries::__Unknown(value),
        }
    }
}

impl StaticType for VideoColorPrimaries {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_color_primaries_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoColorPrimaries {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoColorPrimaries {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for VideoColorPrimaries {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VideoFieldOrder {
    Unknown,
    TopFieldFirst,
    BottomFieldFirst,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl ToGlib for VideoFieldOrder {
    type GlibType = ffi::GstVideoFieldOrder;

    fn to_glib(&self) -> ffi::GstVideoFieldOrder {
        match *self {
            VideoFieldOrder::Unknown => ffi::GST_VIDEO_FIELD_ORDER_UNKNOWN,
            VideoFieldOrder::TopFieldFirst => ffi::GST_VIDEO_FIELD_ORDER_TOP_FIELD_FIRST,
            VideoFieldOrder::BottomFieldFirst => ffi::GST_VIDEO_FIELD_ORDER_BOTTOM_FIELD_FIRST,
            VideoFieldOrder::__Unknown(value) => value
        }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
#[doc(hidden)]
impl FromGlib<ffi::GstVideoFieldOrder> for VideoFieldOrder {
    fn from_glib(value: ffi::GstVideoFieldOrder) -> Self {
        skip_assert_initialized!();
        match value {
            0 => VideoFieldOrder::Unknown,
            1 => VideoFieldOrder::TopFieldFirst,
            2 => VideoFieldOrder::BottomFieldFirst,
            value => VideoFieldOrder::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl StaticType for VideoFieldOrder {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_field_order_get_type()) }
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl<'a> FromValueOptional<'a> for VideoFieldOrder {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl<'a> FromValue<'a> for VideoFieldOrder {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_12", feature = "dox"))]
impl SetValue for VideoFieldOrder {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VideoFormat {
    Unknown,
    Encoded,
    I420,
    Yv12,
    Yuy2,
    Uyvy,
    Ayuv,
    Rgbx,
    Bgrx,
    Xrgb,
    Xbgr,
    Rgba,
    Bgra,
    Argb,
    Abgr,
    Rgb,
    Bgr,
    Y41b,
    Y42b,
    Yvyu,
    Y444,
    V210,
    V216,
    Nv12,
    Nv21,
    Gray8,
    Gray16Be,
    Gray16Le,
    V308,
    Rgb16,
    Bgr16,
    Rgb15,
    Bgr15,
    Uyvp,
    A420,
    Rgb8p,
    Yuv9,
    Yvu9,
    Iyu1,
    Argb64,
    Ayuv64,
    R210,
    I42010be,
    I42010le,
    I42210be,
    I42210le,
    Y44410be,
    Y44410le,
    Gbr,
    Gbr10be,
    Gbr10le,
    Nv16,
    Nv24,
    Nv1264z32,
    A42010be,
    A42010le,
    A42210be,
    A42210le,
    A44410be,
    A44410le,
    Nv61,
    P01010be,
    P01010le,
    Iyu2,
    Vyuy,
    Gbra,
    Gbra10be,
    Gbra10le,
    Gbr12be,
    Gbr12le,
    Gbra12be,
    Gbra12le,
    I42012be,
    I42012le,
    I42212be,
    I42212le,
    Y44412be,
    Y44412le,
    Gray10Le32,
    Nv1210le32,
    Nv1610le32,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for VideoFormat {
    type GlibType = ffi::GstVideoFormat;

    fn to_glib(&self) -> ffi::GstVideoFormat {
        match *self {
            VideoFormat::Unknown => ffi::GST_VIDEO_FORMAT_UNKNOWN,
            VideoFormat::Encoded => ffi::GST_VIDEO_FORMAT_ENCODED,
            VideoFormat::I420 => ffi::GST_VIDEO_FORMAT_I420,
            VideoFormat::Yv12 => ffi::GST_VIDEO_FORMAT_YV12,
            VideoFormat::Yuy2 => ffi::GST_VIDEO_FORMAT_YUY2,
            VideoFormat::Uyvy => ffi::GST_VIDEO_FORMAT_UYVY,
            VideoFormat::Ayuv => ffi::GST_VIDEO_FORMAT_AYUV,
            VideoFormat::Rgbx => ffi::GST_VIDEO_FORMAT_RGBx,
            VideoFormat::Bgrx => ffi::GST_VIDEO_FORMAT_BGRx,
            VideoFormat::Xrgb => ffi::GST_VIDEO_FORMAT_xRGB,
            VideoFormat::Xbgr => ffi::GST_VIDEO_FORMAT_xBGR,
            VideoFormat::Rgba => ffi::GST_VIDEO_FORMAT_RGBA,
            VideoFormat::Bgra => ffi::GST_VIDEO_FORMAT_BGRA,
            VideoFormat::Argb => ffi::GST_VIDEO_FORMAT_ARGB,
            VideoFormat::Abgr => ffi::GST_VIDEO_FORMAT_ABGR,
            VideoFormat::Rgb => ffi::GST_VIDEO_FORMAT_RGB,
            VideoFormat::Bgr => ffi::GST_VIDEO_FORMAT_BGR,
            VideoFormat::Y41b => ffi::GST_VIDEO_FORMAT_Y41B,
            VideoFormat::Y42b => ffi::GST_VIDEO_FORMAT_Y42B,
            VideoFormat::Yvyu => ffi::GST_VIDEO_FORMAT_YVYU,
            VideoFormat::Y444 => ffi::GST_VIDEO_FORMAT_Y444,
            VideoFormat::V210 => ffi::GST_VIDEO_FORMAT_v210,
            VideoFormat::V216 => ffi::GST_VIDEO_FORMAT_v216,
            VideoFormat::Nv12 => ffi::GST_VIDEO_FORMAT_NV12,
            VideoFormat::Nv21 => ffi::GST_VIDEO_FORMAT_NV21,
            VideoFormat::Gray8 => ffi::GST_VIDEO_FORMAT_GRAY8,
            VideoFormat::Gray16Be => ffi::GST_VIDEO_FORMAT_GRAY16_BE,
            VideoFormat::Gray16Le => ffi::GST_VIDEO_FORMAT_GRAY16_LE,
            VideoFormat::V308 => ffi::GST_VIDEO_FORMAT_v308,
            VideoFormat::Rgb16 => ffi::GST_VIDEO_FORMAT_RGB16,
            VideoFormat::Bgr16 => ffi::GST_VIDEO_FORMAT_BGR16,
            VideoFormat::Rgb15 => ffi::GST_VIDEO_FORMAT_RGB15,
            VideoFormat::Bgr15 => ffi::GST_VIDEO_FORMAT_BGR15,
            VideoFormat::Uyvp => ffi::GST_VIDEO_FORMAT_UYVP,
            VideoFormat::A420 => ffi::GST_VIDEO_FORMAT_A420,
            VideoFormat::Rgb8p => ffi::GST_VIDEO_FORMAT_RGB8P,
            VideoFormat::Yuv9 => ffi::GST_VIDEO_FORMAT_YUV9,
            VideoFormat::Yvu9 => ffi::GST_VIDEO_FORMAT_YVU9,
            VideoFormat::Iyu1 => ffi::GST_VIDEO_FORMAT_IYU1,
            VideoFormat::Argb64 => ffi::GST_VIDEO_FORMAT_ARGB64,
            VideoFormat::Ayuv64 => ffi::GST_VIDEO_FORMAT_AYUV64,
            VideoFormat::R210 => ffi::GST_VIDEO_FORMAT_r210,
            VideoFormat::I42010be => ffi::GST_VIDEO_FORMAT_I420_10BE,
            VideoFormat::I42010le => ffi::GST_VIDEO_FORMAT_I420_10LE,
            VideoFormat::I42210be => ffi::GST_VIDEO_FORMAT_I422_10BE,
            VideoFormat::I42210le => ffi::GST_VIDEO_FORMAT_I422_10LE,
            VideoFormat::Y44410be => ffi::GST_VIDEO_FORMAT_Y444_10BE,
            VideoFormat::Y44410le => ffi::GST_VIDEO_FORMAT_Y444_10LE,
            VideoFormat::Gbr => ffi::GST_VIDEO_FORMAT_GBR,
            VideoFormat::Gbr10be => ffi::GST_VIDEO_FORMAT_GBR_10BE,
            VideoFormat::Gbr10le => ffi::GST_VIDEO_FORMAT_GBR_10LE,
            VideoFormat::Nv16 => ffi::GST_VIDEO_FORMAT_NV16,
            VideoFormat::Nv24 => ffi::GST_VIDEO_FORMAT_NV24,
            VideoFormat::Nv1264z32 => ffi::GST_VIDEO_FORMAT_NV12_64Z32,
            VideoFormat::A42010be => ffi::GST_VIDEO_FORMAT_A420_10BE,
            VideoFormat::A42010le => ffi::GST_VIDEO_FORMAT_A420_10LE,
            VideoFormat::A42210be => ffi::GST_VIDEO_FORMAT_A422_10BE,
            VideoFormat::A42210le => ffi::GST_VIDEO_FORMAT_A422_10LE,
            VideoFormat::A44410be => ffi::GST_VIDEO_FORMAT_A444_10BE,
            VideoFormat::A44410le => ffi::GST_VIDEO_FORMAT_A444_10LE,
            VideoFormat::Nv61 => ffi::GST_VIDEO_FORMAT_NV61,
            VideoFormat::P01010be => ffi::GST_VIDEO_FORMAT_P010_10BE,
            VideoFormat::P01010le => ffi::GST_VIDEO_FORMAT_P010_10LE,
            VideoFormat::Iyu2 => ffi::GST_VIDEO_FORMAT_IYU2,
            VideoFormat::Vyuy => ffi::GST_VIDEO_FORMAT_VYUY,
            VideoFormat::Gbra => ffi::GST_VIDEO_FORMAT_GBRA,
            VideoFormat::Gbra10be => ffi::GST_VIDEO_FORMAT_GBRA_10BE,
            VideoFormat::Gbra10le => ffi::GST_VIDEO_FORMAT_GBRA_10LE,
            VideoFormat::Gbr12be => ffi::GST_VIDEO_FORMAT_GBR_12BE,
            VideoFormat::Gbr12le => ffi::GST_VIDEO_FORMAT_GBR_12LE,
            VideoFormat::Gbra12be => ffi::GST_VIDEO_FORMAT_GBRA_12BE,
            VideoFormat::Gbra12le => ffi::GST_VIDEO_FORMAT_GBRA_12LE,
            VideoFormat::I42012be => ffi::GST_VIDEO_FORMAT_I420_12BE,
            VideoFormat::I42012le => ffi::GST_VIDEO_FORMAT_I420_12LE,
            VideoFormat::I42212be => ffi::GST_VIDEO_FORMAT_I422_12BE,
            VideoFormat::I42212le => ffi::GST_VIDEO_FORMAT_I422_12LE,
            VideoFormat::Y44412be => ffi::GST_VIDEO_FORMAT_Y444_12BE,
            VideoFormat::Y44412le => ffi::GST_VIDEO_FORMAT_Y444_12LE,
            VideoFormat::Gray10Le32 => ffi::GST_VIDEO_FORMAT_GRAY10_LE32,
            VideoFormat::Nv1210le32 => ffi::GST_VIDEO_FORMAT_NV12_10LE32,
            VideoFormat::Nv1610le32 => ffi::GST_VIDEO_FORMAT_NV16_10LE32,
            VideoFormat::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFormat> for VideoFormat {
    fn from_glib(value: ffi::GstVideoFormat) -> Self {
        skip_assert_initialized!();
        match value {
            0 => VideoFormat::Unknown,
            1 => VideoFormat::Encoded,
            2 => VideoFormat::I420,
            3 => VideoFormat::Yv12,
            4 => VideoFormat::Yuy2,
            5 => VideoFormat::Uyvy,
            6 => VideoFormat::Ayuv,
            7 => VideoFormat::Rgbx,
            8 => VideoFormat::Bgrx,
            9 => VideoFormat::Xrgb,
            10 => VideoFormat::Xbgr,
            11 => VideoFormat::Rgba,
            12 => VideoFormat::Bgra,
            13 => VideoFormat::Argb,
            14 => VideoFormat::Abgr,
            15 => VideoFormat::Rgb,
            16 => VideoFormat::Bgr,
            17 => VideoFormat::Y41b,
            18 => VideoFormat::Y42b,
            19 => VideoFormat::Yvyu,
            20 => VideoFormat::Y444,
            21 => VideoFormat::V210,
            22 => VideoFormat::V216,
            23 => VideoFormat::Nv12,
            24 => VideoFormat::Nv21,
            25 => VideoFormat::Gray8,
            26 => VideoFormat::Gray16Be,
            27 => VideoFormat::Gray16Le,
            28 => VideoFormat::V308,
            29 => VideoFormat::Rgb16,
            30 => VideoFormat::Bgr16,
            31 => VideoFormat::Rgb15,
            32 => VideoFormat::Bgr15,
            33 => VideoFormat::Uyvp,
            34 => VideoFormat::A420,
            35 => VideoFormat::Rgb8p,
            36 => VideoFormat::Yuv9,
            37 => VideoFormat::Yvu9,
            38 => VideoFormat::Iyu1,
            39 => VideoFormat::Argb64,
            40 => VideoFormat::Ayuv64,
            41 => VideoFormat::R210,
            42 => VideoFormat::I42010be,
            43 => VideoFormat::I42010le,
            44 => VideoFormat::I42210be,
            45 => VideoFormat::I42210le,
            46 => VideoFormat::Y44410be,
            47 => VideoFormat::Y44410le,
            48 => VideoFormat::Gbr,
            49 => VideoFormat::Gbr10be,
            50 => VideoFormat::Gbr10le,
            51 => VideoFormat::Nv16,
            52 => VideoFormat::Nv24,
            53 => VideoFormat::Nv1264z32,
            54 => VideoFormat::A42010be,
            55 => VideoFormat::A42010le,
            56 => VideoFormat::A42210be,
            57 => VideoFormat::A42210le,
            58 => VideoFormat::A44410be,
            59 => VideoFormat::A44410le,
            60 => VideoFormat::Nv61,
            61 => VideoFormat::P01010be,
            62 => VideoFormat::P01010le,
            63 => VideoFormat::Iyu2,
            64 => VideoFormat::Vyuy,
            65 => VideoFormat::Gbra,
            66 => VideoFormat::Gbra10be,
            67 => VideoFormat::Gbra10le,
            68 => VideoFormat::Gbr12be,
            69 => VideoFormat::Gbr12le,
            70 => VideoFormat::Gbra12be,
            71 => VideoFormat::Gbra12le,
            72 => VideoFormat::I42012be,
            73 => VideoFormat::I42012le,
            74 => VideoFormat::I42212be,
            75 => VideoFormat::I42212le,
            76 => VideoFormat::Y44412be,
            77 => VideoFormat::Y44412le,
            78 => VideoFormat::Gray10Le32,
            79 => VideoFormat::Nv1210le32,
            80 => VideoFormat::Nv1610le32,
            value => VideoFormat::__Unknown(value),
        }
    }
}

impl StaticType for VideoFormat {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_format_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoFormat {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoFormat {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for VideoFormat {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VideoInterlaceMode {
    Progressive,
    Interleaved,
    Mixed,
    Fields,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for VideoInterlaceMode {
    type GlibType = ffi::GstVideoInterlaceMode;

    fn to_glib(&self) -> ffi::GstVideoInterlaceMode {
        match *self {
            VideoInterlaceMode::Progressive => ffi::GST_VIDEO_INTERLACE_MODE_PROGRESSIVE,
            VideoInterlaceMode::Interleaved => ffi::GST_VIDEO_INTERLACE_MODE_INTERLEAVED,
            VideoInterlaceMode::Mixed => ffi::GST_VIDEO_INTERLACE_MODE_MIXED,
            VideoInterlaceMode::Fields => ffi::GST_VIDEO_INTERLACE_MODE_FIELDS,
            VideoInterlaceMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoInterlaceMode> for VideoInterlaceMode {
    fn from_glib(value: ffi::GstVideoInterlaceMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => VideoInterlaceMode::Progressive,
            1 => VideoInterlaceMode::Interleaved,
            2 => VideoInterlaceMode::Mixed,
            3 => VideoInterlaceMode::Fields,
            value => VideoInterlaceMode::__Unknown(value),
        }
    }
}

impl StaticType for VideoInterlaceMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_interlace_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoInterlaceMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoInterlaceMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for VideoInterlaceMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VideoMultiviewFramePacking {
    None,
    Mono,
    Left,
    Right,
    SideBySide,
    SideBySideQuincunx,
    ColumnInterleaved,
    RowInterleaved,
    TopBottom,
    Checkerboard,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for VideoMultiviewFramePacking {
    type GlibType = ffi::GstVideoMultiviewFramePacking;

    fn to_glib(&self) -> ffi::GstVideoMultiviewFramePacking {
        match *self {
            VideoMultiviewFramePacking::None => ffi::GST_VIDEO_MULTIVIEW_FRAME_PACKING_NONE,
            VideoMultiviewFramePacking::Mono => ffi::GST_VIDEO_MULTIVIEW_FRAME_PACKING_MONO,
            VideoMultiviewFramePacking::Left => ffi::GST_VIDEO_MULTIVIEW_FRAME_PACKING_LEFT,
            VideoMultiviewFramePacking::Right => ffi::GST_VIDEO_MULTIVIEW_FRAME_PACKING_RIGHT,
            VideoMultiviewFramePacking::SideBySide => ffi::GST_VIDEO_MULTIVIEW_FRAME_PACKING_SIDE_BY_SIDE,
            VideoMultiviewFramePacking::SideBySideQuincunx => ffi::GST_VIDEO_MULTIVIEW_FRAME_PACKING_SIDE_BY_SIDE_QUINCUNX,
            VideoMultiviewFramePacking::ColumnInterleaved => ffi::GST_VIDEO_MULTIVIEW_FRAME_PACKING_COLUMN_INTERLEAVED,
            VideoMultiviewFramePacking::RowInterleaved => ffi::GST_VIDEO_MULTIVIEW_FRAME_PACKING_ROW_INTERLEAVED,
            VideoMultiviewFramePacking::TopBottom => ffi::GST_VIDEO_MULTIVIEW_FRAME_PACKING_TOP_BOTTOM,
            VideoMultiviewFramePacking::Checkerboard => ffi::GST_VIDEO_MULTIVIEW_FRAME_PACKING_CHECKERBOARD,
            VideoMultiviewFramePacking::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoMultiviewFramePacking> for VideoMultiviewFramePacking {
    fn from_glib(value: ffi::GstVideoMultiviewFramePacking) -> Self {
        skip_assert_initialized!();
        match value {
            -1 => VideoMultiviewFramePacking::None,
            0 => VideoMultiviewFramePacking::Mono,
            1 => VideoMultiviewFramePacking::Left,
            2 => VideoMultiviewFramePacking::Right,
            3 => VideoMultiviewFramePacking::SideBySide,
            4 => VideoMultiviewFramePacking::SideBySideQuincunx,
            5 => VideoMultiviewFramePacking::ColumnInterleaved,
            6 => VideoMultiviewFramePacking::RowInterleaved,
            7 => VideoMultiviewFramePacking::TopBottom,
            8 => VideoMultiviewFramePacking::Checkerboard,
            value => VideoMultiviewFramePacking::__Unknown(value),
        }
    }
}

impl StaticType for VideoMultiviewFramePacking {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_multiview_frame_packing_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoMultiviewFramePacking {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoMultiviewFramePacking {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for VideoMultiviewFramePacking {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VideoMultiviewMode {
    None,
    Mono,
    Left,
    Right,
    SideBySide,
    SideBySideQuincunx,
    ColumnInterleaved,
    RowInterleaved,
    TopBottom,
    Checkerboard,
    FrameByFrame,
    MultiviewFrameByFrame,
    Separated,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for VideoMultiviewMode {
    type GlibType = ffi::GstVideoMultiviewMode;

    fn to_glib(&self) -> ffi::GstVideoMultiviewMode {
        match *self {
            VideoMultiviewMode::None => ffi::GST_VIDEO_MULTIVIEW_MODE_NONE,
            VideoMultiviewMode::Mono => ffi::GST_VIDEO_MULTIVIEW_MODE_MONO,
            VideoMultiviewMode::Left => ffi::GST_VIDEO_MULTIVIEW_MODE_LEFT,
            VideoMultiviewMode::Right => ffi::GST_VIDEO_MULTIVIEW_MODE_RIGHT,
            VideoMultiviewMode::SideBySide => ffi::GST_VIDEO_MULTIVIEW_MODE_SIDE_BY_SIDE,
            VideoMultiviewMode::SideBySideQuincunx => ffi::GST_VIDEO_MULTIVIEW_MODE_SIDE_BY_SIDE_QUINCUNX,
            VideoMultiviewMode::ColumnInterleaved => ffi::GST_VIDEO_MULTIVIEW_MODE_COLUMN_INTERLEAVED,
            VideoMultiviewMode::RowInterleaved => ffi::GST_VIDEO_MULTIVIEW_MODE_ROW_INTERLEAVED,
            VideoMultiviewMode::TopBottom => ffi::GST_VIDEO_MULTIVIEW_MODE_TOP_BOTTOM,
            VideoMultiviewMode::Checkerboard => ffi::GST_VIDEO_MULTIVIEW_MODE_CHECKERBOARD,
            VideoMultiviewMode::FrameByFrame => ffi::GST_VIDEO_MULTIVIEW_MODE_FRAME_BY_FRAME,
            VideoMultiviewMode::MultiviewFrameByFrame => ffi::GST_VIDEO_MULTIVIEW_MODE_MULTIVIEW_FRAME_BY_FRAME,
            VideoMultiviewMode::Separated => ffi::GST_VIDEO_MULTIVIEW_MODE_SEPARATED,
            VideoMultiviewMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoMultiviewMode> for VideoMultiviewMode {
    fn from_glib(value: ffi::GstVideoMultiviewMode) -> Self {
        skip_assert_initialized!();
        match value {
            -1 => VideoMultiviewMode::None,
            0 => VideoMultiviewMode::Mono,
            1 => VideoMultiviewMode::Left,
            2 => VideoMultiviewMode::Right,
            3 => VideoMultiviewMode::SideBySide,
            4 => VideoMultiviewMode::SideBySideQuincunx,
            5 => VideoMultiviewMode::ColumnInterleaved,
            6 => VideoMultiviewMode::RowInterleaved,
            7 => VideoMultiviewMode::TopBottom,
            8 => VideoMultiviewMode::Checkerboard,
            32 => VideoMultiviewMode::FrameByFrame,
            33 => VideoMultiviewMode::MultiviewFrameByFrame,
            34 => VideoMultiviewMode::Separated,
            value => VideoMultiviewMode::__Unknown(value),
        }
    }
}

impl StaticType for VideoMultiviewMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_multiview_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoMultiviewMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoMultiviewMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for VideoMultiviewMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VideoTileMode {
    Unknown,
    Zflipz2x2,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for VideoTileMode {
    type GlibType = ffi::GstVideoTileMode;

    fn to_glib(&self) -> ffi::GstVideoTileMode {
        match *self {
            VideoTileMode::Unknown => ffi::GST_VIDEO_TILE_MODE_UNKNOWN,
            VideoTileMode::Zflipz2x2 => ffi::GST_VIDEO_TILE_MODE_ZFLIPZ_2X2,
            VideoTileMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoTileMode> for VideoTileMode {
    fn from_glib(value: ffi::GstVideoTileMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => VideoTileMode::Unknown,
            65536 => VideoTileMode::Zflipz2x2,
            value => VideoTileMode::__Unknown(value),
        }
    }
}

impl StaticType for VideoTileMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_tile_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoTileMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoTileMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for VideoTileMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum VideoTransferFunction {
    Unknown,
    Gamma10,
    Gamma18,
    Gamma20,
    Gamma22,
    Bt709,
    Smpte240m,
    Srgb,
    Gamma28,
    Log100,
    Log316,
    Bt202012,
    Adobergb,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for VideoTransferFunction {
    type GlibType = ffi::GstVideoTransferFunction;

    fn to_glib(&self) -> ffi::GstVideoTransferFunction {
        match *self {
            VideoTransferFunction::Unknown => ffi::GST_VIDEO_TRANSFER_UNKNOWN,
            VideoTransferFunction::Gamma10 => ffi::GST_VIDEO_TRANSFER_GAMMA10,
            VideoTransferFunction::Gamma18 => ffi::GST_VIDEO_TRANSFER_GAMMA18,
            VideoTransferFunction::Gamma20 => ffi::GST_VIDEO_TRANSFER_GAMMA20,
            VideoTransferFunction::Gamma22 => ffi::GST_VIDEO_TRANSFER_GAMMA22,
            VideoTransferFunction::Bt709 => ffi::GST_VIDEO_TRANSFER_BT709,
            VideoTransferFunction::Smpte240m => ffi::GST_VIDEO_TRANSFER_SMPTE240M,
            VideoTransferFunction::Srgb => ffi::GST_VIDEO_TRANSFER_SRGB,
            VideoTransferFunction::Gamma28 => ffi::GST_VIDEO_TRANSFER_GAMMA28,
            VideoTransferFunction::Log100 => ffi::GST_VIDEO_TRANSFER_LOG100,
            VideoTransferFunction::Log316 => ffi::GST_VIDEO_TRANSFER_LOG316,
            VideoTransferFunction::Bt202012 => ffi::GST_VIDEO_TRANSFER_BT2020_12,
            VideoTransferFunction::Adobergb => ffi::GST_VIDEO_TRANSFER_ADOBERGB,
            VideoTransferFunction::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoTransferFunction> for VideoTransferFunction {
    fn from_glib(value: ffi::GstVideoTransferFunction) -> Self {
        skip_assert_initialized!();
        match value {
            0 => VideoTransferFunction::Unknown,
            1 => VideoTransferFunction::Gamma10,
            2 => VideoTransferFunction::Gamma18,
            3 => VideoTransferFunction::Gamma20,
            4 => VideoTransferFunction::Gamma22,
            5 => VideoTransferFunction::Bt709,
            6 => VideoTransferFunction::Smpte240m,
            7 => VideoTransferFunction::Srgb,
            8 => VideoTransferFunction::Gamma28,
            9 => VideoTransferFunction::Log100,
            10 => VideoTransferFunction::Log316,
            11 => VideoTransferFunction::Bt202012,
            12 => VideoTransferFunction::Adobergb,
            value => VideoTransferFunction::__Unknown(value),
        }
    }
}

impl StaticType for VideoTransferFunction {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_video_transfer_function_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for VideoTransferFunction {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for VideoTransferFunction {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for VideoTransferFunction {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

