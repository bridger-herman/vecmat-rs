extern crate num_integer;
extern crate num_traits;

#[macro_use]
mod macros;

pub mod vec;
#[cfg(test)]
mod vec_test;

pub mod mat;
#[cfg(test)]
mod mat_test;

pub mod map;
#[cfg(test)]
mod map_test;
