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
