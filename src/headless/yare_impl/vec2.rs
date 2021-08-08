use crate::bindings::position::Position;

use std::{
    cmp::Ordering,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
  };

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub type Pos = Vec2;

impl From<&Position> for Vec2 {
    fn from(pos: &Position) -> Vec2 {
        Vec2 { x: pos.x, y: pos.y }
    }
}

impl Vec2 {
    pub fn dot(self, other: Vec2) -> f32 {
      self.x * other.x + self.y * other.y
    }
  
    pub fn norm_squared(self) -> f32 {
      self.x * self.x + self.y * self.y
    }
  
    pub fn norm(self) -> f32 {
      self.norm_squared().sqrt()
    }
  
    pub fn normalize(self) -> Vec2 {
      self / self.norm()
    }
  
    pub fn midpoint(self, other: Vec2) -> Vec2 {
      (self + other) / 2.
    }
  
    pub fn lerp(self, other: Vec2, ratio: f32) -> Vec2 {
      self * ratio + other * (1. - ratio)
    }
  
    pub fn dist_squared(self, other: Vec2) -> f32 {
      (self - other).norm_squared()
    }
  
    pub fn in_range(self, other: Vec2, range: f32) -> bool {
      (self - other) < range
    }
  
    pub fn towards(self, other: Vec2, length: f32) -> Vec2 {
      self + (other - self).normalize() * length
    }
  }
  
  impl Default for Vec2 {
    fn default() -> Self {
      Vec2 { x: 0., y: 0. }
    }
  }
  
  impl Add for Vec2 {
    type Output = Self;
  
    fn add(self, other: Self) -> Self {
      Self {
        x: self.x + other.x,
        y: self.y + other.y,
      }
    }
  }
  
  impl Sub for Vec2 {
    type Output = Self;
  
    fn sub(self, other: Self) -> Self {
      Self {
        x: self.x - other.x,
        y: self.y - other.y,
      }
    }
  }
  
  impl Mul<f32> for Vec2 {
    type Output = Self;
  
    fn mul(self, other: f32) -> Self {
      Self {
        x: self.x * other,
        y: self.y * other,
      }
    }
  }
  
  impl Div<f32> for Vec2 {
    type Output = Self;
  
    fn div(self, other: f32) -> Self {
      Self {
        x: self.x / other,
        y: self.y / other,
      }
    }
  }
  
  impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
      self.x += other.x;
      self.y += other.y;
    }
  }
  
  impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
      self.x -= other.x;
      self.y -= other.y;
    }
  }
  
  impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, other: f32) {
      self.x *= other;
      self.y *= other;
    }
  }
  
  impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, other: f32) {
      self.x /= other;
      self.y /= other;
    }
  }
  
  impl PartialEq<f32> for Vec2 {
    fn eq(&self, other: &f32) -> bool {
      self.norm_squared().eq(&(other * other))
    }
  }
  
  impl PartialOrd<f32> for Vec2 {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
      self.norm_squared().partial_cmp(&(other * other))
    }
  }
  
  impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Vec2>>(iter: I) -> Self {
      let mut res = Vec2::default();
      for vec in iter {
        res += vec;
      }
      res
    }
  }