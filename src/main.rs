mod volume;

use volume::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let vg = CornerCube {
        bounds: VolumeBounds(256, 256, 256),
        corner: VolumeIdx(128, 128, 128),
        val: 0.8,
    };
    let vc = VolumeCursor::new(vg);
    let buffer: Vec<u8> = vc.map(|&x| (x * 255) as u8).collect();
    let mut f = try!(File::create("out.dat"));
    try!(f.write_all(buffer.as_slice()));
}
