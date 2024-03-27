use std::fs;

use cgmath::Vector3;
use wavefront_obj::obj::ObjSet;

use crate::materials::material::{Material, MaterialSet};
use crate::object::mesh::Mesh;
use crate::object::sphere::Sphere;
use crate::scene::camera::Camera;
use crate::scene::scene::Scene;
use crate::scene::screen::{HEIGHT, WIDTH};

// Loads an obj file into memory and parses it into an ObjSet
pub fn load_and_parse_obj(path: &str) -> (ObjSet, MaterialSet) {
    let obj_string = fs::read_to_string(path);
    if obj_string.is_err() {
        panic!("There was an error opening and reading '{}'", path);
    }

    let loaded_obj = wavefront_obj::obj::parse(obj_string.unwrap());
    if loaded_obj.is_err() {
        panic!("There was an error parsing '{}'", path);
    }

    let mtl_string = fs::read_to_string(path.replace(".obj", ".mtl"));
    if mtl_string.is_err() {
        panic!(
            "There was an error opening and reading '{}'",
            path.replace(".obj", ".mtl")
        );
    }

    let loaded_mtl = wavefront_obj::mtl::parse(mtl_string.unwrap());
    let mut material_set = MaterialSet::new();

    for mat in loaded_mtl.unwrap().materials {
        material_set.add(mat.name.to_string(), Material::WavefrontObjMaterial(mat));
    }

    return (loaded_obj.unwrap(), material_set);
}

#[allow(dead_code)]
// Creates a scene including complex polygon models.
pub fn generate_polygon_scene(path: &str) -> Scene {
    let (mesh, _mesh_materials) = load_and_parse_obj(path);
    let look_from = Vector3::new(5.0, 2.0, 5.0);
    let look_at = Vector3::new(0.0, 3.0, 0.0);
    let up = Vector3::new(0.0, -1.0, 0.0); // TODO: WTF?
    let camera: Camera = Camera::new(HEIGHT as f32, WIDTH as f32, 60.0, look_from, look_at, up);

    let loaded_mesh = Mesh::new_override_material(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        mesh.clone(),
        Material::Diffuse(Vector3::new(0.8, 0.6, 0.7)),
    );

    let sphere1 = Sphere::new(Vector3::new(5.0, 1.0, 0.0), 2.0, Material::Dielectric(2.0));

    let sphere2 = Sphere::new(Vector3::new(5.0, 1.0, 0.0), 1.8, Material::Dielectric(2.0));

    let ground_sphere = Sphere::new(
        Vector3::new(0.0, -501.0, 0.0),
        500.0,
        Material::Diffuse(Vector3::new(0.8, 0.6, 0.7)),
    );

    return Scene::build_complex_scene(
        vec![loaded_mesh],
        vec![ground_sphere, sphere1, sphere2],
        camera,
    );
}

pub fn generate_cornell_box_scene() -> Scene {
    let look_from = Vector3::new(-0.2, 3.5, 4.2);
    let look_at = Vector3::new(-0.2, 3.5, 0.5);
    let up = Vector3::new(0.0, -1.0, 0.0); // TODO: WTF?
    let camera: Camera = Camera::new(HEIGHT as f32, WIDTH as f32, 60.0, look_from, look_at, up);

    let (mesh, mesh_materials) = load_and_parse_obj("./objs/benchmark/cornell-box.obj");

    let loaded_mesh = Mesh::new_override_material_set(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        mesh.clone(),
        mesh_materials,
    );

    let green_light_sphere = Sphere::new(
        Vector3::new(-1.8, 5.5, -3.5),
        0.35,
        Material::Emissive(Vector3::new(0.2, 1.0, 0.2), 15.0),
    );

    let red_light_sphere = Sphere::new(
        Vector3::new(1.3, 5.5, -3.5),
        0.35,
        Material::Emissive(Vector3::new(1.0, 0.2, 0.2), 15.0),
    );

    let dielectric_sphere = Sphere::new(
        Vector3::new(-1.5, 2.2, -1.5),
        1.0,
        Material::Dielectric(1.4),
    );

    let metal_sphere = Sphere::new(
        Vector3::new(1.0, 3.7, -3.0),
        1.0,
        Material::Metallic(Vector3::new(1.0, 1.0, 1.0), 0.0),
    );

    return Scene::build_complex_scene(
        vec![loaded_mesh],
        vec![
            metal_sphere,
            dielectric_sphere,
            green_light_sphere,
            red_light_sphere,
        ],
        camera,
    );
}

pub fn _generate_sphere_scene() -> Scene {
    let look_from = Vector3::new(0.0, 5.0, 30.0);
    let look_at = Vector3::new(0.0, 5.0, 0.0);
    let up = Vector3::new(0.0, -1.0, 0.0); // TODO: WTF?
    let _camera: Camera = Camera::new(HEIGHT as f32, WIDTH as f32, 40.0, look_from, look_at, up);

    let _ground_sphere = Sphere::new(
        Vector3::new(0.0, 5.0, 0.0),
        5.0,
        Material::Diffuse(Vector3::new(0.999, 0.0, 0.0)),
    );

    let _main_sphere = Sphere::new(
        Vector3::new(0.0, -500.0, 0.0),
        500.0,
        Material::Diffuse(Vector3::new(0.9, 0.9, 0.1)),
    );

    todo!();
    // return Scene::_build_sphere_scene(vec![main_sphere, ground_sphere], camera);
}
