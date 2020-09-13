use approx::{
    ulps_eq,
};
use angle::{
    Radians,
};
use scalar::{
    Scalar,
    ScalarFloat,
};
use structure::{
    Angle,
    DotProduct,
    CrossProduct,
    Euclidean,
    One,
    InvertibleSquareMatrix,
    Magnitude,
};
use matrix::{
    Matrix2,
    Matrix3,
};
use point::{
    Point2,
    Point3,
};
use vector::{
    Vector2,
    Vector3,
};
use quaternion::Quaternion;
use std::fmt;
use std::iter;
use std::ops;


/// A type implementing this trait represents a type that acts as a generic 
/// rotation. A rotation is an operation that creates circular motions and 
/// preserves at least one point. Rotations preserve the length of vectors and 
/// therefore act as a class of rigid body transformations.
pub trait Rotation<P> where 
    P: Euclidean,
    Self: Sized + Copy + One,
{
    fn look_at(direction: P::Difference, up: P::Difference) -> Self;

    /// Construct a rotation that rotates the shortest angular distance between two unit
    /// vectors.
    fn between_vectors(v1: P::Difference, v2: P::Difference) -> Self;

    /// Apply the rotation operation to a vector.
    fn rotate_vector(&self, vector: P::Difference) -> P::Difference;

    /// Construct a rotation that rotates a vector in the opposite direction of `self`. In particular,
    /// given a rotation operator that rotates a vector about an axis by an angle `theta`, construct 
    /// a rotation that rotates a vector about the same axis by an angle `-theta`.
    fn inverse(&self) -> Self;

    /// Construct a rotation that rotates a point in the opposite direction of `self`. In particular,
    /// given a rotation operator that rotates an object about an axis by an angle `theta`, construct 
    /// a rotation that rotates a point about the same axis by an angle `-theta`.
    fn rotate_point(&self, point: P) -> P { 
        P::from_vector(self.rotate_vector(point.to_vector()))
    }
}

/// A trait that implements rotation operators in two-dimensions. Two-dimensional 
/// rotations are different than three-dimensional rotations in that mathematically 
/// we cannot define an axis of rotation in two dimensions. Instead we have to talk 
/// about rotating the xy-plane by an angle. In low-dimensional settings, the 
/// notion of rotation axis is really only well-defined in three dimensions, since only in
/// three-dimensions is every plane guaranteed to have a normal vector. If one wants to 
/// talk about rotating a vector in the the xy-plane about a normal vector, we are implicitly 
/// rotating about the z-axis in three-dimensions. Otherwise, avoiding cheating in that 
/// fashion requires abolishing coordinate axes and only talking about  (hyper)planes, but 
/// this requires different mathematics than is typically used in computer graphics.
pub trait Rotation2<S> where 
    S: ScalarFloat,
    Self: Rotation<Point2<S>> + Into<Matrix2<S>> + Into<RotationMatrix2<S>>,
{
    /// Rotate a two-dimensional vector in the xy-plane by an angle `angle`.
    fn from_angle<A: Into<Radians<S>>>(angle: A) -> Self;
}

/// A trait that implements rotation operators in three dimensions.
pub trait Rotation3<S> where 
    S: ScalarFloat,
    Self: Rotation<Point3<S>>,
    Self: Into<Matrix3<S>> + Into<RotationMatrix3<S>> + Into<Quaternion<S>>,
{
    /// Construct a new three-dimensional rotation about an axis `axis` by an amount `angle`.
    fn from_axis_angle<A: Into<Radians<S>>>(axis: Vector3<S>, angle: A) -> Self;

    /// Construct a new three-dimensional rotation about the x-axis in the yz-plane by an amount 
    /// `angle`.
    #[inline]
    fn from_angle_x<A: Into<Radians<S>>>(angle: A) -> Self {
        Self::from_axis_angle(Vector3::unit_x(), angle)
    }

    /// Construct a new three-dimensional rotation about the y-axis in the xz-plane by an amount 
    /// `angle`.
    #[inline]
    fn from_angle_y<A: Into<Radians<S>>>(angle: A) -> Self {
        Self::from_axis_angle(Vector3::unit_y(), angle)
    }

    /// Construct a new three-dimensional rotation about the z-axis in the xy-plane by an amount 
    /// `angle`.
    #[inline]
    fn from_angle_z<A: Into<Radians<S>>>(angle: A) -> Self {
        Self::from_axis_angle(Vector3::unit_z(), angle)
    }
}


/// A rotation operator in two dimensions.
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct RotationMatrix2<S> {
    /// The underlying matrix for the rotation.
    matrix: Matrix2<S>,
}

impl<S> fmt::Debug for RotationMatrix2<S> where S: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RotationMatrix2 ")?;
        <[S; 4] as fmt::Debug>::fmt(self.matrix.as_ref(), f)
    }
}

impl<S> fmt::Display for RotationMatrix2<S> where S: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RotationMatrix2 ")?;
        <[S; 4] as fmt::Debug>::fmt(self.matrix.as_ref(), f)
    }
}

impl<S> From<RotationMatrix2<S>> for Matrix2<S> where S: Copy {
    #[inline]
    fn from(rotation: RotationMatrix2<S>) -> Matrix2<S> {
        rotation.matrix
    }
}

impl<S> AsRef<Matrix2<S>> for RotationMatrix2<S> {
    #[inline]
    fn as_ref(&self) -> &Matrix2<S> {
        &self.matrix
    }
}

impl<S> ops::Mul<RotationMatrix2<S>> for RotationMatrix2<S> where S: Scalar {
    type Output = RotationMatrix2<S>;
    
    #[inline]
    fn mul(self, other: RotationMatrix2<S>) -> Self::Output {
        RotationMatrix2 { matrix: self.matrix * other.matrix }
    }
}

impl<S> ops::Mul<RotationMatrix2<S>> for &RotationMatrix2<S> where S: Scalar {
    type Output = RotationMatrix2<S>;
    
    #[inline]
    fn mul(self, other: RotationMatrix2<S>) -> Self::Output {
        RotationMatrix2 { matrix: self.matrix * other.matrix }
    }
}

impl<S> ops::Mul<&RotationMatrix2<S>> for RotationMatrix2<S> where S: Scalar {
    type Output = RotationMatrix2<S>;
    
    #[inline]
    fn mul(self, other: &RotationMatrix2<S>) -> Self::Output {
        RotationMatrix2 { matrix: self.matrix * other.matrix }
    }
}

impl<'a, 'b, S> ops::Mul<&'a RotationMatrix2<S>> for &'b RotationMatrix2<S> where S: Scalar {
    type Output = RotationMatrix2<S>;
    
    #[inline]
    fn mul(self, other: &'a RotationMatrix2<S>) -> Self::Output {
        RotationMatrix2 { matrix: self.matrix * other.matrix }
    }
}

impl<S> One for RotationMatrix2<S> where S: Scalar {
    #[inline]
    fn one() -> RotationMatrix2<S> {
        RotationMatrix2 { matrix: Matrix2::one() }
    }
}

impl<S> iter::Product<RotationMatrix2<S>> for RotationMatrix2<S> where S: Scalar {
    #[inline]
    fn product<I: Iterator<Item = RotationMatrix2<S>>>(iter: I) -> RotationMatrix2<S> {
        iter.fold(RotationMatrix2::one(), ops::Mul::mul)
    }
}

impl<'a, S> iter::Product<&'a RotationMatrix2<S>> for RotationMatrix2<S> where S: 'a + Scalar {
    #[inline]
    fn product<I: Iterator<Item = &'a RotationMatrix2<S>>>(iter: I) -> RotationMatrix2<S> {
        iter.fold(RotationMatrix2::one(), ops::Mul::mul)
    }
}

impl<S> approx::AbsDiffEq for RotationMatrix2<S> where S: ScalarFloat {
    type Epsilon = <S as approx::AbsDiffEq>::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        Matrix2::abs_diff_eq(&self.matrix, &other.matrix, epsilon)
    }
}

impl<S> approx::RelativeEq for RotationMatrix2<S> where S: ScalarFloat {
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        Matrix2::relative_eq(&self.matrix, &other.matrix, epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for RotationMatrix2<S> where S: ScalarFloat {
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        Matrix2::ulps_eq(&self.matrix, &other.matrix, epsilon, max_ulps)
    }
}

impl<S> Rotation2<S> for RotationMatrix2<S> where S: ScalarFloat {
    fn from_angle<A: Into<Radians<S>>>(angle: A) -> RotationMatrix2<S> {
        RotationMatrix2 {
            matrix: Matrix2::from_angle(angle),
        }
    }
}

impl<S> Rotation<Point2<S>> for RotationMatrix2<S> where S: ScalarFloat { 
    #[inline]
    fn look_at(dir: Vector2<S>, up: Vector2<S>) -> RotationMatrix2<S> {
        RotationMatrix2 {
            matrix: Matrix2::look_at(dir, up),
        }
    }

    #[inline]
    fn between_vectors(a: Vector2<S>, b: Vector2<S>) -> RotationMatrix2<S> {
        Rotation2::from_angle(Radians::acos(DotProduct::dot(a, b)))
    }

    #[inline]
    fn rotate_vector(&self, vector: Vector2<S>) -> Vector2<S> {
        self.matrix * vector
    }

    #[inline]
    fn inverse(&self) -> RotationMatrix2<S> {
        RotationMatrix2 {
            matrix: self.matrix.inverse().unwrap(),
        }
    }
}


/// A rotation operator in three dimensions.
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct RotationMatrix3<S> {
    /// The underlying matrix representing the rotation.
    matrix: Matrix3<S>,
}

impl<S> RotationMatrix3<S> where S: ScalarFloat {
    /// Construct a three-dimensional rotation matrix from a quaternion.
    #[inline]
    pub fn from_quaternion(quaternion: &Quaternion<S>) -> RotationMatrix3<S> {
        RotationMatrix3 {
            matrix: Matrix3::from(quaternion),
        }
    }
}

impl<S> fmt::Debug for RotationMatrix3<S> where S: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RotationMatrix3 ")?;
        <[S; 9] as fmt::Debug>::fmt(self.matrix.as_ref(), f)
    }
}

impl<S> fmt::Display for RotationMatrix3<S> where S: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RotationMatrix3 ")?;
        <[S; 9] as fmt::Debug>::fmt(self.matrix.as_ref(), f)
    }
}

impl<S> From<RotationMatrix3<S>> for Matrix3<S> where S: Copy {
    #[inline]
    fn from(rotation: RotationMatrix3<S>) -> Matrix3<S> {
        rotation.matrix
    }
}

impl<S> From<Quaternion<S>> for RotationMatrix3<S> where S: ScalarFloat {
    #[inline]
    fn from(quaternion: Quaternion<S>) -> RotationMatrix3<S> {
        RotationMatrix3::from_quaternion(&quaternion)
    }
}

impl<S> From<RotationMatrix3<S>> for Quaternion<S> where S: ScalarFloat {
    #[inline]
    fn from(rotation: RotationMatrix3<S>) -> Quaternion<S> {
        Quaternion::from(&rotation.matrix)
    }
}

impl<S> AsRef<Matrix3<S>> for RotationMatrix3<S> {
    #[inline]
    fn as_ref(&self) -> &Matrix3<S> {
        &self.matrix
    }
}

impl<S> ops::Mul<RotationMatrix3<S>> for RotationMatrix3<S> where S: Scalar {
    type Output = RotationMatrix3<S>;
    
    #[inline]
    fn mul(self, other: RotationMatrix3<S>) -> Self::Output {
        RotationMatrix3 { matrix: self.matrix * other.matrix }
    }
}

impl<S> ops::Mul<RotationMatrix3<S>> for &RotationMatrix3<S> where S: Scalar {
    type Output = RotationMatrix3<S>;
    
    #[inline]
    fn mul(self, other: RotationMatrix3<S>) -> Self::Output {
        RotationMatrix3 { matrix: self.matrix * other.matrix }
    }
}

impl<S> ops::Mul<&RotationMatrix3<S>> for RotationMatrix3<S> where S: Scalar {
    type Output = RotationMatrix3<S>;
    
    #[inline]
    fn mul(self, other: &RotationMatrix3<S>) -> Self::Output {
        RotationMatrix3 { matrix: self.matrix * other.matrix }
    }
}

impl<'a, 'b, S> ops::Mul<&'a RotationMatrix3<S>> for &'b RotationMatrix3<S> where S: Scalar {
    type Output = RotationMatrix3<S>;
    
    #[inline]
    fn mul(self, other: &'a RotationMatrix3<S>) -> Self::Output {
        RotationMatrix3 { matrix: self.matrix * other.matrix }
    }
}

impl<S> One for RotationMatrix3<S> where S: Scalar {
    #[inline]
    fn one() -> RotationMatrix3<S> {
        RotationMatrix3 { matrix: Matrix3::one() }
    }
}

impl<S> iter::Product<RotationMatrix3<S>> for RotationMatrix3<S> where S: Scalar {
    #[inline]
    fn product<I: Iterator<Item = RotationMatrix3<S>>>(iter: I) -> RotationMatrix3<S> {
        iter.fold(RotationMatrix3::one(), ops::Mul::mul)
    }
}

impl<'a, S> iter::Product<&'a RotationMatrix3<S>> for RotationMatrix3<S> where S: 'a + Scalar {
    #[inline]
    fn product<I: Iterator<Item = &'a RotationMatrix3<S>>>(iter: I) -> RotationMatrix3<S> {
        iter.fold(RotationMatrix3::one(), ops::Mul::mul)
    }
}

impl<S> approx::AbsDiffEq for RotationMatrix3<S> where S: ScalarFloat {
    type Epsilon = <S as approx::AbsDiffEq>::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        Matrix3::abs_diff_eq(&self.matrix, &other.matrix, epsilon)
    }
}

impl<S> approx::RelativeEq for RotationMatrix3<S> where S: ScalarFloat {
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        Matrix3::relative_eq(&self.matrix, &other.matrix, epsilon, max_relative)
    }
}

impl<S> approx::UlpsEq for RotationMatrix3<S> where S: ScalarFloat {
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        Matrix3::ulps_eq(&self.matrix, &other.matrix, epsilon, max_ulps)
    }
}

impl<S> Rotation<Point3<S>> for Quaternion<S> where S: ScalarFloat {
    #[inline]
    fn look_at(dir: Vector3<S>, up: Vector3<S>) -> Quaternion<S> {
        Matrix3::look_at(dir, up).into()
    }

    #[inline]
    fn between_vectors(v1: Vector3<S>, v2: Vector3<S>) -> Quaternion<S> {
        let k_cos_theta = v1.dot(v2);

        // The vectors point in the same direction.
        if ulps_eq!(k_cos_theta, S::one()) {
            return Quaternion::<S>::one();
        }

        let k = (v1.magnitude_squared() * v2.magnitude_squared()).sqrt();

        // The vectors point in opposite directions.
        if ulps_eq!(k_cos_theta / k, -S::one()) {
            let mut orthogonal = v1.cross(Vector3::unit_x());
            if ulps_eq!(orthogonal.magnitude_squared(), S::zero()) {
                orthogonal = v1.cross(Vector3::unit_y());
            }
            return Quaternion::from_sv(S::zero(), orthogonal.normalize());
        }

        // The vectors point in any other direction.
        Quaternion::from_sv(k + k_cos_theta, v1.cross(v2)).normalize()
    }

    #[inline]
    fn rotate_vector(&self, vector: Vector3<S>) -> Vector3<S> {
        let rotation_matrix = Matrix3::from(self);
        rotation_matrix * vector
    }

    #[inline]
    fn inverse(&self) -> Quaternion<S> {
        self.conjugate() / self.magnitude_squared()
    }
}

impl<S> Rotation3<S> for RotationMatrix3<S> where S: ScalarFloat {
    fn from_axis_angle<A: Into<Radians<S>>>(axis: Vector3<S>, angle: A) -> RotationMatrix3<S> {
        RotationMatrix3 {
            matrix: Matrix3::from_axis_angle(axis, angle),
        }
    }
}

impl<S> Rotation3<S> for Quaternion<S> where S: ScalarFloat {
    #[inline]
    fn from_axis_angle<A: Into<Radians<S>>>(axis: Vector3<S>, angle: A) -> Quaternion<S> {
        let (sin_angle, cos_angle) = Radians::sin_cos(angle.into() * num_traits::cast(0.5_f64).unwrap());
        Quaternion::from_sv(cos_angle, axis * sin_angle)
    }
}

impl<S> Rotation<Point3<S>> for RotationMatrix3<S> where S: ScalarFloat { 
    #[inline]
    fn look_at(dir: Vector3<S>, up: Vector3<S>) -> RotationMatrix3<S> {
        RotationMatrix3 {
            matrix: Matrix3::look_at(dir, up),
        }
    }

    #[inline]
    fn between_vectors(v1: Vector3<S>, v2: Vector3<S>) -> RotationMatrix3<S> {
        let q: Quaternion<S> = Rotation::between_vectors(v1, v2);
        q.into()
    }

    #[inline]
    fn rotate_vector(&self, vector: Vector3<S>) -> Vector3<S> {
        self.matrix * vector
    }

    #[inline]
    fn inverse(&self) -> RotationMatrix3<S> {
        RotationMatrix3 {
            matrix: self.matrix.inverse().unwrap(),
        }
    }
}

