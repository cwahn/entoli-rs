// def call_command(command: str) -> Io[None]:
//     def _inner() -> None:
//         result = subprocess.run(command, shell=True)
//         if result.returncode != 0:
//             raise subprocess.CalledProcessError(result.returncode, command)

//     return Io(_inner)

use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::prelude::Io;

// createProcess :: CreateProcess -> IO (Maybe Handle, Maybe Handle, Maybe Handle, ProcessHandle)

pub struct CreateProcess {
    cmd_spec: CmdSpec,
    cwd: Option<PathBuf>,
    env: Option<Vec<(String, String)>>,
    std_in: StdStream,
    std_out: StdStream,
    std_err: StdStream,
}

/// Either a shell command or a raw command
///
/// Shell commands are executed in a shell
///
/// Raw command is executable path and arguments

pub fn create_process(process: CreateProcess) -> CreateProcessIo {
    CreateProcessIo { process }
}

pub fn shell<S: Into<String>>(cmd: S) -> CreateProcess {
    CreateProcess {
        cmd_spec: CmdSpec::Shell(cmd.into()),
        cwd: None,
        env: None,
        std_in: StdStream::Inherit,
        std_out: StdStream::Inherit,
        std_err: StdStream::Inherit,
    }
}

pub fn proc(exec: PathBuf, args: Vec<String>) -> CreateProcess {
    CreateProcess {
        cmd_spec: CmdSpec::Raw(exec, args),
        cwd: None,
        env: None,
        std_in: StdStream::Inherit,
        std_out: StdStream::Inherit,
        std_err: StdStream::Inherit,
    }
}

#[derive(Clone)]
pub enum CmdSpec {
    Shell(String),
    Raw(PathBuf, Vec<String>),
}

// ! #[derive(Clone)]
pub enum StdStream {
    Inherit,
    UseHandle(std::process::Stdio),
    CreatePipe,
    NoStream,
}

pub struct CreateProcessIo {
    process: CreateProcess,
}

impl Io for CreateProcessIo {
    type Output = std::process::Child; // Rust already has good abstractions for processes instead of tuples

    fn run(self) -> Self::Output {
        let mut command = match self.process.cmd_spec {
            CmdSpec::Shell(cmd) => {
                let mut command = std::process::Command::new("sh");
                command.arg("-c").arg(cmd);
                command
            }
            CmdSpec::Raw(path, args) => {
                let mut command = std::process::Command::new(path);
                command.args(args);
                command
            }
        };

        if let Some(cwd) = self.process.cwd {
            command.current_dir(cwd);
        }

        if let Some(env) = self.process.env {
            command.envs(env);
        }

        match self.process.std_in {
            StdStream::Inherit => {
                command.stdin(std::process::Stdio::inherit());
            }
            StdStream::UseHandle(handle) => {
                command.stdin(handle);
            }
            StdStream::CreatePipe => {
                command.stdin(std::process::Stdio::piped());
            }
            StdStream::NoStream => {
                command.stdin(std::process::Stdio::null());
            }
        }

        match self.process.std_out {
            StdStream::Inherit => {
                command.stdout(std::process::Stdio::inherit());
            }
            StdStream::UseHandle(handle) => {
                command.stdout(handle);
            }
            StdStream::CreatePipe => {
                command.stdout(std::process::Stdio::piped());
            }
            StdStream::NoStream => {
                command.stdout(std::process::Stdio::null());
            }
        }

        match self.process.std_err {
            StdStream::Inherit => {
                command.stderr(std::process::Stdio::inherit());
            }
            StdStream::UseHandle(handle) => {
                command.stderr(handle);
            }
            StdStream::CreatePipe => {
                command.stderr(std::process::Stdio::piped());
            }
            StdStream::NoStream => {
                command.stderr(std::process::Stdio::null());
            }
        }

        command.spawn().unwrap()
    }
}

// Simpler functions for common tasks
#[derive(Clone)]
pub struct CallProcessIo {
    exec: PathBuf,
    args: Vec<String>,
}

impl Io for CallProcessIo {
    type Output = ();

    fn run(self) -> Self::Output {
        let status = std::process::Command::new(self.exec)
            .args(self.args)
            .status()
            .unwrap();

        if !status.success() {
            panic!("Command failed with exit code: {}", status);
        }
    }
}

pub fn call_process(exec: PathBuf, args: Vec<String>) -> CallProcessIo {
    CallProcessIo { exec, args }
}

#[derive(Clone)]
pub struct CallCommandIo {
    command: String,
}

impl Io for CallCommandIo {
    type Output = ();

    fn run(self) -> Self::Output {
        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .status()
            .unwrap();

        if !status.success() {
            panic!("Command failed with exit code: {}", status);
        }
    }
}

pub fn call_command(command: String) -> CallCommandIo {
    CallCommandIo { command }
}

#[derive(Clone)]
pub struct SpawnProcessIo {
    exec: PathBuf,
    args: Vec<String>,
}

impl Io for SpawnProcessIo {
    type Output = std::process::Child;

    fn run(self) -> Self::Output {
        std::process::Command::new(self.exec)
            .args(self.args)
            .spawn()
            .unwrap()
    }
}

pub fn spawn_process(exec: PathBuf, args: Vec<String>) -> SpawnProcessIo {
    SpawnProcessIo { exec, args }
}

#[derive(Clone)]
pub struct SpawnCommandIo {
    command: String,
}

impl Io for SpawnCommandIo {
    type Output = std::process::Child;

    fn run(self) -> Self::Output {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .spawn()
            .unwrap()
    }
}

pub fn spawn_command(command: String) -> SpawnCommandIo {
    SpawnCommandIo { command }
}

pub struct ReadCreateProcessIo {
    process: CreateProcess,
    stdin: String,
}

impl Io for ReadCreateProcessIo {
    type Output = String;

    fn run(self) -> Self::Output {
        let mut command = match self.process.cmd_spec {
            CmdSpec::Shell(cmd) => {
                let mut command = std::process::Command::new("sh");
                command.arg("-c").arg(cmd);
                command
            }
            CmdSpec::Raw(path, args) => {
                let mut command = std::process::Command::new(path);
                command.args(args);
                command
            }
        };

        if let Some(cwd) = self.process.cwd {
            command.current_dir(cwd);
        }

        if let Some(env) = self.process.env {
            command.envs(env);
        }

        if let StdStream::CreatePipe = self.process.std_in {
            command.stdin(std::process::Stdio::piped());
        }

        if let StdStream::CreatePipe = self.process.std_out {
            command.stdout(std::process::Stdio::piped());
        }

        if let StdStream::CreatePipe = self.process.std_err {
            command.stderr(std::process::Stdio::piped());
        }

        let mut child = command.spawn().unwrap();

        if let StdStream::CreatePipe = self.process.std_in {
            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(self.stdin.as_bytes())
                .unwrap();
        };

        let output = child.wait_with_output().unwrap();

        String::from_utf8(output.stdout).unwrap()
    }
}

pub fn read_create_process(process: CreateProcess, stdin: String) -> ReadCreateProcessIo {
    ReadCreateProcessIo { process, stdin }
}

#[derive(Clone)]
pub struct ReadProcess {
    exec: PathBuf,
    args: Vec<String>,
    stdin: String,
}

impl Io for ReadProcess {
    type Output = String;

    fn run(self) -> Self::Output {
        let mut command = std::process::Command::new(self.exec);
        command.args(self.args);

        command.stdin(std::process::Stdio::piped());
        command.stdout(std::process::Stdio::piped());
        command.stderr(std::process::Stdio::piped());

        let mut child = command.spawn().unwrap();

        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(self.stdin.as_bytes())
            .unwrap();

        let output = child.wait_with_output().unwrap();

        String::from_utf8(output.stdout).unwrap()
    }
}

pub fn read_process(exec: PathBuf, args: Vec<String>, stdin: String) -> ReadProcess {
    ReadProcess { exec, args, stdin }
}

pub struct ReadCreateProcessWithExitCode {
    process: CreateProcess,
    stdin: String,
}

impl Io for ReadCreateProcessWithExitCode {
    type Output = (String, i32);

    fn run(self) -> Self::Output {
        let mut command = match self.process.cmd_spec {
            CmdSpec::Shell(cmd) => {
                let mut command = std::process::Command::new("sh");
                command.arg("-c").arg(cmd);
                command
            }
            CmdSpec::Raw(path, args) => {
                let mut command = std::process::Command::new(path);
                command.args(args);
                command
            }
        };

        if let Some(cwd) = self.process.cwd {
            command.current_dir(cwd);
        }

        if let Some(env) = self.process.env {
            command.envs(env);
        }

        if let StdStream::CreatePipe = self.process.std_in {
            command.stdin(std::process::Stdio::piped());
        }

        if let StdStream::CreatePipe = self.process.std_out {
            command.stdout(std::process::Stdio::piped());
        }

        if let StdStream::CreatePipe = self.process.std_err {
            command.stderr(std::process::Stdio::piped());
        }

        let mut child = command.spawn().unwrap();

        if let StdStream::CreatePipe = self.process.std_in {
            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(self.stdin.as_bytes())
                .unwrap();
        };

        let output = child.wait_with_output().unwrap();

        (
            String::from_utf8(output.stdout).unwrap(),
            output.status.code().unwrap(),
        )
    }
}

#[derive(Clone)]
pub struct ReadProcessWithExitCode {
    exec: PathBuf,
    args: Vec<String>,
    stdin: String,
}

impl Io for ReadProcessWithExitCode {
    type Output = (String, i32);

    fn run(self) -> Self::Output {
        let mut command = std::process::Command::new(self.exec);
        command.args(self.args);

        command.stdin(std::process::Stdio::piped());
        command.stdout(std::process::Stdio::piped());
        command.stderr(std::process::Stdio::piped());

        let mut child = command.spawn().unwrap();

        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(self.stdin.as_bytes())
            .unwrap();

        let output = child.wait_with_output().unwrap();

        (
            String::from_utf8(output.stdout).unwrap(),
            output.status.code().unwrap(),
        )
    }
}

pub fn read_create_process_with_exit_code(
    process: CreateProcess,
    stdin: String,
) -> ReadCreateProcessWithExitCode {
    ReadCreateProcessWithExitCode { process, stdin }
}

// todo withCreateProcess withCreateProcess :: CreateProcess -> (Maybe Handle -> Maybe Handle -> Maybe Handle -> ProcessHandle -> IO a) -> IO a

// todo cleanupProcess :: (Maybe Handle, Maybe Handle, Maybe Handle, ProcessHandle) -> IO ()

// Related utilities

pub fn show_command_for_user(exec: PathBuf, args: Vec<String>) -> String {
    format!("{} {}", exec.display(), args.join(" "))
}

// pub fn get_pid(process: std::process::Child) -> u32 {
//     process.id()
// }

pub struct GetPid {
    process: std::process::Child,
}

impl Io for GetPid {
    type Output = u32;

    fn run(self) -> Self::Output {
        self.process.id()
    }
}

pub fn get_pid(process: std::process::Child) -> GetPid {
    GetPid { process }
}

#[derive(Clone)]
pub struct GetCurrentPid;

impl Io for GetCurrentPid {
    type Output = u32;

    fn run(self) -> Self::Output {
        std::process::id()
    }
}

pub fn get_current_pid() -> GetCurrentPid {
    GetCurrentPid
}

// Process completion

pub enum ExitCode {
    Success,
    Failure(i32),
}

pub struct WaitForProcess {
    process: std::process::Child,
}

impl Io for WaitForProcess {
    type Output = ExitCode;

    fn run(mut self) -> Self::Output {
        let status = self.process.wait().unwrap();

        if status.success() {
            ExitCode::Success
        } else {
            ExitCode::Failure(status.code().unwrap())
        }
    }
}

pub fn wait_for_process(process: std::process::Child) -> WaitForProcess {
    WaitForProcess { process }
}

pub struct GetProcessExitCode {
    process: std::process::Child,
}

impl Io for GetProcessExitCode {
    type Output = Option<ExitCode>;

    fn run(mut self) -> Self::Output {
        let status = self.process.try_wait().unwrap();

        match status {
            Some(status) => {
                if status.success() {
                    Some(ExitCode::Success)
                } else {
                    Some(ExitCode::Failure(status.code().unwrap()))
                }
            }
            None => None,
        }
    }
}

pub fn get_process_exit_code(process: std::process::Child) -> GetProcessExitCode {
    GetProcessExitCode { process }
}

pub struct TerminateProcess {
    process: std::process::Child,
}

impl Io for TerminateProcess {
    type Output = ();

    fn run(mut self) -> Self::Output {
        self.process.kill().unwrap();
    }
}

pub fn terminate_process(process: std::process::Child) -> TerminateProcess {
    TerminateProcess { process }
}

// todo interruptProcessGroupOf

// Interprocess communication
