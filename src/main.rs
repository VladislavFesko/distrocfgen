extern crate inquire;
mod generator;

use inquire::{Select, Confirm, error::InquireResult};
use generator::{Profile, generate};
use std::env;

fn main() -> InquireResult<()> {
    let prgname = env!("CARGO_PKG_NAME");
    let prgver = env!("CARGO_PKG_VERSION");
    
    println!("\x1b[1;36mWelcome to {prgname}, version {prgver}\x1b[0m\n\nThis program helps you to configure a fresh Linux installation by generating a package list and configuration.\nIn order to generate the list, you have to choose a Linux distribution you installed, a desktop environment, a browser and an office suite\nAfter all of these steps you will have a generated text file placed on your home folder.\n\n\x1b[1;33mNOTE:\x1b[1;37m this program is still under development.\n\nGood luck.\x1b[0m\n");

    let distro = Select::new("Select a distribution:", get_list("distro")).prompt()?; 
    let desktop = Select::new("Select a desktop environment:", get_list("desktop")).prompt()?;
    let browser = Select::new("Select a browser:", get_list("browser")).prompt()?;
    let office = Confirm::new("Include an office suite (LibreOffice) in the configuration?").with_default(false).prompt()?;

    let profile = Profile {
        distro: distro,
        desktop: desktop,
        browser: browser,
        office: office
    };
    generate(profile);

    Ok(())
}

fn get_list(category: &'static str) -> Vec<&'static str> {
    match category {
        "distro" => vec![
            "Arch Linux",
            "Artix Linux",
            "Void Linux",
            "Gentoo Linux"
        ],
        "desktop" => vec![
            "KDE Plasma",
            "GNOME",
            "Xfce",
            "MATE",
            "LXQt",
            "Cinnamon",
            "Budgie",
            "LXDE",
            "Enlightenment"
        ],
        "browser" => vec![
            "Firefox",
            "Chromium"
        ],
        _ => vec![]
    }
}
