#[must_use = "this will return the tree"]
pub fn get_tree() -> Vec<String> {
    let mut tree = Vec::new();
    let walk = ignore::WalkBuilder::new(".")
        .add_custom_ignore_filename(".breathing")
        .standard_filters(true)
        .same_file_system(true)
        .threads(4)
        .build();
    for entry in walk.flatten() {
        tree.push(
            entry
                .path()
                .to_str()
                .expect("failed to get filepath")
                .to_string(),
        );
    }
    tree
}
