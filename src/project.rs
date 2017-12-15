use std::fs::{File, DirBuilder};
use std::io::{Write, Read};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    package: Package
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    //type: String,
}
/*
pub enum PackageType {
    Shared, // A .dll or .so target
    Static, // A .o target
    Binary, // An executable target
}*/

impl Project {
    /// Creates a new project and returns it's properties.
    pub fn new(name: &str, /*type: PackageType*/) -> Project {
        // Create the project directory
        DirBuilder::new().create(format!("./{}", name)).unwrap();
        // Create the project file in the new folder
        let mut project_file = File::create(format!("./{}/maid.toml", name)).unwrap();

        // Initialize the project
        let project = Project {
            package: Package {
                name: name.to_owned(),
                version: String::from("0.1.0"),
                authors: vec!(String::from("test")),
            }
        };
        // Serialize the project into TOML
        let toml: String = ::toml::to_string(&project).unwrap();

        // Write the project to the new project file
        write!(project_file, "{}", toml).unwrap();
        // Sync IO operations for the new file before continuing
        project_file.sync_all().unwrap();

        project
    }
    /// Gets the Project in the directory given (no "/" at the end)
    pub fn get(dir: &str) -> Project {
        assert!(!dir.ends_with("/")); // Ensure the given directory doesn't end with a "/"
        // Open the project file
        let mut project_file = File::open(format!("{}/maid.toml", dir)).unwrap();

        let mut contents = String::new();
        // Read the file into the String `contents`
        project_file.read_to_string(&mut contents).unwrap();

        let project: Project = ::toml::from_str(contents.as_str()).unwrap();
        println!("{:?}", project);

        project
    }
}
