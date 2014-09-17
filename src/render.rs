/*
 * This file generated automatically from render.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(unused_unsafe)]
use std;
use libc::*;
use std::{mem,num,ptr,str};
use ffi::base::*;
use base;
use base::*;
use ffi;
use ffi::render::*;
use std::option::Option;
use std::iter::Iterator;

use xproto;

pub type pict_type = c_uint;//{
    pub static XCB_RENDER_PICT_TYPE_INDEXED : pict_type = 1;
    pub static XCB_RENDER_PICT_TYPE_DIRECT : pict_type = 2;
//}

pub type picture_enum = c_uint;//{
    pub static XCB_RENDER_PICTURE_NONE : picture_enum = 1;
//}

pub type pict_op = c_uint;//{
    pub static XCB_RENDER_PICT_OP_CLEAR : pict_op = 1;
    pub static XCB_RENDER_PICT_OP_SRC : pict_op = 2;
    pub static XCB_RENDER_PICT_OP_DST : pict_op = 3;
    pub static XCB_RENDER_PICT_OP_OVER : pict_op = 4;
    pub static XCB_RENDER_PICT_OP_OVER_REVERSE : pict_op = 5;
    pub static XCB_RENDER_PICT_OP_IN : pict_op = 6;
    pub static XCB_RENDER_PICT_OP_IN_REVERSE : pict_op = 7;
    pub static XCB_RENDER_PICT_OP_OUT : pict_op = 8;
    pub static XCB_RENDER_PICT_OP_OUT_REVERSE : pict_op = 9;
    pub static XCB_RENDER_PICT_OP_ATOP : pict_op = 10;
    pub static XCB_RENDER_PICT_OP_ATOP_REVERSE : pict_op = 11;
    pub static XCB_RENDER_PICT_OP_XOR : pict_op = 12;
    pub static XCB_RENDER_PICT_OP_ADD : pict_op = 13;
    pub static XCB_RENDER_PICT_OP_SATURATE : pict_op = 14;
    pub static XCB_RENDER_PICT_OP_DISJOINT_CLEAR : pict_op = 16;
    pub static XCB_RENDER_PICT_OP_DISJOINT_SRC : pict_op = 17;
    pub static XCB_RENDER_PICT_OP_DISJOINT_DST : pict_op = 18;
    pub static XCB_RENDER_PICT_OP_DISJOINT_OVER : pict_op = 19;
    pub static XCB_RENDER_PICT_OP_DISJOINT_OVER_REVERSE : pict_op = 20;
    pub static XCB_RENDER_PICT_OP_DISJOINT_IN : pict_op = 21;
    pub static XCB_RENDER_PICT_OP_DISJOINT_IN_REVERSE : pict_op = 22;
    pub static XCB_RENDER_PICT_OP_DISJOINT_OUT : pict_op = 23;
    pub static XCB_RENDER_PICT_OP_DISJOINT_OUT_REVERSE : pict_op = 24;
    pub static XCB_RENDER_PICT_OP_DISJOINT_ATOP : pict_op = 25;
    pub static XCB_RENDER_PICT_OP_DISJOINT_ATOP_REVERSE : pict_op = 26;
    pub static XCB_RENDER_PICT_OP_DISJOINT_XOR : pict_op = 27;
    pub static XCB_RENDER_PICT_OP_CONJOINT_CLEAR : pict_op = 32;
    pub static XCB_RENDER_PICT_OP_CONJOINT_SRC : pict_op = 33;
    pub static XCB_RENDER_PICT_OP_CONJOINT_DST : pict_op = 34;
    pub static XCB_RENDER_PICT_OP_CONJOINT_OVER : pict_op = 35;
    pub static XCB_RENDER_PICT_OP_CONJOINT_OVER_REVERSE : pict_op = 36;
    pub static XCB_RENDER_PICT_OP_CONJOINT_IN : pict_op = 37;
    pub static XCB_RENDER_PICT_OP_CONJOINT_IN_REVERSE : pict_op = 38;
    pub static XCB_RENDER_PICT_OP_CONJOINT_OUT : pict_op = 39;
    pub static XCB_RENDER_PICT_OP_CONJOINT_OUT_REVERSE : pict_op = 40;
    pub static XCB_RENDER_PICT_OP_CONJOINT_ATOP : pict_op = 41;
    pub static XCB_RENDER_PICT_OP_CONJOINT_ATOP_REVERSE : pict_op = 42;
    pub static XCB_RENDER_PICT_OP_CONJOINT_XOR : pict_op = 43;
    pub static XCB_RENDER_PICT_OP_MULTIPLY : pict_op = 48;
    pub static XCB_RENDER_PICT_OP_SCREEN : pict_op = 49;
    pub static XCB_RENDER_PICT_OP_OVERLAY : pict_op = 50;
    pub static XCB_RENDER_PICT_OP_DARKEN : pict_op = 51;
    pub static XCB_RENDER_PICT_OP_LIGHTEN : pict_op = 52;
    pub static XCB_RENDER_PICT_OP_COLOR_DODGE : pict_op = 53;
    pub static XCB_RENDER_PICT_OP_COLOR_BURN : pict_op = 54;
    pub static XCB_RENDER_PICT_OP_HARD_LIGHT : pict_op = 55;
    pub static XCB_RENDER_PICT_OP_SOFT_LIGHT : pict_op = 56;
    pub static XCB_RENDER_PICT_OP_DIFFERENCE : pict_op = 57;
    pub static XCB_RENDER_PICT_OP_EXCLUSION : pict_op = 58;
    pub static XCB_RENDER_PICT_OP_HSL_HUE : pict_op = 59;
    pub static XCB_RENDER_PICT_OP_HSL_SATURATION : pict_op = 60;
    pub static XCB_RENDER_PICT_OP_HSL_COLOR : pict_op = 61;
    pub static XCB_RENDER_PICT_OP_HSL_LUMINOSITY : pict_op = 62;
//}

pub type poly_edge = c_uint;//{
    pub static XCB_RENDER_POLY_EDGE_SHARP : poly_edge = 1;
    pub static XCB_RENDER_POLY_EDGE_SMOOTH : poly_edge = 2;
//}

pub type poly_mode = c_uint;//{
    pub static XCB_RENDER_POLY_MODE_PRECISE : poly_mode = 1;
    pub static XCB_RENDER_POLY_MODE_IMPRECISE : poly_mode = 2;
//}

pub type cp = c_uint;//{
    pub static XCB_RENDER_CP_REPEAT : cp = 1;
    pub static XCB_RENDER_CP_ALPHA_MAP : cp = 2;
    pub static XCB_RENDER_CP_ALPHA_X_ORIGIN : cp = 4;
    pub static XCB_RENDER_CP_ALPHA_Y_ORIGIN : cp = 8;
    pub static XCB_RENDER_CP_CLIP_X_ORIGIN : cp = 16;
    pub static XCB_RENDER_CP_CLIP_Y_ORIGIN : cp = 32;
    pub static XCB_RENDER_CP_CLIP_MASK : cp = 64;
    pub static XCB_RENDER_CP_GRAPHICS_EXPOSURE : cp = 128;
    pub static XCB_RENDER_CP_SUBWINDOW_MODE : cp = 256;
    pub static XCB_RENDER_CP_POLY_EDGE : cp = 512;
    pub static XCB_RENDER_CP_POLY_MODE : cp = 1024;
    pub static XCB_RENDER_CP_DITHER : cp = 2048;
    pub static XCB_RENDER_CP_COMPONENT_ALPHA : cp = 4096;
//}

pub type sub_pixel = c_uint;//{
    pub static XCB_RENDER_SUB_PIXEL_UNKNOWN : sub_pixel = 1;
    pub static XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB : sub_pixel = 2;
    pub static XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR : sub_pixel = 3;
    pub static XCB_RENDER_SUB_PIXEL_VERTICAL_RGB : sub_pixel = 4;
    pub static XCB_RENDER_SUB_PIXEL_VERTICAL_BGR : sub_pixel = 5;
    pub static XCB_RENDER_SUB_PIXEL_NONE : sub_pixel = 6;
//}

pub type repeat = c_uint;//{
    pub static XCB_RENDER_REPEAT_NONE : repeat = 1;
    pub static XCB_RENDER_REPEAT_NORMAL : repeat = 2;
    pub static XCB_RENDER_REPEAT_PAD : repeat = 3;
    pub static XCB_RENDER_REPEAT_REFLECT : repeat = 4;
//}
pub type Glyph = glyph;

pub impl Glyph {
fn new(data : glyph) -> Glyph { let val = Glyph; val }
}
pub type GlyphIterator = glyph_iterator;

pub type GlyphsetIterator = glyphset_iterator;

pub type PictureIterator = picture_iterator;

pub type PictformatIterator = pictformat_iterator;

pub type FixedIterator = fixed_iterator;

/** Opcode for xcb_render_pict_format. */
pub static XCB_RENDER_PICT_FORMAT : u8 = 0;
pub struct PictFormatError { pub base : base::Error<pict_format_error> }
pub impl PictFormatError {
    fn new(data : base::Error<pict_format_error>) -> PictFormatError {
        PictFormatError { base : base::Error { error : data } }
}
}
/** Opcode for xcb_render_picture. */
pub static XCB_RENDER_PICTURE : u8 = 1;
pub struct PictureError { pub base : base::Error<picture_error> }
pub impl PictureError {
    fn new(data : base::Error<picture_error>) -> PictureError {
        PictureError { base : base::Error { error : data } }
}
}
/** Opcode for xcb_render_pict_op. */
pub static XCB_RENDER_PICT_OP : u8 = 2;
pub struct PictOpError { pub base : base::Error<pict_op_error> }
pub impl PictOpError {
    fn new(data : base::Error<pict_op_error>) -> PictOpError {
        PictOpError { base : base::Error { error : data } }
}
}
/** Opcode for xcb_render_glyph_set. */
pub static XCB_RENDER_GLYPH_SET : u8 = 3;
pub struct GlyphSetError { pub base : base::Error<glyph_set_error> }
pub impl GlyphSetError {
    fn new(data : base::Error<glyph_set_error>) -> GlyphSetError {
        GlyphSetError { base : base::Error { error : data } }
}
}
/** Opcode for xcb_render_glyph. */
pub static XCB_RENDER_GLYPH : u8 = 4;
pub struct GlyphError { pub base : base::Error<glyph_error> }
pub impl GlyphError {
    fn new(data : base::Error<glyph_error>) -> GlyphError {
        GlyphError { base : base::Error { error : data } }
}
}
pub struct Directformat {
    pub base : base::Struct<directformat>
}

impl Directformat {
    fn new(data : Struct<directformat>) -> Directformat {
        Directformat { base : Struct { strct : data } }
}
}
pub type DirectformatIterator = directformat_iterator;

pub type PictforminfoIterator = pictforminfo_iterator;

pub type PictvisualIterator = pictvisual_iterator;

pub type PictdepthIterator = pictdepth_iterator;

pub type PictscreenIterator = pictscreen_iterator;

pub type IndexvalueIterator = indexvalue_iterator;

pub type ColorIterator = color_iterator;

pub type PointfixIterator = pointfix_iterator;

pub type LinefixIterator = linefix_iterator;

pub type TriangleIterator = triangle_iterator;

pub type TrapezoidIterator = trapezoid_iterator;

pub type GlyphinfoIterator = glyphinfo_iterator;

pub struct  QueryVersionCookie<'s> { pub base : base::Cookie<'s, query_version_cookie> }

pub impl<'s> QueryVersionCookie<'s> {
    fn new(data : base::Cookie<'s, query_version_cookie>) -> QueryVersionCookie {
        QueryVersionCookie { base : data }
}
}
/** Opcode for xcb_render_query_version. */
pub static XCB_RENDER_QUERY_VERSION : u8 = 0;
pub struct QueryVersionReply { base:  base::Reply<query_version_reply> }
pub impl QueryVersionCookie {
    fn new(data : base::Reply<query_version_request>) -> QueryVersionRequest {
        QueryVersionRequest { base : data }
}
}
pub struct  QueryPictFormatsCookie<'s> { pub base : base::Cookie<'s, query_pict_formats_cookie> }

pub impl<'s> QueryPictFormatsCookie<'s> {
    fn new(data : base::Cookie<'s, query_pict_formats_cookie>) -> QueryPictFormatsCookie {
        QueryPictFormatsCookie { base : data }
}
}
/** Opcode for xcb_render_query_pict_formats. */
pub static XCB_RENDER_QUERY_PICT_FORMATS : u8 = 1;
pub struct  QueryPictIndexValuesCookie<'s> { pub base : base::Cookie<'s, query_pict_index_values_cookie> }

pub impl<'s> QueryPictIndexValuesCookie<'s> {
    fn new(data : base::Cookie<'s, query_pict_index_values_cookie>) -> QueryPictIndexValuesCookie {
        QueryPictIndexValuesCookie { base : data }
}
}
/** Opcode for xcb_render_query_pict_index_values. */
pub static XCB_RENDER_QUERY_PICT_INDEX_VALUES : u8 = 2;
/** Opcode for xcb_render_create_picture. */
pub static XCB_RENDER_CREATE_PICTURE : u8 = 4;
/** Opcode for xcb_render_change_picture. */
pub static XCB_RENDER_CHANGE_PICTURE : u8 = 5;
/** Opcode for xcb_render_set_picture_clip_rectangles. */
pub static XCB_RENDER_SET_PICTURE_CLIP_RECTANGLES : u8 = 6;
/** Opcode for xcb_render_free_picture. */
pub static XCB_RENDER_FREE_PICTURE : u8 = 7;
/** Opcode for xcb_render_composite. */
pub static XCB_RENDER_COMPOSITE : u8 = 8;
/** Opcode for xcb_render_trapezoids. */
pub static XCB_RENDER_TRAPEZOIDS : u8 = 10;
/** Opcode for xcb_render_triangles. */
pub static XCB_RENDER_TRIANGLES : u8 = 11;
/** Opcode for xcb_render_tri_strip. */
pub static XCB_RENDER_TRI_STRIP : u8 = 12;
/** Opcode for xcb_render_tri_fan. */
pub static XCB_RENDER_TRI_FAN : u8 = 13;
/** Opcode for xcb_render_create_glyph_set. */
pub static XCB_RENDER_CREATE_GLYPH_SET : u8 = 17;
/** Opcode for xcb_render_reference_glyph_set. */
pub static XCB_RENDER_REFERENCE_GLYPH_SET : u8 = 18;
/** Opcode for xcb_render_free_glyph_set. */
pub static XCB_RENDER_FREE_GLYPH_SET : u8 = 19;
/** Opcode for xcb_render_add_glyphs. */
pub static XCB_RENDER_ADD_GLYPHS : u8 = 20;
/** Opcode for xcb_render_free_glyphs. */
pub static XCB_RENDER_FREE_GLYPHS : u8 = 22;
/** Opcode for xcb_render_composite_glyphs_8. */
pub static XCB_RENDER_COMPOSITE_GLYPHS_8 : u8 = 23;
/** Opcode for xcb_render_composite_glyphs_16. */
pub static XCB_RENDER_COMPOSITE_GLYPHS_16 : u8 = 24;
/** Opcode for xcb_render_composite_glyphs_32. */
pub static XCB_RENDER_COMPOSITE_GLYPHS_32 : u8 = 25;
/** Opcode for xcb_render_fill_rectangles. */
pub static XCB_RENDER_FILL_RECTANGLES : u8 = 26;
/** Opcode for xcb_render_create_cursor. */
pub static XCB_RENDER_CREATE_CURSOR : u8 = 27;
pub type TransformIterator = transform_iterator;

/** Opcode for xcb_render_set_picture_transform. */
pub static XCB_RENDER_SET_PICTURE_TRANSFORM : u8 = 28;
pub struct  QueryFiltersCookie<'s> { pub base : base::Cookie<'s, query_filters_cookie> }

pub impl<'s> QueryFiltersCookie<'s> {
    fn new(data : base::Cookie<'s, query_filters_cookie>) -> QueryFiltersCookie {
        QueryFiltersCookie { base : data }
}
}
/** Opcode for xcb_render_query_filters. */
pub static XCB_RENDER_QUERY_FILTERS : u8 = 29;
/** Opcode for xcb_render_set_picture_filter. */
pub static XCB_RENDER_SET_PICTURE_FILTER : u8 = 30;
pub type AnimcursoreltIterator = animcursorelt_iterator;

/** Opcode for xcb_render_create_anim_cursor. */
pub static XCB_RENDER_CREATE_ANIM_CURSOR : u8 = 31;
pub type SpanfixIterator = spanfix_iterator;

pub type TrapIterator = trap_iterator;

/** Opcode for xcb_render_add_traps. */
pub static XCB_RENDER_ADD_TRAPS : u8 = 32;
/** Opcode for xcb_render_create_solid_fill. */
pub static XCB_RENDER_CREATE_SOLID_FILL : u8 = 33;
/** Opcode for xcb_render_create_linear_gradient. */
pub static XCB_RENDER_CREATE_LINEAR_GRADIENT : u8 = 34;
/** Opcode for xcb_render_create_radial_gradient. */
pub static XCB_RENDER_CREATE_RADIAL_GRADIENT : u8 = 35;
/** Opcode for xcb_render_create_conical_gradient. */
pub static XCB_RENDER_CREATE_CONICAL_GRADIENT : u8 = 36;

impl Iterator<Glyph> for GlyphIterator {
    fn next(&mut self) -> Option<Glyph> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut glyph_iterator = self;
            let data = (*iter).data;
            xcb_render_glyph_next(iter);
            Some(Glyph::new(*data))
        }
    }
}

pub type Glyphset = glyphset;

pub impl Glyphset {
fn new(data : glyphset) -> Glyphset { let val = Glyphset; val }
}

impl Iterator<Glyphset> for GlyphsetIterator {
    fn next(&mut self) -> Option<Glyphset> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut glyphset_iterator = self;
            let data = (*iter).data;
            xcb_render_glyphset_next(iter);
            Some(Glyphset::new(*data))
        }
    }
}

pub type Picture = picture;

pub impl Picture {
fn new(data : picture) -> Picture { let val = Picture; val }
}

impl Iterator<Picture> for PictureIterator {
    fn next(&mut self) -> Option<Picture> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut picture_iterator = self;
            let data = (*iter).data;
            xcb_render_picture_next(iter);
            Some(Picture::new(*data))
        }
    }
}

pub type Pictformat = pictformat;

pub impl Pictformat {
fn new(data : pictformat) -> Pictformat { let val = Pictformat; val }
}

impl Iterator<Pictformat> for PictformatIterator {
    fn next(&mut self) -> Option<Pictformat> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pictformat_iterator = self;
            let data = (*iter).data;
            xcb_render_pictformat_next(iter);
            Some(Pictformat::new(*data))
        }
    }
}

pub type Fixed = fixed;

pub impl Fixed {
fn new(data : fixed) -> Fixed { let val = Fixed; val }
}

impl Iterator<Fixed> for FixedIterator {
    fn next(&mut self) -> Option<Fixed> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut fixed_iterator = self;
            let data = (*iter).data;
            xcb_render_fixed_next(iter);
            Some(Fixed::new(*data))
        }
    }
}


impl Directformat {
  pub fn red_shift(&mut self) -> u16 {
    unsafe { accessor!(red_shift -> u16, self.base.strct) }
  }

  pub fn red_mask(&mut self) -> u16 {
    unsafe { accessor!(red_mask -> u16, self.base.strct) }
  }

  pub fn green_shift(&mut self) -> u16 {
    unsafe { accessor!(green_shift -> u16, self.base.strct) }
  }

  pub fn green_mask(&mut self) -> u16 {
    unsafe { accessor!(green_mask -> u16, self.base.strct) }
  }

  pub fn blue_shift(&mut self) -> u16 {
    unsafe { accessor!(blue_shift -> u16, self.base.strct) }
  }

  pub fn blue_mask(&mut self) -> u16 {
    unsafe { accessor!(blue_mask -> u16, self.base.strct) }
  }

  pub fn alpha_shift(&mut self) -> u16 {
    unsafe { accessor!(alpha_shift -> u16, self.base.strct) }
  }

  pub fn alpha_mask(&mut self) -> u16 {
    unsafe { accessor!(alpha_mask -> u16, self.base.strct) }
  }

}

impl Iterator<Directformat> for DirectformatIterator {
    fn next(&mut self) -> Option<Directformat> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut directformat_iterator = self;
            let data = (*iter).data;
            xcb_render_directformat_next(iter);
            Some(Directformat::new(*data))
        }
    }
}

pub struct Pictforminfo {
    pub base : base::Struct<pictforminfo>
}

impl Pictforminfo {
    fn new(data : Struct<pictforminfo>) -> Pictforminfo {
        Pictforminfo { base : Struct { strct : data } }
}
}

impl Pictforminfo {
  pub fn id(&mut self) -> Pictformat {
    unsafe { accessor!(id -> Pictformat, self.base.strct) }
  }

  pub fn type_(&mut self) -> u8 {
    unsafe { accessor!(type_ -> u8, self.base.strct) }
  }

  pub fn depth(&mut self) -> u8 {
    unsafe { accessor!(depth -> u8, self.base.strct) }
  }

  pub fn direct(&self) -> Directformat {
    unsafe { mem::transmute(self.base.strct.direct) }
  }
  pub fn colormap(&mut self) -> xproto::Colormap {
    unsafe { accessor!(colormap -> xproto::Colormap, self.base.strct) }
  }

}

impl Iterator<Pictforminfo> for PictforminfoIterator {
    fn next(&mut self) -> Option<Pictforminfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pictforminfo_iterator = self;
            let data = (*iter).data;
            xcb_render_pictforminfo_next(iter);
            Some(Pictforminfo::new(*data))
        }
    }
}

pub struct Pictvisual {
    pub base : base::Struct<pictvisual>
}

impl Pictvisual {
    fn new(data : Struct<pictvisual>) -> Pictvisual {
        Pictvisual { base : Struct { strct : data } }
}
}

impl Pictvisual {
  pub fn visual(&mut self) -> xproto::Visualid {
    unsafe { accessor!(visual -> xproto::Visualid, self.base.strct) }
  }

  pub fn format(&mut self) -> Pictformat {
    unsafe { accessor!(format -> Pictformat, self.base.strct) }
  }

}

impl Iterator<Pictvisual> for PictvisualIterator {
    fn next(&mut self) -> Option<Pictvisual> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pictvisual_iterator = self;
            let data = (*iter).data;
            xcb_render_pictvisual_next(iter);
            Some(Pictvisual::new(*data))
        }
    }
}

pub struct Pictdepth {
    pub base : base::Struct<pictdepth>
}

impl Pictdepth {
    fn new(data : Struct<pictdepth>) -> Pictdepth {
        Pictdepth { base : Struct { strct : data } }
}
}

impl Pictdepth {
  pub fn depth(&mut self) -> u8 {
    unsafe { accessor!(depth -> u8, self.base.strct) }
  }

  pub fn visuals(&mut self) -> PictvisualIterator {
    unsafe { accessor!(PictvisualIterator, xcb_render_pictdepth_visuals_iterator, self.base.strct) }
  }

}

impl Iterator<Pictdepth> for PictdepthIterator {
    fn next(&mut self) -> Option<Pictdepth> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pictdepth_iterator = self;
            let data = (*iter).data;
            xcb_render_pictdepth_next(iter);
            Some(Pictdepth::new(*data))
        }
    }
}

pub struct Pictscreen {
    pub base : base::Struct<pictscreen>
}

impl Pictscreen {
    fn new(data : Struct<pictscreen>) -> Pictscreen {
        Pictscreen { base : Struct { strct : data } }
}
}

impl Pictscreen {
  pub fn fallback(&mut self) -> Pictformat {
    unsafe { accessor!(fallback -> Pictformat, self.base.strct) }
  }

  pub fn depths(&mut self) -> PictdepthIterator {
    unsafe { accessor!(PictdepthIterator, xcb_render_pictscreen_depths_iterator, self.base.strct) }
  }

}

impl Iterator<Pictscreen> for PictscreenIterator {
    fn next(&mut self) -> Option<Pictscreen> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pictscreen_iterator = self;
            let data = (*iter).data;
            xcb_render_pictscreen_next(iter);
            Some(Pictscreen::new(*data))
        }
    }
}

pub struct Indexvalue {
    pub base : base::Struct<indexvalue>
}

impl Indexvalue {
    fn new(data : Struct<indexvalue>) -> Indexvalue {
        Indexvalue { base : Struct { strct : data } }
}
}

impl Indexvalue {
  pub fn pixel(&mut self) -> u32 {
    unsafe { accessor!(pixel -> u32, self.base.strct) }
  }

  pub fn red(&mut self) -> u16 {
    unsafe { accessor!(red -> u16, self.base.strct) }
  }

  pub fn green(&mut self) -> u16 {
    unsafe { accessor!(green -> u16, self.base.strct) }
  }

  pub fn blue(&mut self) -> u16 {
    unsafe { accessor!(blue -> u16, self.base.strct) }
  }

  pub fn alpha(&mut self) -> u16 {
    unsafe { accessor!(alpha -> u16, self.base.strct) }
  }

}

impl Iterator<Indexvalue> for IndexvalueIterator {
    fn next(&mut self) -> Option<Indexvalue> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut indexvalue_iterator = self;
            let data = (*iter).data;
            xcb_render_indexvalue_next(iter);
            Some(Indexvalue::new(*data))
        }
    }
}

pub struct Color {
    pub base : base::Struct<color>
}

impl Color {
    fn new(data : Struct<color>) -> Color {
        Color { base : Struct { strct : data } }
}
}

impl Color {
  pub fn red(&mut self) -> u16 {
    unsafe { accessor!(red -> u16, self.base.strct) }
  }

  pub fn green(&mut self) -> u16 {
    unsafe { accessor!(green -> u16, self.base.strct) }
  }

  pub fn blue(&mut self) -> u16 {
    unsafe { accessor!(blue -> u16, self.base.strct) }
  }

  pub fn alpha(&mut self) -> u16 {
    unsafe { accessor!(alpha -> u16, self.base.strct) }
  }

}

impl Iterator<Color> for ColorIterator {
    fn next(&mut self) -> Option<Color> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut color_iterator = self;
            let data = (*iter).data;
            xcb_render_color_next(iter);
            Some(Color::new(*data))
        }
    }
}

pub struct Pointfix {
    pub base : base::Struct<pointfix>
}

impl Pointfix {
    fn new(data : Struct<pointfix>) -> Pointfix {
        Pointfix { base : Struct { strct : data } }
}
}

impl Pointfix {
  pub fn x(&mut self) -> Fixed {
    unsafe { accessor!(x -> Fixed, self.base.strct) }
  }

  pub fn y(&mut self) -> Fixed {
    unsafe { accessor!(y -> Fixed, self.base.strct) }
  }

}

impl Iterator<Pointfix> for PointfixIterator {
    fn next(&mut self) -> Option<Pointfix> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut pointfix_iterator = self;
            let data = (*iter).data;
            xcb_render_pointfix_next(iter);
            Some(Pointfix::new(*data))
        }
    }
}

pub struct Linefix {
    pub base : base::Struct<linefix>
}

impl Linefix {
    fn new(data : Struct<linefix>) -> Linefix {
        Linefix { base : Struct { strct : data } }
}
}

impl Linefix {
  pub fn p1(&self) -> Pointfix {
    unsafe { mem::transmute(self.base.strct.p1) }
  }
  pub fn p2(&self) -> Pointfix {
    unsafe { mem::transmute(self.base.strct.p2) }
  }
}

impl Iterator<Linefix> for LinefixIterator {
    fn next(&mut self) -> Option<Linefix> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut linefix_iterator = self;
            let data = (*iter).data;
            xcb_render_linefix_next(iter);
            Some(Linefix::new(*data))
        }
    }
}

pub struct Triangle {
    pub base : base::Struct<triangle>
}

impl Triangle {
    fn new(data : Struct<triangle>) -> Triangle {
        Triangle { base : Struct { strct : data } }
}
}

impl Triangle {
  pub fn p1(&self) -> Pointfix {
    unsafe { mem::transmute(self.base.strct.p1) }
  }
  pub fn p2(&self) -> Pointfix {
    unsafe { mem::transmute(self.base.strct.p2) }
  }
  pub fn p3(&self) -> Pointfix {
    unsafe { mem::transmute(self.base.strct.p3) }
  }
}

impl Iterator<Triangle> for TriangleIterator {
    fn next(&mut self) -> Option<Triangle> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut triangle_iterator = self;
            let data = (*iter).data;
            xcb_render_triangle_next(iter);
            Some(Triangle::new(*data))
        }
    }
}

pub struct Trapezoid {
    pub base : base::Struct<trapezoid>
}

impl Trapezoid {
    fn new(data : Struct<trapezoid>) -> Trapezoid {
        Trapezoid { base : Struct { strct : data } }
}
}

impl Trapezoid {
  pub fn top(&mut self) -> Fixed {
    unsafe { accessor!(top -> Fixed, self.base.strct) }
  }

  pub fn bottom(&mut self) -> Fixed {
    unsafe { accessor!(bottom -> Fixed, self.base.strct) }
  }

  pub fn left(&self) -> Linefix {
    unsafe { mem::transmute(self.base.strct.left) }
  }
  pub fn right(&self) -> Linefix {
    unsafe { mem::transmute(self.base.strct.right) }
  }
}

impl Iterator<Trapezoid> for TrapezoidIterator {
    fn next(&mut self) -> Option<Trapezoid> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut trapezoid_iterator = self;
            let data = (*iter).data;
            xcb_render_trapezoid_next(iter);
            Some(Trapezoid::new(*data))
        }
    }
}

pub struct Glyphinfo {
    pub base : base::Struct<glyphinfo>
}

impl Glyphinfo {
    fn new(data : Struct<glyphinfo>) -> Glyphinfo {
        Glyphinfo { base : Struct { strct : data } }
}
}

impl Glyphinfo {
  pub fn width(&mut self) -> u16 {
    unsafe { accessor!(width -> u16, self.base.strct) }
  }

  pub fn height(&mut self) -> u16 {
    unsafe { accessor!(height -> u16, self.base.strct) }
  }

  pub fn x(&mut self) -> i16 {
    unsafe { accessor!(x -> i16, self.base.strct) }
  }

  pub fn y(&mut self) -> i16 {
    unsafe { accessor!(y -> i16, self.base.strct) }
  }

  pub fn x_off(&mut self) -> i16 {
    unsafe { accessor!(x_off -> i16, self.base.strct) }
  }

  pub fn y_off(&mut self) -> i16 {
    unsafe { accessor!(y_off -> i16, self.base.strct) }
  }

}

impl Iterator<Glyphinfo> for GlyphinfoIterator {
    fn next(&mut self) -> Option<Glyphinfo> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut glyphinfo_iterator = self;
            let data = (*iter).data;
            xcb_render_glyphinfo_next(iter);
            Some(Glyphinfo::new(*data))
        }
    }
}

pub fn QueryVersion<'r> (c : &'r Connection,
                     client_major_version : u32,
                     client_minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_version(c.get_raw_conn(),
        client_major_version as u32, //1
        client_minor_version as u32); //2
    QueryVersionCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn QueryVersionUnchecked<'r> (c : &'r Connection,
                              client_major_version : u32,
                              client_minor_version : u32) -> QueryVersionCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_version_unchecked(c.get_raw_conn(),
        client_major_version as u32, //1
        client_minor_version as u32); //2
    QueryVersionCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}

impl QueryVersionReply {
  pub fn major_version(&mut self) -> u32 {
    unsafe { accessor!(major_version -> u32, (*self.base.reply)) }
  }

  pub fn minor_version(&mut self) -> u32 {
    unsafe { accessor!(minor_version -> u32, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryVersionCookie<'s>, mk_reply_query_version_reply, QueryVersionReply, xcb_render_query_version_reply)

pub struct QueryPictFormatsReply { base:  base::Reply<query_pict_formats_reply> }
pub impl QueryPictFormatsCookie {
    fn new(data : base::Reply<query_pict_formats_request>) -> QueryPictFormatsRequest {
        QueryPictFormatsRequest { base : data }
}
}
pub fn QueryPictFormats<'r> (c : &'r Connection) -> QueryPictFormatsCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_formats(c.get_raw_conn());
    QueryPictFormatsCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn QueryPictFormatsUnchecked<'r> (c : &'r Connection) -> QueryPictFormatsCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_formats_unchecked(c.get_raw_conn());
    QueryPictFormatsCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}

impl QueryPictFormatsReply {
  pub fn num_depths(&mut self) -> u32 {
    unsafe { accessor!(num_depths -> u32, (*self.base.reply)) }
  }

  pub fn num_visuals(&mut self) -> u32 {
    unsafe { accessor!(num_visuals -> u32, (*self.base.reply)) }
  }

  pub fn formats(&mut self) -> PictforminfoIterator {
    unsafe { accessor!(PictforminfoIterator, xcb_render_query_pict_formats_formats_iterator, (*self.base.reply)) }
  }

  pub fn screens(&mut self) -> PictscreenIterator {
    unsafe { accessor!(PictscreenIterator, xcb_render_query_pict_formats_screens_iterator, (*self.base.reply)) }
  }

  pub fn subpixels(&mut self) -> Vec<u32> {
    unsafe { accessor!(u32, xcb_render_query_pict_formats_subpixels_length, xcb_render_query_pict_formats_subpixels, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryPictFormatsCookie<'s>, mk_reply_query_pict_formats_reply, QueryPictFormatsReply, xcb_render_query_pict_formats_reply)

pub struct QueryPictIndexValuesReply { base:  base::Reply<query_pict_index_values_reply> }
pub impl QueryPictIndexValuesCookie {
    fn new(data : base::Reply<query_pict_index_values_request>) -> QueryPictIndexValuesRequest {
        QueryPictIndexValuesRequest { base : data }
}
}
pub fn QueryPictIndexValues<'r> (c : &'r Connection,
                             format : Pictformat) -> QueryPictIndexValuesCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_index_values(c.get_raw_conn(),
        format as pictformat); //1
    QueryPictIndexValuesCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn QueryPictIndexValuesUnchecked<'r> (c : &'r Connection,
                                      format : Pictformat) -> QueryPictIndexValuesCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_pict_index_values_unchecked(c.get_raw_conn(),
        format as pictformat); //1
    QueryPictIndexValuesCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}

impl QueryPictIndexValuesReply {
  pub fn values(&mut self) -> IndexvalueIterator {
    unsafe { accessor!(IndexvalueIterator, xcb_render_query_pict_index_values_values_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryPictIndexValuesCookie<'s>, mk_reply_query_pict_index_values_reply, QueryPictIndexValuesReply, xcb_render_query_pict_index_values_reply)

pub fn CreatePictureChecked<'r> (c : &'r Connection,
                             pid : Picture,
                             drawable : xproto::Drawable,
                             format : Pictformat,
                             value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_render_create_picture_checked(c.get_raw_conn(),
        pid as picture, //1
        drawable as ffi::xproto::drawable, //2
        format as pictformat, //3
        value_list_mask as u32, //4
        value_list_ptr as *mut u32); //5
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn CreatePicture<'r> (c : &'r Connection,
                      pid : Picture,
                      drawable : xproto::Drawable,
                      format : Pictformat,
                      value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_render_create_picture(c.get_raw_conn(),
        pid as picture, //1
        drawable as ffi::xproto::drawable, //2
        format as pictformat, //3
        value_list_mask as u32, //4
        value_list_ptr as *mut u32); //5
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn ChangePictureChecked<'r> (c : &'r Connection,
                             picture : Picture,
                             value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_render_change_picture_checked(c.get_raw_conn(),
        picture as picture, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn ChangePicture<'r> (c : &'r Connection,
                      picture : Picture,
                      value_list : &[(u32,u32)]) -> base::VoidCookie<'r> {
  unsafe {
    let mut value_list_copy = value_list.to_vec();
    let (value_list_mask, value_list_vec) = pack_bitfield(&mut value_list_copy);
    let value_list_ptr = value_list_vec.as_ptr();
    let cookie = xcb_render_change_picture(c.get_raw_conn(),
        picture as picture, //1
        value_list_mask as u32, //2
        value_list_ptr as *mut u32); //3
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn SetPictureClipRectanglesChecked<'r> (c : &'r Connection,
                                        picture : Picture,
                                        clip_x_origin : i16,
                                        clip_y_origin : i16,
                                        rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_render_set_picture_clip_rectangles_checked(c.get_raw_conn(),
        picture as picture, //1
        clip_x_origin as i16, //2
        clip_y_origin as i16, //3
        rectangles_len as u32, //4
        rectangles_ptr as *mut ffi::xproto::rectangle); //5
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn SetPictureClipRectangles<'r> (c : &'r Connection,
                                 picture : Picture,
                                 clip_x_origin : i16,
                                 clip_y_origin : i16,
                                 rectangles : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rectangles_len = rectangles.len();
    let rectangles_ptr = rectangles.as_ptr();
    let cookie = xcb_render_set_picture_clip_rectangles(c.get_raw_conn(),
        picture as picture, //1
        clip_x_origin as i16, //2
        clip_y_origin as i16, //3
        rectangles_len as u32, //4
        rectangles_ptr as *mut ffi::xproto::rectangle); //5
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn FreePictureChecked<'r> (c : &'r Connection,
                           picture : Picture) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_picture_checked(c.get_raw_conn(),
        picture as picture); //1
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn FreePicture<'r> (c : &'r Connection,
                    picture : Picture) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_picture(c.get_raw_conn(),
        picture as picture); //1
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn CompositeChecked<'r> (c : &'r Connection,
                         op : u8,
                         src : Picture,
                         mask : Picture,
                         dst : Picture,
                         src_x : i16,
                         src_y : i16,
                         mask_x : i16,
                         mask_y : i16,
                         dst_x : i16,
                         dst_y : i16,
                         width : u16,
                         height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_composite_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        mask as picture, //3
        dst as picture, //4
        src_x as i16, //5
        src_y as i16, //6
        mask_x as i16, //7
        mask_y as i16, //8
        dst_x as i16, //9
        dst_y as i16, //10
        width as u16, //11
        height as u16); //12
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn Composite<'r> (c : &'r Connection,
                  op : u8,
                  src : Picture,
                  mask : Picture,
                  dst : Picture,
                  src_x : i16,
                  src_y : i16,
                  mask_x : i16,
                  mask_y : i16,
                  dst_x : i16,
                  dst_y : i16,
                  width : u16,
                  height : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_composite(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        mask as picture, //3
        dst as picture, //4
        src_x as i16, //5
        src_y as i16, //6
        mask_x as i16, //7
        mask_y as i16, //8
        dst_x as i16, //9
        dst_y as i16, //10
        width as u16, //11
        height as u16); //12
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn TrapezoidsChecked<'r> (c : &'r Connection,
                          op : u8,
                          src : Picture,
                          dst : Picture,
                          mask_format : Pictformat,
                          src_x : i16,
                          src_y : i16,
                          traps : &[Trapezoid]) -> base::VoidCookie<'r> {
  unsafe {
    let traps_len = traps.len();
    let traps_ptr = traps.as_ptr();
    let cookie = xcb_render_trapezoids_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        traps_len as u32, //7
        traps_ptr as *mut trapezoid); //8
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn Trapezoids<'r> (c : &'r Connection,
                   op : u8,
                   src : Picture,
                   dst : Picture,
                   mask_format : Pictformat,
                   src_x : i16,
                   src_y : i16,
                   traps : &[Trapezoid]) -> base::VoidCookie<'r> {
  unsafe {
    let traps_len = traps.len();
    let traps_ptr = traps.as_ptr();
    let cookie = xcb_render_trapezoids(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        traps_len as u32, //7
        traps_ptr as *mut trapezoid); //8
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn TrianglesChecked<'r> (c : &'r Connection,
                         op : u8,
                         src : Picture,
                         dst : Picture,
                         mask_format : Pictformat,
                         src_x : i16,
                         src_y : i16,
                         triangles : &[Triangle]) -> base::VoidCookie<'r> {
  unsafe {
    let triangles_len = triangles.len();
    let triangles_ptr = triangles.as_ptr();
    let cookie = xcb_render_triangles_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        triangles_len as u32, //7
        triangles_ptr as *mut triangle); //8
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn Triangles<'r> (c : &'r Connection,
                  op : u8,
                  src : Picture,
                  dst : Picture,
                  mask_format : Pictformat,
                  src_x : i16,
                  src_y : i16,
                  triangles : &[Triangle]) -> base::VoidCookie<'r> {
  unsafe {
    let triangles_len = triangles.len();
    let triangles_ptr = triangles.as_ptr();
    let cookie = xcb_render_triangles(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        triangles_len as u32, //7
        triangles_ptr as *mut triangle); //8
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn TriStripChecked<'r> (c : &'r Connection,
                        op : u8,
                        src : Picture,
                        dst : Picture,
                        mask_format : Pictformat,
                        src_x : i16,
                        src_y : i16,
                        points : &[Pointfix]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_render_tri_strip_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut pointfix); //8
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn TriStrip<'r> (c : &'r Connection,
                 op : u8,
                 src : Picture,
                 dst : Picture,
                 mask_format : Pictformat,
                 src_x : i16,
                 src_y : i16,
                 points : &[Pointfix]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_render_tri_strip(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut pointfix); //8
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn TriFanChecked<'r> (c : &'r Connection,
                      op : u8,
                      src : Picture,
                      dst : Picture,
                      mask_format : Pictformat,
                      src_x : i16,
                      src_y : i16,
                      points : &[Pointfix]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_render_tri_fan_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut pointfix); //8
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn TriFan<'r> (c : &'r Connection,
               op : u8,
               src : Picture,
               dst : Picture,
               mask_format : Pictformat,
               src_x : i16,
               src_y : i16,
               points : &[Pointfix]) -> base::VoidCookie<'r> {
  unsafe {
    let points_len = points.len();
    let points_ptr = points.as_ptr();
    let cookie = xcb_render_tri_fan(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        src_x as i16, //5
        src_y as i16, //6
        points_len as u32, //7
        points_ptr as *mut pointfix); //8
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn CreateGlyphSetChecked<'r> (c : &'r Connection,
                              gsid : Glyphset,
                              format : Pictformat) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_glyph_set_checked(c.get_raw_conn(),
        gsid as glyphset, //1
        format as pictformat); //2
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn CreateGlyphSet<'r> (c : &'r Connection,
                       gsid : Glyphset,
                       format : Pictformat) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_glyph_set(c.get_raw_conn(),
        gsid as glyphset, //1
        format as pictformat); //2
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn ReferenceGlyphSetChecked<'r> (c : &'r Connection,
                                 gsid : Glyphset,
                                 existing : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_reference_glyph_set_checked(c.get_raw_conn(),
        gsid as glyphset, //1
        existing as glyphset); //2
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn ReferenceGlyphSet<'r> (c : &'r Connection,
                          gsid : Glyphset,
                          existing : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_reference_glyph_set(c.get_raw_conn(),
        gsid as glyphset, //1
        existing as glyphset); //2
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn FreeGlyphSetChecked<'r> (c : &'r Connection,
                            glyphset : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_glyph_set_checked(c.get_raw_conn(),
        glyphset as glyphset); //1
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn FreeGlyphSet<'r> (c : &'r Connection,
                     glyphset : Glyphset) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_free_glyph_set(c.get_raw_conn(),
        glyphset as glyphset); //1
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn AddGlyphsChecked<'r> (c : &'r Connection,
                         glyphset : Glyphset,
                         glyphids : &[u32],
                         glyphs : &[Glyphinfo],
                         data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphids_len = glyphids.len();
    let glyphids_ptr = glyphids.as_ptr();
    let glyphs_ptr = glyphs.as_ptr();
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_render_add_glyphs_checked(c.get_raw_conn(),
        glyphset as glyphset, //1
        glyphids_len as u32, //2
        glyphids_ptr as *mut u32, //3
        glyphs_ptr as *mut glyphinfo, //4
        data_len as u32, //5
        data_ptr as *mut u8); //6
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn AddGlyphs<'r> (c : &'r Connection,
                  glyphset : Glyphset,
                  glyphids : &[u32],
                  glyphs : &[Glyphinfo],
                  data : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphids_len = glyphids.len();
    let glyphids_ptr = glyphids.as_ptr();
    let glyphs_ptr = glyphs.as_ptr();
    let data_len = data.len();
    let data_ptr = data.as_ptr();
    let cookie = xcb_render_add_glyphs(c.get_raw_conn(),
        glyphset as glyphset, //1
        glyphids_len as u32, //2
        glyphids_ptr as *mut u32, //3
        glyphs_ptr as *mut glyphinfo, //4
        data_len as u32, //5
        data_ptr as *mut u8); //6
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn FreeGlyphsChecked<'r> (c : &'r Connection,
                          glyphset : Glyphset,
                          glyphs : &[Glyph]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphs_len = glyphs.len();
    let glyphs_ptr = glyphs.as_ptr();
    let cookie = xcb_render_free_glyphs_checked(c.get_raw_conn(),
        glyphset as glyphset, //1
        glyphs_len as u32, //2
        glyphs_ptr as *mut glyph); //3
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn FreeGlyphs<'r> (c : &'r Connection,
                   glyphset : Glyphset,
                   glyphs : &[Glyph]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphs_len = glyphs.len();
    let glyphs_ptr = glyphs.as_ptr();
    let cookie = xcb_render_free_glyphs(c.get_raw_conn(),
        glyphset as glyphset, //1
        glyphs_len as u32, //2
        glyphs_ptr as *mut glyph); //3
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn CompositeGlyphs8Checked<'r> (c : &'r Connection,
                                op : u8,
                                src : Picture,
                                dst : Picture,
                                mask_format : Pictformat,
                                glyphset : Glyphset,
                                src_x : i16,
                                src_y : i16,
                                glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_8_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn CompositeGlyphs8<'r> (c : &'r Connection,
                         op : u8,
                         src : Picture,
                         dst : Picture,
                         mask_format : Pictformat,
                         glyphset : Glyphset,
                         src_x : i16,
                         src_y : i16,
                         glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_8(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn CompositeGlyphs16Checked<'r> (c : &'r Connection,
                                 op : u8,
                                 src : Picture,
                                 dst : Picture,
                                 mask_format : Pictformat,
                                 glyphset : Glyphset,
                                 src_x : i16,
                                 src_y : i16,
                                 glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_16_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn CompositeGlyphs16<'r> (c : &'r Connection,
                          op : u8,
                          src : Picture,
                          dst : Picture,
                          mask_format : Pictformat,
                          glyphset : Glyphset,
                          src_x : i16,
                          src_y : i16,
                          glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_16(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn CompositeGlyphs32Checked<'r> (c : &'r Connection,
                                 op : u8,
                                 src : Picture,
                                 dst : Picture,
                                 mask_format : Pictformat,
                                 glyphset : Glyphset,
                                 src_x : i16,
                                 src_y : i16,
                                 glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_32_checked(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn CompositeGlyphs32<'r> (c : &'r Connection,
                          op : u8,
                          src : Picture,
                          dst : Picture,
                          mask_format : Pictformat,
                          glyphset : Glyphset,
                          src_x : i16,
                          src_y : i16,
                          glyphcmds : &[u8]) -> base::VoidCookie<'r> {
  unsafe {
    let glyphcmds_len = glyphcmds.len();
    let glyphcmds_ptr = glyphcmds.as_ptr();
    let cookie = xcb_render_composite_glyphs_32(c.get_raw_conn(),
        op as u8, //1
        src as picture, //2
        dst as picture, //3
        mask_format as pictformat, //4
        glyphset as glyphset, //5
        src_x as i16, //6
        src_y as i16, //7
        glyphcmds_len as u32, //8
        glyphcmds_ptr as *mut u8); //9
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn FillRectanglesChecked<'r> (c : &'r Connection,
                              op : u8,
                              dst : Picture,
                              color : Color,
                              rects : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rects_len = rects.len();
    let rects_ptr = rects.as_ptr();
    let cookie = xcb_render_fill_rectangles_checked(c.get_raw_conn(),
        op as u8, //1
        dst as picture, //2
        color.base.strct, //3
        rects_len as u32, //4
        rects_ptr as *mut ffi::xproto::rectangle); //5
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn FillRectangles<'r> (c : &'r Connection,
                       op : u8,
                       dst : Picture,
                       color : Color,
                       rects : &[xproto::Rectangle]) -> base::VoidCookie<'r> {
  unsafe {
    let rects_len = rects.len();
    let rects_ptr = rects.as_ptr();
    let cookie = xcb_render_fill_rectangles(c.get_raw_conn(),
        op as u8, //1
        dst as picture, //2
        color.base.strct, //3
        rects_len as u32, //4
        rects_ptr as *mut ffi::xproto::rectangle); //5
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn CreateCursorChecked<'r> (c : &'r Connection,
                            cid : xproto::Cursor,
                            source : Picture,
                            x : u16,
                            y : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_cursor_checked(c.get_raw_conn(),
        cid as ffi::xproto::cursor, //1
        source as picture, //2
        x as u16, //3
        y as u16); //4
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn CreateCursor<'r> (c : &'r Connection,
                     cid : xproto::Cursor,
                     source : Picture,
                     x : u16,
                     y : u16) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_cursor(c.get_raw_conn(),
        cid as ffi::xproto::cursor, //1
        source as picture, //2
        x as u16, //3
        y as u16); //4
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub struct Transform {
    pub base : base::Struct<transform>
}

impl Transform {
    fn new(data : Struct<transform>) -> Transform {
        Transform { base : Struct { strct : data } }
}
}

impl Transform {
  pub fn matrix11(&mut self) -> Fixed {
    unsafe { accessor!(matrix11 -> Fixed, self.base.strct) }
  }

  pub fn matrix12(&mut self) -> Fixed {
    unsafe { accessor!(matrix12 -> Fixed, self.base.strct) }
  }

  pub fn matrix13(&mut self) -> Fixed {
    unsafe { accessor!(matrix13 -> Fixed, self.base.strct) }
  }

  pub fn matrix21(&mut self) -> Fixed {
    unsafe { accessor!(matrix21 -> Fixed, self.base.strct) }
  }

  pub fn matrix22(&mut self) -> Fixed {
    unsafe { accessor!(matrix22 -> Fixed, self.base.strct) }
  }

  pub fn matrix23(&mut self) -> Fixed {
    unsafe { accessor!(matrix23 -> Fixed, self.base.strct) }
  }

  pub fn matrix31(&mut self) -> Fixed {
    unsafe { accessor!(matrix31 -> Fixed, self.base.strct) }
  }

  pub fn matrix32(&mut self) -> Fixed {
    unsafe { accessor!(matrix32 -> Fixed, self.base.strct) }
  }

  pub fn matrix33(&mut self) -> Fixed {
    unsafe { accessor!(matrix33 -> Fixed, self.base.strct) }
  }

}

impl Iterator<Transform> for TransformIterator {
    fn next(&mut self) -> Option<Transform> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut transform_iterator = self;
            let data = (*iter).data;
            xcb_render_transform_next(iter);
            Some(Transform::new(*data))
        }
    }
}

pub fn SetPictureTransformChecked<'r> (c : &'r Connection,
                                   picture : Picture,
                                   transform : Transform) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_set_picture_transform_checked(c.get_raw_conn(),
        picture as picture, //1
        transform.base.strct); //2
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn SetPictureTransform<'r> (c : &'r Connection,
                            picture : Picture,
                            transform : Transform) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_set_picture_transform(c.get_raw_conn(),
        picture as picture, //1
        transform.base.strct); //2
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub struct QueryFiltersReply { base:  base::Reply<query_filters_reply> }
pub impl QueryFiltersCookie {
    fn new(data : base::Reply<query_filters_request>) -> QueryFiltersRequest {
        QueryFiltersRequest { base : data }
}
}
pub fn QueryFilters<'r> (c : &'r Connection,
                     drawable : xproto::Drawable) -> QueryFiltersCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_filters(c.get_raw_conn(),
        drawable as ffi::xproto::drawable); //1
    QueryFiltersCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn QueryFiltersUnchecked<'r> (c : &'r Connection,
                              drawable : xproto::Drawable) -> QueryFiltersCookie<'r> {
  unsafe {
    let cookie = xcb_render_query_filters_unchecked(c.get_raw_conn(),
        drawable as ffi::xproto::drawable); //1
    QueryFiltersCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}

impl QueryFiltersReply {
  pub fn aliases(&mut self) -> Vec<u16> {
    unsafe { accessor!(u16, xcb_render_query_filters_aliases_length, xcb_render_query_filters_aliases, (*self.base.reply)) }
  }

  pub fn filters(&mut self) -> xproto::StrIterator {
    unsafe { accessor!(xproto::StrIterator, xcb_render_query_filters_filters_iterator, (*self.base.reply)) }
  }

}
impl_reply_cookie!(QueryFiltersCookie<'s>, mk_reply_query_filters_reply, QueryFiltersReply, xcb_render_query_filters_reply)

pub fn SetPictureFilterChecked<'r> (c : &'r Connection,
                                picture : Picture,
                                filter : &str,
                                values : &[Fixed]) -> base::VoidCookie<'r> {
  unsafe {
    let filter = (filter).as_bytes();
    let filter_len = filter.len();
    let filter_ptr = filter.as_ptr();
    let values_len = values.len();
    let values_ptr = values.as_ptr();
    let cookie = xcb_render_set_picture_filter_checked(c.get_raw_conn(),
        picture as picture, //1
        filter_len as u16, //2
        filter_ptr as *mut c_char, //3
        values_len as u32, //4
        values_ptr as *mut fixed); //5
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn SetPictureFilter<'r> (c : &'r Connection,
                         picture : Picture,
                         filter : &str,
                         values : &[Fixed]) -> base::VoidCookie<'r> {
  unsafe {
    let filter = (filter).as_bytes();
    let filter_len = filter.len();
    let filter_ptr = filter.as_ptr();
    let values_len = values.len();
    let values_ptr = values.as_ptr();
    let cookie = xcb_render_set_picture_filter(c.get_raw_conn(),
        picture as picture, //1
        filter_len as u16, //2
        filter_ptr as *mut c_char, //3
        values_len as u32, //4
        values_ptr as *mut fixed); //5
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub struct Animcursorelt {
    pub base : base::Struct<animcursorelt>
}

impl Animcursorelt {
    fn new(data : Struct<animcursorelt>) -> Animcursorelt {
        Animcursorelt { base : Struct { strct : data } }
}
}

impl Animcursorelt {
  pub fn cursor(&mut self) -> xproto::Cursor {
    unsafe { accessor!(cursor -> xproto::Cursor, self.base.strct) }
  }

  pub fn delay(&mut self) -> u32 {
    unsafe { accessor!(delay -> u32, self.base.strct) }
  }

}

impl Iterator<Animcursorelt> for AnimcursoreltIterator {
    fn next(&mut self) -> Option<Animcursorelt> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut animcursorelt_iterator = self;
            let data = (*iter).data;
            xcb_render_animcursorelt_next(iter);
            Some(Animcursorelt::new(*data))
        }
    }
}

pub fn CreateAnimCursorChecked<'r> (c : &'r Connection,
                                cid : xproto::Cursor,
                                cursors : &[Animcursorelt]) -> base::VoidCookie<'r> {
  unsafe {
    let cursors_len = cursors.len();
    let cursors_ptr = cursors.as_ptr();
    let cookie = xcb_render_create_anim_cursor_checked(c.get_raw_conn(),
        cid as ffi::xproto::cursor, //1
        cursors_len as u32, //2
        cursors_ptr as *mut animcursorelt); //3
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn CreateAnimCursor<'r> (c : &'r Connection,
                         cid : xproto::Cursor,
                         cursors : &[Animcursorelt]) -> base::VoidCookie<'r> {
  unsafe {
    let cursors_len = cursors.len();
    let cursors_ptr = cursors.as_ptr();
    let cookie = xcb_render_create_anim_cursor(c.get_raw_conn(),
        cid as ffi::xproto::cursor, //1
        cursors_len as u32, //2
        cursors_ptr as *mut animcursorelt); //3
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub struct Spanfix {
    pub base : base::Struct<spanfix>
}

impl Spanfix {
    fn new(data : Struct<spanfix>) -> Spanfix {
        Spanfix { base : Struct { strct : data } }
}
}

impl Spanfix {
  pub fn l(&mut self) -> Fixed {
    unsafe { accessor!(l -> Fixed, self.base.strct) }
  }

  pub fn r(&mut self) -> Fixed {
    unsafe { accessor!(r -> Fixed, self.base.strct) }
  }

  pub fn y(&mut self) -> Fixed {
    unsafe { accessor!(y -> Fixed, self.base.strct) }
  }

}

impl Iterator<Spanfix> for SpanfixIterator {
    fn next(&mut self) -> Option<Spanfix> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut spanfix_iterator = self;
            let data = (*iter).data;
            xcb_render_spanfix_next(iter);
            Some(Spanfix::new(*data))
        }
    }
}

pub struct Trap {
    pub base : base::Struct<trap>
}

impl Trap {
    fn new(data : Struct<trap>) -> Trap {
        Trap { base : Struct { strct : data } }
}
}

impl Trap {
  pub fn top(&self) -> Spanfix {
    unsafe { mem::transmute(self.base.strct.top) }
  }
  pub fn bot(&self) -> Spanfix {
    unsafe { mem::transmute(self.base.strct.bot) }
  }
}

impl Iterator<Trap> for TrapIterator {
    fn next(&mut self) -> Option<Trap> {
        if self.rem == 0 { return None; }
        unsafe {
            let iter : *mut trap_iterator = self;
            let data = (*iter).data;
            xcb_render_trap_next(iter);
            Some(Trap::new(*data))
        }
    }
}

pub fn AddTrapsChecked<'r> (c : &'r Connection,
                        picture : Picture,
                        x_off : i16,
                        y_off : i16,
                        traps : &[Trap]) -> base::VoidCookie<'r> {
  unsafe {
    let traps_len = traps.len();
    let traps_ptr = traps.as_ptr();
    let cookie = xcb_render_add_traps_checked(c.get_raw_conn(),
        picture as picture, //1
        x_off as i16, //2
        y_off as i16, //3
        traps_len as u32, //4
        traps_ptr as *mut trap); //5
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn AddTraps<'r> (c : &'r Connection,
                 picture : Picture,
                 x_off : i16,
                 y_off : i16,
                 traps : &[Trap]) -> base::VoidCookie<'r> {
  unsafe {
    let traps_len = traps.len();
    let traps_ptr = traps.as_ptr();
    let cookie = xcb_render_add_traps(c.get_raw_conn(),
        picture as picture, //1
        x_off as i16, //2
        y_off as i16, //3
        traps_len as u32, //4
        traps_ptr as *mut trap); //5
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn CreateSolidFillChecked<'r> (c : &'r Connection,
                               picture : Picture,
                               color : Color) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_solid_fill_checked(c.get_raw_conn(),
        picture as picture, //1
        color.base.strct); //2
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn CreateSolidFill<'r> (c : &'r Connection,
                        picture : Picture,
                        color : Color) -> base::VoidCookie<'r> {
  unsafe {
    let cookie = xcb_render_create_solid_fill(c.get_raw_conn(),
        picture as picture, //1
        color.base.strct); //2
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn CreateLinearGradientChecked<'r> (c : &'r Connection,
                                    picture : Picture,
                                    p1 : Pointfix,
                                    p2 : Pointfix,
                                    stops : &[Fixed],
                                    colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_linear_gradient_checked(c.get_raw_conn(),
        picture as picture, //1
        p1.base.strct, //2
        p2.base.strct, //3
        stops_len as u32, //4
        stops_ptr as *mut fixed, //5
        colors_ptr as *mut color); //6
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn CreateLinearGradient<'r> (c : &'r Connection,
                             picture : Picture,
                             p1 : Pointfix,
                             p2 : Pointfix,
                             stops : &[Fixed],
                             colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_linear_gradient(c.get_raw_conn(),
        picture as picture, //1
        p1.base.strct, //2
        p2.base.strct, //3
        stops_len as u32, //4
        stops_ptr as *mut fixed, //5
        colors_ptr as *mut color); //6
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn CreateRadialGradientChecked<'r> (c : &'r Connection,
                                    picture : Picture,
                                    inner : Pointfix,
                                    outer : Pointfix,
                                    inner_radius : Fixed,
                                    outer_radius : Fixed,
                                    stops : &[Fixed],
                                    colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_radial_gradient_checked(c.get_raw_conn(),
        picture as picture, //1
        inner.base.strct, //2
        outer.base.strct, //3
        inner_radius as fixed, //4
        outer_radius as fixed, //5
        stops_len as u32, //6
        stops_ptr as *mut fixed, //7
        colors_ptr as *mut color); //8
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn CreateRadialGradient<'r> (c : &'r Connection,
                             picture : Picture,
                             inner : Pointfix,
                             outer : Pointfix,
                             inner_radius : Fixed,
                             outer_radius : Fixed,
                             stops : &[Fixed],
                             colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_radial_gradient(c.get_raw_conn(),
        picture as picture, //1
        inner.base.strct, //2
        outer.base.strct, //3
        inner_radius as fixed, //4
        outer_radius as fixed, //5
        stops_len as u32, //6
        stops_ptr as *mut fixed, //7
        colors_ptr as *mut color); //8
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}
pub fn CreateConicalGradientChecked<'r> (c : &'r Connection,
                                     picture : Picture,
                                     center : Pointfix,
                                     angle : Fixed,
                                     stops : &[Fixed],
                                     colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_conical_gradient_checked(c.get_raw_conn(),
        picture as picture, //1
        center.base.strct, //2
        angle as fixed, //3
        stops_len as u32, //4
        stops_ptr as *mut fixed, //5
        colors_ptr as *mut color); //6
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:true})
  }
}
pub fn CreateConicalGradient<'r> (c : &'r Connection,
                              picture : Picture,
                              center : Pointfix,
                              angle : Fixed,
                              stops : &[Fixed],
                              colors : &[Color]) -> base::VoidCookie<'r> {
  unsafe {
    let stops_len = stops.len();
    let stops_ptr = stops.as_ptr();
    let colors_ptr = colors.as_ptr();
    let cookie = xcb_render_create_conical_gradient(c.get_raw_conn(),
        picture as picture, //1
        center.base.strct, //2
        angle as fixed, //3
        stops_len as u32, //4
        stops_ptr as *mut fixed, //5
        colors_ptr as *mut color); //6
    base::VoidCookie::new(Cookie {cookie:cookie,conn:c,checked:false})
  }
}

