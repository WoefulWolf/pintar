use crate::{default_elements::DefaultVertex, mesh::DefaultMesh};

pub fn new(colour: [f32; 4]) -> DefaultMesh {
    let vertices: Vec<DefaultVertex> = vec![
        DefaultVertex {
            position: [-1.0, 1.0, -1.0].into(),
            normal: [-1.0, 0.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, -1.0, -1.0].into(),
            normal: [-1.0, 0.0, 0.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 0.0, 1.0].into()
        },
        DefaultVertex {
            position: [-1.0, -1.0, -1.0].into(),
            normal: [0.0, 0.0, -1.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, 1.0, -1.0].into(),
            normal: [0.0, 0.0, -1.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 1.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, 1.0, 1.0].into(),
            normal: [-1.0, 0.0, 0.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 1.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, -1.0, 1.0].into(),
            normal: [-1.0, 0.0, 0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, -1.0, -1.0].into(),
            normal: [0.0, 0.0, -1.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 0.0, 1.0].into()
        },
        DefaultVertex {
            position: [1.0, 1.0, -1.0].into(),
            normal: [0.0, 0.0, -1.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, 1.0, 1.0].into(),
            normal: [0.0, 0.0, 1.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, -1.0, 1.0].into(),
            normal: [0.0, 0.0, 1.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 0.0, 1.0].into()
        },
        DefaultVertex {
            position: [1.0, 1.0, 1.0].into(),
            normal: [0.0, 0.0, 1.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 1.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, -1.0, 1.0].into(),
            normal: [0.0, 0.0, 1.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, 1.0, 1.0].into(),
            normal: [1.0, 0.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, -1.0, 1.0].into(),
            normal: [1.0, 0.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 0.0, 1.0].into()
        },
        DefaultVertex {
            position: [1.0, 1.0, -1.0].into(),
            normal: [1.0, 0.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 1.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, -1.0, -1.0].into(),
            normal: [1.0, 0.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, -1.0, 1.0].into(),
            normal: [0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, -1.0, 1.0].into(),
            normal: [-0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 0.0, 1.0].into()
        },
        DefaultVertex {
            position: [1.0, -1.0, -1.0].into(),
            normal: [-0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 1.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, -1.0, -1.0].into(),
            normal: [-0.0, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, 1.0, 1.0].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, 1.0, 1.0].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 0.0, 1.0].into()
        },
        DefaultVertex {
            position: [-1.0, 1.0, -1.0].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [0.0, 1.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, 1.0, -1.0].into(),
            normal: [0.0, 1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        ];

    let indices: Vec<u32> = vec![
        22, 20, 21, 3, 6, 2, 22, 21, 23, 18, 17, 19, 18, 16, 17, 14, 12, 13, 14, 13, 15, 3, 7, 6, 10, 9, 11, 10, 8, 9, 4, 0, 1, 4, 1, 5,
    ];

    DefaultMesh::new(&vertices, &indices)
}
