extern crate gdmath;
extern crate num_traits;
extern crate proptest;

use proptest::prelude::*;
use gdmath::{
    Matrix2,
    Matrix3,
    Matrix4,
    Matrix, 
    Scalar,
    ScalarFloat,
};

fn any_matrix2<S>() -> impl Strategy<Value = Matrix2<S>> where S: Scalar + Arbitrary {
    any::<(S, S, S, S)>().prop_map(
    |(c0r0, c0r1, c1r0, c1r1)| Matrix2::new(c0r0, c0r1, c1r0, c1r1)
    )
}

fn any_matrix3<S>() -> impl Strategy<Value = Matrix3<S>> where S: Scalar + Arbitrary {
    any::<((S, S, S), (S, S, S), (S, S, S))>().prop_map(
        |((c0r0, c0r1, c0r2), (c1r0, c1r1, c1r2), (c2r0, c2r1, c2r2))| {
            Matrix3::new(c0r0, c0r1, c0r2, c1r0, c1r1, c1r2, c2r0, c2r1, c2r2)
        }
    )
}

fn any_matrix4<S>() -> impl Strategy<Value = Matrix4<S>> where S: Scalar + Arbitrary {
    any::<((S, S, S, S), (S, S, S, S), (S, S, S, S), (S, S, S, S))>().prop_map(
        |((c0r0, c0r1, c0r2, c0r3), (c1r0, c1r1, c1r2, c1r3), (c2r0, c2r1, c2r2, c2r3), (c3r0, c3r1, c3r2, c3r3))| {
            Matrix4::new(c0r0,c0r1, c0r2, c0r3, c1r0, c1r1, c1r2, c1r3, c2r0, c2r1, c2r2,  c2r3, c3r0, c3r1, c3r2, c3r3)
        }
    )
}


/// Generate the properties for matrix addition over floating point scalars.
///
/// `$TestModuleName` is a name we give to the module we place the properties in to separate them
///  from each other for each field type to prevent namespace collisions.
/// `$MatrixN` denotes the name of the matrix type.
/// `$ScalarType` denotes the underlying system of numbers that compose the matrices.
/// `$Generator` is the name of a function or closure for generating examples.
macro_rules! approx_addition_props {
    ($TestModuleName:ident, $MatrixN:ident, $ScalarType:ty, $Generator:ident, $tolerance:expr) => {
    #[cfg(test)]
    mod $TestModuleName {
        use proptest::prelude::*;
        use gdmath::approx::relative_eq;
        use gdmath::{$MatrixN, Zero};

        proptest! {
            /// A zero matrix should act as the additive unit element for matrices over 
            /// their underlying scalars. 
            ///
            /// Given a matrix `m` and a zero matrix `0`
            /// ```
            /// 0 + m = m
            /// ```
            #[test]
            fn prop_matrix_additive_identity(m in super::$Generator::<$ScalarType>()) {
                let zero_mat = $MatrixN::zero();
                prop_assert_eq!(zero_mat + m, m);
            }
        
            /// A zero matrix should act as the additive unit element for matrices over 
            /// their underlying scalars. 
            ///
            /// Given a matrix `m` and a zero matrix `0`
            /// ```
            /// m + 0 = m
            /// ```
            #[test]
            fn prop_vector_times_zero_equals_zero(m in super::$Generator::<$ScalarType>()) {
                let zero_mat = $MatrixN::zero();
                prop_assert_eq!(m + zero_mat, m);
            }

            /// Matrix addition over exact scalars is commutative.
            ///
            /// Given matrices `m1` and `m2`
            /// ```
            /// m1 + m2 ~= m2 + m1
            /// ```
            #[test]
            fn prop_matrix_addition_approx_commutative(m1 in super::$Generator(), m2 in super::$Generator::<$ScalarType>()) {
                prop_assert!(relative_eq!(m1 + m2, m2 + m1, epsilon = $tolerance));
            }

            /// Matrix addition over exact scalars is associative.
            ///
            /// Given matrices `m1`, `m2`, and `m3`
            /// ```
            /// (m1 + m2) + m3 ~= m1 + (m2 + m3)
            /// ```
            #[test]
            fn prop_matrix_addition_approx_associative(
                m1 in super::$Generator::<$ScalarType>(), 
                m2 in super::$Generator::<$ScalarType>(), m3 in super::$Generator::<$ScalarType>()) {

                prop_assert!(relative_eq!((m1 + m2) + m3, m1 + (m2 + m3), epsilon = $tolerance));
            }

            /// The sum of a matrix and it's additive inverse is the same as subtracting the two matrices from
            /// each other.
            ///
            /// Given matrices `m1` and `m2`
            /// ```
            /// m1 + (-m2) = m1 - m2
            /// ```
            #[test]
            fn prop_matrix_subtraction(
                m1 in super::$Generator::<$ScalarType>(), m2 in super::$Generator::<$ScalarType>()) {
                
                prop_assert_eq!(m1 + (-m2), m1 - m2);
            }
        }
    }
    }
}

approx_addition_props!(matrix2_f64_addition_props, Matrix2, f64, any_matrix2, 1e-7);
approx_addition_props!(matrix3_f64_addition_props, Matrix3, f64, any_matrix3, 1e-7);
approx_addition_props!(matrix4_f64_addition_props, Matrix4, f64, any_matrix4, 1e-7);


/// Generate the properties for matrix addition over exact scalars. We define an exact
/// scalar type as a type where scalar arithmetic is exact (e.g. integers).
///
/// `$TestModuleName` is a name we give to the module we place the properties in to separate them
///  from each other for each field type to prevent namespace collisions.
/// `$MatrixN` denotes the name of the matrix type.
/// `$ScalarType` denotes the underlying system of numbers that compose the matrices.
/// `$Generator` is the name of a function or closure for generating examples.
macro_rules! exact_addition_props {
    ($TestModuleName:ident, $MatrixN:ident, $ScalarType:ty, $Generator:ident) => {
    #[cfg(test)]
    mod $TestModuleName {
        use proptest::prelude::*;
        use gdmath::{$MatrixN, Zero};

        proptest! {
            /// A zero matrix should act as the additive unit element for matrices over 
            /// their underlying scalars. 
            ///
            /// Given a matrix `m`
            /// ```
            /// 0 + m = m
            /// ```
            #[test]
            fn prop_matrix_additive_identity(m in super::$Generator::<$ScalarType>()) {
                let zero_mat = $MatrixN::zero();
                prop_assert_eq!(zero_mat + m, m);
            }
        
            /// A zero matrix should act as the additive unit element for matrices over 
            /// their underlying scalars. 
            ///
            /// Given a matrix `m`
            /// ```
            /// m + 0 = m
            /// ```
            #[test]
            fn prop_vector_times_zero_equals_zero(m in super::$Generator::<$ScalarType>()) {
                let zero_mat = $MatrixN::zero();
                prop_assert_eq!(m + zero_mat, m);
            }

            /// Matrix addition over exact scalars is commutative.
            ///
            /// Given matrices `m1` and `m2`
            /// ```
            /// m1 + m2 = m2 + m1
            /// ```
            #[test]
            fn prop_matrix_addition_commutative(m1 in super::$Generator(), m2 in super::$Generator::<$ScalarType>()) {
                prop_assert_eq!(m1 + m2, m2 + m1);
            }

            /// Matrix addition over exact scalars is associative.
            ///
            /// Given matrices `m1`, `m2`, and `m3`
            /// ```
            /// (m1 + m2) + m3 = m1 + (m2 + m3)
            /// ```
            #[test]
            fn prop_matrix_addition_associative(
                m1 in super::$Generator(), m2 in super::$Generator(), m3 in super::$Generator::<$ScalarType>()) {

                prop_assert_eq!((m1 + m2) + m3, m1 + (m2 + m3));
            }
        }
    }
    }
}

exact_addition_props!(matrix2_u32_addition_props, Matrix2, u32, any_matrix2);
exact_addition_props!(matrix2_i32_addition_props, Matrix2, i32, any_matrix2);
exact_addition_props!(matrix3_u32_addition_props, Matrix3, u32, any_matrix3);
exact_addition_props!(matrix3_i32_addition_props, Matrix3, i32, any_matrix3);
exact_addition_props!(matrix4_u32_addition_props, Matrix4, u32, any_matrix4);
exact_addition_props!(matrix4_i32_addition_props, Matrix4, i32, any_matrix4);


/// Generate the properties for the multiplication of matrices of floating point scalars 
/// by floating point scalars.
///
/// `$TestModuleName` is a name we give to the module we place the properties in to separate them
///  from each other for each field type to prevent namespace collisions.
/// `$MatrixN` denotes the name of the matrix type.
/// `$ScalarType` denotes the underlying system of numbers that compose the matrices.
/// `$Generator` is the name of a function or closure for generating examples.
macro_rules! approx_scalar_multiplication_props {
    ($TestModuleName:ident, $MatrixN:ident, $ScalarType:ty, $Generator:ident, $tolerance:expr) => {
    #[cfg(test)]
    mod $TestModuleName {
        use proptest::prelude::*;
        use gdmath::approx::relative_eq;
        use gdmath::{$MatrixN, Zero};

        proptest! {
            /// Multiplication of matrices by scalars is compatible with matrix addition.
            ///
            /// Given matrices `m1` and `m2`, and a scalar `c`
            /// ```
            /// c * (m1 + m2) ~= c * m1 + c * m2
            /// ```
            #[test]
            fn prop_scalar_matrix_multiplication_compatible_addition(
                c in any::<$ScalarType>(),
                m1 in super::$Generator::<$ScalarType>(), m2 in super::$Generator::<$ScalarType>()) {

                prop_assert!(relative_eq!(c * (m1 + m2), c * m1 + c * m2, epsilon = $tolerance));
            }

            /// Multiplication of matrices by scalars is compatible with matrix subtraction.
            ///
            /// Given matrices `m1` and `m2`, and a scalar `c`
            /// ```
            /// c * (m1 - m2) ~= c * m1 - c * m2
            /// ```
            #[test]
            fn prop_scalar_matrix_multiplication_compatible_subtraction(
                c in any::<$ScalarType>(),
                m1 in super::$Generator::<$ScalarType>(), m2 in super::$Generator::<$ScalarType>()) {

                prop_assert!(relative_eq!(c * (m1 - m2), c * m1 - c * m2, epsilon = $tolerance));
            }

            /// Multiplication of a matrix by a scalar zero is the zero matrix.
            ///
            /// Given a matrix `m` and a zero scalar `0`
            /// ```
            /// 0 * m = m * 0 = 0
            /// ```
            /// Note that we diverge from traditional formalisms of matrix arithmetic in that we allow
            /// multiplication of matrices by scalars on the right-hand side as well as left-hand side. 
            #[test]
            fn prop_zero_times_matrix_equals_zero_matrix(m in super::$Generator::<$ScalarType>()) {
                let zero: $ScalarType = num_traits::Zero::zero();
                let zero_mat = $MatrixN::zero();
                prop_assert_eq!(zero * m, zero_mat);
                prop_assert_eq!(m * zero, zero_mat);
            }

            /// Multiplication of a matrix by a scalar one is the original matrix.
            ///
            /// Given a matrix `m` and a unit scalar `1`
            /// ```
            /// 1 * m = m * 1 = m
            /// ```
            /// Note that we diverge from traditional formalisms of matrix arithmetic in that we allow
            /// multiplication of matrices by scalars on the right-hand side as well as left-hand side. 
            #[test]
            fn prop_one_times_matrix_equals_matrix(m in super::$Generator::<$ScalarType>()) {
                let one: $ScalarType = num_traits::One::one();
                prop_assert_eq!(one * m, m);
                prop_assert_eq!(m * one, m);
            }

            /// Multiplication of a matrix by a scalar negative one is the additive inverse of the 
            /// original matrix.
            ///
            /// Given a matrix `m` and a negative unit scalar `-1`
            /// ```
            /// (-1) * m = = m * (-1) = -m
            /// ```
            /// Note that we diverge from traditional formalisms of matrix arithmetic in that we allow
            /// multiplication of matrices by scalars on the right-hand side as well as left-hand side. 
            #[test]
            fn prop_negative_one_times_matrix_equals_negative_matrix(m in super::$Generator::<$ScalarType>()) {
                let one: $ScalarType = num_traits::One::one();
                let minus_one = -one;
                prop_assert_eq!(minus_one * m, -m);
            }

            /// Multiplication of a matrix by a scalar commutes with scalars.
            ///
            /// Given a matrix `m` and a scalar `c`
            /// ```
            /// c * m ~= m * c
            /// ```
            /// Note that we diverse from traditional formalisms of matrix arithmetic in that we allow
            /// multiplication of matrices by scalars on the left-hand side as well as the 
            /// right-hand side.
            #[test]
            fn prop_scalar_matrix_multiplication_commutative(
                c in any::<$ScalarType>(), m in super::$Generator::<$ScalarType>()) {

                prop_assert!(relative_eq!(c * m, m * c, epsilon = $tolerance));
            }
        }
    }
    }
}

approx_scalar_multiplication_props!(matrix2_f64_scalar_multiplication_props, Matrix2, f64, any_matrix2, 1e-7);
approx_scalar_multiplication_props!(matrix3_f64_scalar_multiplication_props, Matrix3, f64, any_matrix3, 1e-7);
approx_scalar_multiplication_props!(matrix4_f64_scalar_multiplication_props, Matrix4, f64, any_matrix4, 1e-7);


/// Generate the properties for the multiplication of matrices of integer scalars 
/// by integers.
///
/// `$TestModuleName` is a name we give to the module we place the properties in to separate them
///  from each other for each field type to prevent namespace collisions.
/// `$MatrixN` denotes the name of the matrix type.
/// `$ScalarType` denotes the underlying system of numbers that compose the matrices.
/// `$Generator` is the name of a function or closure for generating examples.
macro_rules! exact_scalar_multiplication_props {
    ($TestModuleName:ident, $MatrixN:ident, $ScalarType:ty, $Generator:ident) => {
    #[cfg(test)]
    mod $TestModuleName {
        use proptest::prelude::*;
        use gdmath::{$MatrixN, Zero};

        proptest! {
            /// Multiplication of matrices by scalars is compatible with matrix addition.
            ///
            /// Given matrices `m1` and `m2`, and a scalar `c`
            /// ```
            /// c * (m1 + m2) = c * m1 + c * m2
            /// ```
            #[test]
            fn prop_scalar_matrix_multiplication_compatible_addition(
                c in any::<$ScalarType>(),
                m1 in super::$Generator::<$ScalarType>(), m2 in super::$Generator::<$ScalarType>()) {

                prop_assert_eq!(c * (m1 + m2), c * m1 + c * m2);
            }

            /// Multiplication of matrices by scalars is compatible with matrix subtraction.
            ///
            /// Given matrices `m1` and `m2`, and a scalar `c`
            /// ```
            /// c * (m1 - m2) = c * m1 - c * m2
            /// ```
            #[test]
            fn prop_scalar_matrix_multiplication_compatible_subtraction(
                c in any::<$ScalarType>(),
                m1 in super::$Generator::<$ScalarType>(), m2 in super::$Generator::<$ScalarType>()) {

                prop_assert_eq!(c * (m1 - m2), c * m1 - c * m2);
            }

            /// Multiplication of a matrix by a scalar zero is the zero matrix.
            ///
            /// Given a matrix `m` and a zero scalar `0`
            /// ```
            /// 0 * m = m * 0 = 0
            /// ```
            /// Note that we diverge from tradition formalisms of matrix arithmetic in that we allow
            /// multiplication of matrices by scalars on the right-hand side as well as left-hand side. 
            #[test]
            fn prop_zero_times_matrix_equals_zero_matrix(m in super::$Generator::<$ScalarType>()) {
                let zero: $ScalarType = num_traits::Zero::zero();
                let zero_mat = $MatrixN::zero();
                prop_assert_eq!(zero * m, zero_mat);
                prop_assert_eq!(m * zero, zero_mat);
            }

            /// Multiplication of a matrix by a scalar one is the original matrix.
            ///
            /// Given a matrix `m` and a unit scalar `1`
            /// ```
            /// 1 * m = m * 1 = m
            /// ```
            /// Note that we diverge from tradition formalisms of matrix arithmetic in that we allow
            /// multiplication of matrices by scalars on the right-hand side as well as left-hand side. 
            #[test]
            fn prop_one_times_matrix_equals_matrix(m in super::$Generator::<$ScalarType>()) {
                let one: $ScalarType = num_traits::One::one();
                prop_assert_eq!(one * m, m);
                prop_assert_eq!(m * one, m);
            }

            /// Multiplication of a matrix by a scalar commutes with scalars.
            ///
            /// Given a matrix `m` and a scalar `c`
            /// ```
            /// c * m = m * c
            /// ```
            /// Note that we diverse from traditional formalisms of matrix arithmetic in that we allow
            /// multiplication of matrices by scalars on the left-hand side as well as the 
            /// right-hand side.
            #[test]
            fn prop_scalar_matrix_multiplication_commutative(
                c in any::<$ScalarType>(), m in super::$Generator::<$ScalarType>()) {

                prop_assert_eq!(c * m, m * c);
            }
        }
    }
    }
}

exact_scalar_multiplication_props!(matrix2_u32_scalar_multiplication_props, Matrix2, u32, any_matrix2);
exact_scalar_multiplication_props!(matrix2_i32_scalar_multiplication_props, Matrix2, i32, any_matrix2);
exact_scalar_multiplication_props!(matrix3_u32_scalar_multiplication_props, Matrix3, u32, any_matrix3);
exact_scalar_multiplication_props!(matrix3_i32_scalar_multiplication_props, Matrix3, i32, any_matrix3);
exact_scalar_multiplication_props!(matrix4_u32_scalar_multiplication_props, Matrix4, u32, any_matrix4);
exact_scalar_multiplication_props!(matrix4_i32_scalar_multiplication_props, Matrix4, i32, any_matrix4);


/// Generate the properties for the multiplication of matrices of floating point scalars.
///
/// `$TestModuleName` is a name we give to the module we place the properties in to separate them
///  from each other for each field type to prevent namespace collisions.
/// `$MatrixN` denotes the name of the matrix type.
/// `$ScalarType` denotes the underlying system of numbers that compose the matrices.
/// `$Generator` is the name of a function or closure for generating examples.
macro_rules! approx_multiplication_props {
    ($TestModuleName:ident, $MatrixN:ident, $ScalarType:ty, $Generator:ident, $tolerance:expr) => {
    #[cfg(test)]
    mod $TestModuleName {
        use proptest::prelude::*;
        use gdmath::approx::relative_eq;
        use gdmath::{$MatrixN, One};

        proptest! {
            /// Matrix multiplication is associative.
            ///
            /// Given matrices `m1`, `m2`, and `m3`
            /// ```
            /// (m1 * m2) * m3 ~= m1 * (m2 * m3)
            /// ```
            #[test]
            fn prop_matrix_multiplication_associative(
                m1 in super::$Generator::<$ScalarType>(),
                m2 in super::$Generator::<$ScalarType>(), m3 in super::$Generator::<$ScalarType>()) {

                prop_assert!(relative_eq!((m1 * m2) * m3, m1* (m2 * m3), epsilon = $tolerance));
            }

            /// Matrix multiplication is distributive over matrix addition.
            ///
            /// Given matrices `m1`, `m2`, and `m3`
            /// ```
            /// m1 * (m2 + m3) = m1 * m2 + m1 * m3
            /// ```
            #[test]
            fn prop_matrix_multiplication_distributive(                
                m1 in super::$Generator::<$ScalarType>(),
                m2 in super::$Generator::<$ScalarType>(), m3 in super::$Generator::<$ScalarType>()) {

                prop_assert!(relative_eq!(m1 * (m2 + m3), m1 * m2 + m1 * m3, epsilon = $tolerance));
            }

            /// Matrix multiplication is compatible with scalar multiplication.
            ///
            /// Given matrices `m1` and `m2` and a scalar `c`
            /// ```
            /// c * (m1 * m2) ~= (c * m1) * m2 = m1 * (c * m2)
            /// ```
            #[test]
            fn prop_matrix_multiplication_compatible_with_scalar_multiplication(
                c in any::<$ScalarType>(),
                m1 in super::$Generator::<$ScalarType>(), m2 in super::$Generator::<$ScalarType>()) {

                prop_assert!(relative_eq!(c * (m1 * m2), (c * m1) * m2, epsilon = $tolerance));
                prop_assert!(relative_eq!((c * m1) * m2, m1 * (c * m2), epsilon = $tolerance));
            }

            /// Matrix multiplication is compatible with scalar multiplication.
            ///
            /// Given a matrix `m`, scalars `c1` and `c2`
            /// ```
            /// (c1 * c2) * m ~= c1 * (c2 * m)
            /// ```
            #[test]
            fn prop_matrix_multiplication_compatible_with_scalar_multiplication1(
                c1 in any::<$ScalarType>(), c2 in any::<$ScalarType>(), m in super::$Generator::<$ScalarType>()) {

                prop_assert!(relative_eq!((c1 * c2) * m, c1 * (c2 * m), epsilon = $tolerance));
            }

            /// Matrices over a set of floating point scalars have a multiplicative identity.
            /// 
            /// Given a matrix `m` there is a matrix `identity` such that
            /// ```
            /// m * identity = identity * m = m
            /// ```
            #[test]
            fn prop_matrix_multiplication_identity(m in super::$Generator::<$ScalarType>()) {
                let identity = $MatrixN::one();
                prop_assert_eq!(m * identity, m);
                prop_assert_eq!(identity * m, m);
            }
        }
    }
    }
}

approx_multiplication_props!(matrix2_f64_matrix_multiplication_props, Matrix2, f64, any_matrix2, 1e-7);
approx_multiplication_props!(matrix3_f64_matrix_multiplication_props, Matrix3, f64, any_matrix3, 1e-7);
approx_multiplication_props!(matrix4_f64_matrix_multiplication_props, Matrix4, f64, any_matrix4, 1e-7);


/// Generate the properties for the multiplication of matrices of floating point scalars.
///
/// `$TestModuleName` is a name we give to the module we place the properties in to separate them
///  from each other for each field type to prevent namespace collisions.
/// `$MatrixN` denotes the name of the matrix type.
/// `$ScalarType` denotes the underlying system of numbers that compose the matrices.
/// `$Generator` is the name of a function or closure for generating examples.
macro_rules! exact_multiplication_props {
    ($TestModuleName:ident, $MatrixN:ident, $ScalarType:ty, $Generator:ident) => {
    #[cfg(test)]
    mod $TestModuleName {
        use proptest::prelude::*;
        use gdmath::{$MatrixN, One};

        proptest! {
            /// Matrix multiplication is associative.
            ///
            /// Given matrices `m1`, `m2`, and `m3`
            /// ```
            /// (m1 * m2) * m3 = m1 * (m2 * m3)
            /// ```
            #[test]
            fn prop_matrix_multiplication_associative(
                m1 in super::$Generator::<$ScalarType>(),
                m2 in super::$Generator::<$ScalarType>(), m3 in super::$Generator::<$ScalarType>()) {

                prop_assert_eq!((m1 * m2) * m3, m1* (m2 * m3));
            }

            /// Matrix multiplication is distributive over matrix addition.
            ///
            /// Given matrices `m1`, `m2`, and `m3`
            /// ```
            /// m1 * (m2 + m3) = m1 * m2 + m1 * m3
            /// ```
            #[test]
            fn prop_matrix_multiplication_distributive(                
                m1 in super::$Generator::<$ScalarType>(),
                m2 in super::$Generator::<$ScalarType>(), m3 in super::$Generator::<$ScalarType>()) {

                prop_assert_eq!(m1 * (m2 + m3), m1 * m2 + m1 * m3);
            }

            /// Matrix multiplication is compatible with scalar multiplication.
            ///
            /// Given matrices `m1` and `m2` and a scalar `c`
            /// ```
            /// c * (m1 * m2) = (c * m1) * m2 = m1 * (c * m2)
            /// ```
            #[test]
            fn prop_matrix_multiplication_compatible_with_scalar_multiplication(
                c in any::<$ScalarType>(),
                m1 in super::$Generator::<$ScalarType>(), m2 in super::$Generator::<$ScalarType>()) {

                prop_assert_eq!(c * (m1 * m2), (c * m1) * m2);
                prop_assert_eq!((c * m1) * m2, m1 * (c * m2));
            }

            /// Matrix multiplication is compatible with scalar multiplication.
            ///
            /// Given a matrix `m`, scalars `c1` and `c2`
            /// ```
            /// (c1 * c2) * m = c1 * (c2 * m)
            /// ```
            #[test]
            fn prop_matrix_multiplication_compatible_with_scalar_multiplication1(
                c1 in any::<$ScalarType>(), c2 in any::<$ScalarType>(), m in super::$Generator::<$ScalarType>()) {

                prop_assert_eq!((c1 * c2) * m, c1 * (c2 * m));
            }

            /// Matrices over a set of floating point scalars have a multiplicative identity.
            /// 
            /// Given a matrix `m` there is a matrix `identity` such that
            /// ```
            /// m * identity = identity * m = m
            /// ```
            #[test]
            fn prop_matrix_multiplication_identity(m in super::$Generator::<$ScalarType>()) {
                let identity = $MatrixN::one();
                prop_assert_eq!(m * identity, m);
                prop_assert_eq!(identity * m, m);
            }
        }
    }
    }
}

exact_multiplication_props!(matrix2_u32_matrix_multiplication_props, Matrix2, u32, any_matrix2);
exact_multiplication_props!(matrix2_i32_matrix_multiplication_props, Matrix2, i32, any_matrix2);
exact_multiplication_props!(matrix3_u32_matrix_multiplication_props, Matrix3, u32, any_matrix3);
exact_multiplication_props!(matrix3_i32_matrix_multiplication_props, Matrix3, i32, any_matrix3);
exact_multiplication_props!(matrix4_u32_matrix_multiplication_props, Matrix4, u32, any_matrix4);
exact_multiplication_props!(matrix4_i32_matrix_multiplication_props, Matrix4, i32, any_matrix4);


/// Generate the properties for the transposition of matrices over floating point scalars.
///
/// `$TestModuleName` is a name we give to the module we place the properties in to separate them
///  from each other for each field type to prevent namespace collisions.
/// `$MatrixN` denotes the name of the matrix type.
/// `$ScalarType` denotes the underlying system of numbers that compose the matrices.
/// `$Generator` is the name of a function or closure for generating examples.
macro_rules! approx_transposition_props {
    ($TestModuleName:ident, $MatrixN:ident, $ScalarType:ty, $Generator:ident, $tolerance:expr) => {
    #[cfg(test)]
    mod $TestModuleName {
        use proptest::prelude::*;
        use gdmath::{$MatrixN, Matrix};

        proptest! {
            /// The double transpose of a matrix is the original matrix.
            ///
            /// Given a matrix `m`
            /// ```
            /// transpose(transpose(m)) = m
            /// ```
            #[test]
            fn prop_matrix_transpose_transpose_equals_matrix(m in super::$Generator::<$ScalarType>()) {
                prop_assert_eq!(m.transpose().transpose(), m);
            }

            /// The transposition operation is linear.
            /// 
            /// Given matrices `m1` and `m2`
            /// ```
            /// transpose(m1 + m2) = transpose(m1) + transpose(m2)
            /// ```
            #[test]
            fn prop_transpose_linear(
                m1 in super::$Generator::<$ScalarType>(), m2 in super::$Generator::<$ScalarType>()) {

                prop_assert_eq!((m1 + m2).transpose(), m1.transpose() + m2.transpose());
            }

            /// Scalar multiplication of a matrix and a scalar commutes with transposition.
            /// 
            /// Given a matrix `m` and a scalar `c`
            /// ```
            /// transpose(c * m) = c * transpose(m)
            /// ```
            #[test]
            fn prop_transpose_scalar_multiplication(
                c in any::<$ScalarType>(), m in super::$Generator::<$ScalarType>()) {

                prop_assert_eq!((c * m).transpose(), c * m.transpose());
            }

            /// The transpose of the product of two matrices equals the product of the transposes
            /// of the two matrices swapped.
            /// 
            /// Given matrices `m1` and `m2`
            /// ```
            /// transpose(m1 * m2) = transpose(m2) * transpose(m1)
            /// ```
            #[test]
            fn prop_transpose_product(
                m1 in super::$Generator::<$ScalarType>(), m2 in super::$Generator::<$ScalarType>()) {

                prop_assert_eq!((m1 * m2).transpose(), m2.transpose() * m1.transpose());
            }
        }
    }
    }
}

approx_transposition_props!(matrix2_f64_transposition_props, Matrix2, f64, any_matrix2, 1e-7);
approx_transposition_props!(matrix3_f64_transposition_props, Matrix3, f64, any_matrix3, 1e-7);
approx_transposition_props!(matrix4_f64_transposition_props, Matrix4, f64, any_matrix4, 1e-7);


