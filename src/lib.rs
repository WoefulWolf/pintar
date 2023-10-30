use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::size_of;

use mesh::{Mesh, LineMesh};
use default_elements::DefaultVertex;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_D24_UNORM_S8_UINT, DXGI_FORMAT_D32_FLOAT_S8X24_UINT};
use windows::Win32::Graphics::Dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};
use windows::Win32::Graphics::Direct3D11::*;
use windows::core::ComInterface;
use directx_math::*;

use tracing::*;
pub mod default_elements;
pub mod vertex_group;
pub mod mesh;

pub mod primitives;

struct BackBufferData {
    width: u32,
    height: u32,
}

impl BackBufferData {
    fn new() -> BackBufferData {
        BackBufferData {
            width: 0,
            height: 0,
        }
    }
}

pub struct Pintar {
    device: ID3D11Device,

    blend_state: Option<ID3D11BlendState>,
    ccw_rasterizer_state: Option<ID3D11RasterizerState>,
    cw_rasterizer_state: Option<ID3D11RasterizerState>,
    depth_stencil_state: Option<ID3D11DepthStencilState>,

    render_target_view: Option<ID3D11RenderTargetView>,
    depth_stencil_resource: Option<ID3D11Resource>,

    depth_pass_index: u32,
    depth_pass_target_index: u32,

    vertex_groups: HashMap<String, Box<dyn vertex_group::OrderedRenderable>>,

    back_buffer_data: BackBufferData,
}



impl Pintar {
    pub fn new(swapchain: &IDXGISwapChain, depth_pass_target_index: u32) -> Pintar {
        let device = unsafe { swapchain.GetDevice::<ID3D11Device>().unwrap() };

        let mut swap_chain_desc = DXGI_SWAP_CHAIN_DESC::default();
        unsafe { swapchain.GetDesc(&mut swap_chain_desc).unwrap() };

        let mut feature_data: D3D11_FEATURE_DATA_D3D11_OPTIONS2 = unsafe { std::mem::zeroed() };
        let res = unsafe { device.CheckFeatureSupport(D3D11_FEATURE_D3D11_OPTIONS2, &mut feature_data as *mut D3D11_FEATURE_DATA_D3D11_OPTIONS2 as *mut c_void, size_of::<D3D11_FEATURE_DATA_D3D11_OPTIONS2>() as u32) };
        res.expect("Failed to get device features.");
        // info!("{:?}", feature_data);

        // Blending
        let rt_blend_desc = D3D11_RENDER_TARGET_BLEND_DESC {
            BlendEnable: BOOL(1),
            SrcBlend: D3D11_BLEND_SRC_ALPHA,
            DestBlend: D3D11_BLEND_INV_SRC_ALPHA,
            BlendOp: D3D11_BLEND_OP_ADD,
            SrcBlendAlpha: D3D11_BLEND_ONE,
            DestBlendAlpha: D3D11_BLEND_ZERO,
            BlendOpAlpha: D3D11_BLEND_OP_ADD,
            RenderTargetWriteMask: 0x0F, // D3D11_COLOR_WRITE_ENABLE_ALL
            ..Default::default()
        };

        let blend_desc = D3D11_BLEND_DESC {
            AlphaToCoverageEnable: BOOL(0),
            IndependentBlendEnable: BOOL(0),
            RenderTarget: [rt_blend_desc; 8],
            ..Default::default()
        };

        let mut blend_state: Option<ID3D11BlendState> = None;
        unsafe { device.CreateBlendState(
            &blend_desc, 
            Some( &mut blend_state)).expect("Failed to create blend state");
        }

        // Create rasterizer states
        let mut rasterizer_desc = D3D11_RASTERIZER_DESC {
            FillMode: D3D11_FILL_SOLID,
            CullMode: D3D11_CULL_BACK,
            FrontCounterClockwise: BOOL(1),
            ..Default::default()
        };

        let mut ccw_rasterizer_state: Option<ID3D11RasterizerState> = None;
        unsafe { device.CreateRasterizerState(
            &rasterizer_desc, 
            Some( &mut ccw_rasterizer_state)).expect("Failed to create CCW rasterizer state");
        }

        rasterizer_desc.FrontCounterClockwise = BOOL(0);
        let mut cw_rasterizer_state: Option<ID3D11RasterizerState> = None;
        unsafe { device.CreateRasterizerState(
            &rasterizer_desc, 
            Some( &mut cw_rasterizer_state)).expect("Failed to create CW rasterizer state");
        }

        // Depth stencil
        let depth_stencil_desc = D3D11_DEPTH_STENCIL_DESC {
            DepthEnable: BOOL(1),
            DepthWriteMask: D3D11_DEPTH_WRITE_MASK_ALL, // Change to ZERO to disable writing to
                                                        // depth mask
            DepthFunc: D3D11_COMPARISON_LESS,
            StencilEnable: BOOL(1),
            StencilReadMask: 0xFF,
            StencilWriteMask: 0xFF,
            FrontFace: D3D11_DEPTH_STENCILOP_DESC {
                StencilFailOp: D3D11_STENCIL_OP_KEEP,
                StencilDepthFailOp: D3D11_STENCIL_OP_INCR,
                StencilPassOp: D3D11_STENCIL_OP_KEEP,
                StencilFunc: D3D11_COMPARISON_ALWAYS,
            },
            BackFace: D3D11_DEPTH_STENCILOP_DESC {
                StencilFailOp: D3D11_STENCIL_OP_KEEP,
                StencilDepthFailOp: D3D11_STENCIL_OP_DECR,
                StencilPassOp: D3D11_STENCIL_OP_KEEP,
                StencilFunc: D3D11_COMPARISON_ALWAYS,
            },
            ..Default::default()
        };

        let mut depth_stencil_state: Option<ID3D11DepthStencilState> = None;
        unsafe { device.CreateDepthStencilState(
            &depth_stencil_desc, 
            Some( &mut depth_stencil_state)).expect("Failed to create depth stencil state");
        }

        // BOIT STUFF
        /*
        let back_buffer: ID3D11Texture2D = unsafe { swapchain.GetBuffer::<ID3D11Texture2D>(0).unwrap() };
        let mut back_buffer_desc = D3D11_TEXTURE2D_DESC::default();
        unsafe { back_buffer.GetDesc(&mut back_buffer_desc) };
        let width = back_buffer_desc.Width;
        let height = back_buffer_desc.Height;

        // transparentsTarget
        let trans_desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            ArraySize: 1,
            BindFlags: 8 | 32,
            CPUAccessFlags: 0,
            Format: DXGI_FORMAT_R16G16B16A16_FLOAT,
            MipLevels: 1,
            MiscFlags: 0,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
                ..Default::default()
            },
            Usage: D3D11_USAGE_DEFAULT,
        };
        
        let mut trans_tex: Option<ID3D11Texture2D> = None;
        unsafe { device.CreateTexture2D(&trans_desc, None, Some(&mut trans_tex)).expect("Failed to create transparent target texture.") };

        let trans_rtv_desc = D3D11_RENDER_TARGET_VIEW_DESC {
            Format: DXGI_FORMAT_R16G16B16A16_FLOAT,
            ViewDimension: D3D11_RTV_DIMENSION_TEXTURE2D,
            Anonymous: D3D11_RENDER_TARGET_VIEW_DESC_0 {
                Texture2D: D3D11_TEX2D_RTV {
                    MipSlice: 0
                }
            },
        };
        let mut trans_rtv: Option<ID3D11RenderTargetView> = None;
        unsafe { device.CreateRenderTargetView(&trans_tex.as_ref().unwrap().cast::<ID3D11Resource>().unwrap(), Some(&trans_rtv_desc), Some(&mut trans_rtv)).expect("Failed to create transparent rtv") };

        let mut trans_srv: Option<ID3D11ShaderResourceView> = None;
        unsafe { device.CreateShaderResourceView(&trans_tex.as_ref().unwrap().cast::<ID3D11Resource>().unwrap(), None, Some(&mut trans_srv)).expect("Failed to create transparent rsv") };
        */
        
        let mut pintar = Pintar {
            device: device.clone(),

            blend_state,
            ccw_rasterizer_state,
            cw_rasterizer_state,
            depth_stencil_state,

            render_target_view: None,
            depth_stencil_resource: None,

            depth_pass_index: 0,
            depth_pass_target_index,

            vertex_groups: HashMap::new(),

            back_buffer_data: BackBufferData::new(),
        };

        // Default vertex groups
        let line_vertex_group: vertex_group::VertexGroup<default_elements::LineVertex, default_elements::DefaultConstants> = vertex_group::VertexGroup::new(device.clone(), false)
            .with_constants()
            .vertex_shader(include_bytes!("./line_vs.cso"), &default_elements::LINE_IED)
            .pixel_shader(include_bytes!("./line_ps.cso"));
        pintar.add_vertex_group("default_line".to_string(), Box::new(line_vertex_group));

        let default_vertex_group: vertex_group::VertexGroup<default_elements::DefaultVertex, default_elements::DefaultConstants> = vertex_group::VertexGroup::new(device.clone(), true)
            .with_constants()
            .vertex_shader(include_bytes!("./default_vs.cso"), &default_elements::DEFAULT_IED)
            .pixel_shader(include_bytes!("./default_ps.cso"));
        pintar.add_vertex_group("default".to_string(), Box::new(default_vertex_group));

        pintar.update_back_buffer_data(swapchain);

        pintar
    }

    pub fn add_vertex_group(&mut self, name: String, vertex_group: Box<dyn vertex_group::OrderedRenderable>) {
        self.vertex_groups.insert(name, vertex_group);
    }

    pub fn get_vertex_group_as<T: 'static>(&mut self, name: String) -> Option<&mut T> {
        match self.vertex_groups.get_mut(&name) {
            Some(vertex_group) => {
                (vertex_group.as_any_mut()).downcast_mut::<T>()
            },
            None => None
        }
    }

    pub fn clear_vertex_groups(&mut self) {
        for vertex_group in self.vertex_groups.values_mut() {
            vertex_group.clear();
        }
    }

    pub fn drop_back_buffer_data(&mut self) {
        drop(self.render_target_view.take());
    }

    pub fn update_back_buffer_data(&mut self, swapchain: &IDXGISwapChain) {
        let back_buffer: ID3D11Texture2D = unsafe { swapchain.GetBuffer::<ID3D11Texture2D>(0).unwrap() };
        let mut back_buffer_desc = D3D11_TEXTURE2D_DESC::default();
        unsafe { back_buffer.GetDesc(&mut back_buffer_desc) };

        let res = unsafe { self.device.CreateRenderTargetView(&back_buffer, None, Some(&mut self.render_target_view)) };
        if res.is_err() {
            error!("Failed to create RenderTargetView!");
        }

        self.back_buffer_data.width = back_buffer_desc.Width;
        self.back_buffer_data.height = back_buffer_desc.Height;

        info!("New back buffer dimensions: {}, {}", self.back_buffer_data.width, self.back_buffer_data.height);
    }

    pub fn find_view_resources(&mut self, render_target_views: Option<ID3D11RenderTargetView>, depth_stencil_view: Option<ID3D11DepthStencilView>) {
        if render_target_views.is_none() || depth_stencil_view.is_none() {
            return;
        }

        let depth_stencil_res = unsafe { depth_stencil_view.unwrap().GetResource().ok().unwrap() };
        let depth_stencil_tex = depth_stencil_res.cast::<ID3D11Texture2D>().unwrap();

        let mut depth_stencil_desc = D3D11_TEXTURE2D_DESC::default();
        unsafe { depth_stencil_tex.GetDesc(&mut depth_stencil_desc) };

        if (self.back_buffer_data.width == depth_stencil_desc.Width)
        && (self.back_buffer_data.height == depth_stencil_desc.Height)
        && (depth_stencil_desc.BindFlags & 0x40 != 0) {
            if self.depth_pass_index == self.depth_pass_target_index {
                self.depth_stencil_resource = Some(depth_stencil_res);
                // info!("DSD: {:?}", depth_stencil_desc);
            }

            self.depth_pass_index += 1;
        }
    }

    pub fn render(&mut self) {
        if self.depth_stencil_resource.is_none() {
            return;
        }

        let context: ID3D11DeviceContext = unsafe { self.device.GetImmediateContext().expect("Failed to get context!") };

        unsafe {
            context.OMSetBlendState(self.blend_state.as_ref().unwrap(), None, 0xFFFFFFFF);
            context.OMSetDepthStencilState(self.depth_stencil_state.as_ref().unwrap(), 1);
        }


        let depth_stencil_tex = self.depth_stencil_resource.as_ref().unwrap().cast::<ID3D11Texture2D>().unwrap();
        let mut depth_stencil_desc = D3D11_TEXTURE2D_DESC::default();
        unsafe { depth_stencil_tex.GetDesc(&mut depth_stencil_desc) };

        let dsv_format = match depth_stencil_desc.Format.0 {
            19 => DXGI_FORMAT_D32_FLOAT_S8X24_UINT,
            44 => DXGI_FORMAT_D24_UNORM_S8_UINT,
            _ => DXGI_FORMAT_D24_UNORM_S8_UINT,
        };

        let depth_stencil_view_desc = D3D11_DEPTH_STENCIL_VIEW_DESC {
            Format: dsv_format,
            ViewDimension: D3D11_DSV_DIMENSION_TEXTURE2D,
            Flags: 0,
            Anonymous: D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
                Texture2D: D3D11_TEX2D_DSV { MipSlice: 0 }
            }
        };

        let mut depth_stencil_view: Option<ID3D11DepthStencilView> = None;
        unsafe {
            self.device.CreateDepthStencilView(self.depth_stencil_resource.as_ref(), Some(&depth_stencil_view_desc), Some(&mut depth_stencil_view))
                .expect("Failed to create depth stencil view!");

            context.OMSetRenderTargets(Some(&[self.render_target_view.clone()]), depth_stencil_view.as_ref().unwrap());
        }

        // Map vertex groups' buffers
        for vertex_group in self.vertex_groups.values() {
            vertex_group.map();
        }

        for vertex_group in self.vertex_groups.values_mut() {
            if vertex_group.mesh_headers().len() > 0 {
                vertex_group.bind();

                vertex_group.sort();

                for mesh in vertex_group.mesh_headers() {
                    unsafe {
                        // Draw Clock-wise culled
                        context.RSSetState(self.cw_rasterizer_state.as_ref().unwrap());
                        context.DrawIndexed(mesh.index_count, mesh.start_index, mesh.start_vertex as i32);
                        if vertex_group.double_sided() {
                            // Draw Counter Clock-wise culled
                            context.RSSetState(self.ccw_rasterizer_state.as_ref().unwrap());
                            context.DrawIndexed(mesh.index_count, mesh.start_index, mesh.start_vertex as i32);
                        }
                    }
                }
                
            }
        }

        // Reset to clock-wise culling
        unsafe {
            context.RSSetState(self.cw_rasterizer_state.as_ref().unwrap());
        }

        // Reset depth resource
        self.depth_stencil_resource = None;

        // Reset depth pass index
        self.depth_pass_index = 0;
    }

    pub fn set_default_view_proj(&mut self, view_proj: [[f32; 4]; 4]) {
        let default_vertex_group: &mut vertex_group::VertexGroup<default_elements::DefaultVertex, default_elements::DefaultConstants> = self.get_vertex_group_as("default".to_string()).unwrap();
        default_vertex_group.constants.view_proj = XMMatrix::from(&view_proj);

        let line_vertex_group: &mut vertex_group::VertexGroup<default_elements::LineVertex, default_elements::DefaultConstants> = self.get_vertex_group_as("default_line".to_string()).unwrap();
        line_vertex_group.constants.view_proj = XMMatrix::from(&view_proj);
    }

    pub fn add_default_mesh(&mut self, mesh: impl Mesh<DefaultVertex>) {
        let vertex_group: &mut vertex_group::VertexGroup<default_elements::DefaultVertex, default_elements::DefaultConstants> = self.get_vertex_group_as("default".to_string()).unwrap();
        vertex_group.add_mesh(mesh);
    }

    pub fn add_line(&mut self, start: [f32; 3], end: [f32; 3], colour: [f32; 4], thickness: f32) {
        let vertex_group: &mut vertex_group::VertexGroup<default_elements::LineVertex, default_elements::DefaultConstants> = self.get_vertex_group_as("default_line".to_string()).unwrap();

        let vertices: Vec<default_elements::LineVertex> = vec![
            default_elements::LineVertex {
                position1: start.into(),
                position2: end.into(),
                colour: colour.into(),
                thickness,
            },
            default_elements::LineVertex {
                position1: start.into(),
                position2: end.into(),
                colour: colour.into(),
                thickness: -thickness,
            },
            default_elements::LineVertex {
                position1: end.into(),
                position2: start.into(),
                colour: colour.into(),
                thickness: -thickness,
            },
            default_elements::LineVertex {
                position1: end.into(),
                position2: start.into(),
                colour: colour.into(),
                thickness,
            },
            
        ];

        let indices: Vec<u32> = vec![
            0, 2, 1, 1, 2, 3
        ];

        let line_mesh = LineMesh::new(&vertices, &indices);

        vertex_group.add_mesh(line_mesh);
    }

    pub fn extend_line(&mut self, end: [f32; 3], colour: [f32; 4], thickness: f32) {
        let vertex_group: &mut vertex_group::VertexGroup<default_elements::LineVertex, default_elements::DefaultConstants> = self.get_vertex_group_as("default_line".to_string()).unwrap();

        let last_indices = vertex_group.indices.as_slice()[vertex_group.indices.len()-2..].to_vec();

        let vertices: Vec<default_elements::LineVertex> = vec![
            default_elements::LineVertex {
                position1: end.into(),
                position2: vertex_group.vertices.last().unwrap().position1,
                colour: colour.into(),
                thickness: -thickness,
            },
            default_elements::LineVertex {
                position1: end.into(),
                position2: vertex_group.vertices.last().unwrap().position1,
                colour: colour.into(),
                thickness: thickness,
            },
        ];

        let indices: Vec<u32> = vec![
            last_indices[0],
            last_indices[0] + 2,
            last_indices[1],
            last_indices[1],
            last_indices[1] + 1,
            last_indices[1] + 2,
        ];

        vertex_group.vertices.extend_from_slice(&vertices);
        if vertex_group.vertices.len() > vertex_group.vertex_capacity {
            vertex_group.update_vertex_buffer_capacity();
        }

        vertex_group.indices.extend_from_slice(&indices);
        if vertex_group.indices.len() > vertex_group.index_capacity {
            vertex_group.update_index_buffer_capacity();
        }

        vertex_group.mesh_headers.last_mut().unwrap().vertex_count += vertices.len() as u32;
        vertex_group.mesh_headers.last_mut().unwrap().index_count += indices.len() as u32;
    }
}
