#[derive(Clone, Copy)]
pub struct VolumeBounds(pub usize, pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct VolumeIdx(pub usize, pub usize, pub usize);


/// A Volume 
pub trait Volume<T> where T: Sized {
    fn get_bounds(&self) -> &VolumeBounds;

    /// Return Some(T) for an index, or None if it's out of bounds
    fn get(&self, idx: VolumeIdx) -> Option<T>;
}


pub struct VolumeCursor<'a, T> where T: Sized {
    volume: Box<Volume<T> + 'a>,
    cursor: VolumeIdx,
}


impl<'a, T> VolumeCursor<'a, T> where T: Sized {
    pub fn new(volume: Box<Volume<T> + 'a>) -> VolumeCursor<'a, T> {
        VolumeCursor {
            volume: volume,
            cursor: VolumeIdx(0, 0, 0),
        }
    }
}


impl<'a, T> Iterator for VolumeCursor<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let VolumeBounds(bx, by, bz) = *self.volume.get_bounds();
        let mut out = self.volume.get(self.cursor.clone());
        let VolumeIdx(mut cx, mut cy, mut cz) = self.cursor;
        trace!("VolumeCursor.next {:?}", (cx, cy, cz));

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
            out = None;
        }

        self.cursor = VolumeIdx(cx, cy, cz);
        out
    }
}


/// A simple volume generator for testing
pub struct CornerCube {
    pub bounds: VolumeBounds,
    pub corner: VolumeIdx,
    pub val: f64,
}


impl Volume<f64> for CornerCube {
    fn get_bounds(&self) -> &VolumeBounds {
        &self.bounds
    }

    fn get(&self, idx: VolumeIdx) -> Option<f64> {
        let VolumeIdx(ix, iy, iz) = idx;
        let VolumeIdx(cx, cy, cz) = self.corner;
        if ix < cx && iy < cy && iz < cz { Some(self.val) } else { Some(0.0) }
    }
}
