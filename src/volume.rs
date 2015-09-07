struct VolumeBounds(usize, usize, usize);
struct VolumeIdx(usize, usize, usize);


/// A Volume 
pub trait Volume<T> {
    fn get_bounds(&self) -> VolumeBounds;

    /// Return Some(T) for an index, or None if it's out of bounds
    fn get(&self, idx: VolumeIdx) -> Option<T>;
}


#[derive(Debug)]
pub struct VolumeCursor<T> {
    volume: Volume<T>,
    cursor: VolumeIdx,
}


impl<T> VolumeCursor<T> {
    pub fn new(volume: Volume<T>) -> Self {
        VolumeCursor {
            volume: volume,
            cursor: VolumeIdx(0, 0, 0),
        }
    }
}


impl<T> Iterator for VolumeCursor<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let VolumeBounds(bx, by, bz) = self.get_bounds();
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
#[derive(Debug)]
pub struct CornerCube {
    bounds: VolumeBounds,
    corner: VolumeIdx,
    val: f64,
}


impl Volume<f64> for CornerCube {
    fn get_bounds(&self) -> VolumeBounds {
        self.bounds
    }

    fn get(&self, idx: VolumeIdx) -> f64 {
        let VolumeIdx(ix, iy, iz) = idx;
        let VolumeIdx(cx, cy, cz) = self.corner;
        if ix < cx && iy < cy && iz < cz { self.val } else { 0.0 }
    }
}
