extern crate cglinalg;
extern crate num_traits;
extern crate proptest;


#[cfg(test)]
mod matrix2x2_tests {
    use cglinalg::{
        Vector2,
        Matrix2x2,
        Radians,
        Angle,
        Unit,
    };
    use approx::{
        relative_eq,
    };
    use core::slice::Iter;


    struct TestCase {
        a_mat: Matrix2x2<f32>,
        b_mat: Matrix2x2<f32>,
        expected: Matrix2x2<f32>,
    }

    struct Test {
        tests: Vec<TestCase>,
    }

    impl Test {
        fn iter(&self) -> TestIter {
            TestIter {
                inner: self.tests.iter()
            }
        }
    }

    struct TestIter<'a> {
        inner: Iter<'a, TestCase>,
    }

    impl<'a> Iterator for TestIter<'a> {
        type Item = &'a TestCase;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }

    fn test_cases() -> Test {
        Test {
            tests: vec![
                TestCase {
                    a_mat: Matrix2x2::new(80.0,  23.43,     426.1,   23.5724),
                    b_mat: Matrix2x2::new(36.84, 427.46894, 7.04217, 61.891390),
                    expected: Matrix2x2::new(185091.72, 10939.63, 26935.295, 1623.9266),
                },
                TestCase {
                    a_mat: Matrix2x2::identity(),
                    b_mat: Matrix2x2::identity(),
                    expected: Matrix2x2::identity(),
                },
                TestCase {
                    a_mat: Matrix2x2::zero(),
                    b_mat: Matrix2x2::zero(),
                    expected: Matrix2x2::zero(),
                },
                TestCase {
                    a_mat: Matrix2x2::new(68.32, 0.0, 0.0, 37.397),
                    b_mat: Matrix2x2::new(57.72, 0.0, 0.0, 9.5433127),
                    expected: Matrix2x2::new(3943.4304, 0.0, 0.0, 356.89127),
                },
            ]
        }
    }


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix2x2::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[1][0], 3_i32);
        assert_eq!(matrix[1][1], 4_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix2x2::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix2x2::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[2][0], matrix[2][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix2x2::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[0][2], matrix[0][2]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        test_cases().iter().for_each(|test| {
            let a_mat_times_identity = test.a_mat * Matrix2x2::identity();
            let b_mat_times_identity = test.b_mat * Matrix2x2::identity();

            assert_eq!(a_mat_times_identity, test.a_mat);
            assert_eq!(b_mat_times_identity, test.b_mat);
        })
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        test_cases().iter().for_each(|test| {
            let a_mat_times_zero = test.a_mat * Matrix2x2::zero();
            let b_mat_times_zero = test.b_mat * Matrix2x2::zero();

            assert_eq!(a_mat_times_zero, Matrix2x2::zero());
            assert_eq!(b_mat_times_zero, Matrix2x2::zero());
        })
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        test_cases().iter().for_each(|test| {
            let zero_times_a_mat = Matrix2x2::zero() * test.a_mat;
            let zero_times_b_mat = Matrix2x2::zero() * test.b_mat;

            assert_eq!(zero_times_a_mat, Matrix2x2::zero());
            assert_eq!(zero_times_b_mat, Matrix2x2::zero());
        })
    }

    #[test]
    fn test_mat_times_identity_equals_identity_times_mat() {
        test_cases().iter().for_each(|test| {
            let a_mat_times_identity = test.a_mat * Matrix2x2::identity();
            let identity_times_a_mat = Matrix2x2::identity() * test.a_mat;
            let b_mat_times_identity = test.b_mat * Matrix2x2::identity();
            let identity_times_b_mat = Matrix2x2::identity() * test.b_mat;

            assert_eq!(a_mat_times_identity, identity_times_a_mat);
            assert_eq!(b_mat_times_identity, identity_times_b_mat);
        })
    }

    #[test]
    fn test_mat_transpose_transpose_equals_mat() {
        test_cases().iter().for_each(|test| {
            let a_mat_tr_tr = test.a_mat.transpose().transpose();
            let b_mat_tr_tr = test.b_mat.transpose().transpose();
            
            assert_eq!(a_mat_tr_tr, test.a_mat);
            assert_eq!(b_mat_tr_tr, test.b_mat);
        })
    }

    #[test]
    fn test_identity_transpose_equals_identity() {
        let identity = Matrix2x2::<f32>::identity();
        let identity_tr = identity.transpose();
            
        assert_eq!(identity, identity_tr);
    }

    #[test]
    fn test_matrix_multiplication() {
        test_cases().iter().for_each(|test| {
            let result = test.a_mat * test.b_mat;
            let expected = test.expected;

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector2::new(1.0, 2.0);
        let c1 = Vector2::new(3.0, 4.0);
        let expected = Matrix2x2::new(
            1.0, 2.0, 
            3.0, 4.0
        );
        let result = Matrix2x2::from_columns(c0, c1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_constant_times_identity_is_constant_along_diagonal() {
        let c = 802.3435169;
        let id = Matrix2x2::identity();
        let expected = Matrix2x2::new(
            c, 0.0, 
            0.0, c
        );

        assert_eq!(id * c, expected);
    }

    #[test]
    fn test_identity_divide_constant_is_constant_inverse_along_diagonal() {
        let c = 802.3435169;
        let id = Matrix2x2::identity();
        let expected = Matrix2x2::new(
            1.0 / c, 0.0, 
            0.0,     1.0 / c
        );

        assert_eq!(id / c, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero = Matrix2x2::zero();
        let matrix = Matrix2x2::new(
            36.84, 427.46, 
            7.47,  61.89
        );

        assert_eq!(matrix + zero, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero = Matrix2x2::zero();
        let matrix = Matrix2x2::new(
            36.84, 427.46, 
            7.47,  61.89
        );

        assert_eq!(zero + matrix, matrix);
    }

    #[test]
    fn test_matrix_with_zero_determinant() {
        let matrix: Matrix2x2<f64> = Matrix2x2::new(
            1_f64, 2_f64, 
            4_f64, 8_f64
        );
        
        assert_eq!(matrix.determinant(), 0.0);
    }

    #[test]
    fn test_lower_triangular_matrix_determinant() {
        let matrix: Matrix2x2<f64> = Matrix2x2::new(
            2_f64,  0_f64,
            5_f64,  3_f64
        );

        assert_eq!(matrix.determinant(), 2_f64 * 3_f64);
    }

    #[test]
    fn test_upper_triangular_matrix_determinant() {
        let matrix: Matrix2x2<f64> = Matrix2x2::new(
            2_f64,  5_f64,
            0_f64,  3_f64
        );

        assert_eq!(matrix.determinant(), 2_f64 * 3_f64);
    }

    #[test]
    fn test_matrix_inverse() {
        let matrix: Matrix2x2<f64> = Matrix2x2::new(
            5_f64, 1_f64, 
            1_f64, 5_f64
        );
        let expected: Matrix2x2<f64> = (1_f64 / 24_f64) * Matrix2x2::new(
             5_f64, -1_f64,
            -1_f64,  5_f64
        );
        let result = matrix.inverse().unwrap();
        let epsilon = 1e-7;

        assert!(relative_eq!(result, expected, epsilon = epsilon));
    }

    #[test]
    fn test_identity_is_invertible() {
        assert!(Matrix2x2::<f64>::identity().is_invertible());
    }

    #[test]
    fn test_identity_inverse_is_identity() {
        let result: Matrix2x2<f64> = Matrix2x2::identity().inverse().unwrap();
        let expected: Matrix2x2<f64> = Matrix2x2::identity();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_inverse_diagonal_matrix() {
        let matrix: Matrix2x2<f64> = 4_f64 * Matrix2x2::identity();
        let expected: Matrix2x2<f64> = (1_f64 / 4_f64) * Matrix2x2::identity();
        let result = matrix.inverse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_with_nonzero_determinant_is_invertible() {
        let matrix = Matrix2x2::new(1f32, 2f32, 3f32, 4f32);
        
        assert!(matrix.is_invertible());
    }

    #[test]
    fn test_matrix_with_zero_determinant_is_not_invertible() {
        let matrix = Matrix2x2::new(1f32, 2f32, 4f32, 8f32);
        
        assert!(!matrix.is_invertible());
    }

    #[test]
    fn test_noninvertible_matrix_returns_none() {
        let matrix = Matrix2x2::new(1f32, 2f32, 4f32, 8f32);
        
        assert!(matrix.inverse().is_none());
    }


    #[test]
    fn test_matrix_times_inverse_is_identity() {
        let matrix = Matrix2x2::new(36.84, 427.46, 7.47, 61.89);
        let matrix_inv = matrix.inverse().unwrap();
        let one = Matrix2x2::identity();

        assert!(relative_eq!(matrix * matrix_inv, one, epsilon = 1e-7));
    }

    #[test]
    fn test_inverse_times_matrix_is_identity() {
        let matrix = Matrix2x2::new(36.84, 427.46, 7.47, 61.89);
        let matrix_inv = matrix.inverse().unwrap();
        let one = Matrix2x2::identity();

        assert!(relative_eq!(matrix_inv * matrix, one, epsilon = 1e-7));        
    }

    #[test]
    fn test_constant_times_matrix_inverse_equals_constant_inverse_times_matrix_inverse() {
        let matrix: Matrix2x2<f64> = Matrix2x2::new(
            80.0,   426.1,
            23.43,  23.5724
        );
        let constant: f64 = 4_f64;
        let constant_times_matrix_inverse = (constant * matrix).inverse().unwrap();
        let constant_inverse_times_matrix_inverse = (1_f64 / constant) * matrix.inverse().unwrap();

        assert_eq!(constant_times_matrix_inverse, constant_inverse_times_matrix_inverse);
    }

    #[test]
    fn test_matrix_transpose_inverse_equals_matrix_inverse_transpose() {
        let matrix: Matrix2x2<f64> = Matrix2x2::new(
            80.0,   426.1, 
            23.43,  23.5724
        );
        let matrix_transpose_inverse = matrix.transpose().inverse().unwrap();
        let matrix_inverse_transpose = matrix.inverse().unwrap().transpose();

        assert_eq!(matrix_transpose_inverse, matrix_inverse_transpose);
    }

    #[test]
    fn test_matrix_inverse_inverse_equals_matrix() {
        let matrix: Matrix2x2<f64> = Matrix2x2::new(
            80.0,   426.1, 
            23.43,  23.5724
        );
        let result = matrix.inverse().unwrap().inverse().unwrap();
        let expected = matrix;
        let epsilon = 1e-7;

        assert!(relative_eq!(result, expected, epsilon = epsilon));
    }

    #[test]
    fn test_matrix_elements_should_be_column_major_order() {
        let matrix = Matrix2x2::new(1, 2, 3, 4);
        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
    }

    #[test]
    fn test_matrix_swap_columns() {
        let mut result = Matrix2x2::new(1, 2, 3, 4);
        result.swap_columns(0, 1);
        let expected = Matrix2x2::new(3, 4, 1, 2);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_swap_rows() {
        let mut result = Matrix2x2::new(1, 2, 3, 4);
        result.swap_rows(0, 1);
        let expected = Matrix2x2::new(2, 1, 4, 3);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_swap_elements() {
        let mut result = Matrix2x2::new(1, 2, 3, 4);
        result.swap((0, 0), (1, 1));
        let expected = Matrix2x2::new(4, 2, 3, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_scale() {
        let matrix = Matrix2x2::from_scale(3);
        let unit_x = Vector2::unit_x();
        let unit_y = Vector2::unit_y();
        let expected = unit_x * 3 + unit_y * 3;
        let result = matrix * Vector2::new(1, 1);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_nonuniform_scale() {
        let matrix = Matrix2x2::from_nonuniform_scale(3, 7);
        let unit_x = Vector2::unit_x();
        let unit_y = Vector2::unit_y();
        let expected = unit_x * 3 + unit_y * 7;
        let result = matrix * Vector2::new(1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear_x() {
        let shear_x_with_y = 5;
        let matrix = Matrix2x2::from_shear_x(shear_x_with_y);
        let expected = Vector2::new(1 + shear_x_with_y, 1);
        let result = matrix * Vector2::new(1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear_y() {
        let shear_y_with_x = 5;
        let matrix = Matrix2x2::from_shear_y(shear_y_with_x);
        let expected = Vector2::new(1, 1 + shear_y_with_x);
        let result = matrix * Vector2::new(1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear() {
        let shear_x_with_y = 5;
        let shear_y_with_x = 7;
        let matrix = Matrix2x2::from_shear(shear_x_with_y, shear_y_with_x);
        let expected = Vector2::new(1 + shear_x_with_y, 1 + shear_y_with_x);
        let result = matrix * Vector2::new(1, 1);

        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[test]
    fn test_from_reflection_x_axis1() {
        // The y-axis is the normal vector to the plane of the x-axis.
        let normal = Unit::from_value(Vector2::unit_y());
        let expected = Matrix2x2::new(1.0, 0.0, 0.0, -1.0);
        let result = Matrix2x2::from_reflection(&normal);

        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[test]
    fn test_from_reflection_x_axis2() {
        // The y-axis is the normal vector to the plane of the x-axis.
        let normal = Unit::from_value(-Vector2::unit_y());
        let expected = Matrix2x2::new(1.0, 0.0, 0.0, -1.0);
        let result = Matrix2x2::from_reflection(&normal);

        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[test]
    fn test_from_reflection_y_axis1() {
        // The y-axis is the normal vector to the plane of the y-axis.
        let normal = Unit::from_value(Vector2::unit_x());
        let expected = Matrix2x2::new(-1.0, 0.0, 0.0, 1.0);
        let result = Matrix2x2::from_reflection(&normal);
    
        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[test]
    fn test_from_reflection_y_axis2() {
        // The y-axis is the normal vector to the plane of the y-axis.
        let normal = Unit::from_value(-Vector2::unit_x());
        let expected = Matrix2x2::new(-1.0, 0.0, 0.0, 1.0);
        let result = Matrix2x2::from_reflection(&normal);
    
        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the 
    /// line `y - x = 0`. In two dimensions there is an ambiguity in the orientation 
    /// of the line segment; there are two possible normal vectors for the line.
    #[test]
    fn test_from_reflection_from_plane1() {
        let normal = Unit::from_value(
            Vector2::new(f64::sqrt(2_f64)/ 2_f64, -f64::sqrt(2_f64) / 2_f64)
        );
        let expected = Matrix2x2::new(0.0, 1.0, 1.0, 0.0);
        let result = Matrix2x2::from_reflection(&normal);
        
        assert!(relative_eq!(result, expected, epsilon = 1e-7));
    }

    /// Construct a reflection matrix test case for reflection about the 
    /// line `y - x = 0`. In two dimensions there is an ambiguity in the orientation 
    /// of the line segment; there are two possible normal vectors for the line.
    #[test]
    fn test_from_reflection_from_plane2() {
        let normal = Unit::from_value(
            Vector2::new(-f64::sqrt(2_f64)/ 2_f64, f64::sqrt(2_f64) / 2_f64)
        );
        let expected = Matrix2x2::new(0.0, 1.0, 1.0, 0.0);
        let result = Matrix2x2::from_reflection(&normal);
            
        assert!(relative_eq!(result, expected, epsilon = 1e-7));
    }

    #[test]
    fn test_from_angle() {
        let matrix: Matrix2x2<f64> = Matrix2x2::from_angle(Radians::full_turn_div_4());
        let unit_x = Vector2::unit_x();
        let unit_y = Vector2::unit_y();
        let expected = unit_y;
        let result = matrix * unit_x;

        assert!(relative_eq!(result, expected, epsilon = 1e-8));

        let expected = -unit_x;
        let result = matrix * unit_y;

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_rotation_between() {
        let unit_x: Vector2<f64> = Vector2::unit_x();
        let unit_y: Vector2<f64> = Vector2::unit_y();
        let expected = Matrix2x2::new(
             0_f64, 1_f64,
            -1_f64, 0_f64,
        );
        let result = Matrix2x2::rotation_between(&unit_x, &unit_y);

        assert!(relative_eq!(result, expected, epsilon = 1e-7));
    }

    #[test]
    fn test_rotation_between_axis() {
        let unit_x: Unit<Vector2<f64>> = Unit::from_value(Vector2::unit_x());
        let unit_y: Unit<Vector2<f64>> = Unit::from_value(Vector2::unit_y());
        let expected = Matrix2x2::new(
             0_f64, 1_f64,
            -1_f64, 0_f64,
        );
        let result = Matrix2x2::rotation_between_axis(&unit_x, &unit_y);

        assert!(relative_eq!(result, expected, epsilon = 1e-7));
    }
}


#[cfg(test)]
mod matrix3x3_tests {
    use cglinalg::{
        Vector2,
        Vector3,
        Magnitude,
        Matrix3x3,
        Angle,
        Radians,
        Unit,
    };
    use approx::relative_eq;
    use core::slice::Iter;


    struct TestCase {
        a_mat: Matrix3x3<f32>,
        b_mat: Matrix3x3<f32>,
        expected: Matrix3x3<f32>,
    }

    struct Test {
        tests: Vec<TestCase>,
    }

    impl Test {
        fn iter(&self) -> TestIter {
            TestIter {
                inner: self.tests.iter()
            }
        }
    }

    struct TestIter<'a> {
        inner: Iter<'a, TestCase>,
    }

    impl<'a> Iterator for TestIter<'a> {
        type Item = &'a TestCase;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }

    fn test_cases() -> Test {
        Test {
            tests: vec![
                TestCase {
                    a_mat: Matrix3x3::new(
                        80.0,   426.1,   43.393, 
                        23.43,  23.5724, 1.27, 
                        81.439, 12.19,   43.36
                    ),
                    b_mat: Matrix3x3::new(
                        36.84,     7.04217,  5.74, 
                        427.46894, 61.89139, 96.27, 
                        152.66,    86.333,   26.71
                    ),
                    expected: Matrix3x3::new(
                        3579.6579,  15933.496,   1856.4281, 
                        43487.7660, 184776.9752, 22802.0289, 
                        16410.8178, 67409.1000,  7892.1646
                    ),
                },
                TestCase {
                    a_mat: Matrix3x3::identity(),
                    b_mat: Matrix3x3::identity(),
                    expected: Matrix3x3::identity(),
                },
                TestCase {
                    a_mat: Matrix3x3::zero(),
                    b_mat: Matrix3x3::zero(),
                    expected: Matrix3x3::zero(),
                },
                TestCase {
                    a_mat: Matrix3x3::new(
                        68.32, 0.0,    0.0, 
                        0.0,   37.397, 0.0, 
                        0.0,   0.0,    43.393
                    ),
                    b_mat: Matrix3x3::new(
                        57.72, 0.0,       0.0, 
                        0.0,   9.5433127, 0.0, 
                        0.0,   0.0,       12.19
                    ),
                    expected: Matrix3x3::new(
                        3943.4304, 0.0,       0.0, 
                        0.0,       356.89127, 0.0, 
                        0.0,       0.0,       528.96067
                    ),
                },
            ]
        }
    }


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[1][0], 4_i32);
        assert_eq!(matrix[1][1], 5_i32);
        assert_eq!(matrix[1][2], 6_i32);
        assert_eq!(matrix[2][0], 7_i32);
        assert_eq!(matrix[2][1], 8_i32);
        assert_eq!(matrix[2][2], 9_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix[3][0], matrix[3][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32,
            7_i32, 8_i32, 9_i32
        );

        assert_eq!(matrix[0][3], matrix[0][3]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        test_cases().iter().for_each(|test| {
            let a_mat_times_identity = test.a_mat * Matrix3x3::identity();
            let b_mat_times_identity = test.b_mat * Matrix3x3::identity();

            assert_eq!(a_mat_times_identity, test.a_mat);
            assert_eq!(b_mat_times_identity, test.b_mat);
        })
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        test_cases().iter().for_each(|test| {
            let a_mat_times_zero = test.a_mat * Matrix3x3::zero();
            let b_mat_times_zero = test.b_mat * Matrix3x3::zero();

            assert_eq!(a_mat_times_zero, Matrix3x3::zero());
            assert_eq!(b_mat_times_zero, Matrix3x3::zero());
        })
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        test_cases().iter().for_each(|test| {
            let zero_times_a_mat = Matrix3x3::zero() * test.a_mat;
            let zero_times_b_mat = Matrix3x3::zero() * test.b_mat;

            assert_eq!(zero_times_a_mat, Matrix3x3::zero());
            assert_eq!(zero_times_b_mat, Matrix3x3::zero());
        })
    }

    #[test]
    fn test_mat_times_identity_equals_identity_times_mat() {
        test_cases().iter().for_each(|test| {
            let a_mat_times_identity = test.a_mat * Matrix3x3::identity();
            let identity_times_a_mat = Matrix3x3::identity() * test.a_mat;
            let b_mat_times_identity = test.b_mat * Matrix3x3::identity();
            let identity_times_b_mat = Matrix3x3::identity() * test.b_mat;

            assert_eq!(a_mat_times_identity, identity_times_a_mat);
            assert_eq!(b_mat_times_identity, identity_times_b_mat);
        })
    }

    #[test]
    fn test_mat_transpose_transpose_equals_mat() {
        test_cases().iter().for_each(|test| {
            let a_mat_tr_tr = test.a_mat.transpose().transpose();
            let b_mat_tr_tr = test.b_mat.transpose().transpose();
            
            assert_eq!(a_mat_tr_tr, test.a_mat);
            assert_eq!(b_mat_tr_tr, test.b_mat);
        })
    }

    #[test]
    fn test_identity_transpose_equals_identity() {
        let identity = Matrix3x3::<f32>::identity();
        let identity_tr = identity.transpose();
            
        assert_eq!(identity, identity_tr);
    }

    #[test]
    fn test_matrix_multiplication() {
        test_cases().iter().for_each(|test| {
            let result = test.a_mat * test.b_mat;
            let expected = test.expected;

            assert_eq!(result, expected);
        })
    }

    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector3::new(1.0, 2.0, 3.0);
        let c1 = Vector3::new(4.0, 5.0, 6.0);
        let c2 = Vector3::new(7.0, 8.0, 9.0);
        let expected = Matrix3x3::new(
            1.0, 2.0, 3.0, 
            4.0, 5.0, 6.0, 
            7.0, 8.0, 9.0
        );
        let result = Matrix3x3::from_columns(c0, c1, c2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_constant_times_identity_is_constant_along_diagonal() {
        let c = 802.3435169;
        let id = Matrix3x3::identity();
        let expected = Matrix3x3::new(
            c,   0.0, 0.0, 
            0.0, c,   0.0, 
            0.0, 0.0, c
        );

        assert_eq!(id * c, expected);
    }

    #[test]
    fn test_identity_divide_constant_is_constant_inverse_along_diagonal() {
        let c = 802.3435169;
        let id = Matrix3x3::identity();
        let expected = Matrix3x3::new(
            1.0/c, 0.0,   0.0, 
            0.0,   1.0/c, 0.0, 
            0.0,   0.0,   1.0/c
        );

        assert_eq!(id / c, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero = Matrix3x3::zero();
        let matrix = Matrix3x3::new(
            80.0,   426.1,   43.393, 
            23.43,  23.5724, 1.27, 
            81.439, 12.19,   43.36
        );

        assert_eq!(matrix + zero, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero = Matrix3x3::zero();
        let matrix = Matrix3x3::new(
            80.0,   426.1,   43.393, 
            23.43,  23.5724, 1.27, 
            81.439, 12.19,   43.36
        );

        assert_eq!(zero + matrix, matrix);
    }

    #[test]
    fn test_matrix_with_zero_determinant() {
        let matrix = Matrix3x3::new(
            1_f32, 2_f32, 3_f32, 
            4_f32, 5_f32, 6_f32, 
            4_f32, 5_f32, 6_f32
        );
        
        assert_eq!(matrix.determinant(), 0.0);
    }

    #[test]
    fn test_lower_triangular_matrix_determinant() {
        let matrix: Matrix3x3<f64> = Matrix3x3::new(
            1_f64,  0_f64,  0_f64,
            5_f64,  2_f64,  0_f64,
            5_f64,  5_f64,  3_f64
        );

        assert_eq!(matrix.determinant(), 1_f64 * 2_f64 * 3_f64);
    }

    #[test]
    fn test_upper_triangular_matrix_determinant() {
        let matrix: Matrix3x3<f64> = Matrix3x3::new(
            1_f64,  5_f64,  5_f64,
            0_f64,  2_f64,  5_f64,
            0_f64,  0_f64,  3_f64
        );

        assert_eq!(matrix.determinant(), 1_f64 * 2_f64 * 3_f64);
    }

    #[test]
    fn test_matrix_inverse() {
        let matrix: Matrix3x3<f64> = Matrix3x3::new(
            5_f64, 1_f64, 1_f64,
            1_f64, 5_f64, 1_f64,
            1_f64, 1_f64, 5_f64
        );
        let expected: Matrix3x3<f64> = (1_f64 / 28_f64) * Matrix3x3::new(
             6_f64, -1_f64, -1_f64, 
            -1_f64,  6_f64, -1_f64, 
            -1_f64, -1_f64,  6_f64,
        );
        let result = matrix.inverse().unwrap();
        let epsilon = 1e-7;

        assert!(relative_eq!(result, expected, epsilon = epsilon));
    }

    #[test]
    fn test_identity_is_invertible() {
        assert!(Matrix3x3::<f64>::identity().is_invertible());
    }

    #[test]
    fn test_identity_inverse_is_identity() {
        let result: Matrix3x3<f64> = Matrix3x3::identity().inverse().unwrap();
        let expected: Matrix3x3<f64> = Matrix3x3::identity();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_inverse_diagonal_matrix() {
        let matrix: Matrix3x3<f64> = 4_f64 * Matrix3x3::identity();
        let expected: Matrix3x3<f64> = (1_f64 / 4_f64) * Matrix3x3::identity();
        let result = matrix.inverse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_with_nonzero_determinant_is_invertible() {
        let matrix = Matrix3x3::new(
            1f32, 2f32, 3f32, 
            0f32, 4f32, 5f32, 
            0f32, 0f32, 6f32
        );
        
        assert!(matrix.is_invertible());
    }

    #[test]
    fn test_matrix_with_zero_determinant_is_not_invertible() {
        let matrix = Matrix3x3::new(
            1f32, 2f32, 3f32, 
            4f32, 5f32, 6f32, 
            4f32, 5f32, 6f32
        );
        
        assert!(!matrix.is_invertible());
    }

    #[test]
    fn test_noninvertible_matrix_returns_none() {
        let matrix = Matrix3x3::new(
            1f32, 2f32, 3f32, 
            4f32, 5f32, 6f32, 
            4f32, 5f32, 6f32
        );
        
        assert!(matrix.inverse().is_none());
    }

    #[test]
    fn test_matrix_times_inverse_is_identity() {
        let matrix = Matrix3x3::new(
            80.0,   426.1,   43.393, 
            23.43,  23.5724, 1.27, 
            81.439, 12.19,   43.36
        );
        let matrix_inv = matrix.inverse().unwrap();
        let one = Matrix3x3::identity();

        assert!(relative_eq!(matrix * matrix_inv, one, epsilon = 1e-7));
    }

    #[test]
    fn test_constant_times_matrix_inverse_equals_constant_inverse_times_matrix_inverse() {
        let matrix: Matrix3x3<f64> = Matrix3x3::new(
            80.0,   426.1,   43.393, 
            23.43,  23.5724, 1.27, 
            81.439, 12.19,   43.36
        );
        let constant: f64 = 4_f64;
        let constant_times_matrix_inverse = (constant * matrix).inverse().unwrap();
        let constant_inverse_times_matrix_inverse = (1_f64 / constant) * matrix.inverse().unwrap();

        assert_eq!(constant_times_matrix_inverse, constant_inverse_times_matrix_inverse);
    }

    #[test]
    fn test_matrix_transpose_inverse_equals_matrix_inverse_transpose() {
        let matrix: Matrix3x3<f64> = Matrix3x3::new(
            80.0,   426.1,   43.393, 
            23.43,  23.5724, 1.27, 
            81.439, 12.19,   43.36
        );
        let matrix_transpose_inverse = matrix.transpose().inverse().unwrap();
        let matrix_inverse_transpose = matrix.inverse().unwrap().transpose();

        assert_eq!(matrix_transpose_inverse, matrix_inverse_transpose);
    }

    #[test]
    fn test_inverse_times_matrix_is_identity() {
        let matrix = Matrix3x3::new(
            80.0,   426.1,   43.393, 
            23.43,  23.5724, 1.27, 
            81.439, 12.19,   43.36
        );
        let matrix_inv = matrix.inverse().unwrap();
        let one = Matrix3x3::identity();

        assert!(relative_eq!(matrix_inv * matrix, one, epsilon = 1e-7));
    }

    #[test]
    fn test_matrix_inverse_inverse_equals_matrix() {
        let matrix: Matrix3x3<f64> = Matrix3x3::new(
            80.0,   426.1,   43.393, 
            23.43,  23.5724, 1.27, 
            81.439, 12.19,   43.36
        );
        let result = matrix.inverse().unwrap().inverse().unwrap();
        let expected = matrix;
        let epsilon = 1e-7;

        assert!(relative_eq!(result, expected, epsilon = epsilon));
    }

    #[test]
    fn test_matrix_elements_should_be_column_major_order() {
        let matrix = Matrix3x3::new(
            1, 2, 3, 
            4, 5, 6, 
            7, 8, 9
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
    }

    #[test]
    fn test_matrix_swap_columns() {
        let mut result = Matrix3x3::new(1, 2, 3, 4, 5, 6, 7, 8, 9);
        result.swap_columns(0, 1);
        let expected = Matrix3x3::new(4, 5, 6, 1, 2, 3, 7, 8, 9);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_swap_rows() {
        let mut result = Matrix3x3::new(1, 2, 3, 4, 5, 6, 7, 8, 9);
        result.swap_rows(0, 1);
        let expected = Matrix3x3::new(2, 1, 3, 5, 4, 6, 8, 7, 9);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_swap_elements() {
        let mut result = Matrix3x3::new(1, 2, 3, 4, 5, 6, 7, 8, 9);
        result.swap((0, 0), (2, 1));
        let expected = Matrix3x3::new(8, 2, 3, 4, 5, 6, 7, 1, 9);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_scale() {
        let matrix = Matrix3x3::from_scale(3);
        let unit_x = Vector3::unit_x();
        let unit_y = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let expected = unit_x * 3 + unit_y * 3 + unit_z * 3;
        let result = matrix * Vector3::new(1, 1, 1);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_nonuniform_scale() {
        let matrix = Matrix3x3::from_nonuniform_scale(3, 5, 7);
        let unit_x = Vector3::unit_x();
        let unit_y = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let expected = unit_x * 3 + unit_y * 5 + unit_z * 7;
        let result = matrix * Vector3::new(1, 1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_scale_does_not_change_last_coordinate() {
        let matrix = Matrix3x3::from_affine_scale(5);
        let unit_z = Vector3::unit_z();
        let expected = unit_z;
        let result = matrix * unit_z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_nonuniform_scale() {
        let matrix = Matrix3x3::from_affine_nonuniform_scale(7, 11);
        let expected = Vector3::new(7, 11, 1);
        let result = matrix * Vector3::new(1, 1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_nonuniform_scale_does_not_change_last_coordinate() {
        let matrix = Matrix3x3::from_affine_nonuniform_scale(7, 11);
        let unit_z = Vector3::unit_z();
        let expected = unit_z;
        let result = matrix * unit_z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear_x() {
        let shear_x_with_y = 5;
        let shear_x_with_z = 3;
        let matrix = Matrix3x3::from_shear_x(shear_x_with_y, shear_x_with_z);
        let expected = Vector3::new(1 + shear_x_with_y + shear_x_with_z, 1, 1);
        let result = matrix * Vector3::new(1, 1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear_y() {
        let shear_y_with_x = 5;
        let shear_y_with_z = 3;
        let matrix = Matrix3x3::from_shear_y(shear_y_with_x, shear_y_with_z);
        let expected = Vector3::new(1, 1 + shear_y_with_x + shear_y_with_z, 1);
        let result = matrix * Vector3::new(1, 1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_shear_z() {
        let shear_z_with_x = 5;
        let shear_z_with_y = 3;
        let matrix = Matrix3x3::from_shear_z(shear_z_with_x, shear_z_with_y);
        let expected = Vector3::new(1, 1, 1 + shear_z_with_x + shear_z_with_y);
        let result = matrix * Vector3::new(1, 1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_x() {
        let shear_x_with_y = 5;
        let matrix = Matrix3x3::from_affine_shear_x(shear_x_with_y);
        let expected = Vector3::new(1 + shear_x_with_y, 1, 1);
        let result = matrix * Vector3::new(1, 1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_x_does_not_change_last_coordinate() {
        let shear_x_with_y = 5;
        let matrix = Matrix3x3::from_affine_shear_x(shear_x_with_y);
        let unit_z = Vector3::unit_z();
        let expected = unit_z;
        let result = matrix * unit_z;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_y() {
        let shear_y_with_x = 3;
        let matrix = Matrix3x3::from_affine_shear_y(shear_y_with_x);
        let expected = Vector3::new(1, 1 + shear_y_with_x, 1);
        let result = matrix * Vector3::new(1, 1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_y_does_not_change_last_coordinate() {
        let shear_y_with_x = 3;
        let matrix = Matrix3x3::from_affine_shear_y(shear_y_with_x);
        let unit_z = Vector3::unit_z();
        let expected = unit_z;
        let result = matrix * unit_z;

        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[test]
    fn test_from_affine_reflection_x_axis1() {
        // The y-axis is the normal vector to the plane of the x-axis.
        let bias = Vector2::zero();
        let normal = Unit::from_value(Vector2::unit_y());
        let expected = Matrix3x3::new(
            1.0,  0.0, 0.0, 
            0.0, -1.0, 0.0, 
            0.0,  0.0, 1.0
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);

        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[test]
    fn test_from_affine_reflection_x_axis2() {
        // The y-axis is the normal vector to the plane of the x-axis.
        let bias = Vector2::zero();
        let normal = Unit::from_value(-Vector2::unit_y());
        let expected = Matrix3x3::new(
            1.0,  0.0, 0.0, 
            0.0, -1.0, 0.0, 
            0.0,  0.0, 1.0
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);

        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[test]
    fn test_from_affine_reflection_y_axis1() {
        // The y-axis is the normal vector to the plane of the y-axis.
        let bias = Vector2::zero();
        let normal = Unit::from_value(Vector2::unit_x());
        let expected = Matrix3x3::new(
            -1.0, 0.0, 0.0, 
             0.0, 1.0, 0.0, 
             0.0, 0.0, 1.0
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);
    
        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the **x-axis**.
    /// In two dimensions there is an ambiguity in the orientation of the line 
    /// segment; there are two possible normal vectors for the line.
    #[test]
    fn test_from_affine_reflection_y_axis2() {
        // The y-axis is the normal vector to the plane of the y-axis.
        let bias = Vector2::zero();
        let normal = Unit::from_value(-Vector2::unit_x());
        let expected = Matrix3x3::new(
            -1.0, 0.0, 0.0, 
             0.0, 1.0, 0.0, 
             0.0, 0.0, 1.0
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);
    
        assert_eq!(result, expected);
    }

    /// Construct a reflection matrix test case for reflection about the 
    /// line `y - x = 0`. In two dimensions there is an ambiguity in the 
    /// orientation of the line segment; there are two possible normal vectors 
    /// for the line.
    #[test]
    fn test_from_affine_reflection_from_plane1() {
        let bias = Vector2::zero();
        let normal = Unit::from_value(
            Vector2::new(f64::sqrt(2_f64)/ 2_f64, -f64::sqrt(2_f64) / 2_f64)
        );
        let expected = Matrix3x3::new(
            0.0, 1.0, 0.0, 
            1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);
        
        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    /// Construct a reflection matrix test case for reflection about the 
    /// line `y - x = 0`. In two dimensions there is an ambiguity in the 
    /// orientation of the line segment; there are two possible normal vectors 
    /// for the line.
    #[test]
    fn test_from_affine_reflection_from_plane2() {
        let bias = Vector2::zero();
        let normal = Unit::from_value(
            Vector2::new(-f64::sqrt(2_f64)/ 2_f64, f64::sqrt(2_f64) / 2_f64)
        );
        let expected = Matrix3x3::new(
            0.0, 1.0, 0.0, 
            1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0
        );
        let result = Matrix3x3::from_affine_reflection(&normal, &bias);
            
        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    /// Construct an affine reflection matrix about the line `y = (1/2)x + 2`.
    /// This line does not cross the origin.
    #[test]
    fn test_from_affine_reflection_from_line_that_does_not_cross_origin1() {
        // We can always choose the y-intercept as the known point.
        let bias = Vector2::new(0.0, 2.0);
        let normal = Unit::from_value(
            Vector2::new(-1.0 / f64::sqrt(5.0), 2.0 / f64::sqrt(5.0))
        );
        let matrix = Matrix3x3::from_affine_reflection(&normal, &bias);
        let vector = Vector3::new(1.0, 0.0, 1.0);
        let expected = Vector3::new(-1.0, 4.0, 1.0);
        let result = matrix * vector;

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    /// Construct an affine reflection matrix about the line `y = (1/2)x + 2`.
    /// This line does not cross the origin.
    #[test]
    fn test_from_affine_reflection_from_line_that_does_not_cross_origin2() {
        // We can always choose the y-intercept as the known point.
        let bias = Vector2::new(0.0, 2.0);
        let normal = Unit::from_value(
            Vector2::new(1.0 / f64::sqrt(5.0), -2.0 / f64::sqrt(5.0))
        );
        let matrix = Matrix3x3::from_affine_reflection(&normal, &bias);
        let vector = Vector3::new(1.0, 0.0, 1.0);
        let expected = Vector3::new(-1.0, 4.0, 1.0);
        let result = matrix * vector;

        assert!(relative_eq!(result, expected, epsilon = 1e-8));        
    }

    #[test]
    fn test_from_reflection_xy_plane() {
        let normal = Unit::from_value(Vector3::unit_z());
        let expected = Matrix3x3::new(
            1.0, 0.0,  0.0, 
            0.0, 1.0,  0.0,  
            0.0, 0.0, -1.0
        );
        let result = Matrix3x3::from_reflection(&normal);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_reflection_zx_plane() {
        let normal = Unit::from_value(-Vector3::unit_y());
        let expected = Matrix3x3::new(
            1.0,  0.0, 0.0, 
            0.0, -1.0, 0.0,  
            0.0,  0.0, 1.0
        );
        let result = Matrix3x3::from_reflection(&normal);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_reflection_yz_plane() {
        let normal = Unit::from_value(Vector3::unit_x());
        let expected = Matrix3x3::new(
            -1.0,  0.0, 0.0, 
             0.0, 1.0,  0.0,  
             0.0,  0.0, 1.0
        );
        let result = Matrix3x3::from_reflection(&normal);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_angle_x() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_y = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let matrix = Matrix3x3::from_angle_x(angle);
        let expected = unit_z;
        let result = matrix * unit_y;

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_from_angle_y() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_z = Vector3::unit_z();
        let unit_x = Vector3::unit_x();
        let matrix = Matrix3x3::from_angle_y(angle);
        let expected = unit_x;
        let result = matrix * unit_z;

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_from_angle_z() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_x = Vector3::unit_x();
        let unit_y = Vector3::unit_y();
        let matrix = Matrix3x3::from_angle_z(angle);
        let expected = unit_y;
        let result = matrix * unit_x;

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_from_axis_angle() {
        let angle: Radians<f64> = Radians::full_turn_div_2();
        let axis = Unit::from_value(
            (1.0 / f64::sqrt(2.0)) * Vector3::new(1.0, 1.0, 0.0)
        );
        let vector = Vector3::new(1.0, 1.0, -1.0);
        let matrix = Matrix3x3::from_axis_angle(&axis, angle);
        let expected = Vector3::new(1.0, 1.0, 1.0);
        let result = matrix * vector;

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_from_affine_angle() {
        let matrix: Matrix3x3<f64> = Matrix3x3::from_affine_angle(Radians::full_turn_div_4());
        let unit_x = Vector2::unit_x();
        let unit_y = Vector2::unit_y();
        let expected = unit_y.extend(0.0);
        let result = matrix * unit_x.extend(0.0);

        assert!(relative_eq!(result, expected, epsilon = 1e-8));

        let expected = -unit_x.extend(0.0);
        let result = matrix * unit_y.extend(0.0);

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    /// An affine translation should only displace points and not vectors. We 
    /// distinguish points by using a `1` in the last coordinate, and vectors 
    /// by using a `0` in the last coordinate.
    #[test]
    fn test_from_affine_translation_point() {
        let distance = Vector2::new(3, 7);
        let matrix = Matrix3x3::from_affine_translation(&distance);
        let point = Vector3::new(0, 0, 1);
        let expected = Vector3::new(3, 7, 1);
        let result = matrix * point;

        assert_eq!(result, expected);
    }

    /// An affine translation should only displace points and not vectors. We 
    /// distinguish points by using a `1` in the last coordinate, and vectors 
    /// by using a `0` in the last coordinate.
    #[test]
    fn test_from_affine_translation_vector() {
        let distance = Vector2::new(3, 7);
        let matrix = Matrix3x3::from_affine_translation(&distance);
        let vector = Vector3::zero();
        let expected = vector;
        let result = matrix * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_face_towards() {
        let direction = Vector3::new(1.0, 1.0, 1.0);
        let up = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let look_at = Matrix3x3::face_towards(&direction, &up);
        let expected = direction.normalize();
        let result = look_at * unit_z;

        assert!(relative_eq!(result, expected, epsilon = 1e-7));
    }

    #[test]
    fn test_look_at_rh() {
        let direction = Vector3::new(1.0, 1.0, 1.0).normalize();
        let up = Vector3::unit_y();
        let minus_unit_z = -Vector3::unit_z();
        let look_at = Matrix3x3::look_at_rh(&direction, &up);
        let expected = minus_unit_z;
        let result = look_at * direction;

        assert!(relative_eq!(result, expected, epsilon = 1e-7));
    }

    #[test]
    fn test_look_at_lh() {
        let direction = Vector3::new(1.0, 1.0, 1.0).normalize();
        let up = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let look_at = Matrix3x3::look_at_lh(&direction, &up);
        let expected = unit_z;
        let result = look_at * direction;

        assert!(relative_eq!(result, expected, epsilon = 1e-7));
    }

    #[test]
    fn test_rotation_between() {
        let unit_x: Vector3<f64> = Vector3::unit_x();
        let unit_y: Vector3<f64> = Vector3::unit_y();
        let expected = Matrix3x3::new(
             0_f64, 1_f64, 0_f64, 
            -1_f64, 0_f64, 0_f64,
             0_f64, 0_f64, 1_f64
        );
        let result = Matrix3x3::rotation_between(&unit_x, &unit_y).unwrap();

        assert!(relative_eq!(result, expected, epsilon = 1e-7));
    }
}

#[cfg(test)]
mod matrix4x4_tests {
    use cglinalg::{
        Vector3,
        Vector4,
        Magnitude,
        Matrix4x4,
        Radians,
        Degrees,
        Angle,
        Unit,
        Point3,
    };
    use approx::{
        relative_eq,
    };
    use core::slice::Iter;


    struct TestCase {
        a_mat: Matrix4x4<f64>,
        b_mat: Matrix4x4<f64>,
        expected: Matrix4x4<f64>,
    }

    struct Test {
        tests: Vec<TestCase>,
    }

    impl Test {
        fn iter(&self) -> TestIter {
            TestIter {
                inner: self.tests.iter()
            }
        }
    }

    struct TestIter<'a> {
        inner: Iter<'a, TestCase>,
    }

    impl<'a> Iterator for TestIter<'a> {
        type Item = &'a TestCase;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next()
        }
    }

    fn test_cases() -> Test {
        Test {
            tests: vec![
                TestCase {
                    a_mat: Matrix4x4::new(
                        80.0,  23.43,  43.56, 6.74, 
                        426.1, 23.57,  27.61, 13.90,
                        4.22,  258.08, 31.70, 42.17, 
                        70.0,  49.0,   95.0,  89.91
                    ),
                    b_mat: Matrix4x4::new(
                        36.84, 427.46, 882.19, 89.50, 
                        7.04,  61.89,  56.31,  89.0, 
                        72.0,  936.5,  413.80, 50.31,  
                        37.69, 311.8,  60.81,  73.83
                    ),
                    expected: Matrix4x4::new(
                        195075.7478, 242999.4886, 49874.8440, 51438.8929,
                        33402.1572,  20517.1793,  12255.4723, 11284.3033,
                        410070.5860, 133018.9590, 46889.9950, 35475.9481,
                        141297.8982, 27543.7175,  19192.1014, 13790.4636
                    ),
                },
                TestCase {
                    a_mat: Matrix4x4::identity(),
                    b_mat: Matrix4x4::identity(),
                    expected: Matrix4x4::identity(),
                },
                TestCase {
                    a_mat: Matrix4x4::zero(),
                    b_mat: Matrix4x4::zero(),
                    expected: Matrix4x4::zero(),
                },
                TestCase {
                    a_mat: Matrix4x4::new(
                        68.32, 0.0,    0.0,   0.0,
                        0.0,   37.397, 0.0,   0.0,
                        0.0,   0.0,    9.483, 0.0,
                        0.0,   0.0,    0.0,   887.710
                    ),
                    b_mat: Matrix4x4::new(
                        57.72, 0.0,    0.0,     0.0, 
                        0.0,   9.5433, 0.0,     0.0, 
                        0.0,   0.0,    86.7312, 0.0,
                        0.0,   0.0,    0.0,     269.1134
                    ),
                    expected: Matrix4x4::new(
                        3943.4304, 0.0,         0.0,         0.0,
                        0.0,       356.8907901, 0.0,         0.0,
                        0.0,       0.0,         822.4719696, 0.0,
                        0.0,       0.0,         0.0,         238894.656314
                    ),
                },
            ]
        }
    }


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[0][3], 4_i32);
        assert_eq!(matrix[1][0], 5_i32);
        assert_eq!(matrix[1][1], 6_i32);
        assert_eq!(matrix[1][2], 7_i32);
        assert_eq!(matrix[1][3], 8_i32);
        assert_eq!(matrix[2][0], 9_i32);
        assert_eq!(matrix[2][1], 10_i32);
        assert_eq!(matrix[2][2], 11_i32);
        assert_eq!(matrix[2][3], 12_i32);
        assert_eq!(matrix[3][0], 13_i32);
        assert_eq!(matrix[3][1], 14_i32);
        assert_eq!(matrix[3][2], 15_i32);
        assert_eq!(matrix[3][3], 16_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c0r3, matrix[0][3]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c1r3, matrix[1][3]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
        assert_eq!(matrix.c2r3, matrix[2][3]);
        assert_eq!(matrix.c3r0, matrix[3][0]);
        assert_eq!(matrix.c3r1, matrix[3][1]);
        assert_eq!(matrix.c3r2, matrix[3][2]);
        assert_eq!(matrix.c3r3, matrix[3][3]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix[4][0], matrix[4][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );

        assert_eq!(matrix[0][4], matrix[0][4]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        test_cases().iter().for_each(|test| {
            let a_mat_times_identity = test.a_mat * Matrix4x4::identity();
            let b_mat_times_identity = test.b_mat * Matrix4x4::identity();

            assert_eq!(a_mat_times_identity, test.a_mat);
            assert_eq!(b_mat_times_identity, test.b_mat);
        })
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        test_cases().iter().for_each(|test| {
            let a_mat_times_zero = test.a_mat * Matrix4x4::zero();
            let b_mat_times_zero = test.b_mat * Matrix4x4::zero();

            assert_eq!(a_mat_times_zero, Matrix4x4::zero());
            assert_eq!(b_mat_times_zero, Matrix4x4::zero());
        })
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        test_cases().iter().for_each(|test| {
            let zero_times_a_mat = Matrix4x4::zero() * test.a_mat;
            let zero_times_b_mat = Matrix4x4::zero() * test.b_mat;

            assert_eq!(zero_times_a_mat, Matrix4x4::zero());
            assert_eq!(zero_times_b_mat, Matrix4x4::zero());
        })
    }

    #[test]
    fn test_mat_times_identity_equals_identity_times_mat() {
        test_cases().iter().for_each(|test| {
            let a_mat_times_identity = test.a_mat * Matrix4x4::identity();
            let identity_times_a_mat = Matrix4x4::identity() * test.a_mat;
            let b_mat_times_identity = test.b_mat * Matrix4x4::identity();
            let identity_times_b_mat = Matrix4x4::identity() * test.b_mat;

            assert_eq!(a_mat_times_identity, identity_times_a_mat);
            assert_eq!(b_mat_times_identity, identity_times_b_mat);
        })
    }

    #[test]
    fn test_mat_transpose_transpose_equals_mat() {
        test_cases().iter().for_each(|test| {
            let a_mat_tr_tr = test.a_mat.transpose().transpose();
            let b_mat_tr_tr = test.b_mat.transpose().transpose();
            
            assert_eq!(a_mat_tr_tr, test.a_mat);
            assert_eq!(b_mat_tr_tr, test.b_mat);
        })
    }

    #[test]
    fn test_identity_transpose_equals_identity() {
        let identity = Matrix4x4::<f32>::identity();
        let identity_tr = identity.transpose();
            
        assert_eq!(identity, identity_tr);
    }

    #[test]
    fn test_matrix_multiplication() {
        test_cases().iter().for_each(|test| {
            let result = test.a_mat * test.b_mat;
            let expected = test.expected;
            let epsilon = 1e-7;

            assert!(relative_eq!(result, expected, epsilon = epsilon));
        })
    }

    #[test]
    fn test_construction_from_cols() {
        let c0 = Vector4::new(1, 2, 3, 4);
        let c1 = Vector4::new(5, 6, 7, 8);
        let c2 = Vector4::new(9, 10, 11, 12);
        let c3 = Vector4::new(13, 14, 15, 16);
        let expected = Matrix4x4::new(
            1,  2,  3,  4, 
            5,  6,  7,  8, 
            9,  10, 11, 12, 
            13, 14 ,15, 16
        );
        let result = Matrix4x4::from_columns(c0, c1, c2, c3);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_identity_mat4_translates_vector_along_vector() {
        let vector = Vector3::from((2.0, 2.0, 2.0));
        let trans_mat = Matrix4x4::from_affine_translation(&vector);
        let zero_vec4 = Vector4::from((0.0, 0.0, 0.0, 1.0));
        let zero_vec3 = Vector3::from((0.0, 0.0, 0.0));

        let result = trans_mat * zero_vec4;
        assert_eq!(result, (zero_vec3 + vector).extend(1.0));
    }

    #[test]
    fn test_constant_times_identity_is_constant_along_diagonal() {
        let c = 802.3435169;
        let id = Matrix4x4::identity();
        let expected = Matrix4x4::new(
            c,   0.0, 0.0, 0.0, 
            0.0, c,   0.0, 0.0, 
            0.0, 0.0, c,   0.0, 
            0.0, 0.0, 0.0, c
        );

        assert_eq!(id * c, expected);
    }

    #[test]
    fn test_identity_divide_constant_is_constant_inverse_along_diagonal() {
        let c = 802.3435169;
        let id = Matrix4x4::identity();
        let expected = Matrix4x4::new(
            1.0 / c, 0.0,     0.0,     0.0, 
            0.0,     1.0 / c, 0.0,     0.0, 
            0.0,     0.0,     1.0 / c, 0.0, 
            0.0,     0.0,     0.0,     1.0 / c
        );

        assert_eq!(id / c, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero = Matrix4x4::zero();
        let matrix = Matrix4x4::new(
            36.84,   427.46894, 8827.1983, 89.5049494, 
            7.04217, 61.891390, 56.31,     89.0, 
            72.0,    936.5,     413.80,    50.311160,  
            37.6985,  311.8,    60.81,     73.8393
        );

        assert_eq!(matrix + zero, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero = Matrix4x4::zero();
        let matrix = Matrix4x4::new(
            36.84,   427.46894, 8827.1983, 89.5049494, 
            7.04217, 61.891390, 56.31,     89.0, 
            72.0,    936.5,     413.80,    50.311160,  
            37.6985,  311.8,    60.81,     73.8393
        );

        assert_eq!(zero + matrix, matrix);
    }

    #[test]
    fn test_matrix_with_zero_determinant() {
        // This matrix should have a zero determinant since it has two repeating columns.
        use num_traits::Zero;
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            1_f64,  2_f64,  3_f64,  4_f64, 
            5_f64,  6_f64,  7_f64,  8_f64,
            5_f64,  6_f64,  7_f64,  8_f64, 
            9_f64,  10_f64, 11_f64, 12_f64
        );
        
        assert!(matrix.determinant().is_zero());
    }

    #[test]
    fn test_lower_triangular_matrix_determinant() {
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            1_f64,  0_f64,  0_f64,  0_f64, 
            5_f64,  2_f64,  0_f64,  0_f64,
            5_f64,  5_f64,  3_f64,  0_f64, 
            5_f64,  5_f64,  5_f64,  4_f64
        );

        assert_eq!(matrix.determinant(), 1_f64 * 2_f64 * 3_f64 * 4_f64);
    }

    #[test]
    fn test_upper_triangular_matrix_determinant() {
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            1_f64,  5_f64,  5_f64,  5_f64, 
            0_f64,  2_f64,  5_f64,  5_f64,
            0_f64,  0_f64,  3_f64,  5_f64, 
            0_f64,  0_f64,  0_f64,  4_f64
        );

        assert_eq!(matrix.determinant(), 1_f64 * 2_f64 * 3_f64 * 4_f64);
    }

    #[test]
    fn test_scalar_multiplication() {
        let result: Matrix4x4<f64> = (1_f64 / 32_f64) * Matrix4x4::new(
            7_f64, -1_f64, -1_f64, -1_f64,
           -1_f64,  7_f64, -1_f64, -1_f64,
           -1_f64, -1_f64,  7_f64, -1_f64,
           -1_f64, -1_f64, -1_f64,  7_f64
       );
       let expected: Matrix4x4<f64> = Matrix4x4::new(
        (1_f64 / 32_f64) *  7_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64,
        (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) *  7_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64,
        (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) *  7_f64, (1_f64 / 32_f64) * -1_f64,
        (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) * -1_f64, (1_f64 / 32_f64) *  7_f64
       );

       assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_inverse() {
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            5_f64, 1_f64, 1_f64, 1_f64, 
            1_f64, 5_f64, 1_f64, 1_f64,
            1_f64, 1_f64, 5_f64, 1_f64,
            1_f64, 1_f64, 1_f64, 5_f64, 
        );
        let expected: Matrix4x4<f64> = (1_f64 / 32_f64) * Matrix4x4::new(
             7_f64, -1_f64, -1_f64, -1_f64,
            -1_f64,  7_f64, -1_f64, -1_f64,
            -1_f64, -1_f64,  7_f64, -1_f64,
            -1_f64, -1_f64, -1_f64,  7_f64
        );
        let result = matrix.inverse().unwrap();
        let epsilon = 1e-7;

        assert!(relative_eq!(result, expected, epsilon = epsilon),
            "\nmatrix = {:?}\nmatrix_inv = {:?}\nmatrix * matrix_inv = {:?}\nexpected = {:?}\nepsilon = {:?}\n",
            matrix, result, matrix * result, expected, epsilon
        );
    }

    #[test]
    fn test_identity_is_invertible() {
        assert!(Matrix4x4::<f64>::identity().is_invertible());
    }

    #[test]
    fn test_identity_inverse_is_identity() {
        let result: Matrix4x4<f64> = Matrix4x4::identity().inverse().unwrap();
        let expected: Matrix4x4<f64> = Matrix4x4::identity();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_inverse_diagonal_matrix() {
        let matrix: Matrix4x4<f64> = 4_f64 * Matrix4x4::identity();
        let expected: Matrix4x4<f64> = (1_f64 / 4_f64) * Matrix4x4::identity();
        let result = matrix.inverse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_with_nonzero_determinant_is_invertible() {
        let matrix = Matrix4x4::new(
            1_f64,  2_f64,  3_f64,   4_f64,
            5_f64,  60_f64, 7_f64,   8_f64,
            9_f64,  10_f64, 11_f64,  12_f64,
            13_f64, 14_f64, 150_f64, 16_f64
        );
        
        assert!(matrix.is_invertible());
    }

    #[test]
    fn test_matrix_with_zero_determinant_is_not_invertible() {
        // This matrix should not be invertible since it has two identical columns.
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            1_f64,  2_f64,   3_f64,  4_f64, 
            5_f64,  6_f64,   7_f64,  8_f64,
            5_f64,  6_f64,   7_f64,  8_f64, 
            9_f64,  10_f64,  11_f64, 12_f64
        );
        
        assert!(!matrix.is_invertible());
    }

    #[test]
    fn test_noninvertible_matrix_returns_none() {
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            1_f64,  2_f64,  3_f64,  4_f64, 
            5_f64,  6_f64,  7_f64,  8_f64,
            5_f64,  6_f64,  7_f64,  8_f64, 
            9_f64,  10_f64, 11_f64, 12_f64
        );
        
        assert!(matrix.inverse().is_none());
    }

    #[test]
    fn test_matrix_inversion2() {
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            36.84,   427.468, 882.198,  89.504, 
            7.042,   61.891,  56.31,    89.0, 
            72.0,    936.5,   413.80,   50.311,  
            37.698,  311.8,   60.81,    73.839
        );
        let result = matrix.inverse().unwrap();
        let expected: Matrix4x4<f64> = Matrix4x4::new(
             0.01146093272878252,  -0.06212100841992658, -0.02771783718075694,    0.07986947998777854,
            -0.00148039611514755,   0.004464130960444646, 0.003417891441120325,  -0.005915083057511776,
             0.001453087396607042, -0.0009538600348427,  -0.0005129477357421059, -0.0002621470728476185,
            -0.0007967195911958656, 0.01365031989418242,  0.0001408581712825875, -0.002040325515611523
        );
        let epsilon = 1e-7;

        assert!(relative_eq!(result, expected, epsilon = epsilon));
    }

    #[test]
    fn test_matrix_times_inverse_is_identity() {
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            36.84,  427.468, 882.198, 89.504, 
            7.042 , 61.891,  56.31,   89.0, 
            72.0,   936.5,   413.80,  50.311,  
            37.698, 311.8,   60.81,   73.839
        );
        let matrix_inv = matrix.inverse().unwrap();
        let one = Matrix4x4::identity();
        let epsilon = 1e-7;

        assert!(relative_eq!(matrix * matrix_inv, one, epsilon = epsilon),
            "\nmatrix = {:?}\nmatrix_inv = {:?}\nmmatrix * matrix_inv = {:?}\nepsilon = {:?}\n",
            matrix, matrix_inv, matrix * matrix_inv, epsilon
        );
    }

    #[test]
    fn test_constant_times_matrix_inverse_equals_constant_inverse_times_matrix_inverse() {
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            36.84,  427.468, 882.198, 89.504, 
            7.042 , 61.891,  56.31,   89.0, 
            72.0,   936.5,   413.80,  50.311,  
            37.698, 311.8,   60.81,   73.839
        );
        let constant: f64 = 4_f64;
        let constant_times_matrix_inverse = (constant * matrix).inverse().unwrap();
        let constant_inverse_times_matrix_inverse = (1_f64 / constant) * matrix.inverse().unwrap();

        assert_eq!(constant_times_matrix_inverse, constant_inverse_times_matrix_inverse);
    }

    /// Test whether the inverse of the transpose of a matrix is approximately equal to the 
    /// transpose of the inverse of a matrix. when the matrices are defined over the real numbers,
    /// we have the equality
    /// ```
    /// Inverse(Transpose(M)) == Transpose(Inverse(M)).
    /// ```
    /// The equality does not hold over a set of floating point numbers because floating point arithmetic
    /// is not commutative, so we cannot guarantee exact equality even though transposing a matrix does not
    /// cause a loss of precision in the matrix entries. We can only guarantee approximate equality.
    #[test]
    fn test_matrix_transpose_inverse_equals_matrix_inverse_transpose() {
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            36.84,  427.468, 882.198, 89.504, 
            7.042 , 61.891,  56.31,   89.0, 
            72.0,   936.5,   413.80,  50.311,  
            37.698, 311.8,   60.81,   73.839
        );
        let matrix_transpose_inverse = matrix.transpose().inverse().unwrap();
        let matrix_inverse_transpose = matrix.inverse().unwrap().transpose();
        let epsilon = 1e-7;

        assert!(relative_eq!(matrix_transpose_inverse, matrix_inverse_transpose, epsilon = epsilon));
    }

    #[test]
    fn test_inverse_times_matrix_is_identity() {
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            36.84,   427.468, 882.198,  89.504, 
            7.042,   61.891,  56.31,    89.0, 
            72.0,    936.5,   413.80,   50.311,  
            37.698,  311.8,   60.81,    73.839
        );
        let matrix_inv = matrix.inverse().unwrap();
        let one = Matrix4x4::identity();
        let epsilon = 1e-7;
        
        assert!(relative_eq!(matrix_inv * matrix, one, epsilon = epsilon),
            "\nmatrix = {:?}\nmatrix_inv = {:?}\nmatrix_inv * matrix = {:?}\nepsilon = {:?}\n",
            matrix, matrix_inv, matrix_inv * matrix, epsilon
        );
    }

    #[test]
    fn test_matrix_inverse_inverse_equals_matrix() {
        let matrix: Matrix4x4<f64> = Matrix4x4::new(
            36.84,  427.468, 882.198, 89.504, 
            7.042 , 61.891,  56.31,   89.0, 
            72.0,   936.5,   413.80,  50.311,  
            37.698, 311.8,   60.81,   73.839
        );
        let result = matrix.inverse().unwrap().inverse().unwrap();
        let expected = matrix;
        let epsilon = 1e-7;

        assert!(relative_eq!(result, expected, epsilon = epsilon));
    }

    #[test]
    fn test_matrix_elements_should_be_column_major_order() {
        let matrix = Matrix4x4::new(
            1,  2,  3,  4, 
            5,  6,  7,  8, 
            9,  10, 11, 12, 
            13, 14, 15, 16
        );
        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c0r3, matrix[0][3]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c1r3, matrix[1][3]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
        assert_eq!(matrix.c2r3, matrix[2][3]);
        assert_eq!(matrix.c3r0, matrix[3][0]);
        assert_eq!(matrix.c3r1, matrix[3][1]);
        assert_eq!(matrix.c3r2, matrix[3][2]);
        assert_eq!(matrix.c3r3, matrix[3][3]);
    }

    #[test]
    fn test_matrix_swap_columns() {
        let mut result = Matrix4x4::new(
            1,  2,  3,   4, 
            5,  6,  7,   8, 
            9,  10, 11,  12,
            13, 14, 15,  16
        );
        result.swap_columns(3, 1);
        let expected = Matrix4x4::new(
            1,  2,  3,  4,
            13, 14, 15, 16,
            9,  10, 11, 12,
            5,  6,  7,  8 
        );
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_swap_rows() {
        let mut result = Matrix4x4::new(
            1,  2,  3,  4, 
            5,  6,  7,  8, 
            9,  10, 11, 12, 
            13, 14, 15, 16
        );
        result.swap_rows(3, 1);
        let expected = Matrix4x4::new(
            1,  4,  3,  2,
            5,  8,  7,  6,
            9,  12, 11, 10,
            13, 16, 15, 14
        );
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_swap_elements() {
        let mut result = Matrix4x4::new(
            1,  2,  3,  4, 
            5,  6,  7,  8, 
            9,  10, 11, 12,
            13, 14, 15, 16
        );
        result.swap((2, 0), (1, 3));
        let expected = Matrix4x4::new(
            1,  2,  3,  4,
            5,  6,  7,  9,
            8,  10, 11, 12,
            13, 14, 15, 16
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_scale() {
        let matrix = Matrix4x4::from_affine_scale(5);
        let unit_w = Vector4::unit_w();
        let expected = Vector4::new(5, 5, 5, 1);
        let result = matrix * Vector4::new(1, 1, 1, 1);

        assert_eq!(result, expected);
        assert_eq!(matrix * unit_w, unit_w);
    }

    #[test]
    fn test_from_affine_nonuniform_scale() {
        let matrix = Matrix4x4::from_affine_nonuniform_scale(5, 7, 11);
        let unit_w = Vector4::unit_w();
        let expected = Vector4::new(5, 7, 11, 1);
        let result = matrix * Vector4::new(1, 1, 1, 1);

        assert_eq!(result, expected);
        assert_eq!(matrix * unit_w, unit_w);
    }

    #[test]
    fn test_from_affine_shear_x() {
        let shear_x_with_y = 5;
        let shear_x_with_z = 11;
        let matrix = Matrix4x4::from_affine_shear_x(shear_x_with_y, shear_x_with_z);
        let expected = Vector4::new(1 + shear_x_with_y + shear_x_with_z, 1, 1, 1);
        let result = matrix * Vector4::new(1, 1, 1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_x_does_not_change_last_coordinate() {
        let shear_x_with_y = 5;
        let shear_x_with_z = 11;
        let matrix = Matrix4x4::from_affine_shear_x(shear_x_with_y, shear_x_with_z);
        let unit_w = Vector4::unit_w();
        let expected = unit_w;
        let result = matrix * unit_w;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_y() {
        let shear_y_with_x = 3;
        let shear_y_with_z = 11;
        let matrix = Matrix4x4::from_affine_shear_y(shear_y_with_x, shear_y_with_z);
        let expected = Vector4::new(1, 1 + shear_y_with_x + shear_y_with_z, 1, 1);
        let result = matrix * Vector4::new(1, 1, 1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_y_does_not_change_last_coordinate() {
        let shear_y_with_x = 3;
        let shear_y_with_z = 11;
        let matrix = Matrix4x4::from_affine_shear_y(shear_y_with_x, shear_y_with_z);
        let unit_w = Vector4::unit_w();
        let expected = unit_w;
        let result = matrix * unit_w;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_z() {
        let shear_z_with_x = 3;
        let shear_z_with_y = 11;
        let matrix = Matrix4x4::from_affine_shear_z(shear_z_with_x, shear_z_with_y);
        let expected = Vector4::new(1, 1, 1 + shear_z_with_x + shear_z_with_y, 1);
        let result = matrix * Vector4::new(1, 1, 1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_z_does_not_change_last_coordinate() {
        let shear_z_with_x = 3;
        let shear_z_with_y = 11;
        let matrix = Matrix4x4::from_affine_shear_z(shear_z_with_x, shear_z_with_y);
        let unit_w = Vector4::unit_w();
        let expected = unit_w;
        let result = matrix * unit_w;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear() {
        let shear_x_with_y = 2;
        let shear_x_with_z = 4;
        let shear_y_with_x = 8;
        let shear_y_with_z = 7;
        let shear_z_with_x = 3;
        let shear_z_with_y = 11;
        let matrix = Matrix4x4::from_affine_shear(
            shear_x_with_y, shear_x_with_z, shear_y_with_x, shear_y_with_z, shear_z_with_x, shear_z_with_y
        );
        let expected = Vector4::new(
            1 + shear_x_with_y + shear_x_with_z, 
            1 + shear_y_with_x + shear_y_with_z, 
            1 + shear_z_with_x + shear_z_with_y, 
            1
        );
        let result = matrix * Vector4::new(1, 1, 1, 1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_shear_does_not_change_last_coordinate() {
        let shear_x_with_y = 2;
        let shear_x_with_z = 4;
        let shear_y_with_x = 8;
        let shear_y_with_z = 7;
        let shear_z_with_x = 3;
        let shear_z_with_y = 11;
        let matrix = Matrix4x4::from_affine_shear(
            shear_x_with_y, shear_x_with_z, shear_y_with_x, shear_y_with_z, shear_z_with_x, shear_z_with_y
        );
        let unit_w = Vector4::unit_w();
        let expected = unit_w;
        let result = matrix * unit_w;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_reflection_xy_plane() {
        let bias = Vector3::zero();
        let normal = Unit::from_value(Vector3::unit_z());
        let expected = Matrix4x4::new(
            1.0, 0.0,  0.0, 0.0,
            0.0, 1.0,  0.0, 0.0,
            0.0, 0.0, -1.0, 0.0,
            0.0, 0.0,  0.0, 1.0
        );
        let result = Matrix4x4::from_affine_reflection(&normal, &bias);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_reflection_zx_plane() {
        let bias = Vector3::zero();
        let normal = Unit::from_value(-Vector3::unit_y());
        let expected = Matrix4x4::new(
            1.0,  0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,
            0.0,  0.0, 1.0, 0.0,
            0.0,  0.0, 0.0, 1.0
        );
        let result = Matrix4x4::from_affine_reflection(&normal, &bias);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_affine_reflection_yz_plane() {
        let bias = Vector3::zero();
        let normal = Unit::from_value(Vector3::unit_x());
        let expected = Matrix4x4::new(
            -1.0,  0.0, 0.0,  0.0,
             0.0,  1.0, 0.0,  0.0,
             0.0,  0.0, 1.0,  0.0,
             0.0,  0.0, 0.0,  1.0
        );
        let result = Matrix4x4::from_affine_reflection(&normal, &bias);

        assert_eq!(result, expected);
    }

    /// A test case for the plane `z = 1`.
    #[test]
    fn test_from_affine_reflection_plane1() {
        let bias = Vector3::new(0.0, 0.0, 1.0);
        let normal = Unit::from_value(Vector3::new(0.0, 0.0, 1.0));
        let matrix = Matrix4x4::from_affine_reflection(&normal, &bias);
        let vector = Vector4::new(1.0, 1.0, 0.5, 1.0);
        let expected = Vector4::new(1.0,1.0,1.5, 1.0);
        let result = matrix * vector;

        assert_eq!(result, expected);
    }

    /// A test case for the plane `x = -1`.
    #[test]
    fn test_from_affine_reflection_plane2() {
        let bias = Vector3::new(-1.0, 0.0, 0.0);
        let normal = Unit::from_value(Vector3::new(1.0, 0.0, 0.0));
        let matrix = Matrix4x4::from_affine_reflection(&normal, &bias);
        let vector = Vector4::new(-2.0, 1.0, 1.0, 1.0);
        let expected = Vector4::new(0.0,1.0,1.0, 1.0);
        let result = matrix * vector;

        assert_eq!(result, expected);
    }

    /// A test case for the plane `y = 1`.
    #[test]
    fn test_from_affine_reflection_plane3() {
        let bias = Vector3::new(0.0, 1.0, 0.0);
        let normal = Unit::from_value(Vector3::new(0.0, 1.0, 0.0));
        let matrix = Matrix4x4::from_affine_reflection(&normal, &bias);
        let vector = Vector4::new(0.0, 0.0, 0.0, 1.0);
        let expected = Vector4::new(0.0,2.0,0.0, 1.0);
        let result = matrix * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_angle_x() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_y = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let matrix = Matrix4x4::from_affine_angle_x(angle);
        let expected = unit_z.extend(0.0);
        let result = matrix * unit_y.extend(0.0);

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_from_angle_y() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_z = Vector3::unit_z();
        let unit_x = Vector3::unit_x();
        let matrix = Matrix4x4::from_affine_angle_y(angle);
        let expected = unit_x.extend(0.0);
        let result = matrix * unit_z.extend(0.0);

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_from_angle_z() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_x = Vector3::unit_x();
        let unit_y = Vector3::unit_y();
        let matrix = Matrix4x4::from_affine_angle_z(angle);
        let expected = unit_y.extend(0.0);
        let result = matrix * unit_x.extend(0.0);

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_from_affine_angle_x() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_y = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let matrix = Matrix4x4::from_affine_angle_x(angle);
        let expected = unit_z.extend(0.0);
        let result = matrix * unit_y.extend(0.0);

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_from_affine_angle_y() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_z = Vector3::unit_z();
        let unit_x = Vector3::unit_x();
        let matrix = Matrix4x4::from_affine_angle_y(angle);
        let expected = unit_x.extend(0.0);
        let result = matrix * unit_z.extend(0.0);

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_from_affine_angle_z() {
        let angle: Radians<f64> = Radians::full_turn_div_4();
        let unit_x = Vector3::unit_x();
        let unit_y = Vector3::unit_y();
        let matrix = Matrix4x4::from_affine_angle_z(angle);
        let expected = unit_y.extend(0.0);
        let result = matrix * unit_x.extend(0.0);

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_from_affine_axis_angle() {
        let angle: Radians<f64> = Radians::full_turn_div_2();
        let axis = Unit::from_value(
            (1.0 / f64::sqrt(2.0)) * Vector3::new(1.0, 1.0, 0.0)
        );
        let vector = Vector4::new(1.0, 1.0, -1.0, 0.0);
        let matrix = Matrix4x4::from_affine_axis_angle(&axis, angle);
        let expected = Vector4::new(1.0, 1.0, 1.0,0.0);
        let result = matrix * vector;

        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    /// An affine translation should only displace points and not vectors. We 
    /// distinguish points by using a `1` in the last coordinate, and vectors 
    /// by using a `0` in the last coordinate.
    #[test]
    fn test_from_affine_translation_point() {
        let distance = Vector3::new(3, 7, 11);
        let matrix = Matrix4x4::from_affine_translation(&distance);
        let point = Vector4::new(0, 0, 0, 1);
        let expected = Vector4::new(3, 7, 11, 1);
        let result = matrix * point;

        assert_eq!(result, expected);
    }

    /// An affine translation should only displace points and not vectors. We 
    /// distinguish points by using a `1` in the last coordinate, and vectors 
    /// by using a `0` in the last coordinate.
    #[test]
    fn test_from_affine_translation_vector() {
        let distance = Vector3::new(3, 7, 11);
        let matrix = Matrix4x4::from_affine_translation(&distance);
        let vector = Vector4::new(0, 0, 0, 0);
        let expected = vector;
        let result = matrix * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_orthographic() {
        let left = -4.0;
        let right = 4.0;
        let bottom = -2.0;
        let top = 2.0;
        let near = 1.0;
        let far = 100.0;
        let expected = Matrix4x4::new(
            1.0 / 4.0,  0.0,        0.0,          0.0,
            0.0,        1.0 / 2.0,  0.0,          0.0,
            0.0,        0.0,       -2.0 / 99.0,   0.0,
            0.0,        0.0,       -101.0 / 99.0, 1.0
        );
        let result = Matrix4x4::from_orthographic(left, right, bottom, top, near, far);
    
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_perspective_fov() {
        let vfov = Degrees(72.0);
        let aspect = 800 as f32 / 600 as f32;
        let near = 0.1;
        let far = 100.0;
        let expected = Matrix4x4::new(
            1.0322863, 0.0,        0.0,       0.0, 
            0.0,       1.3763818,  0.0,       0.0, 
            0.0,       0.0,       -1.002002, -1.0, 
            0.0,       0.0,       -0.2002002, 0.0
        );
        let result = Matrix4x4::from_perspective_fov(vfov, aspect, near, far);
    
        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_perspective() {
        let left = -4.0;
        let right = 4.0;
        let bottom = -2.0;
        let top = 3.0;
        let near = 1.0;
        let far = 100.0;
        let expected = Matrix4x4::new(
            1.0 / 4.0,  0.0,        0.0,           0.0,
            0.0,        2.0 / 5.0,  0.0,           0.0,
            0.0,        1.0 / 5.0, -101.0 / 99.0, -1.0,
            0.0,        0.0,       -200.0 / 99.0,  0.0
        );
        let result = Matrix4x4::from_perspective(left, right, bottom, top, near, far);
    
        assert_eq!(result, expected);
    }


    #[test]
    fn test_from_orthographic_fov() {
        let vfov = Degrees(90.0);
        let aspect = 800 as f64 / 600 as f64;
        let near = 1.0;
        let far = 100.0;
        let expected = Matrix4x4::new(
            2.0 / 100.0, 0.0,         0.0,          0.0, 
            0.0,         2.0 / 75.0,  0.0,          0.0, 
            0.0,         0.0,        -2.0 / 99.0,   0.0, 
            0.0,         0.0,        -101.0 / 99.0, 1.0
        );
        let result = Matrix4x4::from_orthographic_fov(vfov, aspect, near, far);
    
        assert!(relative_eq!(result, expected, epsilon = 1e-8));
    }

    #[test]
    fn test_look_at_rh_at_origin() {
        let eye = Point3::new(0.0, 0.0, 0.0);
        let target = Point3::new(1.0, 1.0, 1.0);
        let up = Vector3::unit_y();
        let minus_unit_z = -Vector3::unit_z();
        let look_at = Matrix4x4::look_at_rh(&eye, &target, &up);
        let direction = target - Point3::origin();
        let expected = minus_unit_z.extend(0.0);
        let result = look_at * direction.normalize().extend(0.0);

        assert!(relative_eq!(result, expected, epsilon = 1e-7));
    }

    #[test]
    fn test_look_at_lh_at_origin() {
        let eye = Point3::new(0.0, 0.0, 0.0);
        let target = Point3::new(1.0, 1.0, 1.0);
        let up = Vector3::unit_y();
        let unit_z = Vector3::unit_z();
        let look_at = Matrix4x4::look_at_lh(&eye, &target, &up);
        let direction = target - Point3::origin();
        let expected = unit_z.extend(0.0);
        let result = look_at * direction.normalize().extend(0.0);

        assert!(relative_eq!(result, expected, epsilon = 1e-7));
    }

    #[test]
    fn test_look_at_lh_no_displacement_or_rotation() {
        let eye = Point3::new(0.0, 0.0, 0.0);
        let target = Point3::new(0.0, 0.0, 1.0);
        let up = Vector3::unit_y();
        let look_at = Matrix4x4::look_at_lh(&eye, &target, &up);
        let direction = target - Point3::origin();
        let expected = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let result = look_at * direction.normalize().extend(0.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_look_at_rh_no_displacement_or_rotation() {
        let eye = Point3::new(0.0, 0.0, 0.0);
        let target = Point3::new(0.0, 0.0, 1.0);
        let up = Vector3::unit_y();
        let look_at = Matrix4x4::look_at_rh(&eye, &target, &up);
        let expected = Vector4::new(0.0, 0.0, 0.0, 1.0);
        let result = look_at * eye.to_homogeneous();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_look_at_lh_eye_to_origin() {
        let eye = Point3::new(-1.0, -1.0, -1.0);
        let target = Point3::new(1.0, 1.0, 1.0);
        let up = Vector3::unit_y();
        let look_at = Matrix4x4::look_at_lh(&eye, &target, &up);
        let expected = Vector4::unit_w();
        let result = look_at * eye.to_homogeneous();
        eprintln!("{}", look_at);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_look_at_rh_eye_to_origin() {
        let eye = Point3::new(-1.0, -1.0, -1.0);
        let target = Point3::new(1.0, 1.0, 1.0);
        let up = Vector3::unit_y();
        let look_at = Matrix4x4::look_at_rh(&eye, &target, &up);
        let expected = Vector4::unit_w();
        let result = look_at * eye.to_homogeneous();
        eprintln!("{}", look_at);
        assert_eq!(result, expected);
    }
}


#[cfg(test)]
mod matrix1x2_tests {
    use cglinalg::{
        Vector1,
        Vector2,
        Matrix1x2,
        Matrix2x2,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[1][0], 2_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix[3][0], matrix[3][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix1x2::new(1_i32, 2_i32);

        assert_eq!(matrix[0][2], matrix[0][2]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        let matrix = Matrix1x2::new(2_i32, 3_i32);
        let identity = Matrix2x2::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        let matrix = Matrix1x2::new(33_i32, 54_i32);
        let zero_mat2x2 = Matrix2x2::zero();
        let zero_mat1x2 = Matrix1x2::zero();

        assert_eq!(matrix * zero_mat2x2, zero_mat1x2);
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        let matrix = Matrix1x2::new(33_i32, 54_i32);
        let zero = 0_i32;
        let zero_mat1x2 = Matrix1x2::zero();

        assert_eq!(zero * matrix, zero_mat1x2);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let matrix1x2 = Matrix1x2::new(2_i32, 3_i32);
        let matrix2x2 = Matrix2x2::new(1_i32, 2_i32, 3_i32, 4_i32);
        let expected = Matrix1x2::new(8_i32, 18_i32);
        let result = matrix1x2 * matrix2x2;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix1x2 = Matrix1x2::new(4_i32, 5_i32);
        let vector = Vector2::new(9_i32, 6_i32);
        let expected = Vector1::new(66_i32);
        let result = matrix1x2 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix1x2 = Matrix1x2::new(1_i32, 2_i32);
        let scalar = 13_i32;
        let expected = Matrix1x2::new(13_i32, 26_i32);
        let result = matrix1x2 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix1x2 = Matrix1x2::new(1_i32, 2_i32);
        let scalar = 13_i32;
        let expected = Matrix1x2::new(13_i32, 26_i32);
        let result = scalar * matrix1x2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_mat1x2 = Matrix1x2::zero();
        let matrix = Matrix1x2::new(3684_i32, 42746_i32);

        assert_eq!(matrix + zero_mat1x2, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_mat1x2 = Matrix1x2::zero();
        let matrix = Matrix1x2::new(3684_i32, 42746_i32);

        assert_eq!(zero_mat1x2 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix1x2::new(23_i32, 76_i32);
        let matrix2 = Matrix1x2::new(1_i32, 5_i32);
        let expected = Matrix1x2::new(24_i32, 81_i32);
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix1x2::new(3_i32, 6_i32);
        let matrix2 = Matrix1x2::new(1_i32, 15_i32);
        let expected = Matrix1x2::new(2_i32, -9_i32);
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix1x2::new(3_i32, 6_i32);
        let zero_mat1x2 = Matrix1x2::zero();

        assert_eq!(matrix - matrix, zero_mat1x2);
    }
}

#[cfg(test)]
mod matrix1x3_tests {
    use cglinalg::{
        Vector1,
        Vector3,
        Matrix1x3,
        Matrix3x3,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[1][0], 2_i32);
        assert_eq!(matrix[2][0], 3_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix[3][0], matrix[3][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix1x3::new(1_i32, 2_i32, 3_i32);

        assert_eq!(matrix[0][2], matrix[0][2]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        let matrix = Matrix1x3::new(2_i32, 3_i32, 4_i32);
        let identity = Matrix3x3::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        let matrix = Matrix1x3::new(33_i32, 54_i32, 19_i32);
        let zero_mat3x3 = Matrix3x3::zero();
        let zero_mat1x3 = Matrix1x3::zero();

        assert_eq!(matrix * zero_mat3x3, zero_mat1x3);
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        let matrix = Matrix1x3::new(33_i32, 54_i32, 19_i32);
        let zero = 0_i32;
        let zero_mat1x3 = Matrix1x3::zero();

        assert_eq!(zero * matrix, zero_mat1x3);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let matrix1x3 = Matrix1x3::new(2_i32, 3_i32, 4_i32);
        let matrix3x3 = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32, 
            7_i32, 8_i32, 9_i32
        );
        let expected = Matrix1x3::new(20_i32, 47_i32, 74_i32);
        let result = matrix1x3 * matrix3x3;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix1x3 = Matrix1x3::new(4_i32, 5_i32, 6_i32);
        let vector = Vector3::new(9_i32, 6_i32, -12_i32);
        let expected = Vector1::new(-6_i32);
        let result = matrix1x3 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix1x3 = Matrix1x3::new(1_i32, 2_i32, 3_i32);
        let scalar = 13_i32;
        let expected = Matrix1x3::new(13_i32, 26_i32, 39_i32);
        let result = matrix1x3 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix1x3 = Matrix1x3::new(1_i32, 2_i32, 3_i32);
        let scalar = 13_i32;
        let expected = Matrix1x3::new(13_i32, 26_i32, 39_i32);
        let result = scalar * matrix1x3;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_mat1x3 = Matrix1x3::zero();
        let matrix = Matrix1x3::new(3684_i32, 42746_i32, 345_i32);

        assert_eq!(matrix + zero_mat1x3, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_mat1x3 = Matrix1x3::zero();
        let matrix = Matrix1x3::new(3684_i32, 42746_i32, 345_i32);

        assert_eq!(zero_mat1x3 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix1x3::new(23_i32, 76_i32, 89_i32);
        let matrix2 = Matrix1x3::new(1_i32, 5_i32, 9_i32);
        let expected = Matrix1x3::new(24_i32, 81_i32, 98_i32);
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix1x3::new(3_i32, 6_i32, 9_i32);
        let matrix2 = Matrix1x3::new(1_i32, 15_i32, 29_i32);
        let expected = Matrix1x3::new(2_i32, -9_i32, -20_i32);
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix1x3::new(3_i32, 6_i32, 9_i32);
        let zero_mat1x3 = Matrix1x3::zero();

        assert_eq!(matrix - matrix, zero_mat1x3);
    }
}

#[cfg(test)]
mod matrix1x4_tests {
    use cglinalg::{
        Vector1,
        Vector4,
        Matrix1x4,
        Matrix4x4,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[1][0], 2_i32);
        assert_eq!(matrix[2][0], 3_i32);
        assert_eq!(matrix[3][0], 4_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c3r0, matrix[3][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[4][0], matrix[4][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);

        assert_eq!(matrix[0][2], matrix[0][2]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        let matrix = Matrix1x4::new(2_i32, 3_i32, 4_i32, 5_i32);
        let identity = Matrix4x4::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        let matrix = Matrix1x4::new(33_i32, 54_i32, 19_i32, 5_i32);
        let zero_mat4x4 = Matrix4x4::zero();
        let zero_mat1x4 = Matrix1x4::zero();

        assert_eq!(matrix * zero_mat4x4, zero_mat1x4);
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        let matrix = Matrix1x4::new(33_i32, 54_i32, 19_i32, 5_i32);
        let zero = 0_i32;
        let zero_mat1x4 = Matrix1x4::zero();

        assert_eq!(zero * matrix, zero_mat1x4);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let matrix1x4 = Matrix1x4::new(2_i32, 3_i32, 4_i32, 5_i32);
        let matrix4x4 = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32,
            5_i32,  6_i32,  7_i32,  8_i32, 
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );
        let expected = Matrix1x4::new(40_i32, 96_i32, 152_i32, 208_i32);
        let result = matrix1x4 * matrix4x4;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix1x4 = Matrix1x4::new(4_i32, 5_i32, 6_i32, 7_i32);
        let vector = Vector4::new(9_i32, 6_i32, -12_i32, -72_i32);
        let expected = Vector1::new(-510_i32);
        let result = matrix1x4 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix1x4 = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);
        let scalar = 13_i32;
        let expected = Matrix1x4::new(13_i32, 26_i32, 39_i32, 52_i32);
        let result = matrix1x4 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix1x4 = Matrix1x4::new(1_i32, 2_i32, 3_i32, 4_i32);
        let scalar = 13_i32;
        let expected = Matrix1x4::new(13_i32, 26_i32, 39_i32, 52_i32);
        let result = scalar * matrix1x4;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_mat1x4 = Matrix1x4::zero();
        let matrix = Matrix1x4::new(3684_i32, 42746_i32, 345_i32, 546_i32);

        assert_eq!(matrix + zero_mat1x4, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_mat1x4 = Matrix1x4::zero();
        let matrix = Matrix1x4::new(3684_i32, 42746_i32, 345_i32, 546_i32);

        assert_eq!(zero_mat1x4 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix1x4::new(23_i32, 76_i32, 89_i32, 34_i32);
        let matrix2 = Matrix1x4::new(1_i32, 5_i32, 9_i32, 13_i32);
        let expected = Matrix1x4::new(24_i32, 81_i32, 98_i32, 47_i32);
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix1x4::new(3_i32, 6_i32, 9_i32, 12_i32);
        let matrix2 = Matrix1x4::new(1_i32, 15_i32, 29_i32, 6_i32);
        let expected = Matrix1x4::new(2_i32, -9_i32, -20_i32, 6_i32);
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix1x4::new(3_i32, 6_i32, 9_i32, 12_i32);
        let zero_mat1x4 = Matrix1x4::zero();

        assert_eq!(matrix - matrix, zero_mat1x4);
    }
}



#[cfg(test)]
mod matrix2x3_tests {
    use cglinalg::{
        Vector3,
        Vector2,
        Matrix2x2,
        Matrix2x3,
        Matrix3x2,
        Matrix3x3,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[1][0], 3_i32);
        assert_eq!(matrix[1][1], 4_i32);
        assert_eq!(matrix[2][0], 5_i32);
        assert_eq!(matrix[2][1], 6_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix[3][0], matrix[3][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );

        assert_eq!(matrix[0][2], matrix[0][2]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        let matrix = Matrix2x3::new(
            2_i32, 3_i32, 
            4_i32, 5_i32,
            6_i32, 7_i32
        );
        let identity = Matrix3x3::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        let matrix = Matrix2x3::new(
            33_i32,  54_i32, 
            19_i32,  5_i32,
            793_i32, 23_i32
        );
        let zero_mat3x3 = Matrix3x3::zero();
        let zero_mat2x3 = Matrix2x3::zero();

        assert_eq!(matrix * zero_mat3x3, zero_mat2x3);
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        let matrix = Matrix2x3::new(
            33_i32,  54_i32, 
            19_i32,  5_i32,
            234_i32, 98_i32
        );
        let zero = 0_i32;
        let zero_mat2x3 = Matrix2x3::zero();

        assert_eq!(zero * matrix, zero_mat2x3);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let matrix2x3 = Matrix2x3::new(
            2_i32, 3_i32, 
            4_i32, 5_i32, 
            6_i32, 7_i32
        );
        let matrix3x3 = Matrix3x3::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 8_i32,
            7_i32, 8_i32, 9_i32
        );
        let expected = Matrix2x3::new(
            28_i32,  34_i32, 
            76_i32,  93_i32, 
            100_i32, 124_i32    
        );
        let result = matrix2x3 * matrix3x3;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix2x3 = Matrix2x3::new(
            2_i32, 3_i32, 
            4_i32, 5_i32, 
            6_i32, 7_i32
        );
        let matrix3x2 = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 8_i32
        );
        let expected = Matrix2x2::new(
            28_i32,  34_i32, 
            76_i32,  93_i32
        );
        let result = matrix2x3 * matrix3x2;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication3() {
        let matrix2x3 = Matrix2x3::new(
            4_i32, 5_i32, 
            6_i32, 7_i32, 
            8_i32, 9_i32
        );
        let vector = Vector3::new(9_i32, 6_i32, -12_i32);
        let expected = Vector2::new(-24_i32, -21_i32);
        let result = matrix2x3 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix2x3 = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 7_i32
        );
        let scalar = 13_i32;
        let expected = Matrix2x3::new(
            13_i32, 26_i32,
            39_i32, 52_i32,
            65_i32, 91_i32
        );
        let result = matrix2x3 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix2x3 = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 7_i32
        );
        let scalar = 13_i32;
        let expected = Matrix2x3::new(
            13_i32, 26_i32,
            39_i32, 52_i32,
            65_i32, 91_i32
        );
        let result = scalar * matrix2x3;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_mat2x3 = Matrix2x3::zero();
        let matrix = Matrix2x3::new(
            3684_i32, 42746_i32, 
            345_i32,  546_i32,  
            76_i32,   167_i32
        );

        assert_eq!(matrix + zero_mat2x3, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_mat2x3 = Matrix2x3::zero();
        let matrix = Matrix2x3::new(
            3684_i32, 42746_i32, 
            345_i32,  546_i32,  
            76_i32,   167_i32
        );

        assert_eq!(zero_mat2x3 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix2x3::new(
            23_i32,  76_i32, 
            89_i32,  34_i32,
            324_i32, 75_i32
        );
        let matrix2 = Matrix2x3::new(
            1_i32,  5_i32, 
            9_i32,  13_i32,
            17_i32, 21_i32
        );
        let expected = Matrix2x3::new(
            24_i32,  81_i32, 
            98_i32,  47_i32,
            341_i32, 96_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix2x3::new(
            3_i32, 6_i32, 
            9_i32, 12_i32,
            15_i32, 18_i32
            
        );
        let matrix2 = Matrix2x3::new(
            1_i32,   15_i32, 
            29_i32,  6_i32,
            234_i32, 93_i32,
        );
        let expected = Matrix2x3::new(
             2_i32,  -9_i32, 
            -20_i32,  6_i32,
            -219_i32, -75_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix2x3::new(
            3_i32,  6_i32, 
            9_i32,  12_i32,
            15_i32, 18_i32
        );
        let zero_mat2x3 = Matrix2x3::zero();

        assert_eq!(matrix - matrix, zero_mat2x3);
    }

    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );
        let expected = Matrix3x2::new(
            1_i32, 3_i32, 5_i32,
            2_i32, 4_i32, 6_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }
}


#[cfg(test)]
mod matrix3x2_tests {
    use cglinalg::{
        Vector3,
        Vector2,
        Matrix2x2,
        Matrix2x3,
        Matrix3x2,
        Matrix3x3,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[1][0], 4_i32);
        assert_eq!(matrix[1][1], 5_i32);
        assert_eq!(matrix[1][2], 6_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix[2][0], matrix[2][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 6_i32
        );

        assert_eq!(matrix[0][3], matrix[0][3]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        let matrix = Matrix3x2::new(
            2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32
        );
        let identity = Matrix2x2::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        let matrix = Matrix3x2::new(
            33_i32, 54_i32,  19_i32,
            5_i32,  793_i32, 23_i32
        );
        let zero_mat2x2 = Matrix2x2::zero();
        let zero_mat3x2 = Matrix3x2::zero();

        assert_eq!(matrix * zero_mat2x2, zero_mat3x2);
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        let matrix = Matrix3x2::new(
            33_i32, 54_i32,  19_i32,
            5_i32,  234_i32, 98_i32
        );
        let zero = 0_i32;
        let zero_mat3x2 = Matrix3x2::zero();

        assert_eq!(zero * matrix, zero_mat3x2);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let matrix3x2 = Matrix3x2::new(
            2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32
        );
        let matrix2x2 = Matrix2x2::new(
            1_i32, 2_i32, 
            4_i32, 5_i32,
        );
        let expected = Matrix3x2::new(
            12_i32, 15_i32, 18_i32,  
            33_i32, 42_i32, 51_i32    
        );
        let result = matrix3x2 * matrix2x2;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix3x2 = Matrix3x2::new(
            2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32
        );
        let matrix2x3 = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 8_i32
        );
        let expected = Matrix3x3::new(
            12_i32, 15_i32, 18_i32,
            26_i32, 33_i32, 40_i32,
            50_i32, 63_i32, 76_i32
        );
        let result = matrix3x2 * matrix2x3;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication3() {
        let matrix3x2 = Matrix3x2::new(
            4_i32, 5_i32, 6_i32, 
            7_i32, 8_i32, 9_i32
        );
        let vector = Vector2::new(9_i32, -6_i32);
        let expected = Vector3::new(-6_i32, -3_i32, 0_i32);
        let result = matrix3x2 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix3x2 = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 7_i32
        );
        let scalar = 13_i32;
        let expected = Matrix3x2::new(
            13_i32, 26_i32, 39_i32, 
            52_i32, 65_i32, 91_i32
        );
        let result = matrix3x2 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix3x2 = Matrix3x2::new(
            1_i32, 2_i32, 3_i32, 
            4_i32, 5_i32, 7_i32
        );
        let scalar = 13_i32;
        let expected = Matrix3x2::new(
            13_i32, 26_i32, 39_i32, 
            52_i32, 65_i32, 91_i32
        );
        let result = scalar * matrix3x2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_mat3x2 = Matrix3x2::zero();
        let matrix = Matrix3x2::new(
            3684_i32, 42746_i32, 345_i32, 
            546_i32,  76_i32,    167_i32
        );

        assert_eq!(matrix + zero_mat3x2, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_mat3x2 = Matrix3x2::zero();
        let matrix = Matrix3x2::new(
            3684_i32, 42746_i32, 345_i32, 
            546_i32,  76_i32,    167_i32
        );

        assert_eq!(zero_mat3x2 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix3x2::new(
            23_i32, 76_i32,  89_i32,  
            34_i32, 324_i32, 75_i32
        );
        let matrix2 = Matrix3x2::new(
            1_i32,  5_i32,  9_i32,  
            13_i32, 17_i32, 21_i32
        );
        let expected = Matrix3x2::new(
            24_i32, 81_i32, 98_i32,
            47_i32, 341_i32, 96_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix3x2::new(
            3_i32,  6_i32,  9_i32, 
            12_i32, 15_i32, 18_i32
        );
        let matrix2 = Matrix3x2::new(
            1_i32, 15_i32,  29_i32,  
            6_i32, 234_i32, 93_i32,
        );
        let expected = Matrix3x2::new(
             2_i32, -9_i32,   -20_i32, 
             6_i32, -219_i32, -75_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix3x2::new(
            3_i32,  6_i32,  9_i32,
            12_i32, 15_i32, 18_i32
        );
        let zero_mat3x2 = Matrix3x2::zero();

        assert_eq!(matrix - matrix, zero_mat3x2);
    }

    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix3x2::new(
            1_i32, 3_i32, 5_i32,
            2_i32, 4_i32, 6_i32
        );
        let expected = Matrix2x3::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }
}



#[cfg(test)]
mod matrix2x4_tests {
    use cglinalg::{
        Vector4,
        Vector2,
        Matrix2x4,
        Matrix4x2,
        Matrix4x4,
        Matrix2x2,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32,
            7_i32, 8_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[1][0], 3_i32);
        assert_eq!(matrix[1][1], 4_i32);
        assert_eq!(matrix[2][0], 5_i32);
        assert_eq!(matrix[2][1], 6_i32);
        assert_eq!(matrix[3][0], 7_i32);
        assert_eq!(matrix[3][1], 8_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 6_i32,
            7_i32, 8_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c3r0, matrix[3][0]);
        assert_eq!(matrix.c3r1, matrix[3][1]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 6_i32,
            7_i32, 8_i32
        );

        assert_eq!(matrix[4][0], matrix[4][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 6_i32, 
            7_i32, 8_i32
        );

        assert_eq!(matrix[0][2], matrix[0][2]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        let matrix = Matrix2x4::new(
            2_i32, 3_i32, 
            4_i32, 5_i32,
            6_i32, 7_i32,
            8_i32, 9_i32
        );
        let identity = Matrix4x4::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        let matrix = Matrix2x4::new(
            33_i32,  54_i32, 
            19_i32,  5_i32,
            793_i32, 23_i32,
            49_i32,  11_i32
        );
        let zero_mat4x4 = Matrix4x4::zero();
        let zero_mat2x4 = Matrix2x4::zero();

        assert_eq!(matrix * zero_mat4x4, zero_mat2x4);
    }

    #[test]
    fn test_zero_mat_times_mat_equals_zero() {
        let matrix = Matrix2x4::new(
            33_i32,  54_i32, 
            19_i32,  5_i32,
            793_i32, 23_i32,
            49_i32,  11_i32
        );
        let zero_mat2x2 = Matrix2x2::zero();
        let zero_mat2x4 = Matrix2x4::zero();

        assert_eq!(zero_mat2x2 * matrix, zero_mat2x4);
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        let matrix = Matrix2x4::new(
            33_i32,  54_i32, 
            19_i32,  5_i32,
            234_i32, 98_i32,
            64_i32,  28_i32
        );
        let zero = 0_i32;
        let zero_mat2x4 = Matrix2x4::zero();

        assert_eq!(zero * matrix, zero_mat2x4);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let matrix2x4 = Matrix2x4::new(
            2_i32, 3_i32, 
            4_i32, 5_i32, 
            6_i32, 7_i32,
            8_i32, 9_i32
        );
        let matrix4x4 = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );
        let expected = Matrix2x4::new(
            60_i32,  70_i32,
            140_i32, 166_i32,
            220_i32, 262_i32,
            300_i32, 358_i32
        );
        let result = matrix2x4 * matrix4x4;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix2x4 = Matrix2x4::new(
            4_i32,  5_i32, 
            6_i32,  7_i32, 
            8_i32,  9_i32, 
            10_i32, 11_i32
        );
        let vector = Vector4::new(9_i32, 6_i32, -12_i32, -24_i32);
        let expected = Vector2::new(-264_i32, -285_i32);
        let result = matrix2x4 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication3() {
        let matrix2x4 = Matrix2x4::new(
            2_i32, 3_i32,
            4_i32, 5_i32, 
            6_i32, 7_i32, 
            8_i32, 9_i32,
        );
        let vector = Vector4::new(9_i32, -6_i32, 12_i32, 4_i32);
        let expected = Vector2::new(98_i32, 117_i32);
        let result = matrix2x4 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix2x4 = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 7_i32,
            8_i32, 9_i32
        );
        let scalar = 13_i32;
        let expected = Matrix2x4::new(
            13_i32,  26_i32,
            39_i32,  52_i32,
            65_i32,  91_i32,
            104_i32, 117_i32
        );
        let result = matrix2x4 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix2x4 = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32,
            5_i32, 7_i32,
            8_i32, 9_i32
        );
        let scalar = 13_i32;
        let expected = Matrix2x4::new(
            13_i32,  26_i32,
            39_i32,  52_i32,
            65_i32,  91_i32,
            104_i32, 117_i32
        );
        let result = scalar * matrix2x4;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_mat2x4 = Matrix2x4::zero();
        let matrix = Matrix2x4::new(
            3684_i32, 42746_i32, 
            345_i32,  546_i32,  
            76_i32,   167_i32,
            415_i32,  251_i32
        );

        assert_eq!(matrix + zero_mat2x4, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_mat2x4 = Matrix2x4::zero();
        let matrix = Matrix2x4::new(
            3684_i32, 42746_i32, 
            345_i32,  546_i32,  
            76_i32,   167_i32,
            415_i32,  251_i32
        );

        assert_eq!(zero_mat2x4 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix2x4::new(
            23_i32,  76_i32, 
            89_i32,  34_i32,
            324_i32, 75_i32,
            614_i32, 15_i32
        );
        let matrix2 = Matrix2x4::new(
            1_i32,  5_i32, 
            9_i32,  13_i32,
            17_i32, 21_i32,
            87_i32, 41_i32
        );
        let expected = Matrix2x4::new(
            24_i32,  81_i32, 
            98_i32,  47_i32,
            341_i32, 96_i32,
            701_i32, 56_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix2x4::new(
            3_i32,  6_i32, 
            9_i32,  12_i32,
            15_i32, 18_i32,
            21_i32, 24_i32
        );
        let matrix2 = Matrix2x4::new(
            1_i32,   15_i32, 
            29_i32,  6_i32,
            234_i32, 93_i32,
            93_i32,  7_i32
        );
        let expected = Matrix2x4::new(
             2_i32,   -9_i32, 
            -20_i32,   6_i32,
            -219_i32, -75_i32,
            -72_i32,   17_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix2x4::new(
            3_i32,  6_i32, 
            9_i32,  12_i32,
            15_i32, 18_i32,
            21_i32, 24_i32
        );
        let zero_mat2x4 = Matrix2x4::zero();

        assert_eq!(matrix - matrix, zero_mat2x4);
    }

    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 6_i32,
            7_i32, 8_i32
        );
        let expected = Matrix4x2::new(
            1_i32, 3_i32, 5_i32, 7_i32, 
            2_i32, 4_i32, 6_i32, 8_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }
}


#[cfg(test)]
mod matrix4x2_tests {
    use cglinalg::{
        Vector4,
        Vector2,
        Matrix2x2,
        Matrix2x4,
        Matrix4x2,
        Matrix4x4,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[0][3], 4_i32);
        assert_eq!(matrix[1][0], 5_i32);
        assert_eq!(matrix[1][1], 6_i32);
        assert_eq!(matrix[1][2], 7_i32);
        assert_eq!(matrix[1][3], 8_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c0r3, matrix[0][3]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c1r3, matrix[1][3]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix[0][4], matrix[0][4]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );

        assert_eq!(matrix[2][0], matrix[2][0]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        let matrix = Matrix4x2::new(
            2_i32, 3_i32, 4_i32, 5_i32, 
            6_i32, 7_i32, 8_i32, 9_i32
        );
        let identity = Matrix2x2::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        let matrix = Matrix4x2::new(
            33_i32, 54_i32,  19_i32, 345_i32,
            5_i32,  793_i32, 23_i32, 324_i32
        );
        let zero_mat2x2 = Matrix2x2::zero();
        let zero_mat3x2 = Matrix4x2::zero();

        assert_eq!(matrix * zero_mat2x2, zero_mat3x2);
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        let matrix = Matrix4x2::new(
            33_i32, 54_i32,  19_i32, 29_i32,
            5_i32,  234_i32, 98_i32, 7_i32
        );
        let zero = 0_i32;
        let zero_mat3x2 = Matrix4x2::zero();

        assert_eq!(zero * matrix, zero_mat3x2);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let matrix4x2 = Matrix4x2::new(
            2_i32, 3_i32, 4_i32, 5_i32,
            5_i32, 6_i32, 7_i32, 8_i32
        );
        let matrix2x2 = Matrix2x2::new(
            1_i32, 2_i32, 
            4_i32, 5_i32
        );
        let expected = Matrix4x2::new(
            12_i32, 15_i32, 18_i32, 21_i32,
            33_i32, 42_i32, 51_i32, 60_i32
        );
        let result = matrix4x2 * matrix2x2;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix4x2 = Matrix4x2::new(
            2_i32, 3_i32, 4_i32, 5_i32, 
            5_i32, 6_i32, 7_i32, 8_i32
        );
        let matrix2x4 = Matrix2x4::new(
            1_i32, 2_i32, 
            3_i32, 4_i32, 
            5_i32, 8_i32,
            7_i32, 10_i32
        );
        let expected = Matrix4x4::new(
            12_i32, 15_i32, 18_i32, 21_i32,
            26_i32, 33_i32, 40_i32, 47_i32,
            50_i32, 63_i32, 76_i32, 89_i32,
            64_i32, 81_i32, 98_i32, 115_i32
        );
        let result = matrix4x2 * matrix2x4;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication3() {
        let matrix4x2 = Matrix4x2::new(
            4_i32, 5_i32, 6_i32,  7_i32, 
            8_i32, 9_i32, 10_i32, 11_i32
        );
        let vector = Vector2::new(9_i32, -6_i32);
        let expected = Vector4::new(-12_i32, -9_i32, -6_i32, -3_i32);
        let result = matrix4x2 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix4x2 = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32, 
            4_i32, 5_i32, 7_i32, 8_i32,
        );
        let scalar = 13_i32;
        let expected = Matrix4x2::new(
            13_i32, 26_i32, 39_i32, 52_i32,
            52_i32, 65_i32, 91_i32, 104_i32
        );
        let result = matrix4x2 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix4x2 = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32,
            4_i32, 5_i32, 7_i32, 8_i32
        );
        let scalar = 13_i32;
        let expected = Matrix4x2::new(
            13_i32, 26_i32, 39_i32, 52_i32,
            52_i32, 65_i32, 91_i32, 104_i32
        );
        let result = scalar * matrix4x2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_mat4x2 = Matrix4x2::zero();
        let matrix = Matrix4x2::new(
            3684_i32, 42746_i32, 345_i32, 456_i32,
            546_i32,  76_i32,    167_i32, 915_i32
        );

        assert_eq!(matrix + zero_mat4x2, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_mat4x2 = Matrix4x2::zero();
        let matrix = Matrix4x2::new(
            3684_i32, 42746_i32, 345_i32, 456_i32,
            546_i32,  76_i32,    167_i32, 915_i32
        );

        assert_eq!(zero_mat4x2 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix4x2::new(
            23_i32, 76_i32,  89_i32, 11_i32,
            34_i32, 324_i32, 75_i32, 62_i32
        );
        let matrix2 = Matrix4x2::new(
            1_i32,  5_i32,  9_i32,  82_i32,
            13_i32, 17_i32, 21_i32, 6_i32
        );
        let expected = Matrix4x2::new(
            24_i32, 81_i32, 98_i32,  93_i32,
            47_i32, 341_i32, 96_i32, 68_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix4x2::new(
            3_i32,  6_i32,  9_i32,  65_i32,
            12_i32, 15_i32, 18_i32, 333_i32
        );
        let matrix2 = Matrix4x2::new(
            1_i32, 15_i32,  29_i32, 27_i32,
            6_i32, 234_i32, 93_i32, 38_i32
        );
        let expected = Matrix4x2::new(
            2_i32, -9_i32,   -20_i32, 38_i32,
            6_i32, -219_i32, -75_i32, 295_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix4x2::new(
            3_i32,  6_i32,  9_i32,  12_i32,
            12_i32, 15_i32, 18_i32, 21_i32
        );
        let zero_mat3x2 = Matrix4x2::zero();

        assert_eq!(matrix - matrix, zero_mat3x2);
    }

    #[test]
    fn test_transpose() {
        let matrix = Matrix4x2::new(
            1_i32, 2_i32, 3_i32, 4_i32,
            5_i32, 6_i32, 7_i32, 8_i32
        );
        let expected = Matrix2x4::new(
            1_i32, 5_i32,
            2_i32, 6_i32,
            3_i32, 7_i32,
            4_i32, 8_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }
}


#[cfg(test)]
mod matrix3x4_tests {
    use cglinalg::{
        Vector4,
        Vector3,
        Matrix3x4,
        Matrix4x3,
        Matrix4x4,
        Matrix3x3,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[1][0], 4_i32);
        assert_eq!(matrix[1][1], 5_i32);
        assert_eq!(matrix[1][2], 6_i32);
        assert_eq!(matrix[2][0], 7_i32);
        assert_eq!(matrix[2][1], 8_i32);
        assert_eq!(matrix[2][2], 9_i32);
        assert_eq!(matrix[3][0], 10_i32);
        assert_eq!(matrix[3][1], 11_i32);
        assert_eq!(matrix[3][2], 12_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
        assert_eq!(matrix.c3r0, matrix[3][0]);
        assert_eq!(matrix.c3r1, matrix[3][1]);
        assert_eq!(matrix.c3r2, matrix[3][2]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[4][0], matrix[4][0]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[0][3], matrix[0][3]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        let matrix = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );
        let identity = Matrix4x4::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        let matrix = Matrix3x4::new(
            33_i32,  54_i32, 234_i32,
            19_i32,  5_i32,  308_i32,
            793_i32, 23_i32, 8_i32,
            49_i32,  11_i32, 27_i32
        );
        let zero_mat4x4 = Matrix4x4::zero();
        let zero_mat3x4 = Matrix3x4::zero();

        assert_eq!(matrix * zero_mat4x4, zero_mat3x4);
    }

    #[test]
    fn test_zero_mat_times_mat_equals_zero() {
        let matrix = Matrix3x4::new(
            33_i32,  54_i32, 234_i32,
            19_i32,  5_i32,  308_i32,
            793_i32, 23_i32, 8_i32,
            49_i32,  11_i32, 27_i32
        );
        let zero_mat3x3: Matrix3x3<i32> = Matrix3x3::zero();
        let zero_mat3x4: Matrix3x4<i32> = Matrix3x4::zero();

        assert_eq!(zero_mat3x3 * matrix, zero_mat3x4);
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        let matrix = Matrix3x4::new(
            33_i32,  54_i32, 234_i32,
            19_i32,  5_i32,  308_i32,
            793_i32, 23_i32, 8_i32,
            49_i32,  11_i32, 27_i32
        );
        let zero = 0_i32;
        let zero_mat3x4 = Matrix3x4::zero();

        assert_eq!(zero * matrix, zero_mat3x4);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let matrix3x4 = Matrix3x4::new(
            2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,
            8_i32,  9_i32,  10_i32,
            11_i32, 12_i32, 13_i32
        );
        let matrix4x4 = Matrix4x4::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            9_i32,  10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32, 16_i32
        );
        let expected = Matrix3x4::new(
            80_i32,  90_i32,  100_i32,
            184_i32, 210_i32, 236_i32,
            288_i32, 330_i32, 372_i32,
            392_i32, 450_i32, 508_i32
        );
        let result = matrix3x4 * matrix4x4;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix3x4 = Matrix3x4::new(
            4_i32,  5_i32,  6_i32, 
            7_i32,  8_i32,  9_i32, 
            10_i32, 11_i32, 12_i32,
            13_i32, 14_i32, 15_i32
        );
        let vector = Vector4::new(9_i32, 6_i32, -12_i32, -24_i32);
        let expected = Vector3::new(-354_i32, -375_i32, -396_i32);
        let result = matrix3x4 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication3() {
        let matrix3x4 = Matrix3x4::new(
            2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32, 
            8_i32,  9_i32,  10_i32,
            11_i32, 12_i32, 13_i32
        );
        let matrix4x3 = Matrix4x3::new(
            9_i32, -6_i32,  12_i32, 4_i32,
            35_i32,	96_i32,	27_i32, 4_i32,
            87_i32,	8_i32,  80_i32, 70_i32
        );
        let expected = Matrix3x3::new(
            128_i32,  147_i32,  166_i32,
            810_i32,  972_i32,  1134_i32,
            1624_i32, 1869_i32, 2114_i32
        );
        let result = matrix3x4 * matrix4x3;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix3x4 = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );
        let scalar = 13_i32;
        let expected = Matrix3x4::new(
            13_i32,  26_i32,  39_i32, 
            52_i32,  65_i32,  78_i32,
            91_i32,  104_i32, 117_i32,
            130_i32, 143_i32, 156_i32 
        );
        let result = matrix3x4 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix3x4 = Matrix3x4::new(
            1_i32,  2_i32,  3_i32,
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32
        );
        let scalar = 13_i32;
        let expected = Matrix3x4::new(
            13_i32,  26_i32,  39_i32, 
            52_i32,  65_i32,  78_i32,
            91_i32,  104_i32, 117_i32,
            130_i32, 143_i32, 156_i32 
        );
        let result = scalar * matrix3x4;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_mat3x4 = Matrix3x4::zero();
        let matrix = Matrix3x4::new(
            3684_i32, 42746_i32, 2389_i32,
            345_i32,  546_i32,   234_i32,
            76_i32,   167_i32,   890_i32,
            415_i32,  251_i32,   2340_i32
        );

        assert_eq!(matrix + zero_mat3x4, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_mat3x4 = Matrix3x4::zero();
        let matrix = Matrix3x4::new(
            3684_i32, 42746_i32, 2389_i32,
            345_i32,  546_i32,   234_i32,
            76_i32,   167_i32,   890_i32,
            415_i32,  251_i32,   2340_i32
        );

        assert_eq!(zero_mat3x4 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix3x4::new(
            23_i32,  76_i32,  45_i32,
            89_i32,  34_i32, -21_i32,
            324_i32, 75_i32, -204_i32,
            614_i32, 15_i32,  98_i32
        );
        let matrix2 = Matrix3x4::new(
            1_i32,  5_i32,  23_i32,
            9_i32,  13_i32, 80_i32,
            17_i32, 21_i32, 3_i32,
            87_i32, 41_i32, 34_i32
        );
        let expected = Matrix3x4::new(
            24_i32,  81_i32,  68_i32,
            98_i32,  47_i32,  59_i32,
            341_i32, 96_i32, -201_i32,
            701_i32, 56_i32,  132_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix3x4::new(
            3_i32,  6_i32,  9_i32,
            9_i32,  12_i32, 12_i32,
            15_i32, 18_i32, 15_i32,
            21_i32, 24_i32, 18_i32
        );
        let matrix2 = Matrix3x4::new(
            1_i32,   15_i32, 10_i32,
            29_i32,  6_i32,  71_i32,
            234_i32, 93_i32, 67_i32,
            93_i32,  7_i32,  91_i32
        );
        let expected = Matrix3x4::new(
             2_i32,   -9_i32,  -1_i32,
            -20_i32,   6_i32,  -59_i32,
            -219_i32, -75_i32, -52_i32,
            -72_i32,   17_i32, -73_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix3x4::new(
            3_i32,  6_i32,  9_i32,
            9_i32,  12_i32, 15_i32,
            15_i32, 18_i32, 21_i32,
            21_i32, 24_i32, 27_i32
        );
        let zero_mat3x4 = Matrix3x4::zero();

        assert_eq!(matrix - matrix, zero_mat3x4);
    }

    #[test]
    fn test_matrix_transpose() {
        let matrix = Matrix3x4::new(
            1_i32, 2_i32, 3_i32,
            3_i32, 4_i32, 6_i32,
            5_i32, 6_i32, 9_i32,
            7_i32, 8_i32, 12_i32
        );
        let expected = Matrix4x3::new(
            1_i32, 3_i32, 5_i32, 7_i32, 
            2_i32, 4_i32, 6_i32, 8_i32,
            3_i32, 6_i32, 9_i32, 12_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }
}


#[cfg(test)]
mod matrix4x3_tests {
    use cglinalg::{
        Vector4,
        Vector3,
        Matrix3x3,
        Matrix3x4,
        Matrix4x3,
        Matrix4x4,
    };


    #[test]
    fn test_matrix_components1() {
        let matrix = Matrix4x3::new(
            1_i32, 2_i32,  3_i32,  4_i32, 
            5_i32, 6_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix[0][0], 1_i32);
        assert_eq!(matrix[0][1], 2_i32);
        assert_eq!(matrix[0][2], 3_i32);
        assert_eq!(matrix[0][3], 4_i32);
        assert_eq!(matrix[1][0], 5_i32);
        assert_eq!(matrix[1][1], 6_i32);
        assert_eq!(matrix[1][2], 7_i32);
        assert_eq!(matrix[1][3], 8_i32);
        assert_eq!(matrix[2][0], 9_i32);
        assert_eq!(matrix[2][1], 10_i32);
        assert_eq!(matrix[2][2], 11_i32);
        assert_eq!(matrix[2][3], 12_i32);
    }

    #[test]
    fn test_matrix_components2() {
        let matrix = Matrix4x3::new(
            1_i32, 2_i32,  3_i32,  4_i32, 
            5_i32, 6_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );

        assert_eq!(matrix.c0r0, matrix[0][0]);
        assert_eq!(matrix.c0r1, matrix[0][1]);
        assert_eq!(matrix.c0r2, matrix[0][2]);
        assert_eq!(matrix.c0r3, matrix[0][3]);
        assert_eq!(matrix.c1r0, matrix[1][0]);
        assert_eq!(matrix.c1r1, matrix[1][1]);
        assert_eq!(matrix.c1r2, matrix[1][2]);
        assert_eq!(matrix.c1r3, matrix[1][3]);
        assert_eq!(matrix.c2r0, matrix[2][0]);
        assert_eq!(matrix.c2r1, matrix[2][1]);
        assert_eq!(matrix.c2r2, matrix[2][2]);
        assert_eq!(matrix.c2r3, matrix[2][3]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds1() {
        let matrix = Matrix4x3::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            10_i32, 11_i32, 12_i32, 13_i32
        );

        assert_eq!(matrix[0][4], matrix[0][4]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_components_out_of_bounds2() {
        let matrix = Matrix4x3::new(
            1_i32,  2_i32,  3_i32,  4_i32, 
            5_i32,  6_i32,  7_i32,  8_i32,
            10_i32, 11_i32, 12_i32, 13_i32
        );

        assert_eq!(matrix[3][0], matrix[3][0]);
    }

    #[test]
    fn test_mat_times_identity_equals_mat() {
        let matrix = Matrix4x3::new(
            2_i32,  3_i32,  4_i32,  5_i32, 
            6_i32,  7_i32,  8_i32,  9_i32,
            10_i32, 11_i32, 12_i32, 13_i32
        );
        let identity = Matrix3x3::identity();

        assert_eq!(matrix * identity, matrix);
    }

    #[test]
    fn test_mat_times_zero_equals_zero() {
        let matrix = Matrix4x3::new(
            33_i32, 54_i32,  19_i32, 345_i32,
            5_i32,  793_i32, 23_i32, 324_i32,
            23_i32, 98_i32,  84_i32, 89_i32
        );
        let zero_mat3x3 = Matrix3x3::zero();
        let zero_mat4x3 = Matrix4x3::zero();

        assert_eq!(matrix * zero_mat3x3, zero_mat4x3);
    }

    #[test]
    fn test_zero_times_mat_equals_zero() {
        let matrix = Matrix4x3::new(
            33_i32, 54_i32,  19_i32, 29_i32,
            5_i32,  234_i32, 98_i32, 7_i32,
            23_i32, 98_i32,  84_i32, 89_i32
        );
        let zero = 0_i32;
        let zero_mat4x3 = Matrix4x3::zero();

        assert_eq!(zero * matrix, zero_mat4x3);
    }

    #[test]
    fn test_matrix_multiplication1() {
        let matrix4x3 = Matrix4x3::new(
            2_i32, 3_i32,  4_i32,  5_i32,
            5_i32, 6_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );
        let matrix3x3 = Matrix3x3::new(
            1_i32, 2_i32, 3_i32,
            4_i32, 5_i32, 6_i32, 
            7_i32, 8_i32, 9_i32
        );
        let expected = Matrix4x3::new(
            39_i32,  45_i32,  51_i32,  57_i32,
            87_i32,  102_i32, 117_i32, 132_i32,
            135_i32, 159_i32, 183_i32, 207_i32
        );
        let result = matrix4x3 * matrix3x3;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication2() {
        let matrix4x3 = Matrix4x3::new(
            2_i32, 3_i32, 4_i32,  5_i32, 
            5_i32, 6_i32, 7_i32,  8_i32,
            8_i32, 9_i32, 10_i32, 11_i32
        );
        let matrix3x4 = Matrix3x4::new(
            1_i32,  2_i32,  3_i32, 
            4_i32,  5_i32,  6_i32,
            7_i32,  8_i32,  9_i32, 
            10_i32, 11_i32, 12_i32
        );
        let expected = Matrix4x4::new(
            36_i32,  42_i32,  48_i32,  54_i32,
            81_i32,  96_i32,  111_i32, 126_i32,
            126_i32, 150_i32, 174_i32, 198_i32,
            171_i32, 204_i32, 237_i32, 270_i32
        );
        let result = matrix4x3 * matrix3x4;
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_multiplication3() {
        let matrix4x3 = Matrix4x3::new(
            4_i32,  5_i32,  6_i32,  7_i32, 
            8_i32,  9_i32,  10_i32, 11_i32,
            12_i32, 13_i32, 14_i32, 15_i32
        );
        let vector = Vector3::new(9_i32, -6_i32, 34_i32);
        let expected = Vector4::new(396_i32, 433_i32, 470_i32, 507_i32);
        let result = matrix4x3 * vector;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let matrix4x3 = Matrix4x3::new(
            1_i32, 2_i32,  3_i32,  4_i32, 
            4_i32, 5_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );
        let scalar = 13_i32;
        let expected = Matrix4x3::new(
            13_i32,  26_i32,  39_i32,  52_i32,
            52_i32,  65_i32,  91_i32,  104_i32,
            117_i32, 130_i32, 143_i32, 156_i32
        );
        let result = matrix4x3 * scalar;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scalar_matrix_multiplication() {
        let matrix4x3 = Matrix4x3::new(
            1_i32, 2_i32,  3_i32,  4_i32, 
            4_i32, 5_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );
        let scalar = 13_i32;
        let expected = Matrix4x3::new(
            13_i32,  26_i32,  39_i32,  52_i32,
            52_i32,  65_i32,  91_i32,  104_i32,
            117_i32, 130_i32, 143_i32, 156_i32
        );
        let result = scalar * matrix4x3;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_plus_zero_equals_matrix() {
        let zero_mat4x3 = Matrix4x3::zero();
        let matrix = Matrix4x3::new(
            3684_i32, 42746_i32, 345_i32, 456_i32,
            546_i32,  76_i32,    167_i32, 915_i32,
            320_i32,  2430_i32,  894_i32, 324_i32
        );

        assert_eq!(matrix + zero_mat4x3, matrix);
    }

    #[test]
    fn test_zero_plus_matrix_equals_matrix() {
        let zero_mat4x3 = Matrix4x3::zero();
        let matrix = Matrix4x3::new(
            3684_i32, 42746_i32, 345_i32, 456_i32,
            546_i32,  76_i32,    167_i32, 915_i32,
            320_i32,  2430_i32,  894_i32, 324_i32
        );

        assert_eq!(zero_mat4x3 + matrix, matrix);
    }

    #[test]
    fn test_addition() {
        let matrix1 = Matrix4x3::new(
            23_i32, 76_i32,  89_i32, 11_i32,
            34_i32, 324_i32, 75_i32, 62_i32,
            88_i32, 61_i32,  45_i32, 16_i32
        );
        let matrix2 = Matrix4x3::new(
            1_i32,  5_i32,  9_i32,  82_i32,
            13_i32, 17_i32, 21_i32, 6_i32,
            29_i32, 91_i32, 64_i32, 43_i32 
        );
        let expected = Matrix4x3::new(
            24_i32,  81_i32,  98_i32,  93_i32,
            47_i32,  341_i32, 96_i32,  68_i32,
            117_i32, 152_i32, 109_i32, 59_i32
        );
        let result = matrix1 + matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_subtraction() {
        let matrix1 = Matrix4x3::new(
            3_i32,  6_i32,  9_i32,  65_i32,
            12_i32, 15_i32, 18_i32, 333_i32,
            28_i32, 71_i32, 4_i32,  92_i32
        );
        let matrix2 = Matrix4x3::new(
            1_i32, 15_i32,  29_i32, 27_i32,
            6_i32, 234_i32, 93_i32, 38_i32,
            74_i32, 97_i32, 10_i32, 100_i32
        );
        let expected = Matrix4x3::new(
             2_i32,  -9_i32,   -20_i32,  38_i32,
             6_i32,  -219_i32, -75_i32,  295_i32,
            -46_i32, -26_i32,  -6_i32,  -8_i32
        );
        let result = matrix1 - matrix2;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_minus_matrix_is_zero() {
        let matrix = Matrix4x3::new(
            3_i32,  6_i32,  9_i32,  12_i32,
            12_i32, 15_i32, 18_i32, 21_i32,
            34_i32, 17_i32, 8_i32,  84_i32
        );
        let zero_mat4x3 = Matrix4x3::zero();

        assert_eq!(matrix - matrix, zero_mat4x3);
    }

    #[test]
    fn test_transpose() {
        let matrix = Matrix4x3::new(
            1_i32, 2_i32,  3_i32,  4_i32,
            5_i32, 6_i32,  7_i32,  8_i32,
            9_i32, 10_i32, 11_i32, 12_i32
        );
        let expected = Matrix3x4::new(
            1_i32, 5_i32, 9_i32,
            2_i32, 6_i32, 10_i32,
            3_i32, 7_i32, 11_i32,
            4_i32, 8_i32, 12_i32
        );
        let result = matrix.transpose();

        assert_eq!(result, expected);
    }
}

