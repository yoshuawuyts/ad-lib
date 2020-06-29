//! Generate a new library.

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

use dialoguer::Input;
use std::fs;
use structopt::StructOpt;

#[derive(structopt::StructOpt)]
struct Opts {
    /// The project name.
    project_name: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    femme::start();
    let opts = Opts::from_args();

    // Get user input.
    let project_name = opts.project_name.unwrap_or_else(|| {
        Input::<String>::new()
            .with_prompt("Project name")
            .interact()
            .unwrap()
    });
    let username = Input::<String>::new()
        .with_prompt("GH username")
        .interact()?;
    let description = Input::<String>::new()
        .with_prompt("Project description")
        .interact()?;

    // let project_name = "tmp";
    // let username = "yoshuawuyts";
    // let description = "";

    // Declare the function to replace strings in files.
    let replace = |string: &str| {
        string
            .replace("{{USERNAME}}", &username)
            .replace("{{PROJECTNAME}}", &project_name)
            .replace("{{DESCRIPTION}}", &description)
    };

    // Declare the function to write files.
    let write = |string: &str, target: &str| {
        let string = replace(string);
        log::debug!("> {}", target);
        fs::write(target, string)
    };

    // Create dirs.
    fs::create_dir(&project_name)?;
    std::env::set_current_dir(&project_name)?;
    fs::create_dir_all(".github/workflows")?;
    fs::create_dir("src")?;
    fs::create_dir("tests")?;

    write(include_str!("templates/README.md"), "README.md")?;
    write(include_str!("templates/LICENSE-MIT"), "LICENSE-MIT")?;
    write(include_str!("templates/LICENSE-APACHE"), "LICENSE-APACHE")?;
    write(include_str!("templates/Cargo.toml"), "Cargo.toml")?;
    write(include_str!("templates/.gitignore"), ".gitignore")?;
    write(include_str!("templates/tests/test.rs"), "tests/test.rs")?;
    write(include_str!("templates/src/lib.rs"), "src/lib.rs")?;
    write(
        include_str!("templates/.github/CONTRIBUTING.md"),
        ".github/CONTRIBUTING.md",
    )?;
    write(
        include_str!("templates/.github/CODE_OF_CONDUCT.md"),
        ".github/CODE_OF_CONDUCT.md",
    )?;
    write(
        include_str!("templates/.github/workflows/ci.yaml"),
        ".github/workflows/ci.yaml",
    )?;

    log::info!("Successfully created \"{}\"", project_name);
    Ok(())
}
