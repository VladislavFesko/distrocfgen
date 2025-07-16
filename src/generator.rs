use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::Path;

pub struct Profile {
    pub distro: &'static str,
    pub desktop: &'static str,
    pub browser: &'static str,
    pub office: bool
}

pub fn generate(profile: Profile) {
    match write_file(packages(profile)) {
        Ok(_) => println!("\x1b[1;32m\nSuccessfully generated a file.\x1b[0m\nIt's saved as \x1b[1;37mdistrocfgen.txt\x1b[0m in your home folder. Now you can use it to install packages."),
        Err(_) => eprintln!("\x1b[1;31m\nAn error occurred while generating a file.\x1b[0m")
    }
}

fn write_file(content: String) -> std::io::Result<()> {
    let filepath = Path::new(env!("HOME")).join("distrocfgen.txt");
    let mut file = File::create(filepath)?;
    write!(file, "{}", content)?;
    Ok(())
}

fn packages(profile: Profile) -> String {
    let mut list = String::new();

    let desktop = match profile.desktop {
        "KDE Plasma" => "plasma kde-applications",
        "GNOME" => "gnome gnome-extra",
        "Xfce" => "xfce4 xfce4-goodies",
        "MATE" => "mate mate-extra",
        "LXQt" => "lxqt featherpad",
        "Cinnamon" => "cinnamon xed",
        "Budgie" => "budgie gedit",
        "LXDE" => "lxde-gtk3 mousepad",
        "Enlightenment" => "enlightenment ecrire terminology ephoto rage",
        _ => "",
    };
    let office = match profile.office {
        true => "libreoffice-fresh",
        _ => ""
    };
    let displaymanager = match profile.desktop {
        "KDE Plasma" | "LXQt" => "sddm",
        "GNOME" => "gdm",
        "Xfce" | "MATE" | "Cinnamon" | "Budgie" | "LXDE" | "Enlightenment" => "lightdm",
        _ => "",
    };
    let browser = match profile.browser {
        "Firefox" => "firefox",
        "Chromium" => "chromium",
        _ => "",
    };

    list.push_str(&(desktop.to_owned() + &" " + &displaymanager + &" " + &browser + &" " + &office + &"\n"));
    list
}
