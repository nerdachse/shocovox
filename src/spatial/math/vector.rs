#[derive(Default, Clone, Copy, Debug)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct V3c<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy> V3c<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    pub fn unit(scale: T) -> Self {
        Self {
            x: scale,
            y: scale,
            z: scale,
        }
    }
}

impl V3c<f32> {
    pub fn length(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }
    pub fn normalized(self) -> V3c<f32> {
        self / self.length()
    }
}

impl V3c<u32> {
    pub fn length(&self) -> f32 {
        (((self.x * self.x) + (self.y * self.y) + (self.z * self.z)) as f32).sqrt()
    }
    pub fn normalized(self) -> V3c<f32> {
        let result: V3c<f32> = self.into();
        result / self.length()
    }
    pub fn cut_each_component(&mut self, value: &u32) -> Self {
        self.x = self.x.min(*value);
        self.y = self.y.min(*value);
        self.z = self.z.min(*value);
        *self
    }
}

impl V3c<usize> {
    pub fn length(&self) -> f32 {
        (((self.x * self.x) + (self.y * self.y) + (self.z * self.z)) as f32).sqrt()
    }
    pub fn normalized(self) -> V3c<f32> {
        let result: V3c<f32> = self.into();
        result / self.length()
    }
    pub fn cut_each_component(&mut self, value: &usize) -> Self {
        self.x = self.x.min(*value);
        self.y = self.y.min(*value);
        self.z = self.z.min(*value);
        *self
    }
}

impl<T> V3c<T>
where
    T: std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::marker::Copy,
{
    pub fn dot(&self, other: &V3c<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: V3c<T>) -> V3c<T> {
        V3c {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

use std::ops::{Add, Div, Mul, Sub};
impl<T: Add<Output = T>> Add for V3c<T> {
    type Output = V3c<T>;

    fn add(self, other: V3c<T>) -> V3c<T> {
        V3c {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for V3c<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = V3c<T>;

    fn sub(self, other: V3c<T>) -> V3c<T> {
        V3c {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for V3c<T> {
    type Output = V3c<T>;

    fn mul(self, scalar: T) -> V3c<T> {
        V3c {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<V3c<T>> for V3c<T> {
    type Output = V3c<T>;

    fn mul(self, other: V3c<T>) -> V3c<T> {
        V3c {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for V3c<T> {
    type Output = V3c<T>;

    fn div(self, scalar: T) -> V3c<T> {
        V3c {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<T> PartialEq for V3c<T>
where
    T: Default + Add<Output = T> + Mul<Output = T> + Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl<T> Eq for V3c<T> where T: Default + Add<Output = T> + Mul<Output = T> + Copy + PartialEq {}

impl From<V3c<usize>> for V3c<f32> {
    fn from(vec: V3c<usize>) -> V3c<f32> {
        {
            V3c::new(vec.x as f32, vec.y as f32, vec.z as f32)
        }
    }
}

impl From<V3c<i32>> for V3c<f32> {
    fn from(vec: V3c<i32>) -> V3c<f32> {
        {
            V3c::new(vec.x as f32, vec.y as f32, vec.z as f32)
        }
    }
}

impl From<V3c<u32>> for V3c<f32> {
    fn from(vec: V3c<u32>) -> V3c<f32> {
        {
            V3c::new(vec.x as f32, vec.y as f32, vec.z as f32)
        }
    }
}

impl From<V3c<u32>> for V3c<usize> {
    fn from(vec: V3c<u32>) -> V3c<usize> {
        {
            V3c::new(vec.x as usize, vec.y as usize, vec.z as usize)
        }
    }
}

impl From<V3c<usize>> for V3c<u32> {
    fn from(vec: V3c<usize>) -> V3c<u32> {
        {
            V3c::new(vec.x as u32, vec.y as u32, vec.z as u32)
        }
    }
}

impl From<V3c<i32>> for V3c<usize> {
    fn from(vec: V3c<i32>) -> V3c<usize> {
        {
            V3c::new(vec.x as usize, vec.y as usize, vec.z as usize)
        }
    }
}

impl From<V3c<f32>> for V3c<u32> {
    fn from(vec: V3c<f32>) -> V3c<u32> {
        {
            V3c::new(
                vec.x.round() as u32,
                vec.y.round() as u32,
                vec.z.round() as u32,
            )
        }
    }
}

impl From<V3c<f32>> for V3c<i32> {
    fn from(vec: V3c<f32>) -> V3c<i32> {
        {
            V3c::new(
                vec.x.round() as i32,
                vec.y.round() as i32,
                vec.z.round() as i32,
            )
        }
    }
}

impl From<V3c<u32>> for V3c<i32> {
    fn from(vec: V3c<u32>) -> V3c<i32> {
        {
            V3c::new(vec.x as i32, vec.y as i32, vec.z as i32)
        }
    }
}

impl From<V3c<i32>> for V3c<u32> {
    fn from(vec: V3c<i32>) -> V3c<u32> {
        {
            V3c::new(vec.x as u32, vec.y as u32, vec.z as u32)
        }
    }
}
