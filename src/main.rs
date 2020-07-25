#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod tools;
// project imports
use tools::project::Project;

// module imports
use rocket_contrib::json::Json;
use serde::{Serialize};

// For cors policy
use rocket::http::Status;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response};
use rocket::http::{Header, ContentType, Method};

const PROJECT_BUILDFILE_PATH:&str = "build-paths";


#[derive(Serialize)]
pub struct TaskOutput{
    pub output: Vec<String>,
    pub project: Project
}

#[get("/")]
fn get_project_all() -> Json<Vec<Project>> {
    Json(Project::list(PROJECT_BUILDFILE_PATH))
}

#[post("/build", format = "application/json", data = "<project>")]
fn build_project(project:Json<Project>) -> Json<TaskOutput> {
    Json(TaskOutput{output: Project::build(&project.0), project: project.0})
}

#[post("/delete", format = "application/json", data = "<project>")]
fn delete_project(project:Json<Project>)-> Json<Project>{
    println!("DELETE -> {:?}", &project.0);
    Project::delete(&project.0, PROJECT_BUILDFILE_PATH);
    Json(project.0)
}

#[post("/log", format = "application/json", data = "<project>")]
fn log_project(project:Json<Project>) -> Json<Vec<String>>{
    println!("get logs -> {:?}", &project.0);
    Json(Project::logs(&project.0))
}

#[post("/clean", format = "application/json", data = "<project>")]
fn clean_project(project:Json<Project>) -> Json<Project>{
    println!("clean -> {:?}", &project.0);
    Project::clean(&project.0);
    Json(project.0)
}

#[post("/info", format = "application/json", data = "<project>")]
fn info_project(project:Json<Project>) -> Json<Project>{
    Json(project.0)
}

fn main() {
    rocket::ignite().mount("/", routes![
    get_project_all,
    build_project,
    log_project,
    clean_project,
    delete_project,
    info_project
    ]).attach(CORS()).launch();
}
// Disabling corse control
pub struct CORS();
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:3449"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS, DELETE, PUT"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Access-Control-Allow-Headers, Authorization, X-Requested-With, Access-Control-Allow-Origin"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "false"));
            response.set_status(Status::Ok)
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(std::io::Cursor::new(""));
            response.set_status(Status::Ok)
        }
    }
}
