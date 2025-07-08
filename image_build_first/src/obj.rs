use std::path::Path;
use std::sync::Arc;
use obj::{Obj};

use crate::triangle::Triangle;
use crate::Vec3;
use crate::material::Material;
use crate::rtweekend::random_double;

/// 顶点结构体
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 三角形面结构体
#[derive(Debug)]
pub struct Face {
    pub v0: usize,
    pub v1: usize,
    pub v2: usize,
}

/// 从 .obj 文件加载顶点和面
pub fn load_obj_vertices_faces(file_path: &str) -> Result<(Vec<Vertex>, Vec<Face>), Box<dyn std::error::Error>> {
    let obj = Obj::load(Path::new(file_path))?;

    // 顶点数组转换
    let vertices = obj.data.position.iter().map(|p| Vertex {
        x: p[0] as f64,
        y: p[1] as f64,
        z: p[2] as f64,
    }).collect::<Vec<_>>();

    let mut faces = Vec::new();

    for object in &obj.data.objects {
        for group in &object.groups {
            for poly in &group.polys {
                if poly.0.len() == 3 {
                    // 每个 poly 是一个多边形的 Vec<IndexTuple>
                    let v0 = poly.0[0].0;
                    let v1 = poly.0[1].0;
                    let v2 = poly.0[2].0;

                    faces.push(Face {
                        v0: v0 as usize,
                        v1: v1 as usize,
                        v2: v2 as usize,
                    });
                }
                // 如果是四边形或更多边形你也可以选择做额外处理
            }
        }
    }

    Ok((vertices, faces))
}

/// 构建三角形列表
pub fn build_triangles(vertices: Vec<Vertex>, faces: Vec<Face>, material: Arc<dyn Material>) -> Vec<Triangle> {
    let mut triangles = Vec::new();

    for face in faces {
        let v0 = vertices[face.v0];
        let v1 = vertices[face.v1];
        let v2 = vertices[face.v2];

        let p0 = Vec3::new(v0.x, v0.y + random_double(), v0.z);
        let p1 = Vec3::new(v1.x, v1.y + random_double(), v1.z);
        let p2 = Vec3::new(v2.x, v2.y + random_double(), v2.z);

        triangles.push(Triangle::new(p0, p1, p2, material.clone()));
    }

    triangles
}
