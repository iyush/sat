pub mod sat_collision {
    type Vertex = Vec<f32>;
    type Vector = Vec<f32>;
    type Poly = Vec<Vec<f32>>;

    pub fn substract_vertices(v1: Vertex, v2: Vertex) -> Vertex {
        vec![ v1[0]-v2[0], v1[1]-v2[1] ]
    }

    pub fn get_edges(polynomial: Poly) -> Poly {
        let mut edges: Vec<Vec<f32>> = Vec::new();
        let number_of_vertices = polynomial.len();
        for i in 0..(number_of_vertices) {
            let difference = substract_vertices(
                polynomial[(i+1) % number_of_vertices].clone(),
                polynomial[i].clone()
                );
            edges.push(difference.to_vec())
        }
        edges
    }

    pub fn get_mean(polynomial: Poly) -> Vertex {
        let mut first_row_sum:f32= 0.;
        let mut second_row_sum:f32 = 0.;
        let number_of_vertices:f32 = polynomial.len() as f32;

        for i in 0..polynomial.len(){
            first_row_sum += polynomial[i][0];
            second_row_sum += polynomial[i][1];
        }
        vec![ (first_row_sum / number_of_vertices), (second_row_sum / number_of_vertices) ]
    }

    pub fn get_orthogonal(v: Vertex) -> Vertex {
        vec![ v[1], -v[0] ]
    }

    pub fn centers_displacement(polynomial_1: Poly, polynomial_2: Poly) -> Vertex {
        let mean_polynomial_1 = get_mean(polynomial_1.clone());
        let mean_polynomial_2 = get_mean(polynomial_2.clone());
        let substracted = substract_vertices(mean_polynomial_2, mean_polynomial_1);
        substracted
    }

    pub fn dot_product(v1: &Vector, v2: &Vector) -> f32 {
        v1[0]*v2[0] + v1[1]*v2[1]
    }

    pub fn min(a: f32, b: f32) -> f32 {
        if a > b { b } else { a }
    }

    pub fn max(a: f32, b: f32) -> f32 {
        if a > b { a } else { b }
    }


    pub fn check_if_seperating(o: Vector, p1: Poly, p2: Poly) -> (bool, Vector) {
        let mut min1 =  f32::MAX;
        let mut max1 = -f32::MAX;
        let mut min2 =  f32::MAX;
        let mut max2 = -f32::MAX;

        for v in p1 {
            let projection:f32 = dot_product(&v, &o);
            min1 = min(min1, projection);
            max1 = max(max1, projection);
        }


        for v in p2 {
            let projection:f32 = dot_product(&v, &o);
            min2 = min(min2, projection);
            max2 = max(max2, projection);
        }


        if max1 >= min2 && max2 >= min1 { 
            // calculation of seperation vectors
            let d:f32 = min(max2 - min1, max1 - min2);
            // push a bit more than needed so the shapes do not overlap in future
            // tests due to float precision
            let d_over_o_squared:f32 = d / dot_product(&o, &o) + 1e-10;
            let pv = vec![ d_over_o_squared * o[0], d_over_o_squared * o[1] ];
            return (false, pv)
        } else {
            return (true, Vec::new())
        }
    }

    pub fn min_by_magnitude(list_of_vectors: Poly) -> Vector {
        let mut min_magnitude = f32::MAX;
        let mut min_vector = Vector::new();
        for vector in list_of_vectors {
            let mag = dot_product(&vector, &vector);
            if min_magnitude > mag { 
                min_magnitude = mag;
                min_vector = vector.to_vec();
            }
        }
        min_vector
    }


    pub fn collide_sat(polynomial_1: Poly, polynomial_2: Poly) -> (bool, Vector) {
        let mut edges1 = get_edges(polynomial_1.clone());
        let mut edges2 = get_edges(polynomial_2.clone());

        let mut edge = Poly::new();
        edge.append(&mut edges1);
        edge.append(&mut edges2);

        let mut push_vectors: Poly = Poly::new();

        let orthogonal = edge.into_iter().map( |e| get_orthogonal(e));
        for o in orthogonal { 
            let (seperates, mpv) = check_if_seperating(o, polynomial_1.clone(), polynomial_2.clone());
            if seperates { return (false, Vec::new()); }
            else {
                push_vectors.push(mpv);
            }
        }

        // calculate the minimum distance seperation vector
        let mut mpv = min_by_magnitude(push_vectors);
        let d = centers_displacement(polynomial_1, polynomial_2);
        if dot_product(&d, &mpv) > 0. {
            mpv = vec![-mpv[0], -mpv[1]];
        }
        return (true, mpv);
    }
}
