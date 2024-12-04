pub mod physics_world;

#[cfg(feature = "rapier2d")]
pub use rapier2d as rapier;

#[cfg(feature = "rapier3d")]
pub use rapier3d as rapier;

#[cfg(feature = "rapier2d-f64")]
pub use rapier2d as rapier;

#[cfg(feature = "rapier3d-f64")]
pub use rapier3d as rapier;
