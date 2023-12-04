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
            let state_index = i*height+j; //converting 2d index to 1d
            for m in &moves {
                let step = crate::markov::compute_step((i,j), (m.0,m.1), (width,height));
                let step_index = step.0*height+step.1;
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
        todo!()
    }

    pub fn compute_transition_probability(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        steps: usize,
    ) -> f64 {
        // Hint: use compute_state_distribution
        todo!()
    }

    pub fn manhattan_distance(&self, from: (usize, usize), to: (usize, usize)) -> f64 {
        // Hint: The obvious path might not be the fastest (keep position wrapping in mind)
        todo!()
    }

    pub fn expected_distance(&self, start: (usize, usize), steps: usize) -> f64 {
        // Hint: use manhattan_distance
        todo!();
    }
}
