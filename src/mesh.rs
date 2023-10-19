use directx_math::{XMQuaternionRotationRollPitchYaw, XMVector3Rotate, XMLoadFloat3, XMStoreFloat3};
use crate::default_elements::{DefaultVertex, LineVertex};

pub trait Mesh<T> {
    fn vertices(&self) -> Vec<T>;
    fn indices(&self) -> Vec<u32>;
}

pub struct DefaultMesh {
    vertices: Vec<DefaultVertex>,
    indices: Vec<u32>
}

impl DefaultMesh {
    pub fn new(vertices: &[DefaultVertex], indices: &[u32]) -> Self {
        DefaultMesh { vertices: vertices.to_vec(), indices: indices.to_vec() }
    }

    pub fn translate(mut self, translation: [f32;3]) -> Self {
        for vert in &mut self.vertices {
            vert.position.x += translation[0];
            vert.position.y += translation[1];
            vert.position.z += translation[2];
        }

        self
    }

    pub fn scale(mut self, scale: [f32;3]) -> Self {
        for vert in &mut self.vertices {
            vert.position.x *= scale[0];
            vert.position.y *= scale[1];
            vert.position.z *= scale[2];
        }

        self
    }

    pub fn rotate(mut self, rotation: [f32;3]) -> Self {
        let rot_quat = XMQuaternionRotationRollPitchYaw(rotation[0], rotation[1], rotation[2]);

        for vert in &mut self.vertices {
            let vert_clone = vert.clone();
            XMStoreFloat3(&mut vert.position, XMVector3Rotate(XMLoadFloat3(&vert_clone.position), rot_quat));
            XMStoreFloat3(&mut vert.normal, XMVector3Rotate(XMLoadFloat3(&vert_clone.normal), rot_quat));
        }

        self
    }
}

impl Mesh<DefaultVertex> for DefaultMesh {
    fn vertices(&self) -> Vec<DefaultVertex> {
        self.vertices.clone()
    }

    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
}

pub struct LineMesh {
    vertices: Vec<LineVertex>,
    indices: Vec<u32>,
}

impl LineMesh {
    pub fn new(vertices: &[LineVertex], indices: &[u32]) -> Self {
        LineMesh { vertices: vertices.to_vec(), indices: indices.to_vec() }
    }
}

impl Mesh<LineVertex> for LineMesh {
    fn vertices(&self) -> Vec<LineVertex> {
        self.vertices.clone()
    }

    fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }
}
