#![allow(dead_code)]
use crate::common::VecI2;

pub trait Bounded1
where
    Self: Sized,
{
    fn min(&self) -> i32;
    fn max(&self) -> i32;

    fn size(&self) -> i32 {
        self.max() - self.min()
    }

    fn center(&self) -> i32 {
        (self.min() + self.size()) / 2
    }
}

pub trait Bounded2
where
    Self: Sized,
{
    fn min(&self) -> VecI2;
    fn max(&self) -> VecI2;

    fn size(&self) -> VecI2 {
        self.max() - self.min()
    }

    fn center(&self) -> VecI2 {
        (self.min() + self.size()) / 2
    }
}

pub trait Extendable {
    fn extend(&self, other: &Self) -> Self;
    fn extend_mut(&mut self, other: &Self);
}

impl Bounded1 for i32 {
    fn min(&self) -> i32 {
        *self
    }
    fn max(&self) -> i32 {
        *self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bounds1 {
    min: i32,
    max: i32,
}

impl Bounds1 {
    fn new(number: i32) -> Self {
        Self {
            min: number,
            max: number,
        }
    }
}

impl Bounded1 for Bounds1 {
    fn min(&self) -> i32 {
        self.min
    }
    fn max(&self) -> i32 {
        self.max
    }
}

impl Extendable for Bounds1 {
    fn extend(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min()),
            max: self.max.max(other.max()),
        }
    }

    fn extend_mut(&mut self, other: &Self) {
        self.min = self.min.min(other.min());
        self.max = self.max.max(other.max());
    }
}

impl Extendable for Option<Bounds1> {
    fn extend(&self, other: &Self) -> Self {
        if self.is_none() {
            other.clone()
        } else if other.is_none() {
            self.clone()
        } else {
            Some(self.as_ref().unwrap().extend(other.as_ref().unwrap()))
        }
    }

    fn extend_mut(&mut self, other: &Self) {
        if self.is_none() {
            *self = other.clone();
        } else if other.is_none() {
            // NOP
        } else {
            *self = Some(self.as_ref().unwrap().extend(other.as_ref().unwrap()))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bounds2 {
    horizontal: Bounds1,
    vertical: Bounds1,
}

impl Bounds2 {
    pub fn new(horizontal: Bounds1, vertical: Bounds1) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    pub fn from_point(point: VecI2) -> Self {
        Self::new(Bounds1::new(point.0), Bounds1::new(point.1))
    }
}

impl Bounded2 for Bounds2 {
    fn min(&self) -> VecI2 {
        VecI2(self.horizontal.min(), self.vertical.min())
    }

    fn max(&self) -> VecI2 {
        VecI2(self.horizontal.max(), self.vertical.max())
    }
}

impl Extendable for Bounds2 {
    fn extend(&self, other: &Self) -> Self {
        Self {
            horizontal: self.horizontal.extend(&other.horizontal),
            vertical: self.vertical.extend(&other.vertical),
        }
    }

    fn extend_mut(&mut self, other: &Self) {
        self.horizontal.extend_mut(&other.horizontal);
        self.vertical.extend_mut(&other.vertical);
    }
}

impl Extendable for Option<Bounds2> {
    fn extend(&self, other: &Self) -> Self {
        if self.is_none() {
            other.clone()
        } else if other.is_none() {
            self.clone()
        } else {
            Some(self.as_ref().unwrap().extend(other.as_ref().unwrap()))
        }
    }

    fn extend_mut(&mut self, other: &Self) {
        if self.is_none() {
            *self = other.clone();
        } else if other.is_none() {
            // NOP
        } else {
            *self = Some(self.as_ref().unwrap().extend(other.as_ref().unwrap()))
        }
    }
}
