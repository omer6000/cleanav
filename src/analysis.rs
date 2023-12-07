use nalgebra::{DMatrix, RowDVector};

pub fn generate_transition_matrix(width: usize, height: usize) -> DMatrix<f64> {
    // Hint: use possible_moves() and compute_step() from crate::markov
    // let moves = crate::markov::possible_moves();
    // todo!()
    let total_states = width * height;
    let moves = crate::markov::possible_moves();
    let mut t_matrix = DMatrix::<f64>::zeros(total_states, total_states);
    
    for i in 0..width {
        for j in 0..height {
            // let state_index = i*height+j; //converting 2d index to 1d
            let state_index = crate::markov::flatten_coordinate((i,j), width);
            for m in &moves {
                let step = crate::markov::compute_step((i,j), (m.0,m.1), (width,height));
                // let step_index = step.0*height+step.1;
                let step_index = crate::markov::flatten_coordinate((step.0,step.1), width);
                t_matrix[(state_index,step_index)] = t_matrix[(state_index,step_index)] + 1.0/17.0;
            }
        }
    }
    
    return t_matrix;

}

impl crate::markov::StochasticModel {
    // Take a look at what is already implemented in this struct ( see `markov.rs` )

    pub fn compute_state_distribution(
        &self,
        start: (usize, usize),
        steps: usize,
    ) -> RowDVector<f64> {
        // Hint: s_{t+1} = s_t * P
        // todo!()
        let mut state = RowDVector::zeros(self.width*self.height);
        state[crate::markov::flatten_coordinate(start,self.width)] = 1.0;
        let tm = &self.transition_matrix;
        for _ in 0..steps {
            state *= tm;
        }
        return state;
    }

    pub fn compute_transition_probability(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        steps: usize,
    ) -> f64 {
        // Hint: use compute_state_distribution
        let end_index = crate::markov::flatten_coordinate(end, self.width);
        return self.compute_state_distribution(start, steps)[end_index];
    }

    fn compute_path(&self,from: (f64, f64), to: (f64, f64)) -> f64 {
        let mut x_diff = from.0 - to.0;
        let mut y_diff = from.1 - to.1;
        if x_diff < 0.0 {
            x_diff *= -1.0;
        }
        if y_diff < 0.0 {
            y_diff *= -1.0;
        }
        return x_diff + y_diff;
    }
    
    pub fn manhattan_distance(&self, from: (usize, usize), to: (usize, usize)) -> f64 {
        // Hint: The obvious path might not be the fastest (keep position wrapping in mind)
        // todo!()
        let width = self.width as f64;
        let height = self.height as f64;
        let x_from = from.0 as f64;
        let y_from = from.1 as f64;
        let x_to = to.0 as f64;
        let y_to = to.1 as f64;
        
        let d1 = self.compute_path((x_from,y_from),(x_to,y_to));
        let d2 = self.compute_path((x_from,y_from),(x_to - width,y_to));
        let d3 = self.compute_path((x_from,y_from),(x_to,y_to - height));
        let d4 = self.compute_path((x_from,y_from),(x_to - width,y_to - height));
        let d5 = self.compute_path((x_from- width,y_from),(x_to,y_to));
        let d6 = self.compute_path((x_from,y_from- height),(x_to,y_to));
        let d7 = self.compute_path((x_from - width,y_from - height),(x_to,y_to));

        let distances = RowDVector::from_row_slice(&[d1,d2,d3,d4,d5,d6,d7]);
        
        return distances.min();

    }

    pub fn expected_distance(&self, start: (usize, usize), steps: usize) -> f64 {
        // Hint: use manhattan_distance
        todo!();
    }
}
