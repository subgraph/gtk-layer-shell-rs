// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GtkLayerShellEdge")]
pub enum Edge {
    #[doc(alias = "GTK_LAYER_SHELL_EDGE_LEFT")]
    Left,
    #[doc(alias = "GTK_LAYER_SHELL_EDGE_RIGHT")]
    Right,
    #[doc(alias = "GTK_LAYER_SHELL_EDGE_TOP")]
    Top,
    #[doc(alias = "GTK_LAYER_SHELL_EDGE_BOTTOM")]
    Bottom,
    #[doc(alias = "GTK_LAYER_SHELL_EDGE_ENTRY_NUMBER")]
    EntryNumber,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Edge::{}",
            match *self {
                Edge::Left => "Left",
                Edge::Right => "Right",
                Edge::Top => "Top",
                Edge::Bottom => "Bottom",
                Edge::EntryNumber => "EntryNumber",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for Edge {
    type GlibType = ffi::GtkLayerShellEdge;

    fn to_glib(&self) -> ffi::GtkLayerShellEdge {
        match *self {
            Edge::Left => ffi::GTK_LAYER_SHELL_EDGE_LEFT,
            Edge::Right => ffi::GTK_LAYER_SHELL_EDGE_RIGHT,
            Edge::Top => ffi::GTK_LAYER_SHELL_EDGE_TOP,
            Edge::Bottom => ffi::GTK_LAYER_SHELL_EDGE_BOTTOM,
            Edge::EntryNumber => ffi::GTK_LAYER_SHELL_EDGE_ENTRY_NUMBER,
            Edge::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkLayerShellEdge> for Edge {
    unsafe fn from_glib(value: ffi::GtkLayerShellEdge) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Edge::Left,
            1 => Edge::Right,
            2 => Edge::Top,
            3 => Edge::Bottom,
            4 => Edge::EntryNumber,
            value => Edge::__Unknown(value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GtkLayerShellLayer")]
pub enum Layer {
    #[doc(alias = "GTK_LAYER_SHELL_LAYER_BACKGROUND")]
    Background,
    #[doc(alias = "GTK_LAYER_SHELL_LAYER_BOTTOM")]
    Bottom,
    #[doc(alias = "GTK_LAYER_SHELL_LAYER_TOP")]
    Top,
    #[doc(alias = "GTK_LAYER_SHELL_LAYER_OVERLAY")]
    Overlay,
    #[doc(alias = "GTK_LAYER_SHELL_LAYER_ENTRY_NUMBER")]
    EntryNumber,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Layer::{}",
            match *self {
                Layer::Background => "Background",
                Layer::Bottom => "Bottom",
                Layer::Top => "Top",
                Layer::Overlay => "Overlay",
                Layer::EntryNumber => "EntryNumber",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for Layer {
    type GlibType = ffi::GtkLayerShellLayer;

    fn to_glib(&self) -> ffi::GtkLayerShellLayer {
        match *self {
            Layer::Background => ffi::GTK_LAYER_SHELL_LAYER_BACKGROUND,
            Layer::Bottom => ffi::GTK_LAYER_SHELL_LAYER_BOTTOM,
            Layer::Top => ffi::GTK_LAYER_SHELL_LAYER_TOP,
            Layer::Overlay => ffi::GTK_LAYER_SHELL_LAYER_OVERLAY,
            Layer::EntryNumber => ffi::GTK_LAYER_SHELL_LAYER_ENTRY_NUMBER,
            Layer::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkLayerShellLayer> for Layer {
    unsafe fn from_glib(value: ffi::GtkLayerShellLayer) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Layer::Background,
            1 => Layer::Bottom,
            2 => Layer::Top,
            3 => Layer::Overlay,
            4 => Layer::EntryNumber,
            value => Layer::__Unknown(value),
        }
    }
}
