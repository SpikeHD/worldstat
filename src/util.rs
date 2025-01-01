use std::path::PathBuf;

pub fn player_file(uuid: impl AsRef<str>, root: PathBuf) -> Option<PathBuf> {
  let mut path = None;
  let read_dir = match root.read_dir() {
    Ok(read_dir) => read_dir,
    Err(_) => return None,
  };

  for p in read_dir {
    let p = match p {
      Ok(p) => p,
      Err(_) => continue,
    };

    let pathname = p.path();
    let filename = p.file_name();
    let filename= filename.to_string_lossy();
    // get the filename without the dashes
    let filename = filename.replace('-', "");
    
    if filename == format!("{}.json", uuid.as_ref()) {
      path = Some(pathname);
      break;
    }
  }

  path
}