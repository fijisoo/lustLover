
/*
Rekursywne struktury:

Napisz strukturę TreeNode, która ma pole value typu i32 i pole children, które jest wektorem wskaźników do TreeNode. Utwórz prosty przykładowy węzeł drzewa.
 */
#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: Option<Vec<TreeNode>>
}

pub fn run_ex7(){
    let tree_node = TreeNode {
        value: 0,
        children: Some(vec! [TreeNode {
            value: 1,
            children: Some(vec! [TreeNode {
                value: 2,
                children: None
            }])
        }])
    };

    println!("Testing {:?}", tree_node)
}