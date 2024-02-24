use crate::accel::aabb::{self, AABB};
use crate::materials::material::{self, Reflective};
use crate::utils::vector_utils::{correct_face_normal, Hit, Interval, Ray};

use cgmath::{dot, InnerSpace, Vector3};
use wavefront_obj::obj::{ObjSet, Primitive, VTNIndex, Vertex};

pub trait Hitable {
    type Material: Reflective;
    // A function checking if the object implementing this trait
    // can be intersected by a specific ray, given some bounds
    //
    // Returns an Intersect object, which contains all necessary information to bounce / render.
    // Should return None if there is no intersection
    fn intersect(&self, ray: &Ray, bounds: Interval) -> Option<Hit<Self::Material>>;
    fn bounding_box(&self) -> &AABB;
}

pub struct Sphere<T: Reflective> {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: T,
    bbox: AABB,
}

impl<T: Reflective> Sphere<T> {
    fn compute_bounding_box(center: Vector3<f32>, radius: f32) -> AABB {
        let radius_vec = Vector3::new(radius, radius, radius);
        return AABB::new_from_diagonals(center - radius_vec, center + radius_vec);
    }

    pub fn new(center: Vector3<f32>, radius: f32, material: T) -> Sphere<T> {
        Sphere {
            center,
            radius,
            material,
            bbox: Sphere::<T>::compute_bounding_box(center, radius),
        }
    }
}

impl<Mat: Reflective> Hitable for Sphere<Mat> {
    fn intersect(&self, ray: &Ray, bounds: Interval) -> Option<Hit<Mat>> {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction); // || ray.direction ||^2
        let b = 2.0 * dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let x1 = (-b - discriminant.sqrt()) / (2.0 * a);
            if x1 < bounds.max && x1 > bounds.min {
                return Some(Hit {
                    point_at_intersection: x1,
                    point: ray.point_at(x1),
                    normal: correct_face_normal(
                        ray,
                        (ray.point_at(x1) - self.center) / self.radius,
                    ),
                    material: &self.material,
                });
            }

            let x2 = (-b + discriminant.sqrt()) / (2.0 * a);
            if x2 < bounds.max && x2 > bounds.min {
                return Some(Hit {
                    point_at_intersection: x2,
                    point: ray.point_at(x2),
                    normal: correct_face_normal(
                        ray,
                        (ray.point_at(x2) - self.center) / self.radius,
                    ),
                    material: &self.material,
                });
            }
        }

        return None;
    }

    type Material = Mat;

    fn bounding_box(&self) -> &AABB {
        return &self.bbox;
    }
}

pub struct Mesh<M: Reflective> {
    pub geometry: ObjSet,
    pub material: M,
    bbox: AABB,
}

fn convert_to_cgmath_vec(vertex: Vertex) -> Vector3<f32> {
    return Vector3::new(vertex.x as f32, vertex.y as f32, vertex.z as f32);
}

impl<M: Reflective> Mesh<M> {
    pub fn new(geometry: ObjSet, material: M) -> Mesh<M> {
        let mut bbox: AABB =
            AABB::new_from_diagonals(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));

        // Building a simple bounding box.
        for obj in &geometry.objects {
            for geom in &obj.geometry {
                for shape in &geom.shapes {
                    match shape.primitive {
                        // Each vertex is made out of VertexIndex, Option<TextureIndex>, Option<NormalIndex>
                        Primitive::Triangle(a, b, c) => {
                            bbox = AABB::new_from_aabbs(bbox, triangle_bounding_box(&(a, b, c), &obj.vertices));
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
        vertices_cache: &Vec<Vertex>,
        bounds: Interval,
    ) -> Option<Hit<M>> {
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
            let intersection_point = ray.origin + ray.direction * t;

            return Some(Hit {
                point: intersection_point,
                material: &self.material,
                normal: correct_face_normal(ray, (e1).cross(e2).normalize()),
                point_at_intersection: t,
            });
        }

        return None;
    }
}

impl<M: Reflective + 'static> Hitable for Mesh<M> {
    type Material = M;

    fn intersect(&self, ray: &Ray, bounds: Interval) -> Option<Hit<Self::Material>> {
        let mut hit: Option<Hit<Self::Material>> = None;
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
