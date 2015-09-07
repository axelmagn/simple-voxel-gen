mod volume;

use volume::*;

fn main() {
    let vg = CornerCube {
        bounds: VolumeBounds(256, 256, 256),
        corner: VolumeIdx(128, 128, 128),
        val: 0.8,
    };
    let vc = VolumeCursor::new(vg);
}
