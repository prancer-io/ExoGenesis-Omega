//! glTF Export Module
//!
//! Exports synesthesia worlds to glTF format for use in Unreal Engine, Blender, etc.

use crate::world::{SynesthesiaWorld, WorldElement, ElementType};
use crate::geometry::{MeshGenerator, ProceduralMesh};
use crate::materials::{SynMaterial, AlphaMode};
use crate::{Result, SynesthesiaError};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Configuration for glTF export
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Include binary data in single .glb file
    pub binary: bool,
    /// Export quality (0.0-1.0)
    pub quality: f32,
    /// Merge similar meshes
    pub merge_meshes: bool,
    /// Include vertex colors
    pub vertex_colors: bool,
    /// Include normals
    pub normals: bool,
    /// Include UVs
    pub uvs: bool,
    /// Maximum texture size
    pub max_texture_size: u32,
    /// Embed textures
    pub embed_textures: bool,
    /// Chunk size for LOD (0 = no chunking)
    pub chunk_lod_distance: f32,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            binary: true,
            quality: 1.0,
            merge_meshes: true,
            vertex_colors: true,
            normals: true,
            uvs: true,
            max_texture_size: 2048,
            embed_textures: true,
            chunk_lod_distance: 100.0,
        }
    }
}

/// glTF exporter for synesthesia worlds
pub struct GltfExporter {
    config: ExportConfig,
    mesh_generator: MeshGenerator,
}

impl GltfExporter {
    /// Create a new exporter with config
    pub fn new(config: ExportConfig) -> Self {
        Self {
            config,
            mesh_generator: MeshGenerator::new(),
        }
    }

    /// Export world to glTF file
    pub fn export(&self, world: &SynesthesiaWorld, path: &str) -> Result<()> {
        println!("📦 Exporting to glTF: {}", path);

        // Build glTF document
        let gltf = self.build_gltf(world)?;

        // Write to file
        let path = Path::new(path);
        if self.config.binary {
            self.write_glb(&gltf, path)?;
        } else {
            self.write_gltf(&gltf, path)?;
        }

        println!("✅ Export complete!");
        Ok(())
    }

    /// Build glTF document from world
    fn build_gltf(&self, world: &SynesthesiaWorld) -> Result<GltfDocument> {
        let mut doc = GltfDocument::new();

        // Create materials
        let mut material_cache: HashMap<String, usize> = HashMap::new();
        for chunk in &world.chunks {
            for element in &chunk.elements {
                let mat_key = self.material_key(&element.material);
                if !material_cache.contains_key(&mat_key) {
                    let mat_idx = self.add_material(&mut doc, &element.material);
                    material_cache.insert(mat_key, mat_idx);
                }
            }
        }

        // Create meshes and nodes
        let mut node_indices = Vec::new();
        for chunk in &world.chunks {
            let chunk_node = self.export_chunk(&mut doc, chunk, &material_cache)?;
            node_indices.push(chunk_node);
        }

        // Create scene
        doc.scenes.push(GltfScene {
            name: "SynesthesiaWorld".to_string(),
            nodes: node_indices,
        });

        // Add scene lighting info as extras
        self.add_lighting_info(&mut doc, world);

        Ok(doc)
    }

    /// Export a world chunk
    fn export_chunk(
        &self,
        doc: &mut GltfDocument,
        chunk: &crate::world::WorldChunk,
        material_cache: &HashMap<String, usize>,
    ) -> Result<usize> {
        let mut child_nodes = Vec::new();

        // Group elements by type for potential merging
        if self.config.merge_meshes {
            let grouped = self.group_elements(&chunk.elements);
            for (element_type, elements) in grouped {
                if let Some(merged_node) = self.export_merged_elements(doc, &elements, material_cache, element_type)? {
                    child_nodes.push(merged_node);
                }
            }
        } else {
            for element in &chunk.elements {
                let node_idx = self.export_element(doc, element, material_cache)?;
                child_nodes.push(node_idx);
            }
        }

        // Create chunk parent node
        let chunk_node = GltfNode {
            name: format!("Chunk_{}", chunk.index),
            translation: [chunk.origin.x, chunk.origin.y, chunk.origin.z],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
            mesh: None,
            children: child_nodes,
        };

        let node_idx = doc.nodes.len();
        doc.nodes.push(chunk_node);
        Ok(node_idx)
    }

    /// Group elements by type for merging
    fn group_elements<'a>(&self, elements: &'a [WorldElement]) -> HashMap<ElementType, Vec<&'a WorldElement>> {
        let mut groups: HashMap<ElementType, Vec<&'a WorldElement>> = HashMap::new();
        for element in elements {
            groups.entry(element.element_type).or_default().push(element);
        }
        groups
    }

    /// Export merged elements of same type
    fn export_merged_elements(
        &self,
        doc: &mut GltfDocument,
        elements: &[&WorldElement],
        material_cache: &HashMap<String, usize>,
        element_type: ElementType,
    ) -> Result<Option<usize>> {
        if elements.is_empty() {
            return Ok(None);
        }

        let mut merged_mesh = ProceduralMesh::new();
        let mut primary_material_idx = 0;

        for element in elements {
            let mut mesh = self.mesh_generator.generate_for_hint(element.shape, element.scale);

            // Transform vertices to world space
            for pos in &mut mesh.positions {
                let rotated = self.rotate_point(*pos, element.rotation);
                *pos = rotated + element.position;
            }

            // Transform normals
            for normal in &mut mesh.normals {
                *normal = self.rotate_point(*normal, element.rotation);
            }

            // Add vertex colors if enabled
            if self.config.vertex_colors {
                let color = [element.color[0], element.color[1], element.color[2], 1.0];
                let colors: Vec<[f32; 4]> = vec![color; mesh.positions.len()];
                mesh.colors = Some(colors);
            }

            merged_mesh.merge(&mesh);

            // Use first element's material
            if merged_mesh.vertex_count() == mesh.vertex_count() {
                let mat_key = self.material_key(&element.material);
                primary_material_idx = *material_cache.get(&mat_key).unwrap_or(&0);
            }
        }

        // Create mesh in document
        let mesh_idx = self.add_mesh(doc, &merged_mesh, primary_material_idx)?;

        // Create node
        let node = GltfNode {
            name: format!("{:?}_merged", element_type),
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
            mesh: Some(mesh_idx),
            children: Vec::new(),
        };

        let node_idx = doc.nodes.len();
        doc.nodes.push(node);
        Ok(Some(node_idx))
    }

    /// Export a single element
    fn export_element(
        &self,
        doc: &mut GltfDocument,
        element: &WorldElement,
        material_cache: &HashMap<String, usize>,
    ) -> Result<usize> {
        // Generate mesh
        let mesh = self.mesh_generator.generate_for_hint(element.shape, element.scale);

        // Get material
        let mat_key = self.material_key(&element.material);
        let material_idx = *material_cache.get(&mat_key).unwrap_or(&0);

        // Add mesh to document
        let mesh_idx = self.add_mesh(doc, &mesh, material_idx)?;

        // Create node with transform
        let (quat_x, quat_y, quat_z, quat_w) = euler_to_quaternion(
            element.rotation.x,
            element.rotation.y,
            element.rotation.z,
        );

        let node = GltfNode {
            name: element.id.clone(),
            translation: [element.position.x, element.position.y, element.position.z],
            rotation: [quat_x, quat_y, quat_z, quat_w],
            scale: [element.scale, element.scale, element.scale],
            mesh: Some(mesh_idx),
            children: Vec::new(),
        };

        let node_idx = doc.nodes.len();
        doc.nodes.push(node);
        Ok(node_idx)
    }

    /// Add mesh to document
    fn add_mesh(&self, doc: &mut GltfDocument, mesh: &ProceduralMesh, material_idx: usize) -> Result<usize> {
        // Add position accessor
        let pos_data: Vec<f32> = mesh.positions.iter()
            .flat_map(|v| [v.x, v.y, v.z])
            .collect();
        let pos_accessor = doc.add_accessor(pos_data, AccessorType::Vec3, ComponentType::Float);

        // Add normal accessor
        let normal_accessor = if self.config.normals {
            let normal_data: Vec<f32> = mesh.normals.iter()
                .flat_map(|v| [v.x, v.y, v.z])
                .collect();
            Some(doc.add_accessor(normal_data, AccessorType::Vec3, ComponentType::Float))
        } else {
            None
        };

        // Add UV accessor
        let uv_accessor = if self.config.uvs {
            let uv_data: Vec<f32> = mesh.uvs.iter()
                .flat_map(|v| [v.x, v.y])
                .collect();
            Some(doc.add_accessor(uv_data, AccessorType::Vec2, ComponentType::Float))
        } else {
            None
        };

        // Add index accessor
        let index_accessor = doc.add_accessor(
            mesh.indices.clone(),
            AccessorType::Scalar,
            ComponentType::UnsignedInt,
        );

        // Add color accessor
        let color_accessor = if self.config.vertex_colors {
            if let Some(ref colors) = mesh.colors {
                let color_data: Vec<f32> = colors.iter()
                    .flat_map(|c| [c[0], c[1], c[2], c[3]])
                    .collect();
                Some(doc.add_accessor(color_data, AccessorType::Vec4, ComponentType::Float))
            } else {
                None
            }
        } else {
            None
        };

        // Create primitive
        let primitive = GltfPrimitive {
            attributes: PrimitiveAttributes {
                position: pos_accessor,
                normal: normal_accessor,
                texcoord_0: uv_accessor,
                color_0: color_accessor,
            },
            indices: index_accessor,
            material: material_idx,
            mode: PrimitiveMode::Triangles,
        };

        // Create mesh
        let gltf_mesh = GltfMesh {
            name: format!("mesh_{}", doc.meshes.len()),
            primitives: vec![primitive],
        };

        let mesh_idx = doc.meshes.len();
        doc.meshes.push(gltf_mesh);
        Ok(mesh_idx)
    }

    /// Add material to document
    fn add_material(&self, doc: &mut GltfDocument, material: &crate::mapping::MaterialHint) -> usize {
        let syn_mat = SynMaterial::from_hint(material.texture, material.color);
        let pbr = syn_mat.to_pbr_params();

        let gltf_mat = GltfMaterial {
            name: syn_mat.name.clone(),
            pbr_metallic_roughness: PbrMetallicRoughness {
                base_color_factor: pbr.base_color_factor,
                metallic_factor: pbr.metallic_factor,
                roughness_factor: pbr.roughness_factor,
            },
            emissive_factor: pbr.emissive_factor,
            alpha_mode: match pbr.alpha_mode {
                AlphaMode::Opaque => "OPAQUE".to_string(),
                AlphaMode::Mask => "MASK".to_string(),
                AlphaMode::Blend => "BLEND".to_string(),
            },
            alpha_cutoff: pbr.alpha_cutoff,
            double_sided: pbr.double_sided,
        };

        let mat_idx = doc.materials.len();
        doc.materials.push(gltf_mat);
        mat_idx
    }

    /// Generate unique key for material
    fn material_key(&self, material: &crate::mapping::MaterialHint) -> String {
        format!(
            "{:.2}_{:.2}_{:.2}_{:.2}_{:.2}_{:?}",
            material.color[0], material.color[1], material.color[2],
            material.metallic, material.roughness, material.texture
        )
    }

    /// Rotate point by euler angles
    fn rotate_point(&self, point: glam::Vec3, rotation: glam::Vec3) -> glam::Vec3 {
        let rot_mat = glam::Mat3::from_euler(
            glam::EulerRot::XYZ,
            rotation.x,
            rotation.y,
            rotation.z,
        );
        rot_mat * point
    }

    /// Add lighting info as scene extras
    fn add_lighting_info(&self, doc: &mut GltfDocument, world: &SynesthesiaWorld) {
        doc.extras = Some(GltfExtras {
            synesthesia: SynesthesiaExtras {
                lighting: LightingExtras {
                    ambient_color: world.lighting.ambient_color,
                    ambient_intensity: world.lighting.ambient_intensity,
                    sun_direction: [
                        world.lighting.sun_direction.x,
                        world.lighting.sun_direction.y,
                        world.lighting.sun_direction.z,
                    ],
                    sun_color: world.lighting.sun_color,
                    sun_intensity: world.lighting.sun_intensity,
                    fog_enabled: world.lighting.fog_enabled,
                    fog_color: world.lighting.fog_color,
                    fog_density: world.lighting.fog_density,
                },
                atmosphere: AtmosphereExtras {
                    sky_color_top: world.atmosphere.sky_color_top,
                    sky_color_horizon: world.atmosphere.sky_color_horizon,
                    cloud_coverage: world.atmosphere.cloud_coverage,
                },
            },
        });
    }

    /// Write glTF as JSON
    fn write_gltf(&self, doc: &GltfDocument, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(&doc.to_json())
            .map_err(|e| SynesthesiaError::ExportError(e.to_string()))?;

        let mut file = File::create(path)
            .map_err(|e| SynesthesiaError::ExportError(e.to_string()))?;

        file.write_all(json.as_bytes())
            .map_err(|e| SynesthesiaError::ExportError(e.to_string()))?;

        // Write binary buffer
        let bin_path = path.with_extension("bin");
        let mut bin_file = File::create(&bin_path)
            .map_err(|e| SynesthesiaError::ExportError(e.to_string()))?;

        bin_file.write_all(&doc.buffer_data)
            .map_err(|e| SynesthesiaError::ExportError(e.to_string()))?;

        Ok(())
    }

    /// Write glTF as binary GLB
    fn write_glb(&self, doc: &GltfDocument, path: &Path) -> Result<()> {
        let json = serde_json::to_string(&doc.to_json())
            .map_err(|e| SynesthesiaError::ExportError(e.to_string()))?;

        // Pad JSON to 4-byte boundary
        let json_bytes = json.as_bytes();
        let json_padding = (4 - (json_bytes.len() % 4)) % 4;
        let json_length = json_bytes.len() + json_padding;

        // Pad binary to 4-byte boundary
        let bin_padding = (4 - (doc.buffer_data.len() % 4)) % 4;
        let bin_length = doc.buffer_data.len() + bin_padding;

        // GLB header
        let total_length = 12 + 8 + json_length + 8 + bin_length;

        let mut file = File::create(path)
            .map_err(|e| SynesthesiaError::ExportError(e.to_string()))?;

        // Write header
        file.write_all(&0x46546C67u32.to_le_bytes())?; // magic: "glTF"
        file.write_all(&2u32.to_le_bytes())?;          // version: 2
        file.write_all(&(total_length as u32).to_le_bytes())?;

        // Write JSON chunk
        file.write_all(&(json_length as u32).to_le_bytes())?;
        file.write_all(&0x4E4F534Au32.to_le_bytes())?; // type: "JSON"
        file.write_all(json_bytes)?;
        file.write_all(&vec![0x20u8; json_padding])?;

        // Write binary chunk
        file.write_all(&(bin_length as u32).to_le_bytes())?;
        file.write_all(&0x004E4942u32.to_le_bytes())?; // type: "BIN\0"
        file.write_all(&doc.buffer_data)?;
        file.write_all(&vec![0u8; bin_padding])?;

        Ok(())
    }
}

impl Default for GltfExporter {
    fn default() -> Self {
        Self::new(ExportConfig::default())
    }
}

/// Convert euler angles to quaternion
fn euler_to_quaternion(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32) {
    let (sx, cx) = (x * 0.5).sin_cos();
    let (sy, cy) = (y * 0.5).sin_cos();
    let (sz, cz) = (z * 0.5).sin_cos();

    let qx = sx * cy * cz - cx * sy * sz;
    let qy = cx * sy * cz + sx * cy * sz;
    let qz = cx * cy * sz - sx * sy * cz;
    let qw = cx * cy * cz + sx * sy * sz;

    (qx, qy, qz, qw)
}

// ============ glTF Document Structures ============

/// glTF document builder
#[derive(Debug)]
struct GltfDocument {
    scenes: Vec<GltfScene>,
    nodes: Vec<GltfNode>,
    meshes: Vec<GltfMesh>,
    materials: Vec<GltfMaterial>,
    accessors: Vec<GltfAccessor>,
    buffer_views: Vec<GltfBufferView>,
    buffer_data: Vec<u8>,
    extras: Option<GltfExtras>,
}

impl GltfDocument {
    fn new() -> Self {
        Self {
            scenes: Vec::new(),
            nodes: Vec::new(),
            meshes: Vec::new(),
            materials: Vec::new(),
            accessors: Vec::new(),
            buffer_views: Vec::new(),
            buffer_data: Vec::new(),
            extras: None,
        }
    }

    fn add_accessor<T: ToBytes>(&mut self, data: Vec<T>, accessor_type: AccessorType, component_type: ComponentType) -> usize {
        let byte_offset = self.buffer_data.len();
        let count = match accessor_type {
            AccessorType::Scalar => data.len(),
            AccessorType::Vec2 => data.len() / 2,
            AccessorType::Vec3 => data.len() / 3,
            AccessorType::Vec4 => data.len() / 4,
        };

        // Write data to buffer
        for item in &data {
            self.buffer_data.extend(item.to_bytes());
        }

        let byte_length = self.buffer_data.len() - byte_offset;

        // Create buffer view
        let buffer_view = GltfBufferView {
            buffer: 0,
            byte_offset,
            byte_length,
            target: None,
        };
        let buffer_view_idx = self.buffer_views.len();
        self.buffer_views.push(buffer_view);

        // Create accessor
        let accessor = GltfAccessor {
            buffer_view: buffer_view_idx,
            byte_offset: 0,
            component_type,
            count,
            accessor_type,
            min: None,
            max: None,
        };
        let accessor_idx = self.accessors.len();
        self.accessors.push(accessor);

        accessor_idx
    }

    fn to_json(&self) -> serde_json::Value {
        let mut json = serde_json::json!({
            "asset": {
                "version": "2.0",
                "generator": "Omega Synesthesia"
            },
            "scene": 0,
            "scenes": self.scenes.iter().map(|s| s.to_json()).collect::<Vec<_>>(),
            "nodes": self.nodes.iter().map(|n| n.to_json()).collect::<Vec<_>>(),
            "meshes": self.meshes.iter().map(|m| m.to_json()).collect::<Vec<_>>(),
            "materials": self.materials.iter().map(|m| m.to_json()).collect::<Vec<_>>(),
            "accessors": self.accessors.iter().map(|a| a.to_json()).collect::<Vec<_>>(),
            "bufferViews": self.buffer_views.iter().map(|v| v.to_json()).collect::<Vec<_>>(),
            "buffers": [{
                "byteLength": self.buffer_data.len()
            }]
        });

        if let Some(ref extras) = self.extras {
            json["extras"] = extras.to_json();
        }

        json
    }
}

#[derive(Debug)]
struct GltfScene {
    name: String,
    nodes: Vec<usize>,
}

impl GltfScene {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "nodes": self.nodes
        })
    }
}

#[derive(Debug)]
struct GltfNode {
    name: String,
    translation: [f32; 3],
    rotation: [f32; 4],
    scale: [f32; 3],
    mesh: Option<usize>,
    children: Vec<usize>,
}

impl GltfNode {
    fn to_json(&self) -> serde_json::Value {
        let mut json = serde_json::json!({
            "name": self.name,
            "translation": self.translation,
            "rotation": self.rotation,
            "scale": self.scale
        });

        if let Some(mesh) = self.mesh {
            json["mesh"] = serde_json::json!(mesh);
        }

        if !self.children.is_empty() {
            json["children"] = serde_json::json!(self.children);
        }

        json
    }
}

#[derive(Debug)]
struct GltfMesh {
    name: String,
    primitives: Vec<GltfPrimitive>,
}

impl GltfMesh {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "primitives": self.primitives.iter().map(|p| p.to_json()).collect::<Vec<_>>()
        })
    }
}

#[derive(Debug)]
struct GltfPrimitive {
    attributes: PrimitiveAttributes,
    indices: usize,
    material: usize,
    mode: PrimitiveMode,
}

impl GltfPrimitive {
    fn to_json(&self) -> serde_json::Value {
        let mut attrs = serde_json::json!({
            "POSITION": self.attributes.position
        });

        if let Some(normal) = self.attributes.normal {
            attrs["NORMAL"] = serde_json::json!(normal);
        }
        if let Some(uv) = self.attributes.texcoord_0 {
            attrs["TEXCOORD_0"] = serde_json::json!(uv);
        }
        if let Some(color) = self.attributes.color_0 {
            attrs["COLOR_0"] = serde_json::json!(color);
        }

        serde_json::json!({
            "attributes": attrs,
            "indices": self.indices,
            "material": self.material,
            "mode": self.mode as u8
        })
    }
}

#[derive(Debug)]
struct PrimitiveAttributes {
    position: usize,
    normal: Option<usize>,
    texcoord_0: Option<usize>,
    color_0: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum PrimitiveMode {
    Triangles = 4,
}

#[derive(Debug)]
struct GltfMaterial {
    name: String,
    pbr_metallic_roughness: PbrMetallicRoughness,
    emissive_factor: [f32; 3],
    alpha_mode: String,
    alpha_cutoff: f32,
    double_sided: bool,
}

impl GltfMaterial {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "pbrMetallicRoughness": {
                "baseColorFactor": self.pbr_metallic_roughness.base_color_factor,
                "metallicFactor": self.pbr_metallic_roughness.metallic_factor,
                "roughnessFactor": self.pbr_metallic_roughness.roughness_factor
            },
            "emissiveFactor": self.emissive_factor,
            "alphaMode": self.alpha_mode,
            "alphaCutoff": self.alpha_cutoff,
            "doubleSided": self.double_sided
        })
    }
}

#[derive(Debug)]
struct PbrMetallicRoughness {
    base_color_factor: [f32; 4],
    metallic_factor: f32,
    roughness_factor: f32,
}

#[derive(Debug)]
struct GltfAccessor {
    buffer_view: usize,
    byte_offset: usize,
    component_type: ComponentType,
    count: usize,
    accessor_type: AccessorType,
    #[allow(dead_code)]
    min: Option<Vec<f32>>,
    #[allow(dead_code)]
    max: Option<Vec<f32>>,
}

impl GltfAccessor {
    fn to_json(&self) -> serde_json::Value {
        let type_str = match self.accessor_type {
            AccessorType::Scalar => "SCALAR",
            AccessorType::Vec2 => "VEC2",
            AccessorType::Vec3 => "VEC3",
            AccessorType::Vec4 => "VEC4",
        };

        serde_json::json!({
            "bufferView": self.buffer_view,
            "byteOffset": self.byte_offset,
            "componentType": self.component_type as u32,
            "count": self.count,
            "type": type_str
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum AccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
enum ComponentType {
    Float = 5126,
    UnsignedInt = 5125,
}

#[derive(Debug)]
struct GltfBufferView {
    buffer: usize,
    byte_offset: usize,
    byte_length: usize,
    target: Option<u32>,
}

impl GltfBufferView {
    fn to_json(&self) -> serde_json::Value {
        let mut json = serde_json::json!({
            "buffer": self.buffer,
            "byteOffset": self.byte_offset,
            "byteLength": self.byte_length
        });

        if let Some(target) = self.target {
            json["target"] = serde_json::json!(target);
        }

        json
    }
}

#[derive(Debug)]
struct GltfExtras {
    synesthesia: SynesthesiaExtras,
}

impl GltfExtras {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "synesthesia": {
                "lighting": self.synesthesia.lighting,
                "atmosphere": self.synesthesia.atmosphere
            }
        })
    }
}

#[derive(Debug, serde::Serialize)]
struct SynesthesiaExtras {
    lighting: LightingExtras,
    atmosphere: AtmosphereExtras,
}

#[derive(Debug, serde::Serialize)]
struct LightingExtras {
    ambient_color: [f32; 3],
    ambient_intensity: f32,
    sun_direction: [f32; 3],
    sun_color: [f32; 3],
    sun_intensity: f32,
    fog_enabled: bool,
    fog_color: [f32; 3],
    fog_density: f32,
}

#[derive(Debug, serde::Serialize)]
struct AtmosphereExtras {
    sky_color_top: [f32; 3],
    sky_color_horizon: [f32; 3],
    cloud_coverage: f32,
}

/// Trait for converting types to bytes
trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for f32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ToBytes for u32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exporter_creation() {
        let exporter = GltfExporter::new(ExportConfig::default());
        assert!(exporter.config.binary);
    }

    #[test]
    fn test_euler_to_quaternion() {
        let (x, y, z, w) = euler_to_quaternion(0.0, 0.0, 0.0);
        assert!((w - 1.0).abs() < 0.001);
        assert!(x.abs() < 0.001);
        assert!(y.abs() < 0.001);
        assert!(z.abs() < 0.001);
    }

    #[test]
    fn test_config_default() {
        let config = ExportConfig::default();
        assert_eq!(config.quality, 1.0);
        assert!(config.merge_meshes);
    }
}
