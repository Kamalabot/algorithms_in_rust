use bt_pattern_02::{print_mxn_grid, print_vector_map};

fn main() {
    println!("welcome to  bt_dp_prac crate");
    let tmap = vec![
        vec![0, 2, 2, 1],
        vec![3, 1, 1, 2],
        vec![4, 0, 2, 1],
        vec![2, 3, 2, 0],
    ];
    print_vector_map(tmap);
    print_mxn_grid(3, 3);
}
