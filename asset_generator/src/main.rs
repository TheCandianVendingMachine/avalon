use inquire;
use avalon_asset::packed;
use avalon_asset::bundle;
use avalon_asset::{ shader, texture, text };
use avalon_asset::asset::{ Type, Unit, Metadata };
use anyhow::Result;

trait Operation {
    fn name(&self) -> &str;
    fn execute(&mut self) -> Result<()>;
}

struct CreateBundle {

}

impl CreateBundle {
    fn add_asset(&self) -> Result<Option<Metadata>> {
        println!("Bundle Asset");

        let path_validator = |path: &str| {
            let path = std::path::Path::new(&path);
            if !path.exists() {
                return Ok(inquire::validator::Validation::Invalid(
                    format!("Provided filepath [{}] does not exist", path.as_os_str().to_string_lossy()).into()
                ));
            }
            let path = std::fs::canonicalize(path).unwrap();
            if !path.is_file() {
                return Ok(inquire::validator::Validation::Invalid(
                    format!("Provided path [{}] needs to be a file", path.as_os_str().to_string_lossy()).into()
                ));
            }

            return Ok(inquire::validator::Validation::Valid);
        };

        let path = inquire::Text::new("Asset filepath:")
            .with_help_message("The filepath of the asset, relative to the current working directory")
            .with_validator(path_validator)
            .prompt()
            .map(|path_str| {
                let path = std::path::Path::new(&path_str);
                std::fs::canonicalize(path).unwrap()
            })
            .unwrap();

        let tag = inquire::Text::new("Tag:")
            .with_help_message("A human readable tag that is associated with the asset (not-unique)")
            .prompt()?;

        let file_type = Type::from_path(&path);
        let file_type = if let None = file_type {
            println!("Could not determine filetype from path");
            inquire::Select::new("File type:", vec![
                Type::Texture,
                Type::Shader,
                Type::Model,
                Type::Text,
            ])
                .with_help_message("Manually select filetype from options")
                .prompt()?
        } else {
            file_type.unwrap()
        };

        let unit = match file_type {
            Type::Shader => {
                let stage = inquire::Select::new("Shader Stage:", vec![
                    shader::Stage::Compute,
                    shader::Stage::Fragment,
                    shader::Stage::Vertex,
                ])
                    .with_help_message("The stage which the shader will execute")
                    .prompt()?;
                Unit::Shader(shader::Shader {
                    stage
                })
            },
            Type::Texture => {
                let colour_space = inquire::Select::new("Colour Space:", vec![
                    texture::ColourSpace::RGBA,
                    texture::ColourSpace::SRGBA,
                    texture::ColourSpace::RGB,
                    texture::ColourSpace::SRGB,
                ])
                    .with_help_message("The colour space of the texture")
                    .prompt()?;
                Unit::Texture(texture::Texture {
                    colour_space
                })
            },
            Type::Text => {
                let language = inquire::Select::new("Language:", vec![
                    text::Language::English
                ])
                    .with_help_message("The language the text is written in")
                    .prompt()?;
                Unit::Text(text::Text {
                    language
                })
            },
            Type::Model => {
                Unit::Model
            },
        };

        let valid = inquire::Confirm::new("Is the above configuration correct?")
            .with_default(true)
            .prompt()?;

        if !valid {
            return Ok(None);
        }

        return Ok(Some(Metadata::new(tag, path, unit)));
    }
}

impl Operation for CreateBundle {
    fn name(&self) -> &str {
        "Create New Bundle"
    }

    fn execute(&mut self) -> Result<()> {
        let bundle_tag = inquire::Text::new("Bundle Name:")
            .with_help_message("The human-readable tag/name for the bundle")
            .prompt()?;

        let mut bundle = bundle::Bundle {
            name: bundle_tag,
            group: Vec::new()
        };

        let mut continue_adding = true;
        while continue_adding {
            let metadata_result = self.add_asset();
            if let Ok(metadata) = metadata_result {
                if let Some(metadata) = metadata {
                    bundle.group.push(metadata.into());
                }
            } else {
                println!("Error while adding asset to bundle: {}", metadata_result.err().unwrap());
            }
            continue_adding = inquire::Confirm::new("Add more assets?")
                .with_default(false)
                .prompt()?;
        }

        let start_time = std::time::Instant::now();

        let bundle_len = bundle.group.len();
        let pack_dir = std::env::current_dir()?;

        println!("Serializing bundle \"{}\" to {}",
            bundle.name,
            pack_dir.join(format!("{}.{}", bundle.name, packed::Packed::EXTENSION)).to_str().unwrap()
        );

        let packed = packed::Packed::from(bundle);
        packed.pack_to_file(&pack_dir)?;

        println!(
            "Packed {} assets to {} in {:.3} seconds",
            bundle_len,
            pack_dir.to_str().unwrap(),
            start_time.elapsed().as_secs_f32()
        );

        Ok(())
    }
}

impl Default for CreateBundle {
    fn default() -> CreateBundle {
        CreateBundle {

        }
    }
}

struct Op {
    operation: Box<dyn Operation>
}

impl Op {
    fn new<T: Operation + Default + 'static>() -> Op {
        Op {
            operation: Box::new(T::default())
        }
    }

    fn execute(&mut self) -> Result<()> {
        self.operation.execute()
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.operation.name().fmt(formatter)
    }
}

fn main() {
    clearscreen::clear().unwrap();
    let mut operation = inquire::Select::new("Which operation?", vec![
        Op::new::<CreateBundle>()
    ]).prompt().unwrap();
    let result = operation.execute();
    if let Err(e) = result {
        println!("An error occured:");
        println!("{}", e);
    }
}
