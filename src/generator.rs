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
            "Gentoo Linux" => "kde-plasma/plasma-meta kde-apps/kde-apps-meta",
            _ => "",
        },
        "GNOME" => match profile.distro {
            "Arch Linux" | "Artix Linux" => "gnome gnome-extra",
            "Void Linux" => "gnome gnome-apps",
            "Gentoo Linux" => "gnome-base/gnome",
            _ => "",
        },
        "Xfce" => match profile.distro {
            "Arch Linux" | "Artix Linux" => "xfce4 xfce4-goodies",
            "Void Linux" => "xfce4 xfce4-plugins",
            "Gentoo Linux" => "xfce-base/xfce4-meta",
            _ => "",
        },
        "MATE" => match profile.distro {
            "Gentoo Linux" => "mate-base/mate",
            _ => "mate mate-extra",
        },
        "LXQt" => match profile.distro {
            "Arch Linux" | "Artix Linux" => "lxqt featherpad",
            "Void Linux" => "lxqt FeatherPad",
            "Gentoo Linux" => "lxqt-base/lxqt-meta app-editors/featherpad",
            _ => "",
        }
        "Cinnamon" => match profile.distro { 
            "Arch Linux" | "Artix Linux" => "cinnamon gnome-terminal xed",
            "Void Linux" => "cinnamon gnome-terminal gedit",
            "Gentoo Linux" => "gnome-extra/cinnamon x11-terms/gnome-terminal app-editors/gedit",
            _ => "",
        },
        "Budgie" => match profile.distro {
            "Arch Linux" | "Artix Linux" => "budgie gnome-terminal nautilus gedit",
            "Void Linux" => "budgie-desktop budgie-desktop-view budgie-control-center budgie-screensaver gnome-terminal nautilus gedit",
            "Gentoo Linux" => "# needs sarahmiaoverlay repository enabled\ngnome-extra/budgie-meta gnome-base/nautilus app-editors/gedit",
            _ => "",
        },
        "LXDE" => match profile.distro {
            "Arch Linux" | "Artix Linux" => "lxde-gtk3 mousepad",
            "Void Linux" => "lxde mousepad",
            "Gentoo Linux" => "lxde-base/lxde-meta app-editors/mousepad",
            _ => "",
        },
        "Enlightenment" => match profile.distro {
            "Arch Linux" | "Artix Linux" => "enlightenment ecrire terminology ephoto rage",
            "Void Linux" => "enlightenment terminology",
            "Gentoo Linux" => "x11-wm/enlightenment x11-terms/terminology app-editors/ecrire media-gfx/ephoto media-video/rage",
            _ => "",
        },
        _ => "",
    };
    let office = match profile.office {
        true => match profile.distro {
            "Arch Linux" | "Artix Linux" => "libreoffice-fresh",
            "Gentoo Linux" => "app-office/libreoffice-bin",
            _ => "libreoffice",
        },
        _ => ""
    };
    let displaymanager = match profile.desktop {
        "KDE Plasma" | "LXQt" => match profile.distro {
            "Gentoo Linux" => "x11-misc/sddm",
            _ => "sddm",
        },
        "GNOME" => match profile.distro {
            "Gentoo Linux" => "gnome-base/gdm",
            _ => "gdm",
        },
        "Xfce" | "MATE" | "Cinnamon" | "Budgie" | "LXDE" | "Enlightenment" => match profile.distro {
            "Void Linux" => "lightdm lightdm-gtk3-greeter",
            "Gentoo Linux" => "x11-misc/lightdm x11-misc/lightdm-gtk-greeter",
            _ => "lightdm lightdm-gtk-greeter",
        },
        _ => "",
    };
    let browser = match profile.browser {
        "Firefox" => match profile.distro {
            "Gentoo Linux" => "www-client/firefox-bin",
            _ => "firefox",
        },
        "Chromium" => match profile.distro {
            "Gentoo Linux" => "www-client/chromium",
            _ => "chromium",
        },
        _ => "",
    };

    list.push_str(&(desktop.to_owned() + &" " + &displaymanager + &" " + &browser + &" " + &office + &"\n"));
    list
}
