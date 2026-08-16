use crate::raw::types::{D3DMatrix, D3DXQuaternion, D3DXVector3};

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



/*
 * Raw -> public
 */
impl From<D3DMatrix> for Matrix {
    fn from(value: D3DMatrix) -> Self {
        Self(value.m)
    }
}

impl From<D3DXVector3> for Vector3 {
    fn from(value: D3DXVector3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<D3DXQuaternion> for Quaternion {
    fn from(value: D3DXQuaternion) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}



/*
 * Public -> raw
 */
impl From<Matrix> for D3DMatrix {
    fn from(value: Matrix) -> Self {
        Self {
            m: value.0,
        }
    }
}

impl From<Vector3> for D3DXVector3 {
    fn from(value: Vector3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Quaternion> for D3DXQuaternion {
    fn from(value: Quaternion) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}