use std::marker::PhantomData;
use crate::plattle::Plattle;

/// Any color representation
pub trait Color : Sized {
    /// Convert the RGB representation to the standard RGB tuple
    fn rgb(&self) -> (u8,u8,u8);
    
    /// Get the alpha channel of the color
    fn alpha(&self) -> f64;

    /// Introduce alpha channel to the color
    fn mix(&self, alpha:f64) -> CompsitableColor<Self> {
        CompsitableColor(self, alpha)
    }
}

/// Color without alpha channel
pub trait SimpleColor {
    fn rgb(&self) -> (u8,u8,u8);
}

impl <T:SimpleColor> Color for T {
    fn rgb(&self) -> (u8,u8,u8) {
        SimpleColor::rgb(self)
    }

    fn alpha(&self) -> f64 {
        1.0
    }
}

/// A color in the given plattle
pub struct PlattleColor<P:Plattle>(usize, PhantomData<P>);

impl <P:Plattle> PlattleColor<P> {
    /// Pick a color from the plattle
    pub fn pick(idx:usize) -> PlattleColor<P> {
        return PlattleColor(idx % P::COLORS.len(), PhantomData);
    }
}

impl <P:Plattle> SimpleColor for PlattleColor<P> {
    fn rgb(&self) -> (u8,u8,u8) {
        P::COLORS[self.0]
    }
}


/// Simple color with additional alpha channel
pub struct CompsitableColor<'a, T:Color>(&'a T, f64);

impl <'a, T:Color> Color for CompsitableColor<'a, T> {
    fn rgb(&self) -> (u8,u8,u8) {
        (self.0).rgb()
    }

    fn alpha(&self) -> f64 {
        (self.0).alpha() * self.1
    }
}


/// The color described by it's RGB value
pub struct RGBColor(pub u8,pub u8,pub u8);

impl SimpleColor for RGBColor {
    fn rgb(&self) -> (u8,u8,u8) {
        (self.0, self.1, self.2)
    }
}

