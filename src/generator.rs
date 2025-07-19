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
        "KDE Plasma" => match profile.distro {
            "Arch Linux" | "Artix Linux" => "plasma kde-applications",
            "Void Linux" => "kde-plasma kde-baseapps",
            _ => "",
        },
        "GNOME" => match profile.distro {
            "Arch Linux" | "Artix Linux" => "gnome gnome-extra",
            "Void Linux" => "gnome gnome-apps",
            _ => "",
        },
        "Xfce" => match profile.distro {
            "Arch Linux" | "Artix Linux" => "xfce4 xfce4-goodies",
            "Void Linux" => "xfce4 xfce4-plugins",
            _ => "",
        },
        "MATE" => "mate mate-extra",
        "LXQt" => match profile.distro {
            "Arch Linux" | "Artix Linux" => "lxqt featherpad",
            "Void Linux" => "lxqt FeatherPad",
            _ => "",
        }
        "Cinnamon" => match profile.distro { 
            "Arch Linux" | "Artix Linux" => "cinnamon xed",
            "Void Linux" => "cinnamon gedit",
            _ => "",
        },
        "Budgie" => match profile.distro {
            "Arch Linux" | "Artix Linux" => "budgie gedit",
            "Void Linux" => "budgie-desktop budgie-desktop-view budgie-control-center budgie-screensaver gedit",
            _ => "",
        },
        "LXDE" => match profile.desktop {
            "Arch Linux" | "Artix Linux" => "lxde-gtk3 mousepad",
            "Void Linux" => "lxde mousepad",
            _ => "",
        },
        "Enlightenment" => match profile.desktop {
            "Arch Linux" | "Artix Linux" => "enlightenment ecrire terminology ephoto rage",
            "Void Linux" => "enlightenment terminology",
            _ => "",
        },
        _ => "",
    };
    let office = match profile.office {
        true => match profile.distro {
            "Arch Linux" | "Artix Linux" => "libreoffice-fresh",
            _ => "libreoffice",
        },
        _ => ""
    };
    let displaymanager = match profile.desktop {
        "KDE Plasma" | "LXQt" => "sddm",
        "GNOME" => "gdm",
        "Xfce" | "MATE" | "Cinnamon" | "Budgie" | "LXDE" | "Enlightenment" => match profile.distro {
            "Void Linux" => "lightdm lightdm-gtk3-greeter",
            _ => "lightdm lightdm-gtk-greeter",
        },
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
