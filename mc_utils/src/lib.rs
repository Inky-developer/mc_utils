use std::{env, path::PathBuf};

pub use rcon;
pub use server;

/// Returns the .minecraft directory path for the current platform
pub fn minecraft_dir() -> PathBuf {
    // Original minecraft code:
    // public static File getWorkingDirectory() {
    //     String userHome = System.getProperty("user.home", ".");
    //     File workingDirectory;
    //     switch(OperatingSystem.getCurrentPlatform()) {
    //     case LINUX:
    //         workingDirectory = new File(userHome, ".minecraft/");
    //         break;
    //     case WINDOWS:
    //         String applicationData = System.getenv("APPDATA");
    //         String folder = applicationData != null ? applicationData : userHome;
    //         workingDirectory = new File(folder, ".minecraft/");
    //         break;
    //     case OSX:
    //         workingDirectory = new File(userHome, "Library/Application Support/minecraft");
    //         break;
    //     default:
    //         workingDirectory = new File(userHome, "minecraft/");
    //     }

    //     return workingDirectory;
    // }

    let mut user_home = dirs::home_dir().expect("Could not retrieve home directory");
    if cfg!(windows) {
        let mut dir = env::var("APPDATA").map_or(user_home, PathBuf::from);
        dir.push(".minecraft/");
        dir
    } else if cfg!(macos) {
        user_home.push("Library/Application Support/minecraft");
        user_home
    } else {
        user_home.push(".minecraft/");
        user_home
    }
}
