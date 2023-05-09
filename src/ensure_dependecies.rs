// Purpose: Ensure that all dependencies are installed.

pub fn ensure_dependencies() {
    let deps = vec!["tmux"];
    for dep in deps {
        match which::which(dep) {
            Ok(_) => (),
            Err(_) => panic!("{} is not installed", dep),
        }
    }
}
