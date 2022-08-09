extern crate xcb;

use super::WindowBuilder;
use xcb::xproto;

pub struct Screen<'a> {
    conn: &'a xcb::Connection,
    screen: xcb::Screen<'a>,
}

impl<'a> Screen<'a> {
    pub fn new(conn: &'a xcb::Connection, screen: xcb::Screen<'a>) -> Self {
        Self { conn, screen }
    }

    pub fn get_geometry(&self) -> Result<xcb::xproto::GetGeometryReply, xcb::GenericError> {
        xproto::get_geometry(&self.conn, self.screen.root()).get_reply()
    }

    pub fn dpi(&self) -> (u16, u16) {
        let h = self.screen.height_in_pixels() as f64;
        let w = self.screen.width_in_pixels() as f64;
        let mmw = self.screen.width_in_millimeters();
        let mmh = self.screen.height_in_millimeters();

        let mut xdpi = 0;
        let mut ydpi = 0;
        if mmw != 0 {
            xdpi = (w * 25.4 / mmw as f64).round() as u16;
        }

        if mmh != 0 {
            ydpi = (h * 25.4 / mmh as f64).round() as u16;
        }

        (xdpi, ydpi)
    }

    pub fn get_window_builder(&self) -> WindowBuilder {
        WindowBuilder::new(self.conn, &self.screen)
    }
}
