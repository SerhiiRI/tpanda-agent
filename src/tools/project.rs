use crate::tools;

use std::fs::{File, DirEntry};
use std::io::{self, BufRead, repeat};
use std::path::{Path, PathBuf};
use std::process::Command;

use regex;
use powershell_script;

// To JSON serialization
use rocket::request::{FromForm};
use serde::{Serialize, Deserialize};

// Output Result build logs
// Being outputed in file where
// *PROJECT_BUILDFILE_PATH* exist
pub const PROJECT_BUILDFILE_LOG:&str = "build.log";

#[derive(Debug,Clone, PartialEq, Serialize, Deserialize, FromForm)]
pub struct Project{
    pub project: String,
    pub script: String,
    pub shell: String,
}
impl Project{
    fn re_build_script_pattern() -> Result<regex::Regex, regex::Error> {
        let a = vec!["fish","bash","sh","ps1","bat"];
        regex::Regex::new(format!(r"(?i)(build)(?-i).({})", a.join("|")).as_str())}

    pub fn is_build_script( file: &Path)-> Option<Project>{
        let resolve_project = {
            if cfg!(windows) {
                |project: &str, shell: &str, script_path: &str| -> Option<Project> {
                    println!("{:?}, shell: {:?}", &script_path, &shell);
                    return match shell {
                        "ps1" => Some(Project {project:project.to_string(), script: script_path.to_string(), shell: "cmd".to_string() }),
                        "bat" => Some(Project {project:project.to_string(), script: script_path.to_string(), shell: "cmd".to_string() }),
                        _ => None
                    };
                    None
                }
            } else if cfg!(unix) {
                |project: &str,  shell: &str, script_path: &str| -> Option<Project> {
                    return match shell {
                        "sh" => Some(Project {project:project.to_string(), script: script_path.to_string(), shell: "sh".to_string() }),
                        "bash" => Some(Project {project:project.to_string(), script: script_path.to_string(), shell: "bash".to_string() }),
                        "fish" => Some(Project {project:project.to_string(), script: script_path.to_string(), shell: "fish".to_string() }),
                        _ => None
                    };
                    None
                }
            } else { |project:&str, shell:&str, script_path:&str| -> Option<Project> {None} }
        };

        let s = Project::re_build_script_pattern().unwrap();
        if file.is_file() {
            let f = file.file_name().unwrap();
            for cap in s.captures_iter(f.to_str().unwrap()) {
                if let (Some(build), Some(shell)) = (cap.get(1),cap.get(2)){
                    let script_path = file.to_str().unwrap();
                    if let Some(project_name) =
                    file.parent()
                        .and_then(|d | d.file_name())
                        .and_then(|s| s.to_str()) {
                        return resolve_project(project_name, shell.as_str(), script_path);
                    }
                }
            }
        }
        None
    }

    pub fn from_paths(project_folder:&[&Path]) -> Vec<Project>{
        let mut projects = Vec::new();
        for x in project_folder {
            if x.is_dir(){
                if let Ok(files) = std::fs::read_dir(x) {
                    for f in files{
                        if let Result::Ok(d) = f{
                            // println!("{:?} -> {:?}", x, &d.path());
                            if let Some(project) = Project::is_build_script(&d.path()){
                                projects.push(project);
                            }
                        }
                    }
                }
            }
        }
        projects
    }

    pub fn logs(project:&Project) -> Vec<String>{
        let s = PathBuf::from(&project.script);
        let s = s.parent().unwrap();
        let s = s.join(PROJECT_BUILDFILE_LOG);
        if  s.exists() {
            let mut output = vec![];
            if let Ok(lines) = tools::files::read_lines(s) {
                for line in lines {
                    if let Ok(l) = line {
                        output.push(l);
                    }
                }
                return output;
            }
            else {
                return vec!["io error".to_string()];
            }
        }
        vec!["logs not found".to_string()]
    }

    pub fn clean(project:&Project){
        let s = PathBuf::from(&project.script);
        let s = s.parent().unwrap();
        let s = s.join(PROJECT_BUILDFILE_LOG);
        if s.exists() {
            std::fs::remove_file(s);
        }
    }

    pub fn build(project:&Project) -> Vec<String>{
        let mut script_output = vec![];
        if cfg!(unix) {
            match std::process::Command::new(&project.shell).args(&[&project.script]).output() {
                Ok(output) => {
                    println!("{:?}", output);
                    let output_str = String::from_utf8(output.stdout).unwrap();
                    let line_splited = &output_str.lines();
                    line_splited.clone().for_each(|s| script_output.push(s.to_string()));

                    let output_str = String::from_utf8(output.stderr).unwrap();
                    if !output_str.is_empty() {
                        &output_str.lines().clone().for_each(|s| script_output.push(s.to_string()));
                    }
                    let s = PathBuf::from(&project.script);
                    let s = s.parent().unwrap();
                    tools::files::write_lines_to_file(&script_output, s.join(PROJECT_BUILDFILE_LOG));
                }
                Err(e) => script_output.push("Error execution build task".to_string())
            }
        }
        if cfg!(windows) {
            if project.shell.eq("ps1") {
                if let Ok(script) = std::fs::read_to_string(&project.script) {
                    match powershell_script::run(script.as_str(), true) {
                        Ok(output) => {
                            let lines = output.to_string();
                            let lines = lines.lines();
                            lines.clone().for_each(|s| script_output.push(s.to_string()));
                        }
                        Err(e) => {
                            let lines = e.to_string();
                            let lines = lines.lines();
                            lines.clone().for_each(|s| script_output.push(s.to_string()));
                        }
                    }
                }
            }
            if project.shell.eq("cmd"){
                match std::process::Command::new(&project.shell).args(&["/C", &project.script]).output() {
                    Ok(output) => {
                        println!("{:?}", output);
                        let output_str = String::from_utf8(output.stdout).unwrap();
                        let line_splited = &output_str.lines();
                        line_splited.clone().for_each(|s| script_output.push(s.to_string()));

                        let output_str = String::from_utf8(output.stderr).unwrap();
                        if !output_str.is_empty() {
                            &output_str.lines().clone().for_each(|s| script_output.push(s.to_string()));
                        }
                        let s = PathBuf::from(&project.script);
                        let s = s.parent().unwrap();
                        tools::files::write_lines_to_file(&script_output, s.join(PROJECT_BUILDFILE_LOG));
                    }
                    Err(e) => script_output.push("Error execution build task".to_string())
                }
            }
        }
        return script_output;
    }

    pub fn list(buildfile_path:&str) -> Vec<Project> {
        let mut path_paths = vec![];
        let path_strings = tools::files::get_paths(Path::new(buildfile_path).as_ref());
        for s in &path_strings {
            path_paths.push(Path::new(s.as_str()))
        }
        Project::from_paths(&path_paths.as_slice())
    }

    pub fn delete(project:&Project, buildfile_path:&str ){
        let path_to_file = Path::new(buildfile_path);
        let s = PathBuf::from(&project.script);
        let build_path = s.parent().and_then(|s| s.to_str()).unwrap();
        let is_not_0  =  |s:&String| s.len() > 0;
        let is_path =    |s:&String| Path::new(s).exists();
        let is_project = |s:&String| s.ne(build_path);
        let mut v:Vec<String> = tools::files::file_lines(Path::new(path_to_file).as_ref()).iter()
            .map(|s|s.trim().to_string())
            .filter( is_not_0)
            .filter(is_project)
            .filter(is_path)
            .collect();
        v.dedup_by(|a, b| a.as_str().eq(b.as_str()));
        tools::files::write_lines_to_file(v, path_to_file);
    }
}








