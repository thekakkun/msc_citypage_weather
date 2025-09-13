// use forecast_full::gen_forecast_full;
use general::gen_general;
use site_list::gen_site_list;
use utils::rustfmt_pretty_print;
use weather::gen_weather;
use xsd_parser::Error;

// mod forecast_full;
mod general;
mod site_list;
mod utils;
mod weather;

fn main() -> Result<(), Error> {
    gen_site_list()?;
    // gen_general()?;
    // gen_weather()?;
    // gen_forecast_full()?;

    // let mut config = Config::default();
    // config.parser.schemas = vec![
    //     Schema::File("schema_files/forecastFull.xsd".into()),
    //     Schema::File("schema_files/general.xsd".into()),
    //     Schema::File("schema_files/site.xsd".into()),
    //     Schema::File("schema_files/weather.xsd".into()),
    // ];
    // config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
    // config.interpreter.types = vec![
    //     (
    //         IdentTriple::from((IdentType::Type, "timeStampType")),
    //         MetaType::from(
    //             CustomMeta::new("TimeStampType").include_from("crate::schemas::TimeStampType"),
    //         ),
    //     ),
    //     (
    //         IdentTriple::from((IdentType::Type, "dateTimeUTCType")),
    //         MetaType::from(
    //             CustomMeta::new("DateTimeUtcType").include_from("crate::schemas::DateTimeUtcType"),
    //         ),
    //     ),
    // ];
    // config.optimizer.flags = OptimizerFlags::all()
    //     - OptimizerFlags::REMOVE_EMPTY_ENUM_VARIANTS
    //     - OptimizerFlags::REMOVE_DUPLICATES;
    // config.generator.flags = GeneratorFlags::all() - GeneratorFlags::MIXED_TYPE_SUPPORT;
    //
    // let config = config.with_render_steps([
    //     RenderStep::Types,
    //     RenderStep::Defaults,
    //     RenderStep::NamespaceConstants,
    //     RenderStep::QuickXmlDeserialize {
    //         boxed_deserializer: false,
    //     },
    // ]);
    //
    // let schemas = exec_parser(config.parser)?;
    // let meta_types = exec_interpreter(config.interpreter, &schemas)?;
    // let meta_types = replace_variant_names(meta_types);
    // let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    // let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    // let module = exec_render(config.renderer, &data_types)?;
    // let code = module.to_token_stream().to_string();
    //
    // let code = rustfmt_pretty_print(code).unwrap();
    //
    // let out_dir = env::var_os("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join("generated.rs");
    //
    // let mut file = File::create(dest_path)?;
    // file.write_all(code.to_string().as_bytes())?;

    Ok(())
}
