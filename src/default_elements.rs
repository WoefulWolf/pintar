use windows::{Win32::Graphics::{Direct3D11::*, Dxgi::Common::{DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R32G32B32A32_FLOAT, DXGI_FORMAT_R32_FLOAT}}, core::PCSTR};
use directx_math::*;
use crate::vertex_group::{Position, VertexGroup, Sortable};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DefaultConstants {
    pub view_proj: XMMatrix,
    pub sun_dir: XMFLOAT3,
}

impl Default for DefaultConstants {
    fn default() -> Self {
        Self {
            view_proj: XMMatrix(XMMatrixIdentity()),
            sun_dir: [0.0, -1.0, 0.0].into()
        }
    }
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct DefaultVertex {
    pub position: XMFLOAT3,
    pub normal: XMFLOAT3,
    pub colour: XMFLOAT4,
    pub bary_coords: XMFLOAT3,
}

impl Position for DefaultVertex {
    fn pos(&self) -> [f32; 3] {
        self.position.into()
    }
}

pub const DEFAULT_IED: [D3D11_INPUT_ELEMENT_DESC; 4] = [
    D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(b"POSITION\0".as_ptr() as _),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32B32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: 0,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0,
    },
    D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(b"NORMAL\0".as_ptr() as _),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32B32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0,
    },
    D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(b"COLOR\0".as_ptr() as _),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0,
    },
    D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(b"BARY_COORDS\0".as_ptr() as _),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32B32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0,
    },
];

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LineVertex {
    pub position1: XMFLOAT3,
    pub position2: XMFLOAT3,
    pub colour: XMFLOAT4,
    pub thickness: f32,
}

impl Position for LineVertex {
    fn pos(&self) -> [f32; 3] {
        self.position1.into()
    }
}

pub const LINE_IED: [D3D11_INPUT_ELEMENT_DESC; 4] = [
    D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(b"POSITION\0".as_ptr() as _),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32B32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: 0,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0,
    },
    D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(b"POSITIONNEXT\0".as_ptr() as _),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32B32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0,
    },
    D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(b"COLOR\0".as_ptr() as _),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0,
    },
    D3D11_INPUT_ELEMENT_DESC {
        SemanticName: PCSTR(b"THICKNESS\0".as_ptr() as _),
        SemanticIndex: 0,
        Format: DXGI_FORMAT_R32_FLOAT,
        InputSlot: 0,
        AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
        InstanceDataStepRate: 0,
    },
];


impl Sortable for VertexGroup<DefaultVertex, DefaultConstants> {
    fn sort(&mut self) {
        let view_proj_inv = XMMatrixInverse(None, *self.constants.view_proj);
        let mut scale: XMVECTOR = XMVectorZero();
        let mut rot: XMVECTOR = XMVectorZero();
        let mut trans: XMVECTOR = XMVectorZero();
        XMMatrixDecompose(&mut scale, &mut rot, &mut trans, view_proj_inv);

        let divisor: XMVECTOR = XMVectorReplicate(10.0);
        let cam_position = XMVectorDivide(trans, divisor);

        self.mesh_headers.sort_by(|a, b| {
            let a_pos = XMVectorSet(a.pos[0], a.pos[1], a.pos[2], 0.0);
            let b_pos = XMVectorSet(b.pos[0], b.pos[1], b.pos[2], 0.0);

            let a_dist = XMVectorGetX(XMVector3LengthSq(XMVectorSubtract(a_pos, cam_position)));
            let b_dist = XMVectorGetX(XMVector3LengthSq(XMVectorSubtract(b_pos, cam_position)));

            if a_dist > b_dist {
                return std::cmp::Ordering::Less;
            } else if a_dist < b_dist {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Equal;
            }
        });

    }
}

impl Sortable for VertexGroup<LineVertex, DefaultConstants> {

}
