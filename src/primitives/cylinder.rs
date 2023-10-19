use crate::{mesh::DefaultMesh, default_elements::DefaultVertex};

pub fn new(colour: [f32; 4]) -> DefaultMesh {
    let vertices: Vec<DefaultVertex> = vec![
        DefaultVertex {
            position: [0.0, -1.0, -0.0].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, 1.0, -0.0].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, -1.0, -1.0].into(),
            normal: [-0.0, 0.0, -1.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, 1.0, -1.0].into(),
            normal: [0.0, 0.0, -1.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.258819, -1.0, -0.965926].into(),
            normal: [0.258819, 0.0, -0.965926].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.258819, 1.0, -0.965926].into(),
            normal: [0.258819, 0.0, -0.965926].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.5, -1.0, -0.866025].into(),
            normal: [0.5, 0.0, -0.866025].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.5, 1.0, -0.866025].into(),
            normal: [0.5, 0.0, -0.866025].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.707107, -1.0, -0.707107].into(),
            normal: [0.707107, 0.0, -0.707107].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.707107, 1.0, -0.707107].into(),
            normal: [0.707107, 0.0, -0.707107].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.866025, -1.0, -0.5].into(),
            normal: [0.866025, 0.0, -0.5].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.866025, 1.0, -0.5].into(),
            normal: [0.866025, 0.0, -0.5].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.965926, -1.0, -0.258819].into(),
            normal: [0.965926, 0.0, -0.258819].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.965926, 1.0, -0.258819].into(),
            normal: [0.965926, 0.0, -0.258819].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, -1.0, -0.0].into(),
            normal: [1.0, 0.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, 1.0, -0.0].into(),
            normal: [1.0, 0.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.965926, -1.0, 0.258819].into(),
            normal: [0.965926, 0.0, 0.258819].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.965926, 1.0, 0.258819].into(),
            normal: [0.965926, 0.0, 0.258819].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.866025, -1.0, 0.5].into(),
            normal: [0.866025, 0.0, 0.5].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.866025, 1.0, 0.5].into(),
            normal: [0.866025, 0.0, 0.5].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.707107, -1.0, 0.707107].into(),
            normal: [0.707107, 0.0, 0.707107].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.707107, 1.0, 0.707107].into(),
            normal: [0.707107, 0.0, 0.707107].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.5, -1.0, 0.866025].into(),
            normal: [0.5, 0.0, 0.866025].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.5, 1.0, 0.866025].into(),
            normal: [0.5, 0.0, 0.866025].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.258819, -1.0, 0.965926].into(),
            normal: [0.258819, 0.0, 0.965926].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.258819, 1.0, 0.965926].into(),
            normal: [0.258819, 0.0, 0.965926].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, -1.0, 1.0].into(),
            normal: [0.0, 0.0, 1.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, 1.0, 1.0].into(),
            normal: [0.0, 0.0, 1.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.258819, -1.0, 0.965926].into(),
            normal: [-0.258819, 0.0, 0.965926].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.258819, 1.0, 0.965926].into(),
            normal: [-0.258819, 0.0, 0.965926].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.5, -1.0, 0.866025].into(),
            normal: [-0.5, 0.0, 0.866025].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.5, 1.0, 0.866025].into(),
            normal: [-0.5, 0.0, 0.866025].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.707107, -1.0, 0.707107].into(),
            normal: [-0.707107, 0.0, 0.707107].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.707107, 1.0, 0.707107].into(),
            normal: [-0.707107, 0.0, 0.707107].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.866025, -1.0, 0.5].into(),
            normal: [-0.866025, 0.0, 0.5].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.866025, 1.0, 0.5].into(),
            normal: [-0.866025, 0.0, 0.5].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.965926, -1.0, 0.258819].into(),
            normal: [-0.965926, 0.0, 0.258819].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.965926, 1.0, 0.258819].into(),
            normal: [-0.965926, 0.0, 0.258819].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, -1.0, -0.0].into(),
            normal: [-1.0, 0.0, 0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, 1.0, -0.0].into(),
            normal: [-1.0, 0.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.965926, -1.0, -0.258819].into(),
            normal: [-0.965926, 0.0, -0.258819].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.965926, 1.0, -0.258819].into(),
            normal: [-0.965926, 0.0, -0.258819].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.866025, -1.0, -0.5].into(),
            normal: [-0.866025, 0.0, -0.5].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.866025, 1.0, -0.5].into(),
            normal: [-0.866025, 0.0, -0.5].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.707107, -1.0, -0.707107].into(),
            normal: [-0.707107, 0.0, -0.707107].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.707107, 1.0, -0.707107].into(),
            normal: [-0.707107, 0.0, -0.707107].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.5, -1.0, -0.866025].into(),
            normal: [-0.5, 0.0, -0.866025].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.5, 1.0, -0.866025].into(),
            normal: [-0.5, 0.0, -0.866025].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.258819, -1.0, -0.965926].into(),
            normal: [-0.258819, 0.0, -0.965926].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.258819, 1.0, -0.965926].into(),
            normal: [-0.258819, 0.0, -0.965926].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.258819, 1.0, -0.965926].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, 1.0, -1.0].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.5, 1.0, -0.866025].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.707107, 1.0, -0.707107].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.866025, 1.0, -0.5].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.965926, 1.0, -0.258819].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, 1.0, -0.0].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.965926, 1.0, 0.258819].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.866025, 1.0, 0.5].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.707107, 1.0, 0.707107].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.5, 1.0, 0.866025].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.258819, 1.0, 0.965926].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, 1.0, 1.0].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.258819, 1.0, 0.965926].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.5, 1.0, 0.866025].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.707107, 1.0, 0.707107].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.866025, 1.0, 0.5].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.965926, 1.0, 0.258819].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, 1.0, -0.0].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.965926, 1.0, -0.258819].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.866025, 1.0, -0.5].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.707107, 1.0, -0.707107].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.5, 1.0, -0.866025].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.258819, 1.0, -0.965926].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, -1.0, -1.0].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.258819, -1.0, -0.965926].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.5, -1.0, -0.866025].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.707107, -1.0, -0.707107].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.866025, -1.0, -0.5].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.965926, -1.0, -0.258819].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, -1.0, -0.0].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.965926, -1.0, 0.258819].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.866025, -1.0, 0.5].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.707107, -1.0, 0.707107].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.5, -1.0, 0.866025].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.258819, -1.0, 0.965926].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, -1.0, 1.0].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.258819, -1.0, 0.965926].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.5, -1.0, 0.866025].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.707107, -1.0, 0.707107].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.866025, -1.0, 0.5].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.965926, -1.0, 0.258819].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, -1.0, -0.0].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.965926, -1.0, -0.258819].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.866025, -1.0, -0.5].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.707107, -1.0, -0.707107].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.5, -1.0, -0.866025].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.258819, -1.0, -0.965926].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },

        ];

    let indices: Vec<u32> = vec![
        0, 74, 75, 1, 50, 51, 3, 4, 2, 0, 75, 76, 1, 52, 50, 5, 6, 4, 0, 76, 77, 1, 53, 52, 7, 8, 6, 0, 77, 78, 1, 54, 53, 9, 10, 8, 0, 78, 79, 1, 55, 54, 11, 12, 10, 0, 79, 80, 1, 56, 55, 13, 14, 12, 0, 80, 81, 1, 57, 56, 15, 16, 14, 0, 81, 82, 1, 58, 57, 17, 18, 16, 0, 82, 83, 1, 59, 58, 19, 20, 18, 0, 83, 84, 1, 60, 59, 21, 22, 20, 0, 84, 85, 1, 61, 60, 23, 24, 22, 0, 85, 86, 1, 62, 61, 25, 26, 24, 0, 86, 87, 1, 63, 62, 27, 28, 26, 0, 87, 88, 1, 64, 63, 29, 30, 28, 0, 88, 89, 1, 65, 64, 31, 32, 30, 0, 89, 90, 1, 66, 65, 33, 34, 32, 0, 90, 91, 1, 67, 66, 35, 36, 34, 0, 91, 92, 1, 68, 67, 37, 38, 36, 0, 92, 93, 1, 69, 68, 39, 40, 38, 0, 93, 94, 1, 70, 69, 41, 42, 40, 0, 94, 95, 1, 71, 70, 43, 44, 42, 0, 95, 96, 1, 72, 71, 45, 46, 44, 0, 96, 97, 1, 73, 72, 47, 48, 46, 0, 97, 74, 1, 51, 73, 49, 2, 48, 3, 5, 4, 5, 7, 6, 7, 9, 8, 9, 11, 10, 11, 13, 12, 13, 15, 14, 15, 17, 16, 17, 19, 18, 19, 21, 20, 21, 23, 22, 23, 25, 24, 25, 27, 26, 27, 29, 28, 29, 31, 30, 31, 33, 32, 33, 35, 34, 35, 37, 36, 37, 39, 38, 39, 41, 40, 41, 43, 42, 43, 45, 44, 45, 47, 46, 47, 49, 48, 49, 3, 2,
    ];

    DefaultMesh::new(&vertices, &indices)
}
