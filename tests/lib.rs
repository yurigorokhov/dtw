#[cfg(test)]
mod tests {

    extern crate dtw;

    #[test]
    fn distance_between_identical_vectors_is_zero() {

        // Arrange
        let time_series_1: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let time_series_2: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        // Act
        let distance = dtw::compute_distance(&time_series_1, &time_series_2);

        // Assert
        assert_eq!(0.0, distance)
    }

    #[test]
    fn distance_between_differing_vectors_is_greater_than_zero() {

        // Arrange
        let time_series_1: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let time_series_2: Vec<f64> = vec![4.0, 3.0, 2.0, 1.0];

        // Act
        let distance = dtw::compute_distance(&time_series_1, &time_series_2);

        // Assert
        assert_eq!(20.0, distance);
    }

    #[test]
    fn distance_between_time_shifted_vectors_remains_small() {

        // Arrange
        let time_series_1: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 3.0, 2.0, 1.0];
        let time_series_2: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 3.0, 2.0, 1.0, 0.0];

        // Act
        let distance = dtw::compute_distance(&time_series_1, &time_series_2);

        // Assert
        assert_eq!(2.0, distance);
    }

    #[test]
    fn distance_among_sin_cos_functions_with_random_noise() {

        /*
           Octave code:

           x = 0:0.1:10;
           noisy_cos = cos(x) + 0.3 * randn(1, length(x));
           noisy_sin = sin(x) + 0.3 * randn(1, length(x));
           noisy_one_minus_sin = 1-sin(x) + 0.3 * randn(1, length(x));
           */

        // Arrange
        let noisy_sin = vec![
            -0.3149, 0.4670, 0.4574, 0.1363, 0.2457, 0.8314, 0.2848, 0.6068, 0.9595, 0.6996, 0.8906,
            1.0646, 0.5735, 0.6442, 0.5754, 1.3261, 0.9987, 1.1393, 1.5015, 1.0002, 0.9722, 1.5795,
            1.1415, 0.5535, 1.1421, 0.5815, 1.0787, 0.3087, -0.0661, -0.0210, 0.2951, 0.4667,
            -0.6388, -0.1863, -0.3496, 0.2132, -0.6424, -0.4824, -0.6848, -0.5224, -0.7061,
            -0.7912, -1.0871, -0.8741, -1.2851, -1.0506, -0.5077, -1.2602, -1.3099, -0.8121,
            -0.9111, -1.1098, -0.9393, -0.8260, -0.8515, -1.0587, -1.0846, -0.6056, -0.7942,
            -0.3449, -0.4354, 0.1151, -0.2475, -0.5825, 0.5308, -0.0087, 0.2836, 0.0341, 0.6732,
            1.3887, 0.8350, 0.8308, 0.8659, 0.5974, 1.0757, 1.0443, 1.3954, 1.3125, 1.2988, 0.6878,
            1.5970, 1.5317, 0.9277, 0.3937, 0.1687, 0.8807, 0.6839, 0.7024, 1.0989, 0.9079,
            0.5671, 0.2027, 0.3140, -0.0604, 0.1896, -0.3122, -0.3113, -0.3315, -0.6546,
            -0.3853, -0.3761
                ];
        let noisy_cos = vec![
            1.1474, 1.3191, 0.8687, 0.6959, 0.8403, 0.8673, 1.2560, 0.5385, 0.9553, 0.8714, 0.7716,
            0.6440, 0.2465, 0.0625, 0.4366, 0.6445, 0.0610, 0.0593, -0.6667, -0.4423, -0.7545,
            -0.5746, -0.8498, -0.4879, -1.1742, -0.9053, -0.6738, -0.8861, -1.0111, -0.9847,
            -0.9493, -0.8376, -1.0099, -1.4807, -0.6973, -0.7877, -0.3964, -0.8698, -0.4316,
            -0.5089, -0.3942, -0.7181, -0.7783, -0.3197, -0.5698, 0.2696, -0.3412, 0.2568, 0.1626,
            0.2727, 0.8264, 0.6875, 0.1849, 0.1448, 0.1267, 1.0379, 0.7536, 0.9226, 0.7270,
            0.5743, 0.6641, 0.9744, 0.9794, 1.0639, 1.5643, 1.2334, 0.4182, 0.3300, 0.4051,
            1.0685, 0.9609, 0.5683, 0.7473, 0.6129, -0.1144, 0.5670, 0.4373, 0.1321, 0.2366,
            0.3150, -0.3217, -0.7473, -0.3903, -0.5637, -0.5064, -0.5735, -0.6568, -0.5876,
            -1.0781, -0.7407, -0.7098, -1.0272, -0.6719, -1.2422, -0.9623, -1.2895,
            -0.5520, -0.3832, -1.1163, -0.7609, -0.6909
                ];
        let noisy_one_minus_sin = vec![
            1.0086, 0.8307, 0.5447, 0.4712, 0.3904, 0.6809, 0.2981, 0.4652, 0.3737, 0.4577, 0.2728,
            0.0879, 0.2798, -0.4285, 0.2499, -0.0708, 0.0626, -0.2066, -0.3559, 0.2452, -0.0072,
            -0.2466, -0.0572, -0.1333, 0.3257, 0.6041, 0.2310, 0.4992, 1.2614, 0.1575, 1.0866,
            1.1284, 0.8990, 0.5821, 0.8462, 0.6178, 0.9507, 1.7060, 1.4641, 2.5182, 1.5539, 1.7163,
            1.9616, 1.9388, 2.1303, 1.8393, 1.8060, 1.4109, 1.5633, 2.1661, 1.6833, 1.4541,
            1.7063, 2.1802, 1.6869, 1.7475, 1.3374, 1.8297, 1.6091, 1.3224, 1.4554, 1.2712,
            1.3881, 0.8096, 0.6939, 0.7421, 0.9659, 0.7726, 0.5942, 0.7367, 0.1919, 0.7462,
            0.3092, -0.1305, -0.1139, 0.5524, 0.1063, -0.4299, -0.3369, -0.0505, -0.1580,
            0.2121, 0.4133, 0.1215, 0.1687, 0.5060, 0.0130, 0.3056, 0.5204, 0.6303,
            0.5080, 0.4056, 0.7743, 0.9729, 0.9701, 0.9925, 0.9736, 1.3813, 1.5695,
            1.5881, 1.2159
                ];

        // Act
        let distance_one = dtw::compute_distance(&noisy_sin, &noisy_cos);
        let distance_two = dtw::compute_distance(&noisy_sin, &noisy_one_minus_sin);
        println!("Distance between sin(x) and cos(x) with random noise: {}, distance between sin(x) and 1-sin(x) with random noise: {}", distance_one, distance_two);

        // Assert
        assert!(distance_one < distance_two, "Distance one should be smaller!");
    }

    #[test]
    fn distance_among_sin_cos_functions() {

        /*
           Octave code:

           x = 0:0.1:10;
           sin_x = sin(x)
           cosn_x = cos(x)
           one_minus_sin_x = 1-sin(x)
           */

        // Arrange
        let sin_x = vec![
            0.0000, 0.0998, 0.1987, 0.2955, 0.3894, 0.4794, 0.5646, 0.6442, 0.7174, 0.7833, 0.8415,
            0.8912, 0.9320, 0.9636, 0.9854, 0.9975, 0.9996, 0.9917, 0.9738, 0.9463, 0.9093, 0.8632,
            0.8085, 0.7457, 0.6755, 0.5985, 0.5155, 0.4274, 0.3350, 0.2392, 0.1411, 0.0416,
            -0.0584, -0.1577, -0.2555, -0.3508, -0.4425, -0.5298, -0.6119, -0.6878, -0.7568,
            -0.8183, -0.8716, -0.9162, -0.9516, -0.9775, -0.9937, -0.9999, -0.9962, -0.9825,
            -0.9589, -0.9258, -0.8835, -0.8323, -0.7728, -0.7055, -0.6313, -0.5507, -0.4646,
            -0.3739, -0.2794, -0.1822, -0.0831, 0.0168, 0.1165, 0.2151, 0.3115, 0.4048, 0.4941,
            0.5784, 0.6570, 0.7290, 0.7937, 0.8504, 0.8987, 0.9380, 0.9679, 0.9882, 0.9985,
            0.9989, 0.9894, 0.9699, 0.9407, 0.9022, 0.8546, 0.7985, 0.7344, 0.6630, 0.5849,
            0.5010, 0.4121, 0.3191, 0.2229, 0.1245, 0.0248, -0.0752, -0.1743, -0.2718,
            -0.3665, -0.4575, -0.5440
                ];
        let cos_x = vec![
            1.0000, 0.9950, 0.9801, 0.9553, 0.9211, 0.8776, 0.8253, 0.7648, 0.6967, 0.6216, 0.5403,
            0.4536, 0.3624, 0.2675, 0.1700, 0.0707, -0.0292, -0.1288, -0.2272, -0.3233, -0.4161,
            -0.5048, -0.5885, -0.6663, -0.7374, -0.8011, -0.8569, -0.9041, -0.9422, -0.9710,
            -0.9900, -0.9991, -0.9983, -0.9875, -0.9668, -0.9365, -0.8968, -0.8481, -0.7910,
            -0.7259, -0.6536, -0.5748, -0.4903, -0.4008, -0.3073, -0.2108, -0.1122, -0.0124,
            0.0875, 0.1865, 0.2837, 0.3780, 0.4685, 0.5544, 0.6347, 0.7087, 0.7756, 0.8347,
            0.8855, 0.9275, 0.9602, 0.9833, 0.9965, 0.9999, 0.9932, 0.9766, 0.9502, 0.9144,
            0.8694, 0.8157, 0.7539, 0.6845, 0.6084, 0.5261, 0.4385, 0.3466, 0.2513, 0.1534,
            0.0540, -0.0460, -0.1455, -0.2435, -0.3392, -0.4314, -0.5193, -0.6020,
            -0.6787, -0.7486, -0.8111, -0.8654, -0.9111, -0.9477, -0.9748, -0.9922,
            -0.9997, -0.9972, -0.9847, -0.9624, -0.9304, -0.8892, -0.8391
                ];
        let one_minus_sin_x = vec![

            1.0000, 0.9002, 0.8013, 0.7045, 0.6106, 0.5206, 0.4354, 0.3558, 0.2826, 0.2167, 0.1585,
            0.1088, 0.0680, 0.0364, 0.0146, 0.0025, 0.0004, 0.0083, 0.0262, 0.0537, 0.0907,
            0.1368, 0.1915, 0.2543, 0.3245, 0.4015, 0.4845, 0.5726, 0.6650, 0.7608, 0.8589,
            0.9584, 1.0584, 1.1577, 1.2555, 1.3508, 1.4425, 1.5298, 1.6119, 1.6878, 1.7568,
            1.8183, 1.8716, 1.9162, 1.9516, 1.9775, 1.9937, 1.9999, 1.9962, 1.9825, 1.9589,
            1.9258, 1.8835, 1.8323, 1.7728, 1.7055, 1.6313, 1.5507, 1.4646, 1.3739,
            1.2794, 1.1822, 1.0831, 0.9832, 0.8835, 0.7849, 0.6885, 0.5952, 0.5059,
            0.4216, 0.3430, 0.2710, 0.2063, 0.1496, 0.1013, 0.0620, 0.0321, 0.0118,
            0.0015, 0.0011, 0.0106, 0.0301, 0.0593, 0.0978, 0.1454, 0.2015, 0.2656,
            0.3370, 0.4151, 0.4990, 0.5879, 0.6809, 0.7771, 0.8755, 0.9752,
            1.0752, 1.1743, 1.2718, 1.3665, 1.4575, 1.5440
                ];

        // Act
        let distance_one = dtw::compute_distance(&sin_x, &cos_x);
        let distance_two = dtw::compute_distance(&sin_x, &one_minus_sin_x);
        println!("Distance between sin(x) and cos(x): {}, distance between sin(x) and 1-sin(x): {}", distance_one, distance_two);

        // Assert
        assert!(distance_one < distance_two, "Distance one should be smaller!");
    }

    #[test]
    fn distance_among_warped_cos_functions() {

        /*
           Octave code:

           x = 0:0.1:10;
           cos_x = cos(x)
           cos_1_2 = cos(1.2*x)
           zeroes = zero(1, length(x))
        */

        // Arrange
        let cos_x = vec![
            1.0000, 0.9950, 0.9801, 0.9553, 0.9211, 0.8776, 0.8253, 0.7648, 0.6967, 0.6216,
            0.5403, 0.4536, 0.3624, 0.2675, 0.1700, 0.0707, -0.0292, -0.1288, -0.2272,
            -0.3233, -0.4161, -0.5048, -0.5885, -0.6663, -0.7374, -0.8011, -0.8569,
            -0.9041, -0.9422, -0.9710, -0.9900, -0.9991, -0.9983, -0.9875, -0.9668,
            -0.9365, -0.8968, -0.8481, -0.7910, -0.7259, -0.6536, -0.5748, -0.4903,
            -0.4008, -0.3073, -0.2108, -0.1122, -0.0124, 0.0875, 0.1865, 0.2837, 0.3780,
            0.4685, 0.5544, 0.6347, 0.7087, 0.7756, 0.8347, 0.8855, 0.9275, 0.9602, 0.9833,
            0.9965, 0.9999, 0.9932, 0.9766, 0.9502, 0.9144, 0.8694, 0.8157, 0.7539,
            0.6845, 0.6084, 0.5261, 0.4385, 0.3466, 0.2513, 0.1534, 0.0540, -0.0460,
            -0.1455, -0.2435, -0.3392, -0.4314, -0.5193, -0.6020, -0.6787, -0.7486,
            -0.8111, -0.8654, -0.9111, -0.9477, -0.9748, -0.9922, -0.9997, -0.9972,
            -0.9847, -0.9624, -0.9304, -0.8892, -0.8391];
        let cos_1_2 = vec![
            1.0000, 0.9928, 0.9713, 0.9359, 0.8870, 0.8253, 0.7518, 0.6675, 0.5735, 0.4713, 0.3624,
            0.2482, 0.1304, 0.0108, -0.1090, -0.2272, -0.3421, -0.4522, -0.5557, -0.6512,
            -0.7374, -0.8130, -0.8768, -0.9281, -0.9660, -0.9900, -0.9998, -0.9952, -0.9762,
            -0.9433, -0.8968, -0.8373, -0.7659, -0.6834, -0.5911, -0.4903, -0.3824, -0.2690,
            -0.1518, -0.0324, 0.0875, 0.2061, 0.3218, 0.4328, 0.5376, 0.6347, 0.7226, 0.8002,
            0.8662, 0.9198, 0.9602, 0.9867, 0.9991, 0.9971, 0.9807, 0.9502, 0.9061, 0.8489,
            0.7796, 0.6990, 0.6084, 0.5090, 0.4023, 0.2898, 0.1731, 0.0540, -0.0660, -0.1849,
            -0.3013, -0.4132, -0.5193, -0.6179, -0.7075, -0.7871, -0.8552, -0.9111,
            -0.9539, -0.9830, -0.9979, -0.9985, -0.9847, -0.9567, -0.9150, -0.8602,
            -0.7929, -0.7143, -0.6253, -0.5274, -0.4219, -0.3104, -0.1943, -0.0755, 0.0444,
            0.1637, 0.2806, 0.3935, 0.5007, 0.6007, 0.6921, 0.7736, 0.8439];
        let zeroes = vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0];

        // Act
        let distance_one = dtw::compute_distance(&cos_x, &cos_1_2);
        let distance_two = dtw::compute_distance(&cos_x, &zeroes);
        println!("Distance between cos(x) and cos(1.2*x): {}, distance between cos(x) and y=0: {}", distance_one, distance_two);

        // Assert
        assert!(distance_one < distance_two, "Distance one should be smaller!");
    }
}