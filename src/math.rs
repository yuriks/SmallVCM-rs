use std::num::{Float, FloatMath};
use std::fmt::{Show, Formatter};
use std::fmt::Error as FormatError;

trait Num :
    Add<Self, Self> +
    Sub<Self, Self> +
    Mul<Self, Self> +
    Div<Self, Self> +
    Neg<Self> +
    PartialOrd {
}

macro_rules! impl_Num(
    ($Self:ident) => (
        impl Num for $Self {
        }
    )
)

impl_Num!(i32)
impl_Num!(f32)

pub const PI : f32 = 3.14159265358979;
pub const INV_PI : f32 = 1.0 / PI;

pub fn sqr<T: Mul<T, T>>(a: T) -> T {
    a * a
}

macro_rules! impl_Vector_op(
    ($Trait:ident for $Self:ident { $($field:ident),+ }, $func:ident) => (
        impl<T: Num> $Trait<$Self<T>, $Self<T>> for $Self<T> {
            #[inline]
            fn $func(&self, o: &$Self<T>) -> $Self<T> {
                // TODO: Workaround for Rust bug #18775
                let me = self;
                $Self {
                    $($field: me.$field.$func(&o.$field)),+
                }
            }
        }
    )
)

macro_rules! impl_Vector_traits(
    ($Self:ident { $($field:ident),+ }) => (
        impl_Vector_op!(Add for $Self { $($field),+ }, add)
        impl_Vector_op!(Sub for $Self { $($field),+ }, sub)
        impl_Vector_op!(Mul for $Self { $($field),+ }, mul)
        impl_Vector_op!(Div for $Self { $($field),+ }, div)

        impl<T: Num> Neg<$Self<T>> for $Self<T> {
            #[inline]
            fn neg(&self) -> $Self<T> {
                $Self {
                    $($field: -self.$field),+
                }
            }
        }

        impl<T: Num> Index<uint, T> for $Self<T> {
            #[inline]
            fn index(&self, index: &uint) -> &T {
                [$(&self.$field),+][*index]
            }
        }

        impl<T: Num> IndexMut<uint, T> for $Self<T> {
            #[inline]
            fn index_mut(&mut self, index: &uint) -> &mut T {
                [$(&mut self.$field),+][*index]
            }
        }
    )
)

#[deriving(Copy, Clone)]
pub struct Vector2<T> { pub x: T, pub y: T }
#[deriving(Copy, Clone)]
struct Vector3<T> { pub x: T, pub y: T, pub z: T }

impl_Vector_traits!(Vector2 { x, y })
impl_Vector_traits!(Vector3 { x, y, z })

pub type Vec2f = Vector2<f32>;
pub type Vec2i = Vector2<i32>;
pub fn vec2<T: Num + Copy>(x: T, y: T) -> Vector2<T> { Vector2::new(x, y) }
pub fn vec2s<T: Num + Copy>(a: T) -> Vector2<T> { Vector2::spread(a) }
pub type Vec3f = Vector3<f32>;
pub type Vec3i = Vector3<i32>;
pub fn vec3<T: Num + Copy>(x: T, y: T, z: T) -> Vector3<T> { Vector3::new(x, y, z) }
pub fn vec3s<T: Num + Copy>(a: T) -> Vector3<T> { Vector3::spread(a) }

impl<T: Num + Copy> Vector2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x: x, y: y }
    }

    #[inline]
    /// Creates a vector with the given value in all coordinates.
    fn spread(a: T) -> Vector2<T> {
        Vector2 { x: a, y: a }
    }

    #[inline]
    pub fn dot(&self, o: Vector2<T>) -> T {
        (self.x * o.x) + (self.y * o.y)
    }
}

#[allow(unused_must_use)]
impl<T: Show> Show for Vector2<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
        formatter.write_str("(");
        try!(self.x.fmt(formatter));
        formatter.write_str(", ");
        try!(self.y.fmt(formatter));
        formatter.write_str(")");
        Ok(())
    }
}

impl<T: Num + Copy> Vector3<T> {
    #[inline]
    fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x: x, y: y, z: z }
    }

    #[inline]
    /// Creates a vector with the given value in all coordinates.
    fn spread(a: T) -> Vector3<T> {
        Vector3 { x: a, y: a, z: a }
    }

    #[inline]
    fn get_xy(&self) -> Vector2<T> {
        Vector2 { x: self.x, y: self.y }
    }

    #[inline]
    fn max(&self) -> T {
        let max_xy = if self.x < self.y { self.y } else { self.x };
        if max_xy < self.z { self.z } else { max_xy }
    }

    #[inline]
    pub fn dot(&self, o: Vector3<T>) -> T {
        (self.x * o.x) + (self.y * o.y) + (self.z * o.z)
    }

    #[inline]
    pub fn length_sqr(self) -> T {
        self.dot(self)
    }

    #[inline]
    pub fn cross(&self, o: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }
}

impl<T: Num + Float> Vector3<T> {
    #[inline]
    pub fn length(&self) -> T {
        self.length_sqr().sqrt()
    }

    #[inline]
    pub fn normalized(&self) -> Vector3<T> {
        let len_sqr = self.dot(*self);
        let len = len_sqr.sqrt();
        *self / Vector3::spread(len)
    }
}

#[allow(unused_must_use)]
impl<T: Show> Show for Vector3<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
        formatter.write_str("(");
        try!(self.x.fmt(formatter));
        formatter.write_str(", ");
        try!(self.y.fmt(formatter));
        formatter.write_str(", ");
        try!(self.z.fmt(formatter));
        formatter.write_str(")");
        Ok(())
    }
}

pub struct Mat4f {
    // stored row major: data[row][column]
    data: [[f32, ..4], ..4]
}

impl Mat4f {
    #[inline]
    fn spread(a: f32) -> Mat4f {
        let row = [a, a, a, a];
        Mat4f {
            data: [row, row, row, row]
        }
    }

    #[inline]
    fn set_row(&mut self, row: uint, x: f32, y: f32, z: f32, w: f32) {
        self.data[row] = [x, y, z, w];
    }

    #[inline]
    pub fn set_row_vec3(&mut self, row: uint, xyz: Vec3f, w: f32) {
        self.data[row] = [xyz.x, xyz.y, xyz.z, w];
    }

    fn transform_vector(&self, vec: &Vec3f) -> Vec3f {
        let mut res : Vec3f = vec3s(0.0);
        for r in range(0, 3) {
            for c in range(0, 3) {
                res[r] += vec[c] * self[(r, c)]
            }
        }
        res
    }

    pub fn transform_point(&self, vec: &Vec3f) -> Vec3f {
        let mut w = self[(3, 3)];
        for c in range(0, 3) {
            w += self[(3, c)] * vec[c];
        }

        let inv_w = 1.0 / w;

        let mut res : Vec3f = vec3s(0.0);
        for r in range(0, 3) {
            res[r] = self[(r, 3)];
            for c in range(0, 3) {
                res[r] += vec[c] * self[(r, c)];
            }
            res[r] *= inv_w;
        }
        res
    }

    #[inline]
    fn zero() -> Mat4f {
        Mat4f::spread(0.0)
    }

    #[inline]
    pub fn identity() -> Mat4f {
        let mut res = Mat4f::zero();
        for i in range(0, 4) {
            res[(i, i)] = 1.0;
        }
        res
    }

    pub fn scale(scale: &Vec3f) -> Mat4f {
        let mut res = Mat4f::identity();
        for i in range(0, 3) {
            res[(i, i)] = scale[i];
        }
        res[(3, 3)] = 1.0;
        res
    }

    pub fn translate(translation: &Vec3f) -> Mat4f {
        let mut res = Mat4f::identity();
        for i in range(0, 3) {
            res[(i, 3)] = translation[i];
        }
        res
    }

    pub fn perspective(fov: f32, near: f32, far: f32) -> Mat4f {
        // Camera points towards -z. 0 < near < far.
        // Matrix maps z range [-near, -far] to [-1, 1], after homogeneous division
        let f = 1.0 / (fov * PI / 360.0).tan();
        let d = 1.0 / (near - far);

        Mat4f { data: [
            [f,   0.0, 0.0,              0.0],
            [0.0, -f,  0.0,              0.0],
            [0.0, 0.0, (near + far) * d, 2.0 * near * far * d],
            [0.0, 0.0, -1.0,             0.0],
        ]}
    }

    pub fn inverted(&self) -> Mat4f {
        let mut inv = Mat4f::zero();

        inv[(0,0)] =  self[(1,1)] * self[(2,2)] * self[(3,3)] -
                      self[(1,1)] * self[(3,2)] * self[(2,3)] -
                      self[(1,2)] * self[(2,1)] * self[(3,3)] +
                      self[(1,2)] * self[(3,1)] * self[(2,3)] +
                      self[(1,3)] * self[(2,1)] * self[(3,2)] -
                      self[(1,3)] * self[(3,1)] * self[(2,2)];
        inv[(0,1)] = -self[(0,1)] * self[(2,2)] * self[(3,3)] +
                      self[(0,1)] * self[(3,2)] * self[(2,3)] +
                      self[(0,2)] * self[(2,1)] * self[(3,3)] -
                      self[(0,2)] * self[(3,1)] * self[(2,3)] -
                      self[(0,3)] * self[(2,1)] * self[(3,2)] +
                      self[(0,3)] * self[(3,1)] * self[(2,2)];
        inv[(0,2)] =  self[(0,1)] * self[(1,2)] * self[(3,3)] -
                      self[(0,1)] * self[(3,2)] * self[(1,3)] -
                      self[(0,2)] * self[(1,1)] * self[(3,3)] +
                      self[(0,2)] * self[(3,1)] * self[(1,3)] +
                      self[(0,3)] * self[(1,1)] * self[(3,2)] -
                      self[(0,3)] * self[(3,1)] * self[(1,2)];
        inv[(0,3)] = -self[(0,1)] * self[(1,2)] * self[(2,3)] +
                      self[(0,1)] * self[(2,2)] * self[(1,3)] +
                      self[(0,2)] * self[(1,1)] * self[(2,3)] -
                      self[(0,2)] * self[(2,1)] * self[(1,3)] -
                      self[(0,3)] * self[(1,1)] * self[(2,2)] +
                      self[(0,3)] * self[(2,1)] * self[(1,2)];
        inv[(1,0)] = -self[(1,0)] * self[(2,2)] * self[(3,3)] +
                      self[(1,0)] * self[(3,2)] * self[(2,3)] +
                      self[(1,2)] * self[(2,0)] * self[(3,3)] -
                      self[(1,2)] * self[(3,0)] * self[(2,3)] -
                      self[(1,3)] * self[(2,0)] * self[(3,2)] +
                      self[(1,3)] * self[(3,0)] * self[(2,2)];
        inv[(1,1)] =  self[(0,0)] * self[(2,2)] * self[(3,3)] -
                      self[(0,0)] * self[(3,2)] * self[(2,3)] -
                      self[(0,2)] * self[(2,0)] * self[(3,3)] +
                      self[(0,2)] * self[(3,0)] * self[(2,3)] +
                      self[(0,3)] * self[(2,0)] * self[(3,2)] -
                      self[(0,3)] * self[(3,0)] * self[(2,2)];
        inv[(1,2)] = -self[(0,0)] * self[(1,2)] * self[(3,3)] +
                      self[(0,0)] * self[(3,2)] * self[(1,3)] +
                      self[(0,2)] * self[(1,0)] * self[(3,3)] -
                      self[(0,2)] * self[(3,0)] * self[(1,3)] -
                      self[(0,3)] * self[(1,0)] * self[(3,2)] +
                      self[(0,3)] * self[(3,0)] * self[(1,2)];
        inv[(1,3)] =  self[(0,0)] * self[(1,2)] * self[(2,3)] -
                      self[(0,0)] * self[(2,2)] * self[(1,3)] -
                      self[(0,2)] * self[(1,0)] * self[(2,3)] +
                      self[(0,2)] * self[(2,0)] * self[(1,3)] +
                      self[(0,3)] * self[(1,0)] * self[(2,2)] -
                      self[(0,3)] * self[(2,0)] * self[(1,2)];
        inv[(2,0)] =  self[(1,0)] * self[(2,1)] * self[(3,3)] -
                      self[(1,0)] * self[(3,1)] * self[(2,3)] -
                      self[(1,1)] * self[(2,0)] * self[(3,3)] +
                      self[(1,1)] * self[(3,0)] * self[(2,3)] +
                      self[(1,3)] * self[(2,0)] * self[(3,1)] -
                      self[(1,3)] * self[(3,0)] * self[(2,1)];
        inv[(2,1)] = -self[(0,0)] * self[(2,1)] * self[(3,3)] +
                      self[(0,0)] * self[(3,1)] * self[(2,3)] +
                      self[(0,1)] * self[(2,0)] * self[(3,3)] -
                      self[(0,1)] * self[(3,0)] * self[(2,3)] -
                      self[(0,3)] * self[(2,0)] * self[(3,1)] +
                      self[(0,3)] * self[(3,0)] * self[(2,1)];
        inv[(2,2)] =  self[(0,0)] * self[(1,1)] * self[(3,3)] -
                      self[(0,0)] * self[(3,1)] * self[(1,3)] -
                      self[(0,1)] * self[(1,0)] * self[(3,3)] +
                      self[(0,1)] * self[(3,0)] * self[(1,3)] +
                      self[(0,3)] * self[(1,0)] * self[(3,1)] -
                      self[(0,3)] * self[(3,0)] * self[(1,1)];
        inv[(2,3)] = -self[(0,0)] * self[(1,1)] * self[(2,3)] +
                      self[(0,0)] * self[(2,1)] * self[(1,3)] +
                      self[(0,1)] * self[(1,0)] * self[(2,3)] -
                      self[(0,1)] * self[(2,0)] * self[(1,3)] -
                      self[(0,3)] * self[(1,0)] * self[(2,1)] +
                      self[(0,3)] * self[(2,0)] * self[(1,1)];
        inv[(3,0)] = -self[(1,0)] * self[(2,1)] * self[(3,2)] +
                      self[(1,0)] * self[(3,1)] * self[(2,2)] +
                      self[(1,1)] * self[(2,0)] * self[(3,2)] -
                      self[(1,1)] * self[(3,0)] * self[(2,2)] -
                      self[(1,2)] * self[(2,0)] * self[(3,1)] +
                      self[(1,2)] * self[(3,0)] * self[(2,1)];
        inv[(3,1)] =  self[(0,0)] * self[(2,1)] * self[(3,2)] -
                      self[(0,0)] * self[(3,1)] * self[(2,2)] -
                      self[(0,1)] * self[(2,0)] * self[(3,2)] +
                      self[(0,1)] * self[(3,0)] * self[(2,2)] +
                      self[(0,2)] * self[(2,0)] * self[(3,1)] -
                      self[(0,2)] * self[(3,0)] * self[(2,1)];
        inv[(3,2)] = -self[(0,0)] * self[(1,1)] * self[(3,2)] +
                      self[(0,0)] * self[(3,1)] * self[(1,2)] +
                      self[(0,1)] * self[(1,0)] * self[(3,2)] -
                      self[(0,1)] * self[(3,0)] * self[(1,2)] -
                      self[(0,2)] * self[(1,0)] * self[(3,1)] +
                      self[(0,2)] * self[(3,0)] * self[(1,1)];
        inv[(3,3)] =  self[(0,0)] * self[(1,1)] * self[(2,2)] -
                      self[(0,0)] * self[(2,1)] * self[(1,2)] -
                      self[(0,1)] * self[(1,0)] * self[(2,2)] +
                      self[(0,1)] * self[(2,0)] * self[(1,2)] +
                      self[(0,2)] * self[(1,0)] * self[(2,1)] -
                      self[(0,2)] * self[(2,0)] * self[(1,1)];

        let det = self[(0,0)] * inv[(0,0)] +
                  self[(1,0)] * inv[(0,1)] +
                  self[(2,0)] * inv[(0,2)] +
                  self[(3,0)] * inv[(0,3)];

        if det == 0.0 {
            return Mat4f::identity();
        }

        let inv_det = 1.0 / det;
        for r in range(0, 4) {
            for c in range(0, 4) {
                inv[(r,c)] *= inv_det;
            }
        }
        inv
    }
}

impl Index<(uint, uint), f32> for Mat4f {
    #[inline]
    fn index(&self, index: &(uint, uint)) -> &f32 {
        let (row, column) = *index;
        &self.data[row][column]
    }
}

impl IndexMut<(uint, uint), f32> for Mat4f {
    #[inline]
    fn index_mut(&mut self, index: &(uint, uint)) -> &mut f32 {
        let (row, column) = *index;
        &mut self.data[row][column]
    }
}

impl Mul<Mat4f, Mat4f> for Mat4f {
    #[inline]
    fn mul(&self, o: &Mat4f) -> Mat4f {
        let mut res = Mat4f::zero();
        for r in range(0, 4) {
            for c in range(0, 4) {
                for i in range(0, 4) {
                    res[(r, c)] += self[(r, i)] * o[(i, c)];
                }
            }
        }
        res
    }
}
