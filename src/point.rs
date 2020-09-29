use crate::scalar::{
    Scalar,
    ScalarFloat,
};
use crate::traits::{
    Array,
    Metric,
    DotProduct,
    Magnitude,
};
use crate::vector::{
    Vector1,
    Vector2,
    Vector3,
    Vector4,
};
use crate::num_traits::{
    NumCast,
};

use core::fmt;
use core::ops;


macro_rules! impl_mul_operator {
    ($Lhs:ty, $Rhs:ty, $Output:ty, { $($field:ident),* }) => {
        impl ops::Mul<$Rhs> for $Lhs {
            type Output = $Output;

            #[inline]
            fn mul(self, other: $Rhs) -> $Output {
                <$Output>::new( $(self * other.$field),*)
            }
        }

        impl<'a> ops::Mul<$Rhs> for &'a $Lhs {
            type Output = $Output;

            #[inline]
            fn mul(self, other: $Rhs) -> $Output {
                <$Output>::new( $(self * other.$field),*)
            }
        }
    }
}


/// A point is a location in a one-dimensional Euclidean space.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Point1<S> {
    /// The horizontal coordinate.
    pub x: S,
}

impl<S> Point1<S> {
    /// Construct a new point in one-dimensional Euclidean space.
    #[inline]
    pub const fn new(x: S) -> Point1<S> {
        Point1 { 
            x: x 
        }
    }

    /// Map an operation on that acts on the coordinates of a point, returning a point of the 
    /// new underlying type.
    pub fn map<T, F>(self, mut op: F) -> Point1<T> where F: FnMut(S) -> T {
        Point1 { 
            x: op(self.x) 
        }
    }
}

impl<S> Point1<S> where S: Copy {
    /// Construct a new two-dimensional point from a one-dimensional point by
    /// supplying the y-coordinate.
    #[inline]
    pub fn expand(self, y: S) -> Point2<S> {
        Point2::new(self.x, y)
    }

    /// Construct a new point from a fill value.
    #[inline]
    pub fn from_fill(value: S) -> Point1<S> {
        Point1::new(value)
    }
}

impl<S> Point1<S> where S: NumCast + Copy {
    /// Cast a point of one type of scalars to a point of another type of scalars.
    pub fn cast<T: NumCast>(&self) -> Option<Point1<T>> {
        let x = match num_traits::cast(self.x) {
            Some(value) => value,
            None => return None,
        };

        Some(Point1::new(x))
    }
}

impl<S> Point1<S> where S: Scalar {
    /// Compute the origin of the Euclidean vector space.
    #[inline]
    pub fn origin() -> Point1<S> {
        Point1::new(S::zero())
    }

    /// Convert a vector to a point. 
    /// 
    /// Points are locations in Euclidean space, whereas vectors
    /// are displacements relative to the origin in Euclidean space.
    #[inline]
    pub fn from_vector(v: Vector1<S>) -> Point1<S> {
        Point1::new(v.x)
    }
    
    /// Convert a point to a vector.
    /// 
    /// Points are locations in Euclidean space, whereas vectors
    /// are displacements relative to the origin in Euclidean space.
    #[inline]
    pub fn to_vector(self) -> Vector1<S> {
        Vector1::new(self.x)
    }
}

impl<S> Array for Point1<S> where S: Copy {
    type Element = S;

    #[inline]
    fn len() -> usize {
        1
    }

    #[inline]
    fn shape() -> (usize, usize) {
        (1, 1)
    }

    #[inline]
    fn as_ptr(&self) -> *const Self::Element {
        &self.x
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Element {
        &mut self.x
    }

    #[inline]
    fn as_slice(&self) -> &[Self::Element] {
        <Self as AsRef<[Self::Element; 1]>>::as_ref(self)
    }
}

impl<S> AsRef<[S; 1]> for Point1<S> {
    fn as_ref(&self) -> &[S; 1] {
        unsafe { 
            &*(self as *const Point1<S> as *const [S; 1])
        }
    }
}

impl<S> AsRef<S> for Point1<S> {
    fn as_ref(&self) -> &S {
        unsafe { 
            &*(self as *const Point1<S> as *const S)
        }
    }
}

impl<S> AsRef<(S,)> for Point1<S> {
    fn as_ref(&self) -> &(S,) {
        unsafe { 
            &*(self as *const Point1<S> as *const (S,))
        }
    }
}

impl<S> AsMut<[S; 1]> for Point1<S> {
    fn as_mut(&mut self) -> &mut [S; 1] {
        unsafe { 
            &mut *(self as *mut Point1<S> as *mut [S; 1])
        }
    }
}

impl<S> AsMut<S> for Point1<S> {
    fn as_mut(&mut self) -> &mut S {
        unsafe { 
            &mut *(self as *mut Point1<S> as *mut S)
        }
    }
}

impl<S> AsMut<(S,)> for Point1<S> {
    fn as_mut(&mut self) -> &mut (S,) {
        unsafe { 
            &mut *(self as *mut Point1<S> as *mut (S,))
        }
    }
}

impl<S> ops::Index<usize> for Point1<S> {
    type Output = S;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let v: &[S; 1] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::Range<usize>> for Point1<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        let v: &[S; 1] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeTo<usize>> for Point1<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeTo<usize>) -> &Self::Output {
        let v: &[S; 1] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeFrom<usize>> for Point1<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeFrom<usize>) -> &Self::Output {
        let v: &[S; 1] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeFull> for Point1<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeFull) -> &Self::Output {
        let v: &[S; 1] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::IndexMut<usize> for Point1<S> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut S {
        let v: &mut [S; 1] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::Range<usize>> for Point1<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut [S] {
        let v: &mut [S; 1] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeTo<usize>> for Point1<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeTo<usize>) -> &mut [S] {
        let v: &mut [S; 1] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeFrom<usize>> for Point1<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFrom<usize>) -> &mut [S] {
        let v: &mut [S; 1] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeFull> for Point1<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFull) -> &mut [S] {
        let v: &mut [S; 1] = self.as_mut();
        &mut v[index]
    }
}

impl<S> fmt::Debug for Point1<S> where S: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point1 ")?;
        <[S; 1] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S> fmt::Display for Point1<S> where S: fmt::Display {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Point1 [{}]", self.x)
    }
}

impl<S> From<S> for Point1<S> where S: Scalar {
    #[inline]
    fn from(v: S) -> Point1<S> {
        Point1 { x: v }
    }
}

impl<S> From<[S; 1]> for Point1<S> where S: Scalar {
    #[inline]
    fn from(v: [S; 1]) -> Point1<S> {
        Point1 { x: v[0] }
    }
}

impl<S> From<&[S; 1]> for Point1<S> where S: Scalar {
    #[inline]
    fn from(v: &[S; 1]) -> Point1<S> {
        Point1 { x: v[0] }
    }
}

impl<'a, S> From<&'a [S; 1]> for &'a Point1<S> where S: Scalar {
    #[inline]
    fn from(v: &'a [S; 1]) -> &'a Point1<S> {
        unsafe { 
            &*(v as *const [S; 1] as *const Point1<S>)
        }
    }
}

impl<S> ops::Add<Vector1<S>> for Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn add(self, other: Vector1<S>) -> Self::Output {
        Point1 {
            x: self.x + other.x,
        }
    }
}

impl<S> ops::Add<Vector1<S>> for &Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn add(self, other: Vector1<S>) -> Self::Output {
        Point1 {
            x: self.x + other.x,
        }
    }
}

impl<S> ops::Add<&Vector1<S>> for Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn add(self, other: &Vector1<S>) -> Self::Output {
        Point1 {
            x: self.x + other.x,
        }
    }
}

impl<'a, 'b, S> ops::Add<&'b Vector1<S>> for &'a Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn add(self, other: &'b Vector1<S>) -> Self::Output {
        Point1 {
            x: self.x + other.x,
        }
    }
}

impl<S> ops::Sub<Point1<S>> for &Point1<S> where S: Scalar {
    type Output = Vector1<S>;

    fn sub(self, other: Point1<S>) -> Self::Output {
        Vector1 {
            x: self.x - other.x,
        }
    }
}

impl<S> ops::Sub<Point1<S>> for Point1<S> where S: Scalar {
    type Output = Vector1<S>;

    fn sub(self, other: Point1<S>) -> Self::Output {
        Vector1 {
            x: self.x - other.x,
        }
    }
}

impl<S> ops::Sub<&Point1<S>> for Point1<S> where S: Scalar {
    type Output = Vector1<S>;

    fn sub(self, other: &Point1<S>) -> Self::Output {
        Vector1 {
            x: self.x - other.x,          
        }
    }
}

impl<'a, 'b, S> ops::Sub<&'b Point1<S>> for &'a Point1<S> where S: Scalar {
    type Output = Vector1<S>;

    fn sub(self, other: &'b Point1<S>) -> Self::Output {
        Vector1 {
            x: self.x - other.x,
        }
    }
}

impl<S> ops::Sub<Vector1<S>> for &Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn sub(self, other: Vector1<S>) -> Self::Output {
        Point1 {
            x: self.x - other.x,
        }
    }
}

impl<S> ops::Sub<Vector1<S>> for Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn sub(self, other: Vector1<S>) -> Self::Output {
        Point1 {
            x: self.x - other.x,
        }
    }
}

impl<S> ops::Sub<&Vector1<S>> for Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn sub(self, other: &Vector1<S>) -> Self::Output {
        Point1 {
            x: self.x - other.x,          
        }
    }
}

impl<'a, 'b, S> ops::Sub<&'b Vector1<S>> for &'a Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn sub(self, other: &'b Vector1<S>) -> Self::Output {
        Point1 {
            x: self.x - other.x,
        }
    }
}

impl<S> ops::Mul<S> for Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn mul(self, other: S) -> Self::Output {
        Point1 {
            x: self.x * other,
        }
    }
}

impl<S> ops::Mul<S> for &Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn mul(self, other: S) -> Self::Output {
        Point1 {
            x: self.x * other,
        }
    }
}

impl_mul_operator!(u8,    Point1<u8>,    Point1<u8>,    { x });
impl_mul_operator!(u16,   Point1<u16>,   Point1<u16>,   { x });
impl_mul_operator!(u32,   Point1<u32>,   Point1<u32>,   { x });
impl_mul_operator!(u64,   Point1<u64>,   Point1<u64>,   { x });
impl_mul_operator!(u128,  Point1<u128>,  Point1<u128>,  { x });
impl_mul_operator!(usize, Point1<usize>, Point1<usize>, { x });
impl_mul_operator!(i8,    Point1<i8>,    Point1<i8>,    { x });
impl_mul_operator!(i16,   Point1<i16>,   Point1<i16>,   { x });
impl_mul_operator!(i32,   Point1<i32>,   Point1<i32>,   { x });
impl_mul_operator!(i64,   Point1<i64>,   Point1<i64>,   { x });
impl_mul_operator!(i128,  Point1<i128>,  Point1<i128>,  { x });
impl_mul_operator!(isize, Point1<isize>, Point1<isize>, { x });
impl_mul_operator!(f32,   Point1<f32>,   Point1<f32>,   { x });
impl_mul_operator!(f64,   Point1<f64>,   Point1<f64>,   { x });

impl<S> ops::Div<S> for Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn div(self, other: S) -> Self::Output {
        Point1 {
            x: self.x / other,
        }
    }
}

impl<S> ops::Div<S> for &Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn div(self, other: S) -> Self::Output {
        Point1 {
            x: self.x / other,
        }
    }
}

impl<S> ops::Rem<S> for Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn rem(self, other: S) -> Self::Output {
        let x = self.x % other;
        
        Point1::new(x)
    }
}

impl<S> ops::Rem<S> for &Point1<S> where S: Scalar {
    type Output = Point1<S>;

    fn rem(self, other: S) -> Self::Output {
        let x = self.x % other;
        
        Point1::new(x)
    }
}

impl<S> ops::AddAssign<Vector1<S>> for Point1<S> where S: Scalar {
    fn add_assign(&mut self, other: Vector1<S>) {
        self.x = self.x + other.x;
    }
}

impl<S> ops::AddAssign<&Vector1<S>> for Point1<S> where S: Scalar {
    fn add_assign(&mut self, other: &Vector1<S>) {
        self.x = self.x + other.x;
    }
}

impl<S> ops::SubAssign<Vector1<S>> for Point1<S> where S: Scalar {
    fn sub_assign(&mut self, other: Vector1<S>) {
        self.x = self.x - other.x;
    }
}

impl<S> ops::SubAssign<&Vector1<S>> for Point1<S> where S: Scalar {
    fn sub_assign(&mut self, other: &Vector1<S>) {
        self.x = self.x - other.x;
    }
}

impl<S> ops::MulAssign<S> for Point1<S> where S: Scalar {
    fn mul_assign(&mut self, other: S) {
        self.x *= other;
    }
}

impl<S> ops::DivAssign<S> for Point1<S> where S: Scalar {
    fn div_assign(&mut self, other: S) {
        self.x = self.x / other;
    }
}

impl<S> ops::RemAssign<S> for Point1<S> where S: Scalar {
    fn rem_assign(&mut self, other: S) {
        self.x %= other;
    }
}

impl<S> approx::AbsDiffEq for Point1<S> where S: ScalarFloat {
    type Epsilon = <S as approx::AbsDiffEq>::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        S::abs_diff_eq(&self.x, &other.x, epsilon)
    }
}

impl<S> approx::RelativeEq for Point1<S> where S: ScalarFloat {
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        S::relative_eq(&self.x, &other.x, epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for Point1<S> where S: ScalarFloat {
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        S::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
    }
}

impl<S> DotProduct<Point1<S>> for Point1<S> where S: Scalar {
    type Output = S;

    #[inline]
    fn dot(self, other: Point1<S>) -> Self::Output {
        self.x * other.x
    }
}

impl<S> DotProduct<&Point1<S>> for Point1<S> where S: Scalar {
    type Output = S;

    #[inline]
    fn dot(self, other: &Point1<S>) -> Self::Output {
        self.x * other.x
    }
}

impl<S> DotProduct<Point1<S>> for &Point1<S> where S: Scalar {
    type Output = S;

    #[inline]
    fn dot(self, other: Point1<S>) -> Self::Output {
        self.x * other.x
    }
}

impl<'a, 'b, S> DotProduct<&'a Point1<S>> for &'b Point1<S> where S: Scalar {
    type Output = S;
    
    #[inline]
    fn dot(self, other: &'a Point1<S>) -> Self::Output {
        self.x * other.x
    }
}

impl<S> Magnitude for Point1<S> where S: ScalarFloat {
    type Output = S;

    fn magnitude(&self) -> Self::Output {
        Self::Output::sqrt(self.magnitude_squared())
    }

    fn magnitude_squared(&self) -> Self::Output {
        DotProduct::dot(self, self)
    }

    fn normalize(&self) -> Self {
        self / self.magnitude()
    }

    fn normalize_to(&self, magnitude: Self::Output) -> Self {
        self * (magnitude / self.magnitude())
    }
}

impl<S> Metric<Point1<S>> for Point1<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: Point1<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}

impl<S> Metric<&Point1<S>> for Point1<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: &Point1<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}

impl<S> Metric<Point1<S>> for &Point1<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: Point1<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}

impl<'a, 'b, S> Metric<&'a Point1<S>> for &'b Point1<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: &Point1<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}


/// A point is a location in a two-dimensional Euclidean space.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Point2<S> {
   /// The horizontal coordinate.
   pub x: S,
   /// The vertical coordinate.
   pub y: S,
}

impl<S> Point2<S> {
    /// Construct a new two-dimensional point.
    #[inline]
    pub const fn new(x: S, y: S) -> Point2<S> {
        Point2 { x: x, y: y }
    }

    /// Map an operation on that acts on the coordinates of a point, returning 
    /// a point whose coordinates are of the new scalar type.
    pub fn map<T, F>(self, mut op: F) -> Point2<T> 
        where F: FnMut(S) -> T 
    {
        Point2 {
            x: op(self.x),
            y: op(self.y),
        }
    }
}

impl<S> Point2<S> where S: Copy {
    /// Expand a two-dimensional point to a three-dimensional point using
    /// the supplied z-value.
    #[inline]
    pub fn expand(self, z: S) -> Point3<S> {
        Point3::new(self.x, self.y, z)
    }

    /// Contract a two-dimensional point to a one-dimensional point by
    /// removing its y-component.
    #[inline]
    pub fn contract(self) -> Point1<S> {
        Point1::new(self.x)
    }

    /// Construct a new point from a fill value.
    #[inline]
    pub fn from_fill(value: S) -> Point2<S> {
        Point2::new(value, value)
    }
}

impl<S> Point2<S> where S: NumCast + Copy {
    /// Cast a point of one type of scalars to a point of another type of scalars.
    pub fn cast<T: NumCast>(&self) -> Option<Point2<T>> {
        let x = match num_traits::cast(self.x) {
            Some(value) => value,
            None => return None,
        };
        let y = match num_traits::cast(self.y) {
            Some(value) => value,
            None => return None,
        };

        Some(Point2::new(x, y))
    }
}

impl<S> Point2<S> where S: Scalar {
    /// Convert a homogeneous vector into a point.
    #[inline]
    pub fn from_homogeneous(vector: Vector3<S>) -> Point2<S> {
        let e = vector.contract() * (S::one() / vector.z);
        Point2::new(e.x, e.y)
    }

    /// Convert a point to a vector in homogeneous coordinates.
    #[inline]
    pub fn to_homogeneous(self) -> Vector3<S> {
        Vector3::new(self.x, self.y, S::one())
    }

    /// Compute the origin of the Euclidean vector space.
    #[inline]
    pub fn origin() -> Point2<S> {
        Point2::new(S::zero(), S::zero())
    }

    /// Convert a vector to a point. 
    /// 
    /// Points are locations in Euclidean space, whereas vectors
    /// are displacements relative to the origin in Euclidean space.
    #[inline]
    pub fn from_vector(vector: Vector2<S>) -> Point2<S> {
        Point2::new(vector.x, vector.y)
    }

    /// Convert a point to a vector.
    /// 
    /// Points are locations in Euclidean space, whereas vectors
    /// are displacements relative to the origin in Euclidean space.
    #[inline]
    pub fn to_vector(self) -> Vector2<S> {
        Vector2::new(self.x, self.y)
    }
}

impl<S> Array for Point2<S> where S: Copy {
    type Element = S;

    #[inline]
    fn len() -> usize {
        2
    }

    #[inline]
    fn shape() -> (usize, usize) {
        (2, 1)
    }

    #[inline]
    fn as_ptr(&self) -> *const Self::Element {
        &self.x
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Element {
        &mut self.x
    }

    #[inline]
    fn as_slice(&self) -> &[Self::Element] {
        <Self as AsRef<[Self::Element; 2]>>::as_ref(self)
    }
}

impl<S> AsRef<[S; 2]> for Point2<S> {
    fn as_ref(&self) -> &[S; 2] {
        unsafe { 
            &*(self as *const Point2<S> as *const [S; 2])
        }
    }
}

impl<S> AsRef<(S, S)> for Point2<S> {
    fn as_ref(&self) -> &(S, S) {
        unsafe { 
            &*(self as *const Point2<S> as *const (S, S))
        }
    }
}

impl<S> AsMut<[S; 2]> for Point2<S> {
    fn as_mut(&mut self) -> &mut [S; 2] {
        unsafe { 
            &mut *(self as *mut Point2<S> as *mut [S; 2])
        }
    }
}

impl<S> AsMut<(S, S)> for Point2<S> {
    fn as_mut(&mut self) -> &mut (S, S) {
        unsafe { 
            &mut *(self as *mut Point2<S> as *mut (S, S))
        }
    }
}

impl<S> ops::Index<usize> for Point2<S> {
    type Output = S;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let v: &[S; 2] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::Range<usize>> for Point2<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        let v: &[S; 2] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeTo<usize>> for Point2<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeTo<usize>) -> &Self::Output {
        let v: &[S; 2] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeFrom<usize>> for Point2<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeFrom<usize>) -> &Self::Output {
        let v: &[S; 2] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeFull> for Point2<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeFull) -> &Self::Output {
        let v: &[S; 2] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::IndexMut<usize> for Point2<S> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut S {
        let v: &mut [S; 2] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::Range<usize>> for Point2<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut [S] {
        let v: &mut [S; 2] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeTo<usize>> for Point2<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeTo<usize>) -> &mut [S] {
        let v: &mut [S; 2] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeFrom<usize>> for Point2<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFrom<usize>) -> &mut [S] {
        let v: &mut [S; 2] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeFull> for Point2<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFull) -> &mut [S] {
        let v: &mut [S; 2] = self.as_mut();
        &mut v[index]
    }
}

impl<S> fmt::Debug for Point2<S> where S: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point2 ")?;
        <[S; 2] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S> fmt::Display for Point2<S> where S: fmt::Display {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Point2 [{}, {}]", self.x, self.y)
    }
}

impl<S> From<(S, S)> for Point2<S> where S: Scalar {
    #[inline]
    fn from((x, y): (S, S)) -> Point2<S> {
        Point2 { x: x, y: y }
    }
}

impl<S> From<[S; 2]> for Point2<S> where S: Scalar {
    #[inline]
    fn from(v: [S; 2]) -> Point2<S> {
        Point2 { x: v[0], y: v[1] }
    }
}

impl<S> From<&[S; 2]> for Point2<S> where S: Scalar {
    #[inline]
    fn from(v: &[S; 2]) -> Point2<S> {
        Point2 { x: v[0], y: v[1] }
    }
}

impl<'a, S> From<&'a [S; 2]> for &'a Point2<S> where S: Scalar {
    #[inline]
    fn from(v: &'a [S; 2]) -> &'a Point2<S> {
        unsafe { 
            &*(v as *const [S; 2] as *const Point2<S>)
        }
    }
}

impl<S> ops::Add<Vector2<S>> for Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn add(self, other: Vector2<S>) -> Self::Output {
        Point2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<S> ops::Add<Vector2<S>> for &Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn add(self, other: Vector2<S>) -> Self::Output {
        Point2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<S> ops::Add<&Vector2<S>> for Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn add(self, other: &Vector2<S>) -> Self::Output {
        Point2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b, S> ops::Add<&'b Vector2<S>> for &'a Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn add(self, other: &'b Vector2<S>) -> Self::Output {
        Point2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<S> ops::Sub<Point2<S>> for &Point2<S> where S: Scalar {
    type Output = Vector2<S>;

    fn sub(self, other: Point2<S>) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<S> ops::Sub<Point2<S>> for Point2<S> where S: Scalar {
    type Output = Vector2<S>;

    fn sub(self, other: Point2<S>) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<S> ops::Sub<&Point2<S>> for Point2<S> where S: Scalar {
    type Output = Vector2<S>;

    fn sub(self, other: &Point2<S>) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,             
        }
    }
}

impl<'a, 'b, S> ops::Sub<&'b Point2<S>> for &'a Point2<S> where S: Scalar {
    type Output = Vector2<S>;

    fn sub(self, other: &'b Point2<S>) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<S> ops::Sub<Vector2<S>> for &Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn sub(self, other: Vector2<S>) -> Self::Output {
        Point2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<S> ops::Sub<Vector2<S>> for Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn sub(self, other: Vector2<S>) -> Self::Output {
        Point2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<S> ops::Sub<&Vector2<S>> for Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn sub(self, other: &Vector2<S>) -> Self::Output {
        Point2 {
            x: self.x - other.x,
            y: self.y - other.y,             
        }
    }
}

impl<'a, 'b, S> ops::Sub<&'b Vector2<S>> for &'a Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn sub(self, other: &'b Vector2<S>) -> Self::Output {
        Point2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<S> ops::Mul<S> for Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn mul(self, other: S) -> Self::Output {
        Point2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<S> ops::Mul<S> for &Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn mul(self, other: S) -> Self::Output {
        Point2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl_mul_operator!(u8,    Point2<u8>,    Point2<u8>,    { x, y });
impl_mul_operator!(u16,   Point2<u16>,   Point2<u16>,   { x, y });
impl_mul_operator!(u32,   Point2<u32>,   Point2<u32>,   { x, y });
impl_mul_operator!(u64,   Point2<u64>,   Point2<u64>,   { x, y });
impl_mul_operator!(u128,  Point2<u128>,  Point2<u128>,  { x, y });
impl_mul_operator!(usize, Point2<usize>, Point2<usize>, { x, y });
impl_mul_operator!(i8,    Point2<i8>,    Point2<i8>,    { x, y });
impl_mul_operator!(i16,   Point2<i16>,   Point2<i16>,   { x, y });
impl_mul_operator!(i32,   Point2<i32>,   Point2<i32>,   { x, y });
impl_mul_operator!(i64,   Point2<i64>,   Point2<i64>,   { x, y });
impl_mul_operator!(i128,  Point2<i128>,  Point2<i128>,  { x, y });
impl_mul_operator!(isize, Point2<isize>, Point2<isize>, { x, y });
impl_mul_operator!(f32,   Point2<f32>,   Point2<f32>,   { x, y });
impl_mul_operator!(f64,   Point2<f64>,   Point2<f64>,   { x, y });

impl<S> ops::Div<S> for Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn div(self, other: S) -> Self::Output {
        Point2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<S> ops::Div<S> for &Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn div(self, other: S) -> Self::Output {
        Point2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<S> ops::Rem<S> for Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn rem(self, other: S) -> Self::Output {
        let x = self.x % other;
        let y = self.y % other;
        
        Point2::new(x, y)
    }
}

impl<S> ops::Rem<S> for &Point2<S> where S: Scalar {
    type Output = Point2<S>;

    fn rem(self, other: S) -> Self::Output {
        let x = self.x % other;
        let y = self.y % other;
        
        Point2::new(x, y)
    }
}

impl<S> ops::AddAssign<Vector2<S>> for Point2<S> where S: Scalar {
    fn add_assign(&mut self, other: Vector2<S>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<S> ops::AddAssign<&Vector2<S>> for Point2<S> where S: Scalar {
    fn add_assign(&mut self, other: &Vector2<S>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<S> ops::SubAssign<Vector2<S>> for Point2<S> where S: Scalar {
    fn sub_assign(&mut self, other: Vector2<S>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl<S> ops::SubAssign<&Vector2<S>> for Point2<S> where S: Scalar {
    fn sub_assign(&mut self, other: &Vector2<S>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl<S> ops::MulAssign<S> for Point2<S> where S: Scalar {
    fn mul_assign(&mut self, other: S) {
        self.x *= other;
        self.y *= other;
    }
}

impl<S> ops::DivAssign<S> for Point2<S> where S: Scalar {
    fn div_assign(&mut self, other: S) {
        self.x = self.x / other;
        self.y = self.y / other;
    }
}

impl<S> ops::RemAssign<S> for Point2<S> where S: Scalar {
    fn rem_assign(&mut self, other: S) {
        self.x %= other;
        self.y %= other;
    }
}

impl<S> approx::AbsDiffEq for Point2<S> where S: ScalarFloat {
    type Epsilon = <S as approx::AbsDiffEq>::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        S::abs_diff_eq(&self.x, &other.x, epsilon) && 
        S::abs_diff_eq(&self.y, &other.y, epsilon)
    }
}

impl<S> approx::RelativeEq for Point2<S> where S: ScalarFloat {
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        S::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
        S::relative_eq(&self.y, &other.y, epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for Point2<S> where S: ScalarFloat {
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        S::ulps_eq(&self.x, &other.x, epsilon, max_ulps) &&
        S::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
    }
}

impl<S> DotProduct<Point2<S>> for Point2<S> where S: Scalar {
    type Output = S;

    #[inline]
    fn dot(self, other: Point2<S>) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl<S> DotProduct<&Point2<S>> for Point2<S> where S: Scalar {
    type Output = S;

    #[inline]
    fn dot(self, other: &Point2<S>) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl<S> DotProduct<Point2<S>> for &Point2<S> where S: Scalar {
    type Output = S;

    #[inline]
    fn dot(self, other: Point2<S>) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl<'a, 'b, S> DotProduct<&'a Point2<S>> for &'b Point2<S> where S: Scalar {
    type Output = S;
    
    #[inline]
    fn dot(self, other: &'a Point2<S>) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl<S> Magnitude for Point2<S> where S: ScalarFloat {
    type Output = S;

    fn magnitude(&self) -> Self::Output {
        Self::Output::sqrt(self.magnitude_squared())
    }

    fn magnitude_squared(&self) -> Self::Output {
        DotProduct::dot(self, self)
    }

    fn normalize(&self) -> Self {
        self / self.magnitude()
    }

    fn normalize_to(&self, magnitude: Self::Output) -> Self {
        self * (magnitude / self.magnitude())
    }
}

impl<S> Metric<Point2<S>> for Point2<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: Point2<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}

impl<S> Metric<&Point2<S>> for Point2<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: &Point2<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}

impl<S> Metric<Point2<S>> for &Point2<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: Point2<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}

impl<'a, 'b, S> Metric<&'a Point2<S>> for &'b Point2<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: &Point2<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}


/// A representation of three-dimensional points in a Euclidean space.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub struct Point3<S> {
    /// The horizontal coordinate.
    pub x: S,
    /// The vertical coordinate.
    pub y: S,
    /// The depth coordinate.
    pub z: S,
}

impl<S> Point3<S> {
    /// Construct a new point in three-dimensional Euclidean space.
    #[inline]
    pub const fn new(x: S, y: S, z: S) -> Point3<S> {
        Point3 { x: x, y: y, z: z }
    }

    /// Map an operation on that acts on the coordinates of a point, returning 
    /// a point whose coordinates are of the new scalar type.
    pub fn map<T, F>(self, mut op: F) -> Point3<T> where F: FnMut(S) -> T {
        Point3 {
            x: op(self.x),
            y: op(self.y),
            z: op(self.z),
        }
    }
}

impl<S> Point3<S> where S: Copy {
    /// Construct a new point from a fill value.
    #[inline]
    pub fn from_fill(value: S) -> Point3<S> {
        Point3::new(value, value, value)
    }

    /// Contract a three-dimensional point, removing its z-component.
    #[inline]
    pub fn contract(self) -> Point2<S> {
        Point2::new(self.x, self.y)
    }
}

impl<S> Point3<S> where S: NumCast + Copy {
    /// Cast a point from one type of scalars to another type of scalars.
    pub fn cast<T: NumCast>(&self) -> Option<Point3<T>> {
        let x = match num_traits::cast(self.x) {
            Some(value) => value,
            None => return None,
        };
        let y = match num_traits::cast(self.y) {
            Some(value) => value,
            None => return None,
        };
        let z = match num_traits::cast(self.z) {
            Some(value) => value,
            None => return None,
        };

        Some(Point3::new(x, y, z))
    }
}

impl<S> Point3<S> where S: Scalar {
    /// Convert a vector in homogeneous coordinates into a point.
    #[inline]
    pub fn from_homogeneous(vector: Vector4<S>) -> Point3<S> {
        let e = vector.contract() * (S::one() / vector.w);
        Point3::new(e.x, e.y, e.z)
    }

    /// Convert a point to a vector in homogeneous coordinates.
    #[inline]
    pub fn to_homogeneous(self) -> Vector4<S> {
        Vector4::new(self.x, self.y, self.z, S::one())
    }

    /// Compute the origin of the Euclidean vector space.
    #[inline]
    pub fn origin() -> Point3<S> {
        Point3::new(S::zero(), S::zero(), S::zero())
    }

    /// Convert a vector to a point. 
    /// 
    /// Points are locations in Euclidean space, whereas vectors
    /// are displacements relative to the origin in Euclidean space.
    #[inline]
    pub fn from_vector(v: Vector3<S>) -> Point3<S> {
        Point3::new(v.x, v.y, v.z)
    }

    /// Convert a point to a vector.
    /// 
    /// Points are locations in Euclidean space, whereas vectors
    /// are displacements relative to the origin in Euclidean space.
    #[inline]
    pub fn to_vector(self) -> Vector3<S> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl<S> Array for Point3<S> where S: Copy {
    type Element = S;

    #[inline]
    fn len() -> usize {
        3
    }

    #[inline]
    fn shape() -> (usize, usize) {
        (3, 1)
    }

    #[inline]
    fn as_ptr(&self) -> *const Self::Element {
        &self.x
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Element {
        &mut self.x
    }

    #[inline]
    fn as_slice(&self) -> &[Self::Element] {
        <Self as AsRef<[Self::Element; 3]>>::as_ref(self)
    }
}

impl<S> AsRef<[S; 3]> for Point3<S> {
    fn as_ref(&self) -> &[S; 3] {
        unsafe { 
            &*(self as *const Point3<S> as *const [S; 3])
        }
    }
}

impl<S> AsRef<(S, S, S)> for Point3<S> {
    fn as_ref(&self) -> &(S, S, S) {
        unsafe { 
            &*(self as *const Point3<S> as *const (S, S, S))
        }
    }
}

impl<S> AsMut<[S; 3]> for Point3<S> {
    fn as_mut(&mut self) -> &mut [S; 3] {
        unsafe { 
            &mut *(self as *mut Point3<S> as *mut [S; 3])
        }
    }
}

impl<S> AsMut<(S, S, S)> for Point3<S> {
    fn as_mut(&mut self) -> &mut (S, S, S) {
        unsafe { 
            &mut *(self as *mut Point3<S> as *mut (S, S, S))
        }
    }
}

impl<S> ops::Index<usize> for Point3<S> {
    type Output = S;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let v: &[S; 3] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::Range<usize>> for Point3<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        let v: &[S; 3] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeTo<usize>> for Point3<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeTo<usize>) -> &Self::Output {
        let v: &[S; 3] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeFrom<usize>> for Point3<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeFrom<usize>) -> &Self::Output {
        let v: &[S; 3] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::Index<ops::RangeFull> for Point3<S> {
    type Output = [S];

    #[inline]
    fn index(&self, index: ops::RangeFull) -> &Self::Output {
        let v: &[S; 3] = self.as_ref();
        &v[index]
    }
}

impl<S> ops::IndexMut<usize> for Point3<S> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut S {
        let v: &mut [S; 3] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::Range<usize>> for Point3<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::Range<usize>) -> &mut [S] {
        let v: &mut [S; 3] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeTo<usize>> for Point3<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeTo<usize>) -> &mut [S] {
        let v: &mut [S; 3] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeFrom<usize>> for Point3<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFrom<usize>) -> &mut [S] {
        let v: &mut [S; 3] = self.as_mut();
        &mut v[index]
    }
}

impl<S> ops::IndexMut<ops::RangeFull> for Point3<S> {
    #[inline]
    fn index_mut(&mut self, index: ops::RangeFull) -> &mut [S] {
        let v: &mut [S; 3] = self.as_mut();
        &mut v[index]
    }
}

impl<S> fmt::Debug for Point3<S> where S: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point3 ")?;
        <[S; 3] as fmt::Debug>::fmt(self.as_ref(), f)
    }
}

impl<S> fmt::Display for Point3<S> where S: fmt::Display {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Point3 [{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl<S> From<(S, S, S)> for Point3<S> where S: Scalar {
    #[inline]
    fn from((x, y, z): (S, S, S)) -> Point3<S> {
        Point3::new(x, y, z)
    }
}

impl<S> From<(Point2<S>, S)> for Point3<S> where S: Scalar {
    #[inline]
    fn from((v, z): (Point2<S>, S)) -> Point3<S> {
        Point3::new(v.x, v.y, z)
    }
}

impl<S> From<(&Point2<S>, S)> for Point3<S> where S: Scalar {
    #[inline]
    fn from((v, z): (&Point2<S>, S)) -> Point3<S> {
        Point3::new(v.x, v.y, z)
    }
}

impl<S> From<[S; 3]> for Point3<S> where S: Scalar {
    #[inline]
    fn from(v: [S; 3]) -> Point3<S> {
        Point3::new(v[0], v[1], v[2])
    }
}

impl<'a, S> From<&'a [S; 3]> for &'a Point3<S> where S: Scalar {
    #[inline]
    fn from(v: &'a [S; 3]) -> &'a Point3<S> {
        unsafe { 
            &*(v as *const [S; 3] as *const Point3<S>)
        }
    }
}

impl<'a, S> From<&'a (S, S, S)> for &'a Point3<S> where S: Scalar {
    #[inline]
    fn from(v: &'a (S, S, S)) -> &'a Point3<S> {
        unsafe { 
            &*(v as *const (S, S, S) as *const Point3<S>)
        }
    }
}

impl<S> ops::Add<Vector3<S>> for Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn add(self, other: Vector3<S>) -> Self::Output {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<S> ops::Add<Vector3<S>> for &Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn add(self, other: Vector3<S>) -> Self::Output {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<S> ops::Add<&Vector3<S>> for Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn add(self, other: &Vector3<S>) -> Self::Output {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,               
        }
    }
}

impl<'a, 'b, S> ops::Add<&'b Vector3<S>> for &'a Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn add(self, other: &'b Vector3<S>) -> Self::Output {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<S> ops::Sub<Point3<S>> for &Point3<S> where S: Scalar {
    type Output = Vector3<S>;

    fn sub(self, other: Point3<S>) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<S> ops::Sub<Point3<S>> for Point3<S> where S: Scalar {
    type Output = Vector3<S>;

    fn sub(self, other: Point3<S>) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<S> ops::Sub<&Point3<S>> for Point3<S> where S: Scalar {
    type Output = Vector3<S>;

    fn sub(self, other: &Point3<S>) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,               
        }
    }
}

impl<'a, 'b, S> ops::Sub<&'b Point3<S>> for &'a Point3<S> where S: Scalar {
    type Output = Vector3<S>;

    fn sub(self, other: &'b Point3<S>) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<S> ops::Sub<Vector3<S>> for &Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn sub(self, other: Vector3<S>) -> Self::Output {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<S> ops::Sub<Vector3<S>> for Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn sub(self, other: Vector3<S>) -> Self::Output {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<S> ops::Sub<&Vector3<S>> for Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn sub(self, other: &Vector3<S>) -> Self::Output {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,               
        }
    }
}

impl<'a, 'b, S> ops::Sub<&'b Vector3<S>> for &'a Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn sub(self, other: &'b Vector3<S>) -> Self::Output {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<S> ops::Mul<S> for Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn mul(self, other: S) -> Self::Output {
        Point3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<S> ops::Mul<S> for &Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn mul(self, other: S) -> Self::Output {
        Point3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl_mul_operator!(u8,    Point3<u8>,    Point3<u8>,    { x, y, z });
impl_mul_operator!(u16,   Point3<u16>,   Point3<u16>,   { x, y, z });
impl_mul_operator!(u32,   Point3<u32>,   Point3<u32>,   { x, y, z });
impl_mul_operator!(u64,   Point3<u64>,   Point3<u64>,   { x, y, z });
impl_mul_operator!(u128,  Point3<u128>,  Point3<u128>,  { x, y, z });
impl_mul_operator!(usize, Point3<usize>, Point3<usize>, { x, y, z });
impl_mul_operator!(i8,    Point3<i8>,    Point3<i8>,    { x, y, z });
impl_mul_operator!(i16,   Point3<i16>,   Point3<i16>,   { x, y, z });
impl_mul_operator!(i32,   Point3<i32>,   Point3<i32>,   { x, y, z });
impl_mul_operator!(i64,   Point3<i64>,   Point3<i64>,   { x, y, z });
impl_mul_operator!(i128,  Point3<i128>,  Point3<i128>,  { x, y, z });
impl_mul_operator!(isize, Point3<isize>, Point3<isize>, { x, y, z });
impl_mul_operator!(f32,   Point3<f32>,   Point3<f32>,   { x, y, z });
impl_mul_operator!(f64,   Point3<f64>,   Point3<f64>,   { x, y, z });

impl<S> ops::Div<S> for Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn div(self, other: S) -> Self::Output {
        Point3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<S> ops::Div<S> for &Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn div(self, other: S) -> Self::Output {
        Point3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<S> ops::Rem<S> for Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn rem(self, other: S) -> Self::Output {
        let x = self.x % other;
        let y = self.y % other;
        let z = self.z % other;
        
        Point3::new(x, y, z)
    }
}

impl<S> ops::Rem<S> for &Point3<S> where S: Scalar {
    type Output = Point3<S>;

    fn rem(self, other: S) -> Self::Output {
        let x = self.x % other;
        let y = self.y % other;
        let z = self.z % other;
        
        Point3::new(x, y, z)
    }
}

impl<S> ops::AddAssign<Vector3<S>> for Point3<S> where S: Scalar {
    fn add_assign(&mut self, other: Vector3<S>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<S> ops::AddAssign<&Vector3<S>> for Point3<S> where S: Scalar {
    fn add_assign(&mut self, other: &Vector3<S>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<S> ops::SubAssign<Vector3<S>> for Point3<S> where S: Scalar {
    fn sub_assign(&mut self, other: Vector3<S>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<S> ops::SubAssign<&Vector3<S>> for Point3<S> where S: Scalar {
    fn sub_assign(&mut self, other: &Vector3<S>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<S> ops::MulAssign<S> for Point3<S> where S: Scalar {
    fn mul_assign(&mut self, other: S) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<S> ops::DivAssign<S> for Point3<S> where S: Scalar {
    fn div_assign(&mut self, other: S) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl<S> ops::RemAssign<S> for Point3<S> where S: Scalar {
    fn rem_assign(&mut self, other: S) {
        self.x %= other;
        self.y %= other;
        self.z %= other;
    }
}

impl<S> approx::AbsDiffEq for Point3<S> where S: ScalarFloat {
    type Epsilon = <S as approx::AbsDiffEq>::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        S::abs_diff_eq(&self.x, &other.x, epsilon) && 
        S::abs_diff_eq(&self.y, &other.y, epsilon) &&
        S::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl<S> approx::RelativeEq for Point3<S> where S: ScalarFloat {
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        S::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
        S::relative_eq(&self.y, &other.y, epsilon, max_relative) &&
        S::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for Point3<S> where S: ScalarFloat {
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        S::ulps_eq(&self.x, &other.x, epsilon, max_ulps) &&
        S::ulps_eq(&self.y, &other.y, epsilon, max_ulps) &&
        S::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
    }
}

impl<S> DotProduct<Point3<S>> for Point3<S> where S: Scalar {
    type Output = S;

    #[inline]
    fn dot(self, other: Point3<S>) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<S> DotProduct<&Point3<S>> for Point3<S> where S: Scalar {
    type Output = S;

    #[inline]
    fn dot(self, other: &Point3<S>) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<S> DotProduct<Point3<S>> for &Point3<S> where S: Scalar {
    type Output = S;

    #[inline]
    fn dot(self, other: Point3<S>) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<'a, 'b, S> DotProduct<&'a Point3<S>> for &'b Point3<S> where S: Scalar {
    type Output = S;
    
    #[inline]
    fn dot(self, other: &'a Point3<S>) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<S> Magnitude for Point3<S> where S: ScalarFloat {
    type Output = S;

    /// Compute the norm (length) of a vector.
    fn magnitude(&self) -> Self::Output {
        Self::Output::sqrt(self.magnitude_squared())
    }

    /// Compute the squared length of a vector.
    fn magnitude_squared(&self) -> Self::Output {
        DotProduct::dot(self, self)
    }

    /// Convert a vector into a unit vector.
    fn normalize(&self) -> Self {
        self / self.magnitude()
    }

    /// Normalize a vector with a specified magnitude.
    fn normalize_to(&self, magnitude: Self::Output) -> Self {
        self * (magnitude / self.magnitude())
    }
}

impl<S> Metric<Point3<S>> for Point3<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: Point3<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}

impl<S> Metric<&Point3<S>> for Point3<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: &Point3<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}

impl<S> Metric<Point3<S>> for &Point3<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: Point3<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}

impl<'a, 'b, S> Metric<&'a Point3<S>> for &'b Point3<S> where S: ScalarFloat {
    type Output = S;

    #[inline]
    fn distance_squared(self, to: &Point3<S>) -> Self::Output {
        (self - to).magnitude_squared()
    }
}

