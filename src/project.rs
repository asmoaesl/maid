use std::fs::{File, DirBuilder};
use std::io::{Write, Read};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub package: Package,
    pub build: Build,
    pub dependencies: Dependencies,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub target: Target,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
    pub preferred_compiler: Option<::user::Compiler>,
    pub gnu_options: Vec<String>,
    pub clang_options: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependencies {
    pub header_search_directories: Vec<String>,
    pub linker_search_directories: Vec<String>,
    pub link_libraries: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Target {
    // C and C++
    Executable,
    Static,
    Dynamic,
    // Scripting
    Python,
}

impl Project {
    /// Creates a new project and returns its properties.
    pub fn new(name: String, target: Target) -> Result<Project, &'static str> {
        // Check if there is already a folder with the same name as the project
        if Path::new(format!("./{}", name).as_str()).is_dir() {
            return Err("A folder with the same name already exists.");
        }

        // Create the project directory
        let mut dir_builder = DirBuilder::new();
        dir_builder.recursive(true);
        dir_builder.create(format!("./{}/source", name)).unwrap();
        dir_builder.create(format!("./{}/include", name)).unwrap();

        // Create the template main.c source file
        let mut source_file = File::create(format!("./{}/source/main.c", name)).unwrap();
        write!(source_file, "{}",
r#"#include <stdio.h>

int main(int argc, char **argv)
{
    printf("Hello, world!\n");
    return 0;
}
"#).unwrap();
        source_file.sync_data().unwrap();

        // Create the project file in the new folder
        let mut project_file = File::create(format!("./{}/Maid.toml", name)).unwrap();

        // Initialize the project
        let project = Project {
            package: Package {
                name: name.to_owned(),
                version: String::from("0.1.0"),
                authors: vec!(String::from("Johnny Appleseed")),
                target: target,
            },
            build: Build {
                preferred_compiler: None,
                gnu_options: vec!(),
                clang_options: vec!(),
            },
            dependencies: Dependencies {
                header_search_directories: vec!(),
                linker_search_directories: vec!(),
                link_libraries: vec!(),
            }
        };

        // Serialize the project into TOML
        let toml = ::toml::to_string(&project).unwrap();

        // Write the project to the new project file
        write!(project_file, "{}", toml).unwrap();
        // Sync IO operations for the new file before continuing
        project_file.sync_all().unwrap();

        Ok(project)
    }
    /// Gets the Project in the directory given
    pub fn get(dir: &Path) -> Result<Project, &'static str> {
        // Open the project file
        let mut project_file: File;
        match File::open(dir.join("Maid.toml")) {
            Ok(val) => project_file = val,
            Err(_) => return Err("There is no Maid.toml file in the current directory."),
        }

        let mut contents = String::new();
        // Read the file into the String `contents`
        project_file.read_to_string(&mut contents).unwrap();

        // Deserialize the TOML
        let project: Project = ::toml::from_str(contents.as_str()).unwrap();

        Ok(project)
    }

    /// Returns true if this project is not using conventional build settings. (They are not using
    // target = "executable", "static", or "dynamic", in their project file)
    pub fn is_custom(&self) -> bool {
        if self.package.target == Target::Executable
        || self.package.target == Target::Static
        || self.package.target == Target::Dynamic {
            false
        } else {
            true
        }
    }
}
