#[cfg(not(feature = "std"))]
use num_traits::Float;

#[derive(Clone, Copy)]
pub struct GridRef<'a, T> {
    w: usize,
    h: usize,
    buffer: &'a [T],
}

impl<'a, T: Copy> GridRef<'a, T> {
    pub fn new(w: usize, h: usize, buffer: &'a [T]) -> Self {
        Self { w, h, buffer }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.w as f32 / self.h as f32
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.buffer[y * self.w + x]
    }
}

pub struct GridRefMut<'a, T> {
    w: usize,
    h: usize,
    buffer: &'a mut [T],
}

impl<'a, T: Copy> GridRefMut<'a, T> {
    pub fn new(w: usize, h: usize, buffer: &'a mut [T]) -> Self {
        Self { w, h, buffer }
    }

    pub fn as_ref(&self) -> GridRef<'_, T> {
        GridRef::new(self.w, self.h, self.buffer)
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.w as f32 / self.h as f32
    }

    pub fn get(&self, x: usize, y: usize) -> T {
        self.buffer[y * self.w + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        self.buffer[y * self.w + x] = value;
    }

    pub fn swap(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let tmp = self.buffer[y0 * self.w + x0];
        self.buffer[y0 * self.w + x0] = self.buffer[y1 * self.w + x1];
        self.buffer[y1 * self.w + x1] = tmp;
    }
}
