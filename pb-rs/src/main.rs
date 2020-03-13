#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate failure;
extern crate log;
extern crate pb_rs;

use clap::{App, Arg};
use pb_rs::types::FileDescriptor;
use pb_rs::ConfigBuilder;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

fn run() -> Result<(), ::failure::Error> {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .arg(
            Arg::with_name("OUTPUT")
                .required(false)
                .long("output")
                .short("o")
                .takes_value(true)
                .help("Generated file name, defaults to INPUT with 'rs' extension, cannot be used with --output_directory")
                .validator(|x| extension_matches(x, "rs")),
        ).arg(
            Arg::with_name("OUTPUT_DIR")
                .required(false)
                .long("output_directory")
                .short("d")
                .takes_value(true)
                .help("Output directory of generated code, cannot be used with --output"),
        ).arg(
            Arg::with_name("INCLUDE_PATH")
                .required(false)
                .long("include")
                .short("I")
                .takes_value(true)
                .help("Path to search for imported protobufs"),
        ).arg(
            Arg::with_name("SINGLE_MOD")
                .required(false)
                .long("single-mod")
                .short("s")
                .help("Omit generation of modules for each package when there is only one package"),
        ).arg(
            Arg::with_name("NO_OUTPUT")
                .required(false)
                .long("no-output")
                .short("n")
                .help(
                    "Show enums and messages in this .proto file, including those imported. \
                     No code generated",
                ),
        ).arg(
            Arg::with_name("INPUT")
                .multiple(true)
                .help("The .proto files used to generate quick-protobuf code")
                .validator(|x| extension_matches(x, "proto")),
        ).arg(
            Arg::with_name("CYCLE")
                .long("error-cycle")
                .short("e")
                .required(false)
                .help("Error out if recursive messages do not have optional fields"),
        ).arg(
            Arg::with_name("NO_HEADERS")
                .long("no-headers")
                .short("H")
                .required(false)
                .help("Do not add module comments and module attributes in generated file"),
        ).arg(
            Arg::with_name("CUSTOM_STRUCT_DERIVE")
                .long("custom_struct_derive")
                .short("C")
                .required(false)
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
                .help("The comma separated values to add to #[derive(...)] for every struct"),
        ).arg(
            Arg::with_name("CUSTOM_REPR")
                .long("custom_repr")
                .short("R")
                .required(false)
                .takes_value(true)
                .help("The value to use for the optional #[repr(...)] for every struct"),
        ).arg(
            Arg::with_name("DONT_USE_COW")
                .required(false)
                .long("dont_use_cow")
                .short("D")
                .help("Don't use Cow for String and Byte types"),
        ).arg(
            Arg::with_name("OWNED")
                .long("owned")
                .required(false)
                .help("Generate Owned structs when the proto stuct has a lifetime"),
        ).get_matches();

    let in_files = path_vec(values_t!(matches, "INPUT", String));
    let include_paths = path_vec(values_t!(matches, "INCLUDE_PATH", String));
    let out_file = matches.value_of("OUTPUT").map(|o| PathBuf::from(o));
    let out_dir = matches.value_of("OUTPUT_DIR").map(|o| PathBuf::from(o));
    let custom_repr = matches.value_of("CUSTOM_REPR").map(|o| o.into());
    let mut default_custom_struct_derive = String::new();
    let custom_struct_derive: HashMap<String, String> = matches
        .values_of("CUSTOM_STRUCT_DERIVE")
        .unwrap_or_default()
        .filter_map(|derive| {
            if let Some(i) = derive.find('=') {
                let name: String = derive[0..i].into();
                let derive = derive[i + 1..].split(',').collect::<Vec<_>>().join(", ") + ", ";
                Some((name, derive))
            } else if default_custom_struct_derive.is_empty() {
                default_custom_struct_derive =
                    derive.split(',').collect::<Vec<_>>().join(", ") + ", ";
                None
            } else {
                panic!("only one default_custom_struct_derive allowed!");
            }
        })
        .collect();

    let compiler = ConfigBuilder::new(
        &in_files,
        out_file.as_ref(),
        out_dir.as_ref(),
        &include_paths,
    )?
    .single_module(matches.is_present("SINGLE_MOD"))
    .no_output(matches.is_present("NO_OUTPUT"))
    .error_cycle(matches.is_present("CYCLE"))
    .headers(!matches.is_present("NO_HEADERS"))
    .dont_use_cow(matches.is_present("DONT_USE_COW"))
    .default_custom_struct_derive(default_custom_struct_derive)
    .custom_struct_derive(custom_struct_derive)
    .custom_repr(custom_repr)
    .owned(matches.is_present("OWNED"));

    FileDescriptor::run(&compiler.build()).map_err(|e| e.into())
}

fn extension_matches<P: AsRef<Path>>(path: P, expected: &str) -> std::result::Result<(), String> {
    match path.as_ref().extension() {
        Some(x) if x == expected => Ok(()),
        Some(x) => Err(format!(
            "Expected path with extension '{}', not: '{}'",
            expected,
            x.to_string_lossy()
        )),
        None => Err(format!("Expected path with extension '{}'", expected)),
    }
}

fn path_vec(maybe_vec: std::result::Result<Vec<String>, clap::Error>) -> Vec<PathBuf> {
    maybe_vec
        .unwrap_or_else(|_| Vec::new())
        .iter()
        .map(|s| s.into())
        .collect()
}

fn main() {
    env_logger::init();
    ::std::process::exit({
        if let Err(e) = run() {
            eprintln!("pb-rs fatal error");
            for e in e.iter_chain() {
                eprintln!("  - {}", e);
            }
            1
        } else {
            0
        }
    });
}
