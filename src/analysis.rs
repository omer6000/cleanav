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
            let state_index = crate::markov::flatten_coordinate((i,j), width);
            for m in &moves {
                let step = crate::markov::compute_step((i,j), (m.0,m.1), (width,height));
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
    
    pub fn manhattan_distance(&self, from: (usize, usize), to: (usize, usize)) -> f64 {
        // Hint: The obvious path might not be the fastest (keep position wrapping in mind)
        // todo!()

        let mut x_diff = from.0 as f64 - to.0 as f64;
        let mut y_diff = from.1 as f64 - to.1 as f64;

        let width = self.width as f64;
        let height = self.height as f64;

        if x_diff < 0.0 {
            x_diff *= -1.0
        }
        if x_diff > width - x_diff {
            x_diff = width - x_diff;
        }
        if y_diff < 0.0 {
            y_diff *= -1.0;
        }
        if y_diff > (height - y_diff) {
            y_diff = height - y_diff;
        }
        return x_diff + y_diff;
        
    }

    pub fn expected_distance(&self, start: (usize, usize), steps: usize) -> f64 {
        // Hint: use manhattan_distance
        let total_states = self.width * self.height;
        let state_dist = self.compute_state_distribution(start, steps);
        let mut manhattan_dist = RowDVector::<f64>::zeros(total_states);
        for i in 0..self.width {
            for j in 0..self.height {
                let index = self.to_index((i,j));
                manhattan_dist[index] = self.manhattan_distance(start, (i,j));
            }
        }
        let expected_dist = manhattan_dist * state_dist.transpose();
        return expected_dist[0];
    }
}
