#[macro_use]
extern crate log;
extern crate env_logger;

mod volume;

use volume::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    env_logger::init().unwrap();

    info!("creating Volume Generator");
    let vg = Box::new(CornerCube {
        bounds: VolumeBounds(256, 256, 256),
        corner: VolumeIdx(128, 128, 128),
        val: 0.8,
    });
    info!("creating Volume Cursor");
    let vc = VolumeCursor::new(vg);
    info!("Creating Volume Buffer");
    let buffer: Vec<u8> = vc.map(|x| (x * 255.0_f64) as u8).collect();
    info!("Opening File");
    let mut f = File::create("out.dat").unwrap();
    info!("Writing File");
    f.write_all(&buffer[..]).unwrap();
}
