//
// sprocketnes/gfx.rs
//
// Author: Patrick Walton
//

use ppu;

use core::uint::range;
use sdl::sdl::{InitTimer, InitVideo};
use sdl::sdl;
use sdl::video::{DoubleBuf, HWSurface, Surface};
use sdl::video;

const SCREEN_WIDTH: uint = 256;
const SCREEN_HEIGHT: uint = 240;

pub struct Gfx {
    screen: ~Surface
}

pub impl Gfx {
    static fn new() -> Gfx {
        sdl::init([ InitVideo, InitTimer ]);
        let screen = video::set_video_mode(SCREEN_WIDTH as int,
                                           SCREEN_HEIGHT as int,
                                           24,
                                           [ HWSurface ],
                                           [ DoubleBuf ]);
        Gfx { screen: screen.unwrap() }
    }

    fn blit(&self, ppu_screen: &([u8 * 184320])) {
        do self.screen.with_lock |pixels| {
            vec::bytes::copy_memory(pixels, *ppu_screen, ppu_screen.len());
        }
    }
}

