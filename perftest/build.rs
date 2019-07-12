extern crate pb_rs;
extern crate prost_build;
extern crate protobuf_codegen_pure;

use pb_rs::types::{Config, FileDescriptor, RpcService};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn generate_rpc_test<W: Write + ?Sized>(
    rpc: &RpcService,
    w: &mut W,
) -> Result<(), pb_rs::errors::Error> {
    /* Example:
        trait <service> {
            fn <func>(&self, arg: &<arg>) -> Result<<ret>, failure::Error>;
        }
    */

    writeln!(w, "\npub trait {SERVICE} {{", SERVICE = rpc.service_name)?;
    for func in rpc.functions.iter() {
        writeln!(
            w,
            "   fn {FUNC}(&self, arg: &{ARG}) -> std::result::Result<{RET}, failure::Error>;",
            FUNC = func.name,
            ARG = func.arg,
            RET = func.ret
        )?;
    }
    writeln!(w, "}}\n")?;

    Ok(())
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // protobuf
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src",
        input: &["src/perftest_data.proto"],
        includes: &["src"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    })
    .expect("protoc");

    // quick-protobuf
    let quick_dest = Path::new(&out_dir).join("perftest_data_quick.rs");
    let config = Config {
        in_file: PathBuf::from("src/perftest_data.proto"),
        out_file: quick_dest,
        single_module: true,
        import_search_path: vec![PathBuf::from("src")],
        no_output: false,
        error_cycle: false,
        headers: false,
        dont_use_cow: false,
        custom_struct_derive: vec![],
        custom_rpc_generator: Box::new(|rpc, writer| generate_rpc_test(rpc, writer)),
        custom_includes: Vec::new(),
        owned: false,
    };
    FileDescriptor::write_proto(&config).unwrap();

    // prost
    let old = ::std::env::var("OUT_DIR");
    env::set_var("OUT_DIR", ".");
    prost_build::compile_protos(&["src/perftest_data.proto"], &["src"]).unwrap();
    let _ = old.map(|val| env::set_var("OUT_DIR", &val));
    fs::rename("perftest_data.rs", "src/perftest_data_prost.rs").unwrap();
}
