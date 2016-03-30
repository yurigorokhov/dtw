use std::cmp;

//--- Types ---
struct Matrix {
    n: i32,
    array: Vec<f64>
}

impl Matrix {
    fn new(n: i32, m: i32) -> Matrix {
        Matrix {
            n: n,
            array: vec![0.0; (n*m) as usize]
        }
    }

    fn Assign(&mut self, i: i32, j: i32, val: f64) {
        self.array[(i + j*self.n) as usize] = val;
    }

    fn Get(&self, i: i32, j: i32) -> f64 {
        self.array[(i + j*self.n) as usize]
    }

    fn Length(&self) -> usize {
        self.array.len()
    }
}

//--- Methods ---
/*
 * Computes the DTW distance
 */
pub fn compute_distance(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    
    // initialize matrix
    let mut distance_mat = Matrix::new(v1.len() as i32, v2.len() as i32);
   
    //           i (v1)
    //          ----------
    //          |        |
    //          |        | j (v2)
    //          |        |   
    //          ----------


    // initialize all distances in matrix
    for i in 0..v1.len() {
       for j in 0..v2.len() {
           distance_mat.Assign(i as i32, j as i32, (v1[i]-v2[j]).powi(2));
       } 
    }

    // find shortest path
    let n = v1.len() as i32;
    let m = v2.len() as i32;
    for i in 0..v1.len() {
       for j in 0..v2.len() {
           let i = i as i32;
           let j = j as i32;

           // consider path from left
           let from_left = if (i % n == 0) { std::f64::MAX } else { distance_mat.Get(i-1, j) };
           let from_top = if (j == 0) { std::f64::MAX } else { distance_mat.Get(i, j-1) };
           let from_top_left = if (i % n == 0 || j == 0) { std::f64::MAX } else { distance_mat.Get(i-1, j-1) };
           let min = from_left.min(from_top).min(from_top_left);
           if min < std::f64::MAX {
               let curr_value = distance_mat.Get(i, j);
               distance_mat.Assign(i, j, min + curr_value);
           }
       } 
    }

    // return last element, which will contain the distance
    distance_mat.Get(n-1, m-1)
}

