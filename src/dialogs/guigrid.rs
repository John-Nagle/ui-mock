///#  guigrid.rs -- grid selection
//
//  This is the first screen displayed on startup.
//
//  Animats
//  October 2022
//
use anyhow::{Error, Context, anyhow};
////use crate::t;
use std::fs::File;
use std::path::PathBuf;
use std::io::Read;
use serde::{Deserialize};
use crate::{GuiAssets};
/// Basic info about a grid for the splash page
/// GridSelectParams file contents.
#[derive(Debug, Clone, Deserialize)]
pub struct GridSelectParamsData {
    pub metaverse: String,                      // Second Life, OsGrid, etc.
    pub grid: String,                           // agni, etc.
    pub picture_bar: String,                    // local file name in images directory
    pub home_url: String,                       // home page for site   
    pub join_url: Option<String>,               // How to join
    pub login_url: Option<String>,              // if none, this is a replay
    pub comment: Option<String>,                // to allow a comment in the source JSON file
}

/// This describes the format of the grids.json file for serde deserialization.
#[derive(Debug, Clone, Deserialize)]
struct GridSelectParamsDataJson {
    pub grids: Vec<GridSelectParamsData>
}

#[derive(Debug, Clone)]
pub struct GridSelectParams {
    pub data: GridSelectParamsData,             // as read from JSON
    pub picture_bar: egui::TextureId,           // texture has been loaded and is ready to go
}

impl GridSelectParams {
    /// Read the JSON grid select params file tnto a GridSelectParams structure.
    pub fn read_grid_select_params(filename: &PathBuf) -> Result<Vec<GridSelectParams>, Error> {
        //  Read one translations file
        let file = File::open(filename)
            .with_context(|| anyhow!("Failed to open the grid select params config file: {:?}", filename))?;
        let mut reader = std::io::BufReader::new(file);
        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .context("Failed to read the grid select params config.")?;
        let grids_data: GridSelectParamsDataJson = serde_json::from_str(&content).context("Failed to parse grid select params config file.")?;
        let mut params = Vec::new();
        for grid_data in grids_data.grids {
            println!("Metaverse: {} Grid: {}", grid_data.metaverse, grid_data.grid);    // ***TEMP***
        } 
        //  ***MORE***
        Ok(params)
    }
}

/// The grid selection window.
//  Appears at startup.
//  The persistent part
pub struct GridSelectWindow {
    title: String, // title of window
    id: egui::Id,  // unique ID
    web_icon: egui::TextureId,  // icon for web button
    grids: Vec<GridSelectParams>, // available grids
}

impl GridSelectWindow {
    /// Create scrollable message window
    pub fn new(id: &str, title: &str, assets: &GuiAssets, grids: Vec<GridSelectParams>) -> Self {
        GridSelectWindow {
            id: egui::Id::new(id),
            title: title.to_string(),
            web_icon: assets.web_icon,
            grids
        }        
    }
        
    /// Draw window of text
    pub fn draw(&self, ctx: &egui::Context)-> Option<GridSelectParams> {
        let window = egui::containers::Window::new(self.title.as_str()).id(self.id)
            .anchor(egui::Align2::CENTER_TOP, egui::Vec2::ZERO)
            .auto_sized()
            .collapsible(false);
        let mut result = None;  // what, if anything, was clicked upon
        window.show(ctx, |ui| {
            //  Ref: https://docs.rs/egui/latest/egui/containers/struct.ScrollArea.html#method.show_rows
            ////let text_style = egui::TextStyle::Body;
            ////let row_height = ui.text_style_height(&text_style);
            let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
            //  Add image and website link to each row
            egui::ScrollArea::vertical().show_rows(ui, row_height, self.grids.len(), |ui, row_range| {
                ////println!("Rows: {:?} of {}, row height {}", row_range, self.grids.len(), row_height);  // ***TEMP***
                for row in row_range {
                    let grid = &self.grids[row];
                    ui.horizontal(|ui| {
                        ui.label(&grid.data.metaverse);
                        ui.label(" -- ");
                        ui.label(&grid.data.grid);
                    });
                    ui.horizontal(|ui| {
                        //  Grid select
                        if ui.add(
                            egui::widgets::ImageButton::new(
                                grid.picture_bar,
                                egui::Vec2::new(1024.0,128.0),
                                )
                            .frame(true),
                        )
                        .clicked()
                        {
                            result = Some(grid.clone());
                        }
                        //  Grid page open
                        if ui.add(
                            egui::widgets::ImageButton::new(
                                self.web_icon,
                                egui::Vec2::new(128.0,128.0),
                                )
                            .frame(true),
                        )
                        .clicked()
                        {   //  Clicking on web icon opens home web page for that grid
                            match webbrowser::open(&grid.data.home_url) {
                                Ok(_) => {},
                                Err(e) => {
                                    log::error!("Trouble trying to open web page \"{}\": {:?}", grid.data.home_url, e);
                                    //  Popup if trouble
                                    /* ***MORE*** need access to state
                                    let errmsg = format!("{:?}",e);
                                    let messages = [t!("message.web_error", state.get_lang()), errmsg.as_str()];
                                    state.add_error_window(t!("window.internet_error", params.;amg, &messages);
                                    */
                                }
                            }
                        }
                    });
                }
            });
        });
        result      // selected grid, or None
    }
}




