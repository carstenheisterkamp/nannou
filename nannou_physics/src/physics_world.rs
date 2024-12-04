#[cfg(feature = "rapier2d")]
use rapier2d::prelude::*;

#[cfg(feature = "rapier3d")]
use rapier3d::prelude::*;

#[cfg(feature = "rapier2d-f64")]
use rapier2d::prelude::*;

#[cfg(feature = "rapier3d-f64")]
use rapier3d::prelude::*;

pub struct PhysicsWorld {
    #[cfg(any(feature = "rapier2d", feature = "rapier2d-f64"))]
    pub rigid_bodies: rapier2d::prelude::RigidBodySet,
    
    #[cfg(any(feature = "rapier3d", feature = "rapier3d-f64"))]
    pub rigid_bodies: rapier3d::prelude::RigidBodySet,
    
    #[cfg(any(feature = "rapier2d", feature = "rapier2d-f64"))]
    pub colliders: rapier2d::prelude::ColliderSet,
    
    #[cfg(any(feature = "rapier3d", feature = "rapier3d-f64"))]
    pub colliders: rapier3d::prelude::ColliderSet,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        #[cfg(any(feature = "rapier2d", feature = "rapier2d-f64"))]
        return Self {
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
        };

        #[cfg(any(feature = "rapier3d", feature = "rapier3d-f64"))]
        return Self {
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
        };
    }

    pub fn step(&mut self) {
        #[cfg(any(feature = "rapier2d", feature = "rapier2d-f64"))]
        {
            // TODO: 2D Physics step
        }

        #[cfg(any(feature = "rapier3d", feature = "rapier3d-f64"))]
        {
            // TODO: 3D Physics step
        }
    }
}