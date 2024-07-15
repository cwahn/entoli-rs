// def file_exists(path: Path) -> Io[bool]:
//     return Io(lambda: path.exists())

use std::{fs::OpenOptions, io::Write, os::unix::fs::PermissionsExt, path::PathBuf};

use crate::prelude::Io;

#[derive(Clone)]
pub struct FileExistsIo {
    path: PathBuf,
}

impl Io for FileExistsIo {
    type Output = bool;

    fn run(self) -> Self::Output {
        self.path.exists()
    }
}

pub fn file_exists(path: PathBuf) -> FileExistsIo {
    FileExistsIo { path }
}

// def dir_exists(path: Path) -> Io[bool]:
//     return Io(lambda: path.is_dir())

#[derive(Clone)]
pub struct DirExistsIo {
    path: PathBuf,
}

impl Io for DirExistsIo {
    type Output = bool;

    fn run(self) -> Self::Output {
        self.path.is_dir()
    }
}

pub fn dir_exists(path: PathBuf) -> DirExistsIo {
    DirExistsIo { path }
}

// def list_dir(path: Path) -> Io[list[Path]]:
//     return Io(lambda: list(path.iterdir()))

#[derive(Clone)]
pub struct ListDirIo {
    path: PathBuf,
}

impl Io for ListDirIo {
    type Output = Vec<PathBuf>;

    fn run(self) -> Self::Output {
        self.path
            .read_dir()
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect()
    }
}

pub fn list_dir(path: PathBuf) -> ListDirIo {
    ListDirIo { path }
}

// def read_file(path: Path) -> Io[str]:
//     return Io(lambda: path.read_text())

#[derive(Clone)]
pub struct ReadFileIo {
    path: PathBuf,
}

impl Io for ReadFileIo {
    type Output = String;

    fn run(self) -> Self::Output {
        std::fs::read_to_string(&self.path).unwrap()
    }
}

pub fn read_file(path: PathBuf) -> ReadFileIo {
    ReadFileIo { path }
}

// def write_file(path: Path, content: str) -> Io[None]:
//     def _inner() -> None:
//         path.write_text(content)

//     return Io(_inner)

#[derive(Clone)]
pub struct WriteFileIo {
    path: PathBuf,
    content: String,
}

impl Io for WriteFileIo {
    type Output = ();

    fn run(self) -> Self::Output {
        std::fs::write(&self.path, self.content).unwrap()
    }
}

pub fn write_file(path: PathBuf, content: String) -> WriteFileIo {
    WriteFileIo { path, content }
}

// def append_file(path: Path, content: str) -> Io[None]:
//     def _inner() -> None:
//         path.write_text(path.read_text() + content)

//     return Io(_inner)

#[derive(Clone)]
pub struct AppendFileIo {
    path: PathBuf,
    content: String,
}

impl Io for AppendFileIo {
    type Output = ();

    fn run(self) -> Self::Output {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
            .unwrap();
        file.write_all(self.content.as_bytes()).unwrap();
    }
}

pub fn append_file(path: PathBuf, content: String) -> AppendFileIo {
    AppendFileIo { path, content }
}

// def create_dir(path: Path) -> Io[None]:
//     return Io(lambda: path.mkdir())

#[derive(Clone)]
pub struct CreateDirIo {
    path: PathBuf,
}

impl Io for CreateDirIo {
    type Output = ();

    fn run(self) -> Self::Output {
        std::fs::create_dir(&self.path).unwrap()
    }
}

pub fn create_dir(path: PathBuf) -> CreateDirIo {
    CreateDirIo { path }
}

// def create_dir_if_missing(parent_as_well: bool, path: Path) -> Io[None]:
//     return Io(lambda: path.mkdir(parents=parent_as_well, exist_ok=True))

#[derive(Clone)]
pub struct CreateDirIfMissingIo {
    parent_as_well: bool,
    path: PathBuf,
}

impl Io for CreateDirIfMissingIo {
    type Output = ();

    fn run(self) -> Self::Output {
        if self.parent_as_well {
            std::fs::create_dir_all(&self.path).unwrap();
        } else {
            std::fs::create_dir(&self.path).unwrap();
        }
    }
}

pub fn create_dir_if_missing(parent_as_well: bool, path: PathBuf) -> CreateDirIfMissingIo {
    CreateDirIfMissingIo {
        parent_as_well,
        path,
    }
}

// def remove_file(path: Path) -> Io[None]:
//     return Io(lambda: path.unlink())

#[derive(Clone)]
pub struct RemoveFileIo {
    path: PathBuf,
}

impl Io for RemoveFileIo {
    type Output = ();

    fn run(self) -> Self::Output {
        std::fs::remove_file(&self.path).unwrap()
    }
}

pub fn remove_file(path: PathBuf) -> RemoveFileIo {
    RemoveFileIo { path }
}

// def remove_dir(path: Path) -> Io[None]:
//     return Io(lambda: path.rmdir())

#[derive(Clone)]
pub struct RemoveDirIo {
    path: PathBuf,
}

impl Io for RemoveDirIo {
    type Output = ();

    fn run(self) -> Self::Output {
        std::fs::remove_dir(&self.path).unwrap()
    }
}

pub fn remove_dir(path: PathBuf) -> RemoveDirIo {
    RemoveDirIo { path }
}

// def remove_dir_rec(path: Path) -> Io[None]:
//     return Io(lambda: path.rmdir())

#[derive(Clone)]
pub struct RemoveDirRecIo {
    path: PathBuf,
}

impl Io for RemoveDirRecIo {
    type Output = ();

    fn run(self) -> Self::Output {
        std::fs::remove_dir_all(&self.path).unwrap()
    }
}

pub fn remove_dir_rec(path: PathBuf) -> RemoveDirRecIo {
    RemoveDirRecIo { path }
}

// # todo

// def get_permissions(path: Path) -> Io[os.stat_result]:
//     return Io(lambda: path.stat())

#[derive(Clone)]
pub struct GetPermissionsIo {
    path: PathBuf,
}

impl Io for GetPermissionsIo {
    type Output = std::fs::Metadata;

    fn run(self) -> Self::Output {
        self.path.metadata().unwrap()
    }
}

pub fn get_permissions(path: PathBuf) -> GetPermissionsIo {
    GetPermissionsIo { path }
}

// def set_permissions(path: Path, mode: int) -> Io[None]:
//     return Io(lambda: path.chmod(mode))

#[derive(Clone)]
pub struct SetPermissionsIo {
    path: PathBuf,
    mode: u32,
}

impl Io for SetPermissionsIo {
    type Output = ();

    fn run(self) -> Self::Output {
        std::fs::set_permissions(&self.path, std::fs::Permissions::from_mode(self.mode)).unwrap()
    }
}

pub fn set_permissions(path: PathBuf, mode: u32) -> SetPermissionsIo {
    SetPermissionsIo { path, mode }
}

// def get_modification_time(path: Path) -> Io[datetime]:
//     return Io(lambda: datetime.fromtimestamp(path.stat().st_mtime))

#[derive(Clone)]
pub struct GetModificationTimeIo {
    path: PathBuf,
}

impl Io for GetModificationTimeIo {
    type Output = std::time::SystemTime;

    fn run(self) -> Self::Output {
        self.path.metadata().unwrap().modified().unwrap()
    }
}

pub fn get_modification_time(path: PathBuf) -> GetModificationTimeIo {
    GetModificationTimeIo { path }
}
