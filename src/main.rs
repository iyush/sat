mod sat_collide;

pub use crate::sat_collide::sat_collision;

fn main() {
    let polynomial_1 = vec![ vec![ 1.,1. ], vec![ 3.,1. ], vec![ 2.,4. ], vec![ 1.,3. ] ];
    let polynomial_2 = vec![ vec![ 2.,2. ], vec![ 4.,1. ], vec![ 3.,4. ] ];

    println!("Result is : {:?}", sat_collision::collide_sat(polynomial_1, polynomial_2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use sat_collision::*;

    #[test]
    fn test_collide_sat() {
        let polynomial_1 = vec![ vec![ 1.,1. ], vec![ 3.,1. ], vec![ 2.,4. ], vec![ 1.,3. ] ];
        let polynomial_2 = vec![ vec![ 2.,2. ], vec![ 4.,1. ], vec![ 3.,4. ] ];

        let result = collide_sat(polynomial_1, polynomial_2);
        assert_eq!(result.0, true);
        assert_eq!(result.1, vec![-0.6, -0.2]);
    }

    #[test]
    fn test_edges_of() {
        let polynomial_1 = vec![ vec![ 1.,1. ], vec![ 3.,1. ], vec![ 2.,4. ], vec![ 1.,3. ] ];

        let result = get_edges(polynomial_1);
        assert_eq!(result[0], vec![ 2., 0.]);
        assert_eq!(result[1], vec![-1., 3.]);
        assert_eq!(result[2], vec![-1.,-1.]);
        assert_eq!(result[3], vec![ 0., -2.]);
    }

    #[test]
    fn test_get_orthogonal() {
        let vector = vec![ 1., 2. ];

        let result = get_orthogonal(vector);
        assert_eq!(result, vec![ 2., -1.]);
    }

    #[test]
    fn test_get_mean() {
        let polynomial_1 = vec![ vec![ 1.,1. ], vec![ 3.,1. ], vec![ 2.,4. ], vec![ 1.,3. ] ];
        let polynomial_2 = vec![ vec![ 2.,2. ], vec![ 4.,1. ], vec![ 3.,4. ] ];

        let result1 = get_mean(polynomial_1);
        let result2 = get_mean(polynomial_2);
        assert_eq!(result1, vec![ 7./4., 9./4.]);
        assert_eq!(result2, vec![ 3., 7./3.]);
    }

    #[test]
    fn test_centers_displacement() {
        let polynomial_1 = vec![ vec![ 1.,1. ], vec![ 3.,1. ], vec![ 2.,4. ], vec![ 1.,3. ] ];
        let polynomial_2 = vec![ vec![ 2.,2. ], vec![ 4.,1. ], vec![ 3.,4. ] ];

        let result = centers_displacement(polynomial_1, polynomial_2);
        let substract_difference = substract_vertices(vec![ 1.25, 1./12.], result);
        assert_eq!(dot_product(&substract_difference, &substract_difference) < 0.0001, true)
    }
}


