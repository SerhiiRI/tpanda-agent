#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;


mod tools;
use std::fs::{File, DirEntry};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use regex;



#[derive(Debug,Clone)]
pub enum System{Windows(String), Unix(String)}
#[derive(Debug,Clone)]
pub struct Project{
    pub script: String,
    pub system: System,
}
impl Project{

    fn re_build_script_pattern() -> Result<regex::Regex, regex::Error> {
        let a = vec!["fish","bash","sh","ps1","bat"];
        regex::Regex::new(format!(r"(?i)(build)(?-i).({})", a.join("|")).as_str())}

    pub fn is_build_script( file: &Path)-> Option<Project>{
        let resolve_project = {
            if cfg!(windows) {
                |shell: &str, script_path: &str| -> Option<Project> {
                    return match shell {
                        "ps1" => Some(Project { script: script_path.to_string(), system: System::Windows("cmd".to_string()) }),
                        "bat" => Some(Project { script: script_path.to_string(), system: System::Windows("cmd".to_string()) }),
                        _ => None
                    };
                    None
                }
            } else if cfg!(unix) {
                |shell: &str, script_path: &str| -> Option<Project> {
                    return match shell {
                        "sh" => Some(Project { script: script_path.to_string(), system: System::Unix("sh".to_string()) }),
                        "bash" => Some(Project { script: script_path.to_string(), system: System::Unix("bash".to_string()) }),
                        "fish" => Some(Project { script: script_path.to_string(), system: System::Unix("fish".to_string()) }),
                        _ => None
                    };
                    None
                }
            } else { |shell:&str, script_path:&str| -> Option<Project> {None} }

        };

        let s = Project::re_build_script_pattern().unwrap();
        if file.is_file() {
            let f = file.file_name().unwrap();
            for cap in s.captures_iter(f.to_str().unwrap()) {
                if let (Some(build), Some(shell)) = (cap.get(1),cap.get(2)){
                    let script_path = file.to_str().unwrap();
                    return resolve_project(shell.as_str(), script_path);
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
        // Vec::new()
        // projects
    }
    // pub fn new(build_script:&Path)->Option<Project>{
    //     let _001tmp = build_script.to_str();
    //     let _001tmp = _001tmp.unwrap();
    //     let _001tmp = _001tmp.to_string();
    //     match build_script.extension(){
    //         Some(s) => {
    //             // if cfg!(windows) {
    //             //     return match s.to_str().unwrap() {
    //             //         "ps1"|"bat" => Some(Project{script: _001tmp, system: System::Windows}),
    //             //         _ => None
    //             //     };
    //             // } else if cfg!(unix) {
    //             //     match s.to_str().unwrap() {
    //             //         "sh" | "bash" | "fish" => Some(Project{script: _001tmp, system: System::Windows}),
    //             //         _ => return None
    //             //     };
    //             // };
    //
    //         }
    //         None => {return None}
    //     }
    //     None
    // }
}

use powershell_script;
use std::process::Command;

fn run_sh(){
    match std::pro cess::Command::new("cmd").args(&["/C", ".\\build_dir\\build.bat"]).output(){
        Ok(output) => println!("{:?}", std::str::from_utf8(output.stdout.as_slice()).unwrap()),
        Err(e) => println!("Evaluation problem")
    }
}
fn run_cmd(){
    match std::process::Command::new("cmd").args(&["/C", ".\\build_dir\\build.bat"]).output(){
        Ok(output) => println!("{:?}", std::str::from_utf8(output.stdout.as_slice()).unwrap()),
        Err(e) => println!("Evaluation problem")
    }
}
fn run_ps1(){
    match powershell_script::run("./build_dir/build.ps1", false){
        Ok(output) => println!("{:?}",output.stdout().unwrap()),
        Err(e) => println!("Evaluation problem")
    }
}
fn initialize_projects() -> Vec<Project> {
    let mut path_paths = vec![];
    let path_strings = tools::files::get_paths(Path::new("build-paths").as_ref());
    for s in &path_strings {
        path_paths.push(Path::new(s.as_str()))
    }
    Project::from_paths(&path_paths.as_slice())
}

#[get("/")]
fn index() -> String {
    // let a = tools::files::file_lines("build-paths");
    // let s = a.join(",");
    // s
    "".to_string()
}

fn main() {
    // rocket::ignite().mount("/", routes![index]).launch();

    // let builddir = vec![Path::new("."),Path::new("src")];
    //Project::from_paths(builddir.as_slice());
    // let PROJECT_LIST = initialize_projects();
    run_ps1();
    run_cmd();


    // println!("Paths:\n{:?}", &PROJECT_LIST);
}