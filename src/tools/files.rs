use std::fs;
use std::path;
use std::env as environment;
use std::io;
// use std::io::{BufWriter, Write, Read, ErrorKind, Result, Lines, BufReader};
use std::path::Path;
use std::io::{Write, BufRead};
use std::str::Lines;

const PROGRAM:&str = "Jarman";

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) fn file_lines(path_to_file:&Path) -> Vec<String>{
    let mut buffer = Vec::new();
    if let Ok(lines) = read_lines(path_to_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // println!("{}", ip);
                buffer.push(ip);
            }
        }
    }
    buffer
}
// use std::cmp::PartialEq;
pub fn get_paths(path_to_file:&Path) -> Vec<String>{
    let is_not_0  =  |s:&String| s.len() > 0;
    let is_comment = |s:&String| ! ('#' == s.chars().nth(0).unwrap());
    let is_path =    |s:&String| Path::new(s).exists();
    let mut v:Vec<String> = file_lines(Path::new(path_to_file).as_ref()).iter()
        .map(|s|s.trim().to_string())
        .filter( is_not_0)
        .filter(is_comment)
        .filter(is_path)
        .collect();
    v.dedup_by(|a, b| a.as_str().eq(b.as_str()));
    v
}

pub fn write_static_file<U: AsRef<path::Path>>(FILE: &'static [u8], destination_path: U) -> std::io::Result<()> {
    let mut buffer = fs::File::create(destination_path)?;
    let mut pos = 0;
    while pos < FILE.len() {
        let bytes_written = buffer.write(&FILE[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}

pub fn write_lines_to_file<U: AsRef<Vec<String>>, P: AsRef<path::Path>>(lines: U, destination_path: P) -> std::io::Result<()>{

    let file = fs::File::create(destination_path)?;
    let mut file = std::io::LineWriter::new(file);

    for l in lines.as_ref() {
        file.write_all(l.as_bytes())?;
        file.write(b"\n")?;
    }
    file.flush()?;
    Ok(())
}

pub fn run_exe_file(exe_path: &Path){
    println!("{:?}",absolute_path(exe_path).unwrap().to_str());
    let _exe = exe_path.file_name().unwrap();
    let exe = _exe.to_str().unwrap();
    let _dir = exe_path.parent().unwrap();
    let dir = _dir.to_str().unwrap();
    let command = format!("start /D '{}' {}", dir, exe);
    println!("{}", command);
    std::process::Command::new("cmd")
        .args(&["/C", "start", "/D", dir, exe])
        .output()
        .expect(format!("Failed to run {:?}", &exe_path).as_str());
}

pub fn copy_dir<U: AsRef<path::Path>, V: AsRef<path::Path>>(from: U, to: V) -> Result<(), io::Error> {
    let mut stack = Vec::new();
    stack.push(path::PathBuf::from(from.as_ref()));

    let output_root = path::PathBuf::from(to.as_ref());
    let input_root = path::PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: path::PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn install_path_resolver() -> String {
    // concat PROGRAM to `folder_path` and create folder;
    // Return new path
    let install_to_folder= |folder_path:&str| -> String {
        let mut folder_path = folder_path.to_string();
        if folder_path.ends_with(path::MAIN_SEPARATOR) {
            folder_path.push_str(PROGRAM)
        } else {
            folder_path.push(path::MAIN_SEPARATOR);
            folder_path.push_str(PROGRAM);
        }
        let installation_folder= folder_path.as_str();
        fs::create_dir_all(path::Path::new(installation_folder)).unwrap();
        println!("Program will be installed to {} folder",installation_folder);
        installation_folder.to_string()
    };
    // depend on architecuture, select install folder
    if cfg!(windows) {
        if let Ok(s) = environment::var("PROGRAMFILES") {
            return install_to_folder(s.as_str());
        } else {

            if path::Path::new(r"C:\Program Files").exists() {
                return install_to_folder("C:\\Program Files");
            } else {
                return install_to_folder(".");
            }
        }
    } else if cfg!(unix) {
        return install_to_folder(".");
    };
    install_to_folder(".")
}



pub fn remove_path(some_path: &Path){
    if let _ = fs::metadata(some_path)
    {
        if some_path.is_dir() {
            fs::remove_dir_all(some_path);
        } else {
            fs::remove_file(some_path);
        }
    }
}

pub fn absolute_path<P>(path: P) -> std::io::Result<path::PathBuf> where P: AsRef<path::Path>{
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        environment::current_dir()?.join(path)
    };
    Ok(absolute_path)
}

pub fn make_desktop_icon(exe_path: &path::Path) -> Result<String,String> {
    if cfg!(windows) {
        use powershell_script;
        if let Ok(p) = environment::var("USERPROFILE") {
            let p = path::Path::new(p.as_str()).join("Desktop").join(format!("{}.lnk",PROGRAM));
            let link = p.to_str().unwrap();
            let _temporary = absolute_path(&exe_path).unwrap().to_owned();
            let exe = _temporary.to_str().unwrap();
            println!("Desktop -> {}", &link);
            println!("Exe -> {}", &exe);
            let create_shortcut_ps1 = format!(
                r#"
                $SourceFileLocation="{executable}"
                $ShortcutLocation="{desctop_link}"
                $WScriptShell=New-Object -ComObject WScript.Shell
                $Shortcut=$WScriptShell.CreateShortcut($ShortcutLocation)
                $Shortcut.TargetPath=$SourceFileLocation
                $Shortcut.Save()"#, executable=&exe, desctop_link=&link);
            match powershell_script::run(&create_shortcut_ps1.as_str(), true) {
                Ok(output) => {
                    println!("{}", output.to_string());
                    return Ok("created".to_string())
                }
                Err(e) => {
                    println!("{}", e.to_string());
                    return Err("fail".to_string())
                }
            }
            return Ok("".to_string())
        } else { return Err("Path not exist".to_string()) }
    }
    Err("Desktop icon can be create only on windows-based systems".to_owned())
}