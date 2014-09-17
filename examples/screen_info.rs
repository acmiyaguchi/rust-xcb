#![feature(globs)]
extern crate xcb;

use std::iter::{Iterator};
use xcb::base::*;

fn main() {
    let (conn, screen_num) = Connection::connect();

    let mut setup = conn.get_setup();

    let mut iter = setup.roots();

    let mut screen;
    loop {
        let n : Option<xcb::xproto::Screen> = iter.next();
        match n {
            Some(s) => {
                if 0 == screen_num {
                    screen = s;
                    break;
                }
            }
            None => { fail!("Whut") }
        }
    }
    println!("Status: {}, MajorMinor: {}{}, Length: {}, Vendor: {}", 
			setup.status(), 
			setup.protocol_major_version(),
			setup.protocol_minor_version(),
			setup.length(),
			setup.vendor());
    println!("");
    println!("Informations of screen {}:", screen.root());
    println!("  width..........: {}", screen.width_in_pixels());
    println!("  height.........: {}", screen.height_in_pixels());
    println!("  white pixel....: {}", screen.white_pixel());
    println!("  black pixel....: {}", screen.black_pixel());
}
