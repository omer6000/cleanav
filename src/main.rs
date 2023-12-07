use cleanav::map::Map;
use cleanav::navigation::Output;
use cleanav::run;
use cleanav::tui;
use std::str::FromStr;

fn main() {
    // let tui::Opt {
    //     slope,
    //     termination,
    //     map: map_path,
    //     output,
    // } = tui::parse_args();
    // let map = Map::from_path(&map_path);
    // let result = run(
    //     map,
    //     slope,
    //     termination.into(),
    //     Output::from_str(&output).unwrap(),
    // );
    // tui::output(&result)
    let width = 8;
    let height = 8;
    // let start = (1,1);
    // let end = (2,2);
    let tm = cleanav::analysis::generate_transition_matrix(width, height);
    let model = cleanav::markov::StochasticModel {
        transition_matrix: tm,
        width: width,
        height: height
    };
    // let vec = model.compute_state_distribution((0,0),5);
    // println!("{:?}",model.compute_transition_probability(start, end, 3));
    print!("{:?}",model.manhattan_distance((7, 7), (0, 0)))

    // use nalgebra::{Matrix4, Vector4, U2, U3};

// fn main() {
//     let s = Vector4::new(1.0, 0.0, 0.0, 0.0).transpose();

//     let p = Matrix4::new(
//         0.6, 0.2, 0.15, 0.05, 0.64, 0.16, 0.16, 0.04, 0.45, 0.15, 0.3, 0.1, 0.48, 0.12, 0.32, 0.08,
//     );

//     let mut p_lim = s;
//     for _ in 0..100 {
//         p_lim = p_lim * p;
//     }
//     println!("Final probabilities: {:?}", p.shape());
}
