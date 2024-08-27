// use std::{collections::HashSet, env, fs::{self, File}, io::{self, Read}, path::{Path, PathBuf}};

// use eframe::egui::{
//     self, color_picker::{color_edit_button_srgba, Alpha}, CentralPanel, ComboBox, Frame, Rounding, Slider, TopBottomPanel, Ui, WidgetText
// };

// use egui_dock::{
//     AllowedSplits, DockArea, DockState, NodeIndex, OverlayType, Style, SurfaceIndex,
//     TabInteractionStyle, TabViewer,
// };

// use syn::Item;
// use uengine::{log::trace, utils::fs::{path_relative_from, visit_dirs}};

// /// Adds a widget with a label next to it, can be given an extra parameter in order to show a hover text
// macro_rules! labeled_widget {
//     ($ui:expr, $x:expr, $l:expr) => {
//         $ui.horizontal(|ui| {
//             ui.add($x);
//             ui.label($l);
//         });
//     };
//     ($ui:expr, $x:expr, $l:expr, $d:expr) => {
//         $ui.horizontal(|ui| {
//             ui.add($x).on_hover_text($d);
//             ui.label($l).on_hover_text($d);
//         });
//     };
// }

// // Creates a slider which has a unit attached to it
// // When given an extra parameter it will be used as a multiplier (e.g 100.0 when working with percentages)
// macro_rules! unit_slider {
//     ($val:expr, $range:expr) => {
//         egui::Slider::new($val, $range)
//     };
//     ($val:expr, $range:expr, $unit:expr) => {
//         egui::Slider::new($val, $range).custom_formatter(|value, decimal_range| {
//             egui::emath::format_with_decimals_in_range(value, decimal_range) + $unit
//         })
//     };
//     ($val:expr, $range:expr, $unit:expr, $mul:expr) => {
//         egui::Slider::new($val, $range)
//             .custom_formatter(|value, decimal_range| {
//                 egui::emath::format_with_decimals_in_range(value * $mul, decimal_range) + $unit
//             })
//             .custom_parser(|string| string.parse::<f64>().ok().map(|valid| valid / $mul))
//     };
// }

// fn rounding_ui(ui: &mut Ui, rounding: &mut Rounding) {
//     labeled_widget!(ui, Slider::new(&mut rounding.nw, 0.0..=15.0), "North-West");
//     labeled_widget!(ui, Slider::new(&mut rounding.ne, 0.0..=15.0), "North-East");
//     labeled_widget!(ui, Slider::new(&mut rounding.sw, 0.0..=15.0), "South-West");
//     labeled_widget!(ui, Slider::new(&mut rounding.se, 0.0..=15.0), "South-East");
// }

// fn scan_for_source_files(dir: &Path, files: &mut Vec::<PathBuf>) -> io::Result<()> {
//     visit_dirs(dir, &mut |entry| {
//         let path = entry.path();
//         if let Some(result) = path.extension() {
//             if result == "rs" {
//                 files.push(path);
//             }
//         }
//     })
// }

// #[derive(Default)]
// struct Creator {
//     dir_path: PathBuf,
//     engine_src_dir_path: PathBuf,
//     resources_dir_path: PathBuf,
//     templates_dir_path: PathBuf
// }

// #[derive(Default)]
// struct Project {
//     dir_path: PathBuf,
//     src_dir_path: PathBuf,
//     cache_dir_path: PathBuf,
//     root_dir_path: PathBuf,
//     lib_path: PathBuf,
//     config_dir_path: PathBuf,
//     config_path: PathBuf,
//     cargo_path: PathBuf
// }

// struct CreatorContext {
//     pub title: String,
//     pub age: u32,
//     pub style: Option<Style>,
//     open_tabs: HashSet<String>,

//     show_close_buttons: bool,
//     show_add_buttons: bool,
//     draggable_tabs: bool,
//     show_tab_name_on_hover: bool,
//     allowed_splits: AllowedSplits,
//     show_window_close: bool,
//     show_window_collapse: bool,
// }

// impl TabViewer for CreatorContext {
//     type Tab = String;

//     fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
//         tab.as_str().into()
//     }

//     fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
//         match tab.as_str() {
//             "Scene" => self.scene(ui),
//             "Style Editor" => self.style_editor(ui),
//             _ => {}
//         }
//     }

//     fn context_menu(
//         &mut self,
//         _: &mut Ui,
//         _: &mut Self::Tab,
//         _surface: SurfaceIndex,
//         _node: NodeIndex,
//     ) {}

//     fn closeable(&mut self, _: &mut Self::Tab) -> bool {
//         true
//     }

//     fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
//         self.open_tabs.remove(tab);
//         true
//     }
// }

// impl CreatorContext {
//     fn scene(&mut self, ui: &mut Ui) {

//     }

//     fn style_editor(&mut self, ui: &mut Ui) {
//         ui.heading("Style Editor");

//         ui.collapsing("DockArea Options", |ui| {
//             ui.checkbox(&mut self.show_close_buttons, "Show close buttons");
//             ui.checkbox(&mut self.show_add_buttons, "Show add buttons");
//             ui.checkbox(&mut self.draggable_tabs, "Draggable tabs");
//             ui.checkbox(&mut self.show_tab_name_on_hover, "Show tab name on hover");
//             ui.checkbox(&mut self.show_window_close, "Show close button on windows");
//             ui.checkbox(
//                 &mut self.show_window_collapse,
//                 "Show collaspse button on windows",
//             );
//             ComboBox::new("cbox:allowed_splits", "Split direction(s)")
//                 .selected_text(format!("{:?}", self.allowed_splits))
//                 .show_ui(ui, |ui| {
//                     ui.selectable_value(&mut self.allowed_splits, AllowedSplits::All, "All");
//                     ui.selectable_value(
//                         &mut self.allowed_splits,
//                         AllowedSplits::LeftRightOnly,
//                         "LeftRightOnly",
//                     );
//                     ui.selectable_value(
//                         &mut self.allowed_splits,
//                         AllowedSplits::TopBottomOnly,
//                         "TopBottomOnly",
//                     );
//                     ui.selectable_value(&mut self.allowed_splits, AllowedSplits::None, "None");
//                 });
//         });

//         let style = self.style.as_mut().unwrap();

//         ui.collapsing("Border", |ui| {
//             egui::Grid::new("border").show(ui, |ui| {
//                 ui.label("Width:");
//                 ui.add(Slider::new(
//                     &mut style.main_surface_border_stroke.width,
//                     1.0..=50.0,
//                 ));
//                 ui.end_row();

//                 ui.label("Color:");
//                 color_edit_button_srgba(
//                     ui,
//                     &mut style.main_surface_border_stroke.color,
//                     Alpha::OnlyBlend,
//                 );
//                 ui.end_row();

//                 ui.label("Rounding:");
//                 rounding_ui(ui, &mut style.main_surface_border_rounding);
//                 ui.end_row();
//             });
//         });

//         ui.collapsing("Separator", |ui| {
//             egui::Grid::new("separator").show(ui, |ui| {
//                 ui.label("Width:");
//                 ui.add(Slider::new(&mut style.separator.width, 1.0..=50.0));
//                 ui.end_row();

//                 ui.label("Extra Interact Width:");
//                 ui.add(Slider::new(
//                     &mut style.separator.extra_interact_width,
//                     0.0..=50.0,
//                 ));
//                 ui.end_row();

//                 ui.label("Offset limit:");
//                 ui.add(Slider::new(&mut style.separator.extra, 1.0..=300.0));
//                 ui.end_row();

//                 ui.label("Idle color:");
//                 color_edit_button_srgba(ui, &mut style.separator.color_idle, Alpha::OnlyBlend);
//                 ui.end_row();

//                 ui.label("Hovered color:");
//                 color_edit_button_srgba(ui, &mut style.separator.color_hovered, Alpha::OnlyBlend);
//                 ui.end_row();

//                 ui.label("Dragged color:");
//                 color_edit_button_srgba(ui, &mut style.separator.color_dragged, Alpha::OnlyBlend);
//                 ui.end_row();
//             });
//         });

//         ui.collapsing("Tabs", |ui| {
//             ui.separator();

//             ui.checkbox(&mut style.tab_bar.fill_tab_bar, "Expand tabs");
//             ui.checkbox(
//                 &mut style.tab_bar.show_scroll_bar_on_overflow,
//                 "Show scroll bar on tab overflow",
//             );
//             ui.checkbox(
//                 &mut style.tab.hline_below_active_tab_name,
//                 "Show a line below the active tab name",
//             );
//             ui.horizontal(|ui| {
//                 ui.add(Slider::new(&mut style.tab_bar.height, 20.0..=50.0));
//                 ui.label("Tab bar height");
//             });

//             ComboBox::new("add_button_align", "Add button align")
//                 .selected_text(format!("{:?}", style.buttons.add_tab_align))
//                 .show_ui(ui, |ui| {
//                     for align in [egui_dock::TabAddAlign::Left, egui_dock::TabAddAlign::Right] {
//                         ui.selectable_value(
//                             &mut style.buttons.add_tab_align,
//                             align,
//                             format!("{:?}", align),
//                         );
//                     }
//                 });

//             ui.separator();

//             fn tab_style_editor_ui(ui: &mut Ui, tab_style: &mut TabInteractionStyle) {
//                 ui.separator();

//                 ui.label("Rounding");
//                 labeled_widget!(
//                     ui,
//                     Slider::new(&mut tab_style.rounding.nw, 0.0..=15.0),
//                     "North-West"
//                 );
//                 labeled_widget!(
//                     ui,
//                     Slider::new(&mut tab_style.rounding.ne, 0.0..=15.0),
//                     "North-East"
//                 );
//                 labeled_widget!(
//                     ui,
//                     Slider::new(&mut tab_style.rounding.sw, 0.0..=15.0),
//                     "South-West"
//                 );
//                 labeled_widget!(
//                     ui,
//                     Slider::new(&mut tab_style.rounding.se, 0.0..=15.0),
//                     "South-East"
//                 );

//                 ui.separator();

//                 egui::Grid::new("tabs_colors").show(ui, |ui| {
//                     ui.label("Title text color:");
//                     color_edit_button_srgba(ui, &mut tab_style.text_color, Alpha::OnlyBlend);
//                     ui.end_row();

//                     ui.label("Outline color:")
//                         .on_hover_text("The outline around the active tab name.");
//                     color_edit_button_srgba(ui, &mut tab_style.outline_color, Alpha::OnlyBlend);
//                     ui.end_row();

//                     ui.label("Background color:");
//                     color_edit_button_srgba(ui, &mut tab_style.bg_fill, Alpha::OnlyBlend);
//                     ui.end_row();
//                 });
//             }

//             ui.collapsing("Active", |ui| {
//                 tab_style_editor_ui(ui, &mut style.tab.active);
//             });

//             ui.collapsing("Inactive", |ui| {
//                 tab_style_editor_ui(ui, &mut style.tab.inactive);
//             });

//             ui.collapsing("Focused", |ui| {
//                 tab_style_editor_ui(ui, &mut style.tab.focused);
//             });

//             ui.collapsing("Hovered", |ui| {
//                 tab_style_editor_ui(ui, &mut style.tab.hovered);
//             });

//             ui.separator();

//             egui::Grid::new("tabs_colors").show(ui, |ui| {
//                 ui.label("Close button color unfocused:");
//                 color_edit_button_srgba(ui, &mut style.buttons.close_tab_color, Alpha::OnlyBlend);
//                 ui.end_row();

//                 ui.label("Close button color focused:");
//                 color_edit_button_srgba(
//                     ui,
//                     &mut style.buttons.close_tab_active_color,
//                     Alpha::OnlyBlend,
//                 );
//                 ui.end_row();

//                 ui.label("Close button background color:");
//                 color_edit_button_srgba(ui, &mut style.buttons.close_tab_bg_fill, Alpha::OnlyBlend);
//                 ui.end_row();

//                 ui.label("Bar background color:");
//                 color_edit_button_srgba(ui, &mut style.tab_bar.bg_fill, Alpha::OnlyBlend);
//                 ui.end_row();

//                 ui.label("Horizontal line color:").on_hover_text(
//                     "The line separating the tab name area from the tab content area",
//                 );
//                 color_edit_button_srgba(ui, &mut style.tab_bar.hline_color, Alpha::OnlyBlend);
//                 ui.end_row();
//             });
//         });

//         ui.collapsing("Tab body", |ui| {
//             ui.separator();

//             ui.label("Rounding");
//             rounding_ui(ui, &mut style.tab.tab_body.rounding);

//             ui.label("Stroke width:");
//             ui.add(Slider::new(
//                 &mut style.tab.tab_body.stroke.width,
//                 0.0..=10.0,
//             ));
//             ui.end_row();

//             egui::Grid::new("tab_body_colors").show(ui, |ui| {
//                 ui.label("Stroke color:");
//                 color_edit_button_srgba(ui, &mut style.tab.tab_body.stroke.color, Alpha::OnlyBlend);
//                 ui.end_row();

//                 ui.label("Background color:");
//                 color_edit_button_srgba(ui, &mut style.tab.tab_body.bg_fill, Alpha::OnlyBlend);
//                 ui.end_row();
//             });
//         });
//         ui.collapsing("Overlay", |ui| {
//             let selected_text = match style.overlay.overlay_type {
//                 OverlayType::HighlightedAreas => "Highlighted Areas",
//                 OverlayType::Widgets => "Widgets",
//             };
//             ui.label("Overlay Style:");
//             ComboBox::new("overlay styles", "")
//                 .selected_text(selected_text)
//                 .show_ui(ui, |ui| {
//                     ui.selectable_value(
//                         &mut style.overlay.overlay_type,
//                         OverlayType::HighlightedAreas,
//                         "Highlighted Areas",
//                     );
//                     ui.selectable_value(
//                         &mut style.overlay.overlay_type,
//                         OverlayType::Widgets,
//                         "Widgets",
//                     );
//                 });
//             ui.collapsing("Feel", |ui|{
//                 labeled_widget!(
//                     ui,
//                     unit_slider!(&mut style.overlay.feel.center_drop_coverage, 0.0..=1.0, "%", 100.0),
//                     "Center drop coverage",
//                     "how big the area where dropping a tab into the center of another should be."
//                 );
//                 labeled_widget!(
//                     ui,
//                     unit_slider!(&mut style.overlay.feel.fade_hold_time, 0.0..=4.0, "s"),
//                     "Fade hold time",
//                     "How long faded windows should hold their fade before unfading, in seconds."
//                 );
//                 labeled_widget!(
//                     ui,
//                     unit_slider!(&mut style.overlay.feel.max_preference_time, 0.0..=4.0, "s"),
//                     "Max preference time",
//                     "How long the overlay may prefer to stick to a surface despite hovering over another, in seconds."
//                 );
//                 labeled_widget!(
//                     ui,
//                     unit_slider!(&mut style.overlay.feel.window_drop_coverage, 0.0..=1.0, "%", 100.0),
//                     "Window drop coverage",
//                     "How big the area for undocking a window should be. [is overshadowed by center drop coverage]"
//                 );
//                 labeled_widget!(
//                     ui,
//                     unit_slider!(&mut style.overlay.feel.interact_expansion, 1.0..=100.0, "ps"),
//                     "Interact expansion",
//                     "How much extra interaction area should be allocated for buttons on the overlay"
//                 );
//             });

//             ui.collapsing("Visuals", |ui|{
//                 labeled_widget!(
//                     ui,
//                     unit_slider!(&mut style.overlay.max_button_size, 10.0..=500.0, "ps"),
//                     "Max button size",
//                     "The max length of a side on a overlay button in egui points"
//                 );
//                 labeled_widget!(
//                     ui,
//                     unit_slider!(&mut style.overlay.button_spacing, 0.0..=50.0, "ps"),
//                     "Button spacing",
//                     "Spacing between buttons on the overlay, in egui units."
//                 );
//                 labeled_widget!(
//                     ui,
//                     unit_slider!(&mut style.overlay.surface_fade_opacity, 0.0..=1.0, "%", 100.0),
//                     "Window fade opacity",
//                     "how visible windows are when dragging a tab behind them."
//                 );
//                 labeled_widget!(
//                     ui,
//                     egui::Slider::new(&mut style.overlay.selection_stroke_width, 0.0..=50.0),
//                     "Selection stroke width",
//                     "width of a selection which uses a outline stroke instead of filled rect."
//                 );
//                 egui::Grid::new("overlay style preferences").show(ui, |ui| {
//                     ui.label("Button color:");
//                     color_edit_button_srgba(ui, &mut style.overlay.button_color, Alpha::OnlyBlend);
//                     ui.end_row();

//                     ui.label("Button border color:");
//                     color_edit_button_srgba(ui, &mut style.overlay.button_border_stroke.color, Alpha::OnlyBlend);
//                     ui.end_row();

//                     ui.label("Selection color:");
//                     color_edit_button_srgba(ui, &mut style.overlay.selection_color, Alpha::OnlyBlend);
//                     ui.end_row();

//                     ui.label("Button stroke color:");
//                     color_edit_button_srgba(ui, &mut style.overlay.button_border_stroke.color, Alpha::OnlyBlend);
//                     ui.end_row();

//                     ui.label("Button stroke width:");
//                     ui.add(Slider::new(&mut style.overlay.button_border_stroke.width, 0.0..=50.0));
//                     ui.end_row();
//                 });
//             });

//             ui.collapsing("Hover highlight", |ui|{
//                 egui::Grid::new("leaf highlighting prefs").show(ui, |ui|{
//                     ui.label("Fill color:");
//                     color_edit_button_srgba(ui, &mut style.overlay.hovered_leaf_highlight.color, Alpha::OnlyBlend);
//                     ui.end_row();

//                     ui.label("Stroke color:");
//                     color_edit_button_srgba(ui, &mut style.overlay.hovered_leaf_highlight.stroke.color, Alpha::OnlyBlend);
//                     ui.end_row();

//                     ui.label("Stroke width:");
//                     ui.add(Slider::new(&mut style.overlay.hovered_leaf_highlight.stroke.width, 0.0..=50.0));
//                     ui.end_row();

//                     ui.label("Expansion:");
//                     ui.add(Slider::new(&mut style.overlay.hovered_leaf_highlight.expansion, -50.0..=50.0));
//                     ui.end_row();
//                 });
//                 ui.label("Rounding:");
//                 rounding_ui(ui, &mut style.overlay.hovered_leaf_highlight.rounding);
//             })
//         });
//     }
// }

// pub struct CreatorLayer {
//     context: CreatorContext,
//     tree: DockState<String>,
//     creator: Creator,
//     project: Project
// }

// impl Default for CreatorLayer {
//     fn default() -> Self {
//         let mut dock_state =
//             DockState::new(vec!["Scene".to_owned(), "Style Editor".to_owned()]);

//         dock_state.translations.tab_context_menu.eject_button = "Undock".to_owned();

//         let [_, _] = dock_state.main_surface_mut().split_below(
//             NodeIndex::root(),
//             0.7,
//             vec!["Explorer".to_owned()],
//         );

//         let [_, _] = dock_state.main_surface_mut().split_left(
//             NodeIndex::root(),
//             0.3,
//             vec!["Inspector".to_owned()],
//         );

//         let [_, _] = dock_state.main_surface_mut().split_right(
//             NodeIndex::root(),
//             0.725,
//             vec!["Hierarchy".to_owned()]
//         );

//         let mut open_tabs = HashSet::new();

//         for node in dock_state[SurfaceIndex::main()].iter() {
//             if let Some(tabs) = node.tabs() {
//                 for tab in tabs {
//                     open_tabs.insert(tab.clone());
//                 }
//             }
//         }

//         let context = CreatorContext {
//             title: "Hello".to_string(),
//             age: 24,
//             style: None,
//             open_tabs,

//             show_window_close: true,
//             show_window_collapse: true,
//             show_close_buttons: true,
//             show_add_buttons: false,
//             draggable_tabs: true,
//             show_tab_name_on_hover: false,
//             allowed_splits: AllowedSplits::default(),
//         };

//         Self {
//             context,
//             tree: dock_state,
//             creator: Default::default(),
//             project: Default::default()
//         }
//     }
// }

// impl CreatorLayer {
//     fn validate_project(&self) {
//         fs::create_dir_all(&self.project.src_dir_path).unwrap();
//         fs::create_dir_all(&self.project.cache_dir_path).unwrap();
//         fs::create_dir_all(&self.project.root_dir_path).unwrap();
//         fs::create_dir_all(&self.project.config_dir_path).unwrap();

//         // Config
//         let config_template_path = self.creator.templates_dir_path.join("project/config");
//         let binding = fs::read(&config_template_path).unwrap();
//         let config_template_text = String::from_utf8_lossy(&binding).to_string();

//         let dependency_path = self.creator.dir_path.join("deps");

//         let rlib_path = dependency_path.join("libuengine.rlib")
//             .to_str().unwrap().replace("\\", "/");

//         let dll_path = dependency_path.join("uengine.dll")
//             .to_str().unwrap().replace("\\", "/");

//         let deps = dependency_path.to_str().unwrap().replace("\\", "/");

//         let config_template_text = config_template_text.replace("{{deps}}", &deps);
//         let config_template_text = config_template_text.replace("{{rlib_path}}", &rlib_path);
//         let config_template_text = config_template_text.replace("{{dll_path}}", &dll_path);

//         fs::write(&self.project.config_path, &config_template_text).unwrap();

//         // Cargo
//         let cargo_template_path = self.creator.templates_dir_path.join("project/cargo");
//         let binding = fs::read(&cargo_template_path).unwrap();
//         let cargo_template_text = String::from_utf8_lossy(&binding).to_string();

//         let uengine_dependency = self.creator.engine_src_dir_path
//             .to_str().unwrap().replace("\\", "/");

//         let cargo_template_text = cargo_template_text.replace("{{uengine_dependency}}", &uengine_dependency);

//         fs::write(&self.project.cargo_path, cargo_template_text).unwrap();

//         // Entry
//         let mut source_files = Vec::<PathBuf>::new();
//         scan_for_source_files(&self.project.src_dir_path, &mut source_files).unwrap();

//         let mut entry_module_block_text = String::new();
//         let mut entry_bootstrap_block_text = String::new();
//         for path in &source_files {
//             let pure_path = path.with_extension("");
//             let relative = path_relative_from(&path, &self.project.root_dir_path).unwrap();
//             let relative_str = relative.as_os_str().to_str().unwrap().replace("\\", "/");
//             entry_module_block_text += (format!("#[path=\"{}\"]\n", relative_str)).as_str();

//             let name = path_relative_from(&pure_path, &self.project.dir_path).unwrap();
//             let name = name.to_str().unwrap().replace("\\", "/").replace("/", "_");
//             entry_module_block_text += (format!("mod {};\n", name)).as_str();

//             let mut file = File::open(&path).expect("unable to open file");
//             let mut src = String::new();
//             file.read_to_string(&mut src).expect("unable to read file");

//             let syntax = syn::parse_file(&src).expect("unable to parse file");

//             let mut system_name = String::new();

//             for item in &syntax.items {
//                 if let Item::Struct(item_struct) = item {
//                     for attr in &item_struct.attrs {
//                         if attr.path().is_ident("system") {
//                             system_name += name.as_str();
//                             system_name += "::";
//                             system_name += item_struct.ident.to_string().as_str();
//                         }
//                     }
//                 }
//             }

//             if !system_name.is_empty() {
//                 entry_bootstrap_block_text += (format!("_world.register_system::<{}>().unwrap();\n", system_name)).as_str();
//             }
//         }

//         entry_module_block_text.pop();

//         let entry_template_path = self.creator.templates_dir_path.join("project/entry");
//         let binding = fs::read(&entry_template_path).unwrap();
//         let entry_template_text = String::from_utf8_lossy(&binding).to_string();

//         let entry_template_text = entry_template_text.replace("{{modules}}", &entry_module_block_text);
//         let entry_template_text = entry_template_text.replace("{{bootstrap}}", &entry_bootstrap_block_text);

//         fs::write(&self.project.lib_path, entry_template_text).unwrap();
//     }

//     pub fn on_create(&mut self, ctx: &eframe::CreationContext) {


//         let binding = env::current_exe().unwrap();
//         self.creator.dir_path = binding.parent().unwrap().into();
//         trace!("UCreator dir: {:?}", &self.creator.dir_path);

//         self.creator.engine_src_dir_path = self.creator.dir_path.join("src/engine");
//         self.creator.resources_dir_path = self.creator.dir_path.join("resources");
//         self.creator.templates_dir_path = self.creator.resources_dir_path.join("templates");

//         // TODO: Select project.
//         self.project.dir_path = Path::new(r"C:\Development\UCreatorExample").into();
//         trace!("Project dir: {:?}", &self.project.dir_path);

//         self.project.src_dir_path = self.project.dir_path.join("src");
//         self.project.cache_dir_path = self.project.dir_path.join(".ucreator");
//         self.project.root_dir_path = self.project.cache_dir_path.join("src");
//         self.project.lib_path = self.project.root_dir_path.join("lib.rs");
//         self.project.cargo_path = self.project.dir_path.join("Cargo.toml");
//         self.project.config_dir_path = self.project.dir_path.join(".cargo");
//         self.project.config_path = self.project.config_dir_path.join("config.toml");

//         self.validate_project();
//     }
// }

// impl eframe::App for CreatorLayer {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         TopBottomPanel::top("egui_dock::MenuBar").show(ctx, |ui| {
//             egui::menu::bar(ui, |ui| {
//                 ui.menu_button("View", |ui| {
//                     // allow certain tabs to be toggled
//                     for tab in &["Explorer", "Scene", "Hierarchy", "Inspector"] {
//                         if ui
//                             .selectable_label(self.context.open_tabs.contains(*tab), *tab)
//                             .clicked()
//                         {
//                             if let Some(index) = self.tree.find_tab(&tab.to_string()) {
//                                 self.tree.remove_tab(index);
//                                 self.context.open_tabs.remove(*tab);
//                             } else {
//                                 self.tree[SurfaceIndex::main()]
//                                     .push_to_focused_leaf(tab.to_string());
//                                 self.context.open_tabs.insert(tab.to_string());
//                             }

//                             ui.close_menu();
//                         }
//                     }
//                 });
//             })
//         });

//         CentralPanel::default()
//             .frame(Frame::central_panel(&ctx.style()).inner_margin(0.))
//             .show(ctx, |ui| {
//                 let style = self
//                     .context
//                     .style
//                     .get_or_insert(Style::from_egui(ui.style()))
//                     .clone();

//                 DockArea::new(&mut self.tree)
//                     .style(style)
//                     .show_close_buttons(self.context.show_close_buttons)
//                     .show_add_buttons(self.context.show_add_buttons)
//                     .draggable_tabs(self.context.draggable_tabs)
//                     .show_tab_name_on_hover(self.context.show_tab_name_on_hover)
//                     .allowed_splits(self.context.allowed_splits)
//                     .show_window_close_buttons(self.context.show_window_close)
//                     .show_window_collapse_buttons(self.context.show_window_collapse)
//                     .show_inside(ui, &mut self.context);
//             });
//     }
// }
