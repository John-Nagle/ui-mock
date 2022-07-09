/// guiutil.rs -- GUI utility functions
//
//  Animats
//  June, 2022


use std::sync::Arc;
use rend3::Renderer;
use image::GenericImageView;
use egui::{TextureId};
use rend3_egui::EguiRenderRoutine;
use egui::FontFamily::{Proportional};
use egui::TextStyle::{Button, Small, Heading, Body, /*Name,*/ Monospace};
use egui::FontId;
use anyhow::{anyhow, Error, Context};
use std::str::FromStr;

const DEVELOPER: &str = "animats";              // used for directory generation - lower case
const LOG_FILE_NAME: &str = "log.txt";          // name of log file


/// Load an icon at compile time. Image is built into executable.
pub fn load_canned_icon(
    image_bytes: &[u8],
    egui_routine: &mut EguiRenderRoutine,
    renderer: &Arc<Renderer>,
) -> TextureId {
    //Images
    let image_image = image::load_from_memory(image_bytes).unwrap();
    let image_rgba = image_image.as_rgba8().unwrap().clone().into_raw();
    let dimensions = image_image.dimensions();
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    //  Create and return texture
    rend3_egui::EguiRenderRoutine::create_egui_texture(
        &mut egui_routine.internal,
        renderer,
        format,
        &image_rgba,
        dimensions,
        Some("Canned icon"),
    )
}

/// Set our default styles
//  Called once at startup.
pub fn set_default_styles(ctx: &egui::Context) {
    // Get current context style
    let mut style = (*ctx.style()).clone();
    //  Redefine text_styles
    //  Have to define all of them
    style.text_styles = [
    (Heading, FontId::new(20.0, Proportional)),
    ////(Name("Heading2".into()), FontId::new(25.0, Proportional)),
    ////(Name("Context".into()), FontId::new(23.0, Proportional)),
    (Body, FontId::new(16.0, Proportional)),
    (Monospace, FontId::new(18.0, Proportional)),
    (Button, FontId::new(18.0, Proportional)),
    (Small, FontId::new(14.0, Proportional)),
    ].into();
    // Mutate global style with above changes
    ctx.set_style(style);
}

/// Get name of program.
pub fn get_executable_name() -> String {
    //  Get name of program. This is unreasonably difficult.
    std::env::current_exe().unwrap().file_stem().unwrap().to_string_lossy().to_string().to_lowercase() // just to get program name
}

/// Get log path -- get file name for log
pub fn get_log_file_name() -> Result<Box<std::path::PathBuf>, Error> {
    let executable = get_executable_name();     // name of program
    if let Some(proj_dirs) = directories::ProjectDirs::from("com", DEVELOPER,  &executable) {
        let local_dir = proj_dirs.data_local_dir(); // directory into which logs will go
        println!("Proj dirs data local dir: {:?}", local_dir); // ***TEMP***
        std::fs::create_dir_all(local_dir).with_context(|| format!("Trouble creating logging directory: {:?}", local_dir))?;  // create any needed directories
        let path = local_dir.join(LOG_FILE_NAME);      
        println!("Log path: {:?}", path); // ***TEMP***
        Ok(Box::new(path))
    } else {
        Err(anyhow!("Unable to determine project directories"))
    }
}

/// Get asset directory.
///
/// - First choice: EXECUTABLEDIR/ASSETFOLDERNAME
/// - Second choice: EXECUTABLEDIR/PROGRAMNAME-ASSETFOLDERNAME
/// - Third choice: DEVASSETDIR/ASSETFOLDERNAME
//
pub fn get_asset_dir(dev_asset_dir_opt: Option<&str>, asset_folder_name: &str) -> Result<Box<std::path::PathBuf>, Error> {
    let mut executable_path = std::env::current_exe().unwrap().canonicalize()?; // path to executable
    if !executable_path.exists() || !executable_path.is_file() {
        return Err(anyhow!("Cannot find our own executable program file: {:?}", executable_path))
    }

    let executable_name = executable_path.file_stem().unwrap().to_string_lossy().to_string().to_lowercase(); // executable as lowercase
    executable_path.pop();                  // reduce to directory containing executable
    if !executable_path.exists() || !executable_path.is_dir() {
        return Err(anyhow!("Cannot find our own executable program's directory: {:?}", executable_path))
    }
    //  We found the executable directory.
    //  Try first choice: executabledir/assets
    let choice1 = executable_path.join(asset_folder_name);
    if choice1.exists() && choice1.is_dir() {
        return Ok(Box::new(choice1));
    }
    //  Try second choice: executabledir/progname-assets
    let choice2 = executable_path.join(executable_name + "-" + asset_folder_name);
    if choice2.exists() && choice2.is_dir() {
        return Ok(Box::new(choice2));
    }
    //  Try third choice. Only used during development. projectdir/src/assets
    if let Some(dev_asset_dir) = dev_asset_dir_opt {
        let choice3 = std::path::PathBuf::from_str(dev_asset_dir)?.join("src").join(asset_folder_name);
        if choice3.exists() && choice3.is_dir() {
            return Ok(Box::new(choice3));
        }
    }
    Err(anyhow!("Cannot find our asset directory {:?} in any usual place.", asset_folder_name))
}
