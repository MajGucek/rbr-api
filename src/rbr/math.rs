#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Matrix(pub [[f32; 4]; 4]);
impl Matrix {
    pub(crate) const fn from_raw(
        values: [[f32; 4]; 4],
    ) -> Self {
        Self(values)
    }
}
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}