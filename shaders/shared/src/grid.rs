use crate::gridref::{GridRef, GridRefMut};

pub struct Grid<T> {
    pub w: usize,
    pub h: usize,
    pub buffer: Vec<T>,
}

impl<T: Send> Grid<T>
where
    T: Default + Clone + Copy,
{
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            buffer: vec![Default::default(); w * h],
        }
    }

    pub fn from_fn<F: Fn(usize, usize) -> T + Send + Sync>(w: usize, h: usize, f: F) -> Self {
        let mut res = Self::new(w, h);
        res.update(f);
        res
    }

    pub fn as_ref(&self) -> GridRef<'_, T> {
        GridRef::new(self.w, self.h, &self.buffer)
    }

    pub fn as_ref_mut(&mut self) -> GridRefMut<'_, T> {
        GridRefMut::new(self.w, self.h, &mut self.buffer)
    }

    pub fn update<F: Fn(usize, usize) -> T + Send + Sync>(&mut self, f: F) {
        use rayon::prelude::*;

        let iter = self.buffer.par_iter_mut();

        iter.enumerate().for_each(|(i, value)| {
            let y = i / self.w;
            let x = i - y * self.w;
            *value = f(x, y);
        });
    }

    pub fn resize(&mut self, w: usize, h: usize) {
        self.w = w;
        self.h = h;
        let new_size = w * h;
        if new_size > self.buffer.len() {
            self.buffer.resize(new_size, Default::default());
        }
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.buffer[y * self.w + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.buffer[y * self.w + x] = value;
    }

    pub fn swap(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        GridRefMut::new(self.w, self.h, &mut self.buffer).swap(x0, y0, x1, y1)
    }
}
