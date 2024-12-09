use crate::gridref::{GridRef, GridRefMut};
use glam::*;

pub struct Grid<T> {
    pub w: usize,
    pub h: usize,
    pub buffer: Vec<T>,
}

impl<#[cfg(feature = "rayon")] T: Send, #[cfg(not(feature = "rayon"))] T> Grid<T>
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

    pub fn as_ref(&self) -> GridRef<'_, T> {
        GridRef::new(self.w, self.h, &self.buffer)
    }

    pub fn as_ref_mut(&mut self) -> GridRefMut<'_, T> {
        GridRefMut::new(self.w, self.h, &mut self.buffer)
    }

    pub fn update(&mut self, f: fn(usize, usize) -> T) {
        #[cfg(feature = "rayon")]
        use rayon::prelude::*;

        #[cfg(feature = "rayon")]
        let iter = self.buffer.par_iter_mut();
        #[cfg(not(feature = "rayon"))]
        let iter = self.buffer.iter_mut();

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

    pub fn signed_distance(&self, p: Vec2) -> T {
        self.as_ref().signed_distance(p)
    }
}
