use cgmath::prelude::*;
use cgmath::{Point3, Vector3};
use object::Position;


pub struct Camera {
    position: Point3<f64>,
    front: Vector3<f64>,
    up: Vector3<f64>,
    right: Vector3<f64>,
    world_up: Vector3<f64>,
    yaw: f64,
    pitch: f64,
}

impl Camera {
    fn update(&mut self) {
        self.front.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        self.front.y = self.pitch.to_radians().sin();
        self.front.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();
        self.front.normalize();
        self.right = self.front.cross(self.world_up);
        self.right.normalize();
        self.up = self.right.cross(self.front);
        self.up.normalize();
    }
}

impl Position for Camera {
    fn new(position: Point3<f64>) -> Camera {
        let mut camera = Camera {
            position: position,
            front: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            up: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            right: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            world_up: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            yaw: 0.0,
            pitch: 0.0,
        };
        camera.update();
        camera
    }

    fn get_position(&self) -> Point3<f64> {
        self.position
    }

    fn set_position(&mut self, position: Point3<f64>) {
        self.position = position;
    }
}
