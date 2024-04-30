use std::{env, fs, io, path::{Path, PathBuf}};

fn get_output_path() -> PathBuf {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

fn main() {
    let target_dir = get_output_path();

    let resources_src = Path::join(&env::current_dir().unwrap(), "resources");
    let resources_dst = Path::join(Path::new(&target_dir), Path::new("resources"));
    copy_dir_all(resources_src, resources_dst).unwrap();

    let engine_src = Path::join(&env::current_dir().unwrap(), "engine");
    let editor_dst = Path::join(Path::new(&target_dir), Path::new("src/engine"));
    copy_dir_all(engine_src, editor_dst).unwrap();

    let editor_src = Path::join(&env::current_dir().unwrap(), "editor");
    let editor_dst = Path::join(Path::new(&target_dir), Path::new("src/editor"));
    copy_dir_all(editor_src, editor_dst).unwrap();
}
