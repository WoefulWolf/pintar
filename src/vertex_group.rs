
use std::any::Any;

use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Direct3D::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32_UINT;

use crate::mesh::Mesh;

pub trait OrderedRenderable: Sortable + Renderable {}

pub trait Sortable {
    fn sort(&mut self) {}
}

pub trait Renderable {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn map(&self);
    fn bind(&self);
    fn mesh_headers(&self) -> Vec<MeshHeader>;
    fn double_sided(&self) -> bool;
    fn clear(&mut self);
}

pub trait Position {
    fn pos(&self) -> [f32; 3];
}

#[derive(Clone)]
pub struct MeshHeader {
    pub pos: [f32; 3],
    pub start_vertex: u32,
    pub vertex_count: u32,

    pub start_index: u32,
    pub index_count: u32,
}

pub struct VertexGroup<T, C> {
    device: ID3D11Device,

    pixel_shader: Option<ID3D11PixelShader>,
    vertex_shader: Option<ID3D11VertexShader>,
    input_layout: Option<ID3D11InputLayout>,

    constants_buffer: Option<ID3D11Buffer>,

    vertex_buffer: Option<ID3D11Buffer>,
    pub vertex_capacity: usize,
    index_buffer: Option<ID3D11Buffer>,
    pub index_capacity: usize,


    pub constants: C,
    pub vertices: Vec<T>,
    pub indices: Vec<u32>,

    pub mesh_headers: Vec<MeshHeader>,
    double_sided: bool,
}

impl<T: std::marker::Copy + Position, C: std::marker::Copy + std::default::Default> VertexGroup<T, C> {
    pub fn new(device: ID3D11Device, double_sided: bool) -> Self {
        let vertex_capacity = 256;
        let index_capacity = 256;

        // Vertex buffer
        let vertex_buffer_desc = D3D11_BUFFER_DESC {
            Usage: D3D11_USAGE_DYNAMIC,
            ByteWidth: (std::mem::size_of::<T>() * vertex_capacity) as u32,
            BindFlags: 0x1, //D3D11_BIND_VERTEX_BUFFER,
            CPUAccessFlags: 0x10000, //D3D11_CPU_ACCESS_WRITE,
            ..Default::default()
        };

        let mut vertex_buffer: Option<ID3D11Buffer> = None;
        unsafe { device.CreateBuffer(
                &vertex_buffer_desc, 
                None, 
                Some( &mut vertex_buffer)).expect("Failed to create vertex buffer");
        }

        // Index buffer
        let index_buffer_desc = D3D11_BUFFER_DESC {
            Usage: D3D11_USAGE_DYNAMIC,
            ByteWidth: (std::mem::size_of::<u32>() * index_capacity) as u32,
            BindFlags: 0x2, //D3D11_BIND_INDEX_BUFFER,
            CPUAccessFlags: 0x10000, //D3D11_CPU_ACCESS_WRITE,
            MiscFlags: 0x0, //D3D11_RESOURCE_MISC_FLAG(0),
            ..Default::default()
        };

        let mut index_buffer: Option<ID3D11Buffer> = None;
        unsafe { device.CreateBuffer(
                &index_buffer_desc, 
                None, 
                Some( &mut index_buffer)).expect("Failed to create index buffer");
        }

        VertexGroup {
            device,

            vertex_shader: None,
            pixel_shader: None,
            input_layout: None,

            constants_buffer: None,

            vertex_buffer,
            vertex_capacity,
            index_buffer,
            index_capacity,

            constants: Default::default(),
            vertices: Vec::with_capacity(vertex_capacity),
            indices: Vec::with_capacity(index_capacity),

            mesh_headers: Vec::new(),
            double_sided
        }
    }

    pub fn with_constants(mut self) -> Self {
        let constants_buf_desc = D3D11_BUFFER_DESC {
            Usage: D3D11_USAGE_DYNAMIC,
            ByteWidth: std::mem::size_of::<C>() as u32,
            BindFlags: 0x4, //D3D11_BIND_CONSTANT_BUFFER,
            CPUAccessFlags: 0x10000, //D3D11_CPU_ACCESS_WRITE,
            MiscFlags: 0x0, //D3D11_RESOURCE_MISC_FLAG(0),
            StructureByteStride: 0,
            ..Default::default()
        };

        let mut constants_buf: Option<ID3D11Buffer> = None;
        unsafe { self.device.CreateBuffer(
            &constants_buf_desc, 
            None, 
            Some( &mut constants_buf)).expect("Failed to create constants buffer");
        }

        self.constants_buffer = constants_buf;
        self
    }

    pub fn vertex_shader(mut self, shader_bytes: &[u8], input_desc: &[D3D11_INPUT_ELEMENT_DESC]) -> Self {
        let vertex_shader = {
            let mut shader: Option<ID3D11VertexShader> = None;
            unsafe {
                self.device.CreateVertexShader(shader_bytes, None, Some(&mut shader) )
                    .expect("Failed to create vertex shader")
            };
            shader
        };

        let mut input_layout: Option<ID3D11InputLayout> = None;
        unsafe {
            self.device.CreateInputLayout(
                input_desc,
                shader_bytes,
                Some(&mut input_layout),
                ).expect("Failed to create input layout");
        }
        
        self.vertex_shader = vertex_shader;
        self.input_layout = input_layout;
        self
    }

    pub fn pixel_shader(mut self, shader_bytes: &[u8]) -> Self {
        let pixel_shader = {
            let mut shader: Option<ID3D11PixelShader> = None;
            unsafe {
                self.device.CreatePixelShader(shader_bytes, None, Some(&mut shader) )
                    .expect("Failed to create pixel shader")
            };
            shader
        };

        self.pixel_shader = pixel_shader;
        self
    }

    pub fn map_constants(&self) {
        let context: ID3D11DeviceContext = unsafe { self.device.GetImmediateContext().expect("Failed to get context!") };

        let mut mapped_constants: D3D11_MAPPED_SUBRESOURCE = Default::default();
        unsafe {
            context.Map(self.constants_buffer.as_ref().unwrap(), 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut mapped_constants))
                .expect("Failed to map constants buffer!");

            let constants: &mut C = std::mem::transmute(mapped_constants.pData);
            *constants = self.constants;

            context.Unmap(self.constants_buffer.as_ref().unwrap(), 0);
        }
    }

    pub fn map_vertices(&self) {
        let context: ID3D11DeviceContext = unsafe { self.device.GetImmediateContext().expect("Failed to get context!") };

        let mut mapped_vertices: D3D11_MAPPED_SUBRESOURCE = Default::default();
        unsafe {
            context.Map(self.vertex_buffer.as_ref().unwrap(), 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut mapped_vertices))
                .expect("Failed to map vertex buffer!");

            let vertices: &mut [T] = std::slice::from_raw_parts_mut(mapped_vertices.pData as *mut T, self.vertices.len());
            vertices.copy_from_slice(&self.vertices);

            context.Unmap(self.vertex_buffer.as_ref().unwrap(), 0);
        }
    }

    pub fn map_indices(&self) {
        let context: ID3D11DeviceContext = unsafe { self.device.GetImmediateContext().expect("Failed to get context!") };

        let mut mapped_indices: D3D11_MAPPED_SUBRESOURCE = Default::default();
        unsafe {
            context.Map(self.index_buffer.as_ref().unwrap(), 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut mapped_indices))
                .expect("Failed to map index buffer!");

            let indices: &mut [u32] = std::slice::from_raw_parts_mut(mapped_indices.pData as *mut u32, self.indices.len());
            indices.copy_from_slice(&self.indices);
            
            context.Unmap(self.index_buffer.as_ref().unwrap(), 0);
        }
    }

    pub fn add_mesh(&mut self, mesh: impl Mesh<T>) {
        if mesh.vertices().len() == 0 {
            return;
        }

        let start_vertex = self.vertices.len() as u32;
        let start_index = self.indices.len() as u32;

        self.vertices.extend(mesh.vertices());
        if self.vertices.len() > self.vertex_capacity {
            self.update_vertex_buffer_capacity();
        }

        self.indices.extend(mesh.indices());
        if self.indices.len() > self.index_capacity {
            self.update_index_buffer_capacity();
        }

        let vertex_count = self.vertices.len() as u32 - start_vertex;
        let index_count = self.indices.len() as u32 - start_index;


        let mut min_x = mesh.vertices().first().unwrap().pos()[0];
        let mut max_x = min_x;
        
        let mut min_y = mesh.vertices().first().unwrap().pos()[1];
        let mut max_y = min_y;

        let mut min_z = mesh.vertices().first().unwrap().pos()[2];
        let mut max_z = min_z;

        for vert in mesh.vertices() {
            let pos = vert.pos();

            min_x = min_x.min(pos[0]);
            max_x = max_x.max(pos[0]);

            min_y = min_y.min(pos[1]);
            max_y = max_y.max(pos[1]);
        
            min_z = min_z.min(pos[2]);
            max_z = max_z.max(pos[2]);
        }

        let x = (min_x + max_x) / 2.0;
        let y = (min_y + max_y) / 2.0;
        let z = (min_z + max_z) / 2.0;

        self.mesh_headers.push(MeshHeader {
            pos: [x, y, z],
            start_vertex,
            vertex_count,
            start_index,
            index_count,
        });
    }

    pub fn update_vertex_buffer_capacity(&mut self) {
        if self.vertex_capacity < self.vertices.capacity() {
            self.vertex_capacity = self.vertices.capacity();
        } else {
            return;
        }

        let vertex_buffer_desc = D3D11_BUFFER_DESC {
            Usage: D3D11_USAGE_DYNAMIC,
            ByteWidth: (std::mem::size_of::<T>() * self.vertex_capacity) as u32,
            BindFlags: 0x1, //D3D11_BIND_VERTEX_BUFFER,
            CPUAccessFlags: 0x10000, //D3D11_CPU_ACCESS_WRITE,
            ..Default::default()
        };

        let mut vertex_buffer: Option<ID3D11Buffer> = None;
        unsafe { self.device.CreateBuffer(
                &vertex_buffer_desc, 
                None, 
                Some( &mut vertex_buffer)).expect("Failed to create vertex buffer");
        }

        self.vertex_buffer = vertex_buffer;
    }

    pub fn update_index_buffer_capacity(&mut self) {
        if self.index_capacity < self.indices.capacity() {
            self.index_capacity = self.indices.capacity();
        } else {
            return;
        }

        let index_buffer_desc = D3D11_BUFFER_DESC {
            Usage: D3D11_USAGE_DYNAMIC,
            ByteWidth: (std::mem::size_of::<u32>() * self.index_capacity) as u32,
            BindFlags: 0x2, //D3D11_BIND_INDEX_BUFFER,
            CPUAccessFlags: 0x10000, //D3D11_CPU_ACCESS_WRITE,
            MiscFlags: 0x0, //D3D11_RESOURCE_MISC_FLAG(0),
            ..Default::default()
        };

        let mut index_buffer: Option<ID3D11Buffer> = None;
        unsafe { self.device.CreateBuffer(
                &index_buffer_desc, 
                None, 
                Some( &mut index_buffer)).expect("Failed to create index buffer");
        }

        self.index_buffer = index_buffer;
    }
}

impl<T: std::marker::Copy + Position + 'static, C: std::marker::Copy + std::default::Default + 'static> OrderedRenderable for VertexGroup<T, C>
where VertexGroup<T, C>: Sortable 
{}


impl<T: std::marker::Copy + Position + 'static, C: std::marker::Copy + std::default::Default + 'static> Renderable for VertexGroup<T, C> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn map(&self) {
        if self.constants_buffer.is_some() {
            self.map_constants();
        }
        self.map_vertices();
        self.map_indices();
    }

    fn bind(&self) {
        let context: ID3D11DeviceContext = unsafe { self.device.GetImmediateContext().expect("Failed to get context!") };

        // Constants
        unsafe {
            if self.constants_buffer.is_some() {
                context.VSSetConstantBuffers(0, Some(&[Some(self.constants_buffer.as_ref().unwrap().clone())]));
                context.PSSetConstantBuffers(0, Some(&[Some(self.constants_buffer.as_ref().unwrap().clone())]));
            }
        }

        // Input Layout, Buffers & Shaders
        let stride = std::mem::size_of::<T>() as u32;
        let offset = 0;
        unsafe {
            context.IASetInputLayout(self.input_layout.as_ref().unwrap());
            context.IASetVertexBuffers(0, 1, Some(&self.vertex_buffer), Some(&stride), Some(&offset));
            context.IASetIndexBuffer(self.index_buffer.as_ref().unwrap(), DXGI_FORMAT_R32_UINT, 0);
            context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

            context.VSSetShader(self.vertex_shader.as_ref().unwrap(), None);
            context.PSSetShader(self.pixel_shader.as_ref().unwrap(), None);
        }
    }

    fn mesh_headers(&self) -> Vec<MeshHeader> {
        self.mesh_headers.clone()
    }

    fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.mesh_headers.clear();
    }

    fn double_sided(&self) -> bool {
        self.double_sided
    }
}
