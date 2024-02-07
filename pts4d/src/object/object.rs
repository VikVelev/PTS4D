

trait Hitable {
    fn hit() -> bool;
}

impl Hitable for Object {
    fn hit() -> bool {
        return true;
    }
}

pub struct Object {}