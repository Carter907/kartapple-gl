use std::collections::HashMap;

//
// INT(1),
// BOOL(1),
// FLOAT(1),
// VEC2(2),
// VEC3(3),
// VEC4(4)
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub enum GLvartype {
    Int,
    Bool,
    Float,
    Vec2,
    Vec3,
    Vec4
}
impl GLvartype {

    pub fn get_type_size(kind: GLvartype) -> i32 {

        let gl_type_to_size: HashMap<GLvartype, i32> = HashMap::from(
            [
                (GLvartype::Bool, 1),
                (GLvartype::Int, 1),
                (GLvartype::Float, 1),
                (GLvartype::Vec2, 2),
                (GLvartype::Vec3, 3),
                (GLvartype::Vec4, 4)
            ]
        );

        *gl_type_to_size.get(&kind).unwrap()
    }
}