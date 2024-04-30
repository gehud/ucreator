use std::{env, fs::{self, File}, io::{self, Read}, path::{Path, PathBuf}, process::Command};

use syn::{Item, Meta, UseTree};
use uengine::{log::trace, utils::fs::{path_relative_from, visit_dirs}, Layer};

fn scan_for_source_files(dir: &Path, files: &mut Vec::<PathBuf>) -> io::Result<()> {
    visit_dirs(dir, &mut |entry| {
        let path = entry.path();
        if let Some(result) = path.extension() {
            if result == "rs" {
                files.push(path);
            }
        }
    })
}

fn grow_group_import(use_tree: &UseTree, result: &mut String, group: &String) {
    match use_tree {
        UseTree::Path(use_path) => {
            result.push_str(use_path.ident.to_string().as_str());
            result.push_str("::");
            grow_group_import(&use_path.tree, result, group);
        },
        UseTree::Name(name_path) => {
            result.push_str(name_path.ident.to_string().as_str());
        },
        UseTree::Rename(use_rename) => {
            result.push_str(use_rename.ident.to_string().as_str());
            result.push_str(" as ");
            result.push_str(use_rename.rename.to_string().as_str());
        },
        UseTree::Glob(_) => {
            result.push_str("*");
        },
        UseTree::Group(use_group) => {
            result.push_str("{");

            for item in &use_group.items {
                grow_group_import(item, result, group);
                result.push_str(", ");
            }

            result.pop();
            result.pop();

            result.push_str("}");
        },
    }
}

#[derive(Default)]
struct Creator {
    dir_path: PathBuf,
    engine_src_dir_path: PathBuf,
    resources_dir_path: PathBuf,
    templates_dir_path: PathBuf
}

#[derive(Default)]
struct Project {
    dir_path: PathBuf,
    src_dir_path: PathBuf,
    cache_dir_path: PathBuf,
    root_dir_path: PathBuf,
    lib_path: PathBuf,
    config_dir_path: PathBuf,
    config_path: PathBuf,
    cargo_path: PathBuf
}

#[derive(Default)]
pub struct CreatorLayer {
    creator: Creator,
    project: Project
}

impl CreatorLayer {
    fn validate_project(&self) {
        fs::create_dir_all(&self.project.src_dir_path).unwrap();
        fs::create_dir_all(&self.project.cache_dir_path).unwrap();
        fs::create_dir_all(&self.project.root_dir_path).unwrap();
        fs::create_dir_all(&self.project.config_dir_path).unwrap();

        // Config
        let config_template_path = self.creator.templates_dir_path.join("project/config");
        let binding = fs::read(&config_template_path).unwrap();
        let config_template_text = String::from_utf8_lossy(&binding).to_string();

        let dependency_path = self.creator.dir_path.join("deps");

        let rlib_path = dependency_path.join("libuengine.rlib")
            .to_str().unwrap().replace("\\", "/");

        let dll_path = dependency_path.join("uengine.dll")
            .to_str().unwrap().replace("\\", "/");

        let deps = dependency_path.to_str().unwrap().replace("\\", "/");

        let config_template_text = config_template_text.replace("{{deps}}", &deps);
        let config_template_text = config_template_text.replace("{{rlib_path}}", &rlib_path);
        let config_template_text = config_template_text.replace("{{dll_path}}", &dll_path);

        fs::write(&self.project.config_path, &config_template_text).unwrap();

        // Cargo
        let cargo_template_path = self.creator.templates_dir_path.join("project/cargo");
        let binding = fs::read(&cargo_template_path).unwrap();
        let cargo_template_text = String::from_utf8_lossy(&binding).to_string();

        let uengine_dependency = self.creator.engine_src_dir_path
            .to_str().unwrap().replace("\\", "/");

        let cargo_template_text = cargo_template_text.replace("{{uengine_dependency}}", &uengine_dependency);

        fs::write(&self.project.cargo_path, cargo_template_text).unwrap();

        // Entry
        let mut source_files = Vec::<PathBuf>::new();
        scan_for_source_files(&self.project.src_dir_path, &mut source_files).unwrap();

        let mut entry_module_block_text = String::new();
        for path in &source_files {
            let pure_path = path.with_extension("");
            let name = pure_path.file_name().unwrap().to_str().unwrap();
            let relative = path_relative_from(&path, &self.project.root_dir_path).unwrap();
            let relative_str = relative.as_os_str().to_str().unwrap().replace("\\", "/");
            entry_module_block_text += (format!("#[path=\"{}\"]\n", relative_str)).as_str();
            entry_module_block_text += (format!("mod {};\n", name)).as_str();
        }

        entry_module_block_text.pop();

        let entry_template_path = self.creator.templates_dir_path.join("project/entry");
        let binding = fs::read(&entry_template_path).unwrap();
        let entry_template_text = String::from_utf8_lossy(&binding).to_string();

        let entry_template_text = entry_template_text.replace("{{modules}}", &entry_module_block_text);

        fs::write(&self.project.lib_path, entry_template_text).unwrap();
    }

    fn check_project(&self) {
        self.validate_project();

        Command::new("cargo")
            .args(["-Z", "unstable-options", "-C", self.project.dir_path.to_str().unwrap(), "fix", "--broken-code", "--allow-no-vcs", "--allow-dirty"])
            .spawn()
            .unwrap();
    }

    fn build_project(&self) {
        self.check_project();

        let mut source_files = Vec::<PathBuf>::new();
        scan_for_source_files(&self.project.src_dir_path, &mut source_files).unwrap();

        let mut entry_registration_block_text = String::new();
        for path in &source_files {
            let pure_path = path.with_extension("");
            let name = pure_path.file_name().unwrap().to_str().unwrap();

            let mut file = File::open(&path).expect("unable to open file");
            let mut src = String::new();
            file.read_to_string(&mut src).expect("unable to read file");
            let syntax = syn::parse_file(&src).expect("unable to parse file");
            let mut group = String::from("uengine::ecs::Simulation");
            let mut system_name = String::new();

            for item in &syntax.items {
                match item {
                    Item::Const(_) => {},
                    Item::Enum(_) => {},
                    Item::ExternCrate(_) => {},
                    Item::Fn(_) => {},
                    Item::ForeignMod(_) => {},
                    Item::Impl(_) => {},
                    Item::Macro(_) => {},
                    Item::Mod(_) => {},
                    Item::Static(_) => {},
                    Item::Struct(item_struct) => {
                        for attr in &item_struct.attrs {
                            if attr.path().is_ident("group") {
                                if let Meta::List(meta_list) = &attr.meta {
                                    let group_path = meta_list.parse_args::<syn::Path>().expect("wrong argument");
                                    group = String::new();
                                    for segment in group_path.segments {
                                        group += segment.ident.to_string().as_str();
                                        group += "::";
                                    }

                                    group.pop();
                                    group.pop();
                                }
                            } else if attr.path().is_ident("system") {
                                system_name += name;
                                system_name += "::";
                                system_name += item_struct.ident.to_string().as_str();
                            }
                        }
                    },
                    Item::Trait(_) => {},
                    Item::TraitAlias(_) => {},
                    Item::Type(_) => {},
                    Item::Union(_) => {},
                    Item::Use(item_use) => {
                        let mut result = String::new();
                        result.push_str("use ");
                        grow_group_import(&item_use.tree, &mut result, &group);
                        result.push_str(";");
                    },
                    Item::Verbatim(_) => {},
                    _ => {},
                };
            }

            if !system_name.is_empty() {
                entry_registration_block_text += (format!("_world.register_system::<{}, {}>().unwrap();\n", group, system_name)).as_str();
            }
        }
    }
}


impl Layer for CreatorLayer {
    fn on_create(&mut self, _: &mut uengine::App) {
        let binding = env::current_exe().unwrap();
        self.creator.dir_path = binding.parent().unwrap().into();
        trace!("UCreator dir: {:?}", &self.creator.dir_path);

        self.creator.engine_src_dir_path = self.creator.dir_path.join("src/engine");
        self.creator.resources_dir_path = self.creator.dir_path.join("resources");
        self.creator.templates_dir_path = self.creator.resources_dir_path.join("templates");

        // TODO: Select project.
        self.project.dir_path = Path::new(r"C:\Development\UCreatorExample").into();
        trace!("Project dir: {:?}", &self.project.dir_path);

        self.project.src_dir_path = self.project.dir_path.join("src");
        self.project.cache_dir_path = self.project.dir_path.join(".ucreator");
        self.project.root_dir_path = self.project.cache_dir_path.join("src");
        self.project.lib_path = self.project.root_dir_path.join("lib.rs");
        self.project.cargo_path = self.project.dir_path.join("Cargo.toml");
        self.project.config_dir_path = self.project.dir_path.join(".cargo");
        self.project.config_path = self.project.config_dir_path.join("config.toml");

        self.build_project();
    }

    fn on_update(&mut self, _: &mut uengine::App) {

    }
}
