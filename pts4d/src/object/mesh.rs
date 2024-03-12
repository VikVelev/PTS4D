use cgmath::{InnerSpace, Matrix3, Point3, Transform, Vector3};
use wavefront_obj::obj::{ObjSet, Primitive, VTNIndex, Vertex};

use crate::{
    accel::aabb::AABB,
    materials::material::Material,
    utils::vector_utils::{correct_face_normal, Interval, Ray},
};

use super::object::{Hit, Hitable};

pub struct Mesh {
    pub geometry: ObjSet,
    pub material: Material,
    bbox: AABB,
}

#[inline]
fn convert_to_cgmath_vec(vertex: Vertex) -> Vector3<f32> {
    return Vector3::new(vertex.x as f32, vertex.y as f32, vertex.z as f32);
}

fn translate_and_scale_vertex_cgmath_vec(vec1: Vertex, vec2: Vector3<f32>, scale: f32) -> Vertex {
    let scale_mat: Matrix3<f32> = Matrix3 {
        x: Vector3::new(scale, 0.0, 0.0),
        y: Vector3::new(0.0, scale, 0.0),
        z: Vector3::new(0.0, 0.0, scale),
    };

    let scaled_object: Vector3<f32> = <Matrix3<f32> as Transform<Point3<f32>>>::transform_vector(
        &scale_mat,
        convert_to_cgmath_vec(vec1),
    );
    let result = scaled_object + vec2;

    return Vertex {
        x: result.x as f64,
        y: result.y as f64,
        z: result.z as f64,
    };
}

fn translate_and_scale_object(
    vertex_set: &Vec<Vertex>,
    to: Vector3<f32>,
    scale: f32,
) -> Vec<Vertex> {
    let mut translated_set: Vec<Vertex> = Vec::new();
    for point in vertex_set {
        translated_set.push(translate_and_scale_vertex_cgmath_vec(*point, to, scale));
    }

    return translated_set;
}

impl Mesh {
    pub fn new(center: Vector3<f32>, scale: f32, geometry: ObjSet) -> Mesh {
        return Mesh::new_override_material(
            center,
            scale,
            geometry,
            Material::Lambertian(Vector3::new(0.6, 0.6, 0.6)),
        );
    }

    pub fn new_override_material(
        center: Vector3<f32>,
        scale: f32,
        mut geometry: ObjSet,
        material: Material,
    ) -> Mesh {
        let mut bbox: AABB =
            AABB::new_from_diagonals(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));

        // Building a simple bounding box.
        for obj in geometry.objects.as_mut_slice() {
            obj.vertices = translate_and_scale_object(&obj.vertices, center, scale);
            for geom in &obj.geometry {
                for shape in &geom.shapes {
                    match shape.primitive {
                        // Each vertex is made out of VertexIndex, Option<TextureIndex>, Option<NormalIndex>
                        Primitive::Triangle(a, b, c) => {
                            bbox = AABB::new_from_aabbs(
                                bbox,
                                triangle_bounding_box(&(a, b, c), &obj.vertices),
                            );
                        }
                        _ => continue,
                    }
                }
            }
        }

        return Mesh {
            geometry,
            material,
            bbox,
        };
    }

    fn intersect_triangle(
        &self,
        ray: &Ray,
        triangle: &(VTNIndex, VTNIndex, VTNIndex),
        material: &Option<String>,
        vertices_cache: &Vec<Vertex>,
        bounds: Interval,
    ) -> Option<Hit> {
        let (p1, p2, p3) = triangle;

        let (vertex_index_1, _, _) = p1;
        let (vertex_index_2, _, _) = p2;
        let (vertex_index_3, _, _) = p3;

        let maybe_v1 = vertices_cache.get(*vertex_index_1);
        let maybe_v2 = vertices_cache.get(*vertex_index_2);
        let maybe_v3 = vertices_cache.get(*vertex_index_3);

        if maybe_v1.is_none() || maybe_v2.is_none() || maybe_v3.is_none() {
            panic!("Some vertices weren't assembled together into a triangle");
        }

        let a = convert_to_cgmath_vec(*maybe_v1.unwrap());
        let b = convert_to_cgmath_vec(*maybe_v2.unwrap());
        let c = convert_to_cgmath_vec(*maybe_v3.unwrap());

        let e1 = b - a;
        let e2 = c - a;

        let ray_cross_e2 = ray.direction.cross(e2);
        let det = e1.dot(ray_cross_e2);

        if det > -f32::EPSILON && det < f32::EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let inv_det = 1.0 / det;
        let s = ray.origin - a;
        let u = inv_det * s.dot(ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = s.cross(e1);
        let v = inv_det * ray.direction.dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = inv_det * e2.dot(s_cross_e1);
        if t > bounds.max || t < bounds.min {
            return None;
        }

        if t > f32::EPSILON {
            let intersection_point = ray.point_at(t);
            let normal = (e1).cross(e2).normalize();
            return Some(Hit {
                point: intersection_point,
                material: &self.material,
                normal: correct_face_normal(ray, normal),
                is_facing_you: ray.direction.dot(normal) < 0.0,
                point_at_intersection: t,
            });
        }

        return None;
    }
}

impl Hitable for Mesh {
    fn intersect(&self, ray: &Ray, bounds: Interval) -> Option<Hit> {
        let mut hit: Option<Hit> = None;
        let mut closest_so_far: f32 = bounds.max;

        for obj in &self.geometry.objects {
            for geom in &obj.geometry {
                for shape in &geom.shapes {
                    match shape.primitive {
                        // Each vertex is made out of VertexIndex, Option<TextureIndex>, Option<NormalIndex>
                        Primitive::Triangle(a, b, c) => {
                            let maybe_hit = self.intersect_triangle(
                                ray,
                                &(a, b, c),
                                &geom.material_name,
                                &obj.vertices,
                                Interval::new(bounds.min, closest_so_far),
                            );
                            if let Some(hit_point) = maybe_hit {
                                if closest_so_far > hit_point.point_at_intersection {
                                    closest_so_far = hit_point.point_at_intersection;
                                    hit = Some(hit_point);
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
        return hit;
    }

    fn bounding_box(&self) -> &AABB {
        return &self.bbox;
    }
}

fn triangle_bounding_box(
    triangle: &(VTNIndex, VTNIndex, VTNIndex),
    vertices_cache: &Vec<Vertex>,
) -> AABB {
    let (p1, p2, p3) = triangle;

    let (vertex_index_1, _, _) = p1;
    let (vertex_index_2, _, _) = p2;
    let (vertex_index_3, _, _) = p3;

    let maybe_v1 = vertices_cache.get(*vertex_index_1);
    let maybe_v2 = vertices_cache.get(*vertex_index_2);
    let maybe_v3 = vertices_cache.get(*vertex_index_3);

    if maybe_v1.is_none() || maybe_v2.is_none() || maybe_v3.is_none() {
        panic!("Some vertices weren't assembled together into a triangle");
    }

    let a = convert_to_cgmath_vec(*maybe_v1.unwrap());
    let b = convert_to_cgmath_vec(*maybe_v2.unwrap());
    let c = convert_to_cgmath_vec(*maybe_v3.unwrap());

    let aabb1 = AABB::new_from_diagonals(a, b);
    let aabb2 = AABB::new_from_diagonals(a, c);
    let final_aabb = AABB::new_from_aabbs(aabb1, aabb2);

    return final_aabb;
}
