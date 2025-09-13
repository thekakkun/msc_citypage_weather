use crate::rustfmt_pretty_print;
use quote::ToTokens;
use std::{env, fs::File, io::Write, path::Path};
use xsd_parser::{
    Config, Error, IdentType, MetaTypes,
    config::{
        GeneratorFlags, IdentTriple, InterpreterFlags, MetaType, OptimizerFlags, ParserFlags,
        RenderStep, Schema,
    },
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::meta::{CustomMeta, MetaTypeVariant},
};

pub(crate) fn gen_weather() -> Result<(), Error> {
    let mut config = Config::default();
    config.parser.schemas = vec![Schema::File("schema_files/weather.xsd".into())];
    config.parser.flags = ParserFlags::DEFAULT_NAMESPACES;
    config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
    config.interpreter.types = vec![
        (
            IdentTriple::from((IdentType::Type, "dateStampType")),
            MetaType::from(
                CustomMeta::new("DateStampType")
                    .include_from("crate::models::general::DateStampType"),
            ),
        ),
        (
            IdentTriple::from((IdentType::Type, "validDayNames")),
            MetaType::from(
                CustomMeta::new("ValidDayNamesType")
                    .include_from("crate::models::general::ValidDayNamesType"),
            ),
        ),
    ];
    config.optimizer.flags = OptimizerFlags::all()
        - OptimizerFlags::REMOVE_EMPTY_ENUM_VARIANTS
        - OptimizerFlags::REMOVE_DUPLICATES;
    config.generator.flags = GeneratorFlags::all() - GeneratorFlags::MIXED_TYPE_SUPPORT;

    let config = config.with_render_steps([
        RenderStep::Types,
        RenderStep::Defaults,
        RenderStep::NamespaceConstants,
        RenderStep::QuickXmlDeserialize {
            boxed_deserializer: false,
        },
    ]);

    let schemas = exec_parser(config.parser)?;
    let meta_types = exec_interpreter(config.interpreter, &schemas)?;
    let meta_types = replace_variant_names(meta_types);
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let module = exec_render(config.renderer, &data_types)?;

    let code = module.to_token_stream().to_string();
    let code = rustfmt_pretty_print(code).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("weather.rs");

    let mut file = File::create(dest_path)?;
    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}

fn replace_variant_names(mut types: MetaTypes) -> MetaTypes {
    for (_ident, ty) in types.items.iter_mut() {
        if let MetaTypeVariant::Enumeration(enum_meta) = &mut ty.variant {
            for variant in enum_meta.variants.iter_mut() {
                if let "%" = variant.ident.name.as_str() {
                    variant.display_name = Some("Percent".to_string());
                }
            }
        }
    }

    types
}
