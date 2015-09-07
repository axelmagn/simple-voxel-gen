use std::fs::File;
use std::io;

pub struct VolumeBounds(usize, usize, usize);
pub struct VolumeIdx(usize, usize, usize);


/// A Volume 
pub trait Volume<T> where T: Sized {
    fn get_bounds(&self) -> VolumeBounds;

    /// Return Some(T) for an index, or None if it's out of bounds
    fn get(&self, idx: VolumeIdx) -> Option<T>;
}


pub struct VolumeCursor<'a, T> where T: Sized {
    volume: &'a (Volume<T> + 'static),
    cursor: VolumeIdx,
}


impl<'a, T> VolumeCursor<'a, T> where T: Sized {
    pub fn new(volume: &'a Volume<T>) -> VolumeCursor<'a, T> {
        VolumeCursor {
            volume: volume,
            cursor: VolumeIdx(0, 0, 0),
        }
    }
}


impl<'a, T> Iterator for VolumeCursor<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let VolumeBounds(bx, by, bz) = self.volume.get_bounds();
        let VolumeIdx(mut cx, mut cy, mut cz) = self.cursor;

        cx += 1;
        if cx >= bx {
            cx = 0;
            cy += 1;
        }
        if cy >= by {
            cy = 0;
            cz += 1;
        }
        if cz >= bz {
            return None;
        }

        self.get(VolumeIdx(cx, cy, cz))
    }
}


/// A simple volume generator for testing
pub struct CornerCube {
    bounds: VolumeBounds,
    corner: VolumeIdx,
    val: f64,
}


impl Volume<f64> for CornerCube {
    fn get_bounds(&self) -> VolumeBounds {
        self.bounds
    }

    fn get(&self, idx: VolumeIdx) -> Option<f64> {
        let VolumeIdx(ix, iy, iz) = idx;
        let VolumeIdx(cx, cy, cz) = self.corner;
        if ix < cx && iy < cy && iz < cz { Some(self.val) } else { Some(0.0) }
    }
}
