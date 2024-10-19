#![allow(unused_imports)]
#![deny(warnings)]

use ds_prac0::{binary_first_tt, depth_first_tt, make_tree};

fn main() {
    // Example tree:
    //         1
    //        / \
    //       2   3
    //        \  /
    //         4 5
    let tree_data = vec![1, 2, 3, 4, 5];

    let root = make_tree(tree_data);

    // DFS In-order traversal
    let mut dfs_result = Vec::new();
    depth_first_tt(&root, &mut dfs_result);
    println!("DFS (In-order) Traversal: {:?}", dfs_result);

    // BFS traversal
    let bfs_result = binary_first_tt(&root);
    println!("BFS Traversal: {:?}", bfs_result);
}
