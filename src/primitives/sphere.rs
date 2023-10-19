use crate::{default_elements::DefaultVertex, mesh::DefaultMesh};

pub fn new(colour: [f32; 4]) -> DefaultMesh {
    let vertices: Vec<DefaultVertex> = vec![
        DefaultVertex {
            position: [0.0, -1.0, -0.0].into(),
            normal: [-1e-06, -1.0, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.723607, -0.44722, 0.525725].into(),
            normal: [0.723607, -0.447219, 0.525726].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.276388, -0.44722, 0.850649].into(),
            normal: [-0.276389, -0.447219, 0.850649].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.894426, -0.447216, -0.0].into(),
            normal: [-0.894426, -0.447216, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.276388, -0.44722, -0.850649].into(),
            normal: [-0.276389, -0.447219, -0.850649].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.723607, -0.44722, -0.525725].into(),
            normal: [0.723607, -0.447219, -0.525726].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.276388, 0.44722, 0.850649].into(),
            normal: [0.276389, 0.44722, 0.850649].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.723607, 0.44722, 0.525725].into(),
            normal: [-0.723607, 0.447219, 0.525726].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.723607, 0.44722, -0.525725].into(),
            normal: [-0.723607, 0.447219, -0.525726].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.276388, 0.44722, -0.850649].into(),
            normal: [0.276389, 0.447219, -0.850649].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.894426, 0.447216, -0.0].into(),
            normal: [0.894426, 0.447216, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, 1.0, -0.0].into(),
            normal: [1e-06, 1.0, 0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.232822, -0.657519, 0.716563].into(),
            normal: [-0.230792, -0.664973, 0.710314].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.162456, -0.850654, 0.499995].into(),
            normal: [-0.162456, -0.850654, 0.499996].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.077607, -0.96795, 0.238853].into(),
            normal: [-0.080575, -0.965406, 0.247988].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.203181, -0.96795, 0.147618].into(),
            normal: [0.210952, -0.965406, 0.153264].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.425323, -0.850654, 0.309011].into(),
            normal: [0.425323, -0.850654, 0.309012].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.609547, -0.657519, 0.442856].into(),
            normal: [0.604231, -0.664972, 0.438995].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.531941, -0.502302, 0.681712].into(),
            normal: [0.523657, -0.503818, 0.686986].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.262869, -0.525738, 0.809012].into(),
            normal: [0.262869, -0.525737, 0.809012].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.029639, -0.502302, 0.864184].into(),
            normal: [-0.019838, -0.503819, 0.863582].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.812729, -0.502301, -0.295238].into(),
            normal: [0.815185, -0.503817, -0.28573].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.850648, -0.525736, -0.0].into(),
            normal: [0.850648, -0.525735, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.812729, -0.502301, 0.295238].into(),
            normal: [0.815185, -0.503817, 0.28573].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.203181, -0.96795, -0.147618].into(),
            normal: [0.210952, -0.965406, -0.153264].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.425323, -0.850654, -0.309011].into(),
            normal: [0.425323, -0.850654, -0.309012].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.609547, -0.657519, -0.442856].into(),
            normal: [0.604231, -0.664972, -0.438995].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.753442, -0.657515, -0.0].into(),
            normal: [-0.746871, -0.664969, 0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.52573, -0.850652, -0.0].into(),
            normal: [-0.525729, -0.850652, 0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.251147, -0.967949, -0.0].into(),
            normal: [-0.260752, -0.965406, 0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.483971, -0.502302, 0.716565].into(),
            normal: [-0.491546, -0.503818, 0.710316].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.688189, -0.525736, 0.499997].into(),
            normal: [-0.688189, -0.525736, 0.499998].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.831051, -0.502299, 0.238853].into(),
            normal: [-0.827449, -0.503816, 0.247989].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.232822, -0.657519, -0.716563].into(),
            normal: [-0.230792, -0.664973, -0.710314].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.162456, -0.850654, -0.499995].into(),
            normal: [-0.162457, -0.850654, -0.499995].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.077607, -0.96795, -0.238853].into(),
            normal: [-0.080575, -0.965406, -0.247988].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.831051, -0.502299, -0.238853].into(),
            normal: [-0.827449, -0.503816, -0.247989].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.688189, -0.525736, -0.499997].into(),
            normal: [-0.688189, -0.525735, -0.499998].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.483971, -0.502302, -0.716565].into(),
            normal: [-0.491546, -0.503819, -0.710316].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.029639, -0.502302, -0.864184].into(),
            normal: [-0.019838, -0.503819, -0.863582].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.262869, -0.525738, -0.809012].into(),
            normal: [0.262869, -0.525737, -0.809012].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.531941, -0.502302, -0.681712].into(),
            normal: [0.523657, -0.503819, -0.686986].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.956626, 0.251149, 0.147618].into(),
            normal: [0.957826, 0.243062, 0.153265].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.951058, -0.0, 0.309013].into(),
            normal: [0.951058, -0.0, 0.309014].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.860698, -0.251151, 0.442858].into(),
            normal: [0.864987, -0.243063, 0.438997].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.860698, -0.251151, -0.442858].into(),
            normal: [0.864987, -0.243063, -0.438997].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.951058, 0.0, -0.309013].into(),
            normal: [0.951058, 0.0, -0.309014].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.956626, 0.251149, -0.147618].into(),
            normal: [0.957826, 0.243062, -0.153265].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.155215, 0.251152, 0.955422].into(),
            normal: [0.150217, 0.243063, 0.958308].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, -0.0, 1.0].into(),
            normal: [0.0, -0.0, 1.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.155215, -0.251152, 0.955422].into(),
            normal: [-0.150216, -0.243063, 0.958309].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.687159, -0.251152, 0.681715].into(),
            normal: [0.684811, -0.243064, 0.686989].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.587786, 0.0, 0.809017].into(),
            normal: [0.587786, 0.0, 0.809017].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.436007, 0.251152, 0.864188].into(),
            normal: [0.441748, 0.243064, 0.863585].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.860698, 0.251151, 0.442858].into(),
            normal: [-0.864987, 0.243062, 0.438997].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.951058, -0.0, 0.309013].into(),
            normal: [-0.951057, -0.0, 0.309014].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.956626, -0.251149, 0.147618].into(),
            normal: [-0.957826, -0.243061, 0.153266].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.436007, -0.251152, 0.864188].into(),
            normal: [-0.441748, -0.243064, 0.863585].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.587786, 0.0, 0.809017].into(),
            normal: [-0.587786, -0.0, 0.809017].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.687159, 0.251152, 0.681715].into(),
            normal: [-0.684812, 0.243064, 0.686989].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.687159, 0.251152, -0.681715].into(),
            normal: [-0.684811, 0.243064, -0.686989].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.587786, -0.0, -0.809017].into(),
            normal: [-0.587786, -0.0, -0.809017].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.436007, -0.251152, -0.864188].into(),
            normal: [-0.441748, -0.243064, -0.863585].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.956626, -0.251149, -0.147618].into(),
            normal: [-0.957826, -0.243061, -0.153265].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.951058, 0.0, -0.309013].into(),
            normal: [-0.951058, -0.0, -0.309014].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.860698, 0.251151, -0.442858].into(),
            normal: [-0.864987, 0.243062, -0.438997].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.436007, 0.251152, -0.864188].into(),
            normal: [0.441748, 0.243064, -0.863585].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.587786, -0.0, -0.809017].into(),
            normal: [0.587786, 0.0, -0.809016].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.687159, -0.251152, -0.681715].into(),
            normal: [0.684812, -0.243063, -0.686988].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.155215, -0.251152, -0.955422].into(),
            normal: [-0.150217, -0.243063, -0.958309].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.0, 0.0, -1.0].into(),
            normal: [0.0, -0.0, -1.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.155215, 0.251152, -0.955422].into(),
            normal: [0.150217, 0.243063, -0.958309].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.831051, 0.502299, 0.238853].into(),
            normal: [0.827449, 0.503816, 0.247989].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.688189, 0.525736, 0.499997].into(),
            normal: [0.68819, 0.525735, 0.499998].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.483971, 0.502302, 0.716565].into(),
            normal: [0.491547, 0.503818, 0.710316].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.029639, 0.502302, 0.864184].into(),
            normal: [0.019838, 0.503819, 0.863582].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.262869, 0.525738, 0.809012].into(),
            normal: [-0.262869, 0.525737, 0.809012].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.531941, 0.502302, 0.681712].into(),
            normal: [-0.523657, 0.503818, 0.686986].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.812729, 0.502301, 0.295238].into(),
            normal: [-0.815185, 0.503818, 0.28573].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.850648, 0.525736, -0.0].into(),
            normal: [-0.850648, 0.525735, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.812729, 0.502301, -0.295238].into(),
            normal: [-0.815185, 0.503818, -0.28573].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.531941, 0.502302, -0.681712].into(),
            normal: [-0.523657, 0.503819, -0.686986].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.262869, 0.525738, -0.809012].into(),
            normal: [-0.262869, 0.525737, -0.809012].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.029639, 0.502302, -0.864184].into(),
            normal: [0.019838, 0.503819, -0.863582].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.483971, 0.502302, -0.716565].into(),
            normal: [0.491547, 0.503818, -0.710316].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.688189, 0.525736, -0.499997].into(),
            normal: [0.688189, 0.525736, -0.499998].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.831051, 0.502299, -0.238853].into(),
            normal: [0.827448, 0.503816, -0.247989].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.077607, 0.96795, 0.238853].into(),
            normal: [0.080575, 0.965407, 0.247988].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.162456, 0.850654, 0.499995].into(),
            normal: [0.162456, 0.850654, 0.499995].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.232822, 0.657519, 0.716563].into(),
            normal: [0.230792, 0.664973, 0.710314].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.753442, 0.657515, -0.0].into(),
            normal: [0.746871, 0.664969, 0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.52573, 0.850652, -0.0].into(),
            normal: [0.52573, 0.850652, 0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.251147, 0.967949, -0.0].into(),
            normal: [0.260752, 0.965406, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.203181, 0.96795, 0.147618].into(),
            normal: [-0.210952, 0.965406, 0.153264].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.425323, 0.850654, 0.309011].into(),
            normal: [-0.425323, 0.850654, 0.309012].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.609547, 0.657519, 0.442856].into(),
            normal: [-0.604231, 0.664972, 0.438995].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.203181, 0.96795, -0.147618].into(),
            normal: [-0.210952, 0.965406, -0.153264].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.425323, 0.850654, -0.309011].into(),
            normal: [-0.425323, 0.850654, -0.309012].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.609547, 0.657519, -0.442856].into(),
            normal: [-0.604231, 0.664972, -0.438995].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.077607, 0.96795, -0.238853].into(),
            normal: [0.080575, 0.965407, -0.247988].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.162456, 0.850654, -0.499995].into(),
            normal: [0.162456, 0.850654, -0.499995].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.232822, 0.657519, -0.716563].into(),
            normal: [0.230792, 0.664973, -0.710314].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.3618, 0.894429, -0.262863].into(),
            normal: [0.368207, 0.890426, -0.267517].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.638194, 0.72361, -0.262864].into(),
            normal: [0.631748, 0.727549, -0.267519].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.447209, 0.723612, -0.525728].into(),
            normal: [0.449644, 0.727551, -0.51816].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.138197, 0.89443, -0.425319].into(),
            normal: [-0.140644, 0.890427, -0.43285].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.05279, 0.723612, -0.688185].into(),
            normal: [-0.059208, 0.727551, -0.683494].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.361804, 0.723612, -0.587778].into(),
            normal: [-0.353854, 0.727551, -0.587756].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.44721, 0.894429, -0.0].into(),
            normal: [-0.455128, 0.890426, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.670817, 0.723611, -0.162457].into(),
            normal: [-0.668338, 0.72755, -0.154903].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.670817, 0.723611, 0.162457].into(),
            normal: [-0.668338, 0.72755, 0.154903].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.138197, 0.89443, 0.425319].into(),
            normal: [-0.140644, 0.890427, 0.43285].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.361804, 0.723612, 0.587778].into(),
            normal: [-0.353854, 0.727551, 0.587756].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.05279, 0.723612, 0.688185].into(),
            normal: [-0.059208, 0.727551, 0.683494].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.3618, 0.894429, 0.262863].into(),
            normal: [0.368207, 0.890426, 0.267517].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.447209, 0.723612, 0.525728].into(),
            normal: [0.449644, 0.727551, 0.51816].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.638194, 0.72361, 0.262864].into(),
            normal: [0.631748, 0.727549, 0.267518].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.861804, 0.276396, -0.425322].into(),
            normal: [0.859318, 0.272417, -0.432853].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.809019, 0.0, -0.587782].into(),
            normal: [0.808987, 0.008872, -0.587759].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.670821, 0.276397, -0.688189].into(),
            normal: [0.677214, 0.272418, -0.683498].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.138199, 0.276397, -0.951055].into(),
            normal: [-0.146129, 0.272418, -0.951018].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.309016, -0.0, -0.951057].into(),
            normal: [-0.309005, 0.008872, -0.951019].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.447215, 0.276397, -0.850649].into(),
            normal: [-0.440777, 0.272418, -0.85528].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.947213, 0.276396, -0.162458].into(),
            normal: [-0.949628, 0.272417, -0.154904].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-1.0, 1e-06, 0.0].into(),
            normal: [-0.999961, 0.008873, 0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.947213, 0.276397, 0.162458].into(),
            normal: [-0.949628, 0.272418, 0.154904].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.447216, 0.276397, 0.850648].into(),
            normal: [-0.440778, 0.272418, 0.85528].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.309017, -1e-06, 0.951056].into(),
            normal: [-0.309005, 0.008872, 0.951019].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.138199, 0.276397, 0.951055].into(),
            normal: [-0.14613, 0.272418, 0.951018].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.67082, 0.276396, 0.68819].into(),
            normal: [0.677214, 0.272417, 0.683498].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.809019, -2e-06, 0.587783].into(),
            normal: [0.808987, 0.008871, 0.58776].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.861804, 0.276394, 0.425323].into(),
            normal: [0.859318, 0.272416, 0.432854].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.309017, -0.0, -0.951056].into(),
            normal: [0.309005, -0.008872, -0.951019].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.447216, -0.276398, -0.850648].into(),
            normal: [0.440778, -0.272418, -0.85528].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.138199, -0.276398, -0.951055].into(),
            normal: [0.14613, -0.272418, -0.951018].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.809018, -0.0, -0.587783].into(),
            normal: [-0.808986, -0.008872, -0.58776].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.670819, -0.276397, -0.688191].into(),
            normal: [-0.677213, -0.272418, -0.683499].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.861803, -0.276396, -0.425324].into(),
            normal: [-0.859317, -0.272417, -0.432855].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.809018, 0.0, 0.587783].into(),
            normal: [-0.808986, -0.008872, 0.58776].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.861803, -0.276396, 0.425324].into(),
            normal: [-0.859317, -0.272417, 0.432855].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.670819, -0.276397, 0.688191].into(),
            normal: [-0.677213, -0.272418, 0.683499].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.309017, 0.0, 0.951056].into(),
            normal: [0.309005, -0.008873, 0.951019].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.138199, -0.276398, 0.951055].into(),
            normal: [0.14613, -0.272418, 0.951018].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.447216, -0.276398, 0.850648].into(),
            normal: [0.440778, -0.272418, 0.85528].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [1.0, 0.0, -0.0].into(),
            normal: [0.999961, -0.008873, -0.0].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.947213, -0.276396, 0.162458].into(),
            normal: [0.949628, -0.272417, 0.154904].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.947213, -0.276396, -0.162458].into(),
            normal: [0.949628, -0.272417, -0.154904].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.361803, -0.723612, -0.587779].into(),
            normal: [0.353854, -0.727551, -0.587756].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.138197, -0.894429, -0.425321].into(),
            normal: [0.140644, -0.890426, -0.432852].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.052789, -0.723611, -0.688186].into(),
            normal: [0.059207, -0.72755, -0.683495].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.447211, -0.723612, -0.525727].into(),
            normal: [-0.449645, -0.727551, -0.518159].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.361801, -0.894429, -0.262863].into(),
            normal: [-0.368207, -0.890426, -0.267517].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.638195, -0.723609, -0.262863].into(),
            normal: [-0.631749, -0.727549, -0.267518].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.638195, -0.723609, 0.262864].into(),
            normal: [-0.631749, -0.727549, 0.267518].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.361801, -0.894428, 0.262864].into(),
            normal: [-0.368207, -0.890425, 0.267519].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [-0.447211, -0.72361, 0.525729].into(),
            normal: [-0.449645, -0.72755, 0.518161].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.670817, -0.723611, -0.162457].into(),
            normal: [0.668338, -0.72755, -0.154903].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.670818, -0.72361, 0.162458].into(),
            normal: [0.668339, -0.727549, 0.154904].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.447211, -0.894428, 1e-06].into(),
            normal: [0.455129, -0.890426, 1e-06].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.05279, -0.723612, 0.688185].into(),
            normal: [0.059208, -0.727551, 0.683494].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.138199, -0.894429, 0.425321].into(),
            normal: [0.140645, -0.890426, 0.432852].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        DefaultVertex {
            position: [0.361805, -0.723611, 0.587779].into(),
            normal: [0.353855, -0.72755, 0.587756].into(),
            colour: colour.into(),
            bary_coords: [1.0, 0.0, 0.0].into()
        },
        ];

    let indices: Vec<u32> = vec![
        0, 15, 14, 1, 17, 23, 0, 14, 29, 0, 29, 35, 0, 35, 24, 1, 23, 44, 2, 20, 50, 3, 32, 56, 4, 38, 62, 5, 41, 68, 1, 44, 51, 2, 50, 57, 3, 56, 63, 4, 62, 69, 5, 68, 45, 6, 74, 89, 7, 77, 95, 8, 80, 98, 9, 83, 101, 10, 86, 90, 92, 99, 11, 91, 102, 92, 90, 103, 91, 92, 102, 99, 102, 100, 99, 91, 103, 102, 103, 104, 102, 102, 104, 100, 104, 101, 100, 90, 86, 103, 86, 85, 103, 103, 85, 104, 85, 84, 104, 104, 84, 101, 84, 9, 101, 99, 96, 11, 100, 105, 99, 101, 106, 100, 99, 105, 96, 105, 97, 96, 100, 106, 105, 106, 107, 105, 105, 107, 97, 107, 98, 97, 101, 83, 106, 83, 82, 106, 106, 82, 107, 82, 81, 107, 107, 81, 98, 81, 8, 98, 96, 93, 11, 97, 108, 96, 98, 109, 97, 96, 108, 93, 108, 94, 93, 97, 109, 108, 109, 110, 108, 108, 110, 94, 110, 95, 94, 98, 80, 109, 80, 79, 109, 109, 79, 110, 79, 78, 110, 110, 78, 95, 78, 7, 95, 93, 87, 11, 94, 111, 93, 95, 112, 94, 93, 111, 87, 111, 88, 87, 94, 112, 111, 112, 113, 111, 111, 113, 88, 113, 89, 88, 95, 77, 112, 77, 76, 112, 112, 76, 113, 76, 75, 113, 113, 75, 89, 75, 6, 89, 87, 92, 11, 88, 114, 87, 89, 115, 88, 87, 114, 92, 114, 91, 92, 88, 115, 114, 115, 116, 114, 114, 116, 91, 116, 90, 91, 89, 74, 115, 74, 73, 115, 115, 73, 116, 73, 72, 116, 116, 72, 90, 72, 10, 90, 47, 86, 10, 46, 117, 47, 45, 118, 46, 47, 117, 86, 117, 85, 86, 46, 118, 117, 118, 119, 117, 117, 119, 85, 119, 84, 85, 45, 68, 118, 68, 67, 118, 118, 67, 119, 67, 66, 119, 119, 66, 84, 66, 9, 84, 71, 83, 9, 70, 120, 71, 69, 121, 70, 71, 120, 83, 120, 82, 83, 70, 121, 120, 121, 122, 120, 120, 122, 82, 122, 81, 82, 69, 62, 121, 62, 61, 121, 121, 61, 122, 61, 60, 122, 122, 60, 81, 60, 8, 81, 65, 80, 8, 64, 123, 65, 63, 124, 64, 65, 123, 80, 123, 79, 80, 64, 124, 123, 124, 125, 123, 123, 125, 79, 125, 78, 79, 63, 56, 124, 56, 55, 124, 124, 55, 125, 55, 54, 125, 125, 54, 78, 54, 7, 78, 59, 77, 7, 58, 126, 59, 57, 127, 58, 59, 126, 77, 126, 76, 77, 58, 127, 126, 127, 128, 126, 126, 128, 76, 128, 75, 76, 57, 50, 127, 50, 49, 127, 127, 49, 128, 49, 48, 128, 128, 48, 75, 48, 6, 75, 53, 74, 6, 52, 129, 53, 51, 130, 52, 53, 129, 74, 129, 73, 74, 52, 130, 129, 130, 131, 129, 129, 131, 73, 131, 72, 73, 51, 44, 130, 44, 43, 130, 130, 43, 131, 43, 42, 131, 131, 42, 72, 42, 10, 72, 66, 71, 9, 67, 132, 66, 68, 133, 67, 66, 132, 71, 132, 70, 71, 67, 133, 132, 133, 134, 132, 132, 134, 70, 134, 69, 70, 68, 41, 133, 41, 40, 133, 133, 40, 134, 40, 39, 134, 134, 39, 69, 39, 4, 69, 60, 65, 8, 61, 135, 60, 62, 136, 61, 60, 135, 65, 135, 64, 65, 61, 136, 135, 136, 137, 135, 135, 137, 64, 137, 63, 64, 62, 38, 136, 38, 37, 136, 136, 37, 137, 37, 36, 137, 137, 36, 63, 36, 3, 63, 54, 59, 7, 55, 138, 54, 56, 139, 55, 54, 138, 59, 138, 58, 59, 55, 139, 138, 139, 140, 138, 138, 140, 58, 140, 57, 58, 56, 32, 139, 32, 31, 139, 139, 31, 140, 31, 30, 140, 140, 30, 57, 30, 2, 57, 48, 53, 6, 49, 141, 48, 50, 142, 49, 48, 141, 53, 141, 52, 53, 49, 142, 141, 142, 143, 141, 141, 143, 52, 143, 51, 52, 50, 20, 142, 20, 19, 142, 142, 19, 143, 19, 18, 143, 143, 18, 51, 18, 1, 51, 42, 47, 10, 43, 144, 42, 44, 145, 43, 42, 144, 47, 144, 46, 47, 43, 145, 144, 145, 146, 144, 144, 146, 46, 146, 45, 46, 44, 23, 145, 23, 22, 145, 145, 22, 146, 22, 21, 146, 146, 21, 45, 21, 5, 45, 26, 41, 5, 25, 147, 26, 24, 148, 25, 26, 147, 41, 147, 40, 41, 25, 148, 147, 148, 149, 147, 147, 149, 40, 149, 39, 40, 24, 35, 148, 35, 34, 148, 148, 34, 149, 34, 33, 149, 149, 33, 39, 33, 4, 39, 33, 38, 4, 34, 150, 33, 35, 151, 34, 33, 150, 38, 150, 37, 38, 34, 151, 150, 151, 152, 150, 150, 152, 37, 152, 36, 37, 35, 29, 151, 29, 28, 151, 151, 28, 152, 28, 27, 152, 152, 27, 36, 27, 3, 36, 27, 32, 3, 28, 153, 27, 29, 154, 28, 27, 153, 32, 153, 31, 32, 28, 154, 153, 154, 155, 153, 153, 155, 31, 155, 30, 31, 29, 14, 154, 14, 13, 154, 154, 13, 155, 13, 12, 155, 155, 12, 30, 12, 2, 30, 21, 26, 5, 22, 156, 21, 23, 157, 22, 21, 156, 26, 156, 25, 26, 22, 157, 156, 157, 158, 156, 156, 158, 25, 158, 24, 25, 23, 17, 157, 17, 16, 157, 157, 16, 158, 16, 15, 158, 158, 15, 24, 15, 0, 24, 12, 20, 2, 13, 159, 12, 14, 160, 13, 12, 159, 20, 159, 19, 20, 13, 160, 159, 160, 161, 159, 159, 161, 19, 161, 18, 19, 14, 15, 160, 15, 16, 160, 160, 16, 161, 16, 17, 161, 161, 17, 18, 17, 1, 18,
    ];

    DefaultMesh::new(&vertices, &indices)
}
