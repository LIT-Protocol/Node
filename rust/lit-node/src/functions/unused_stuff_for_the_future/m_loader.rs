// uncomment this if you want to build your own module loader.
/*

use data_url::DataUrl;
use deno_core::error::type_error;
use deno_core::error::uri_error;
use deno_core::error::AnyError;
use deno_core::ModuleLoader;
use deno_core::ModuleSpecifier;
use deno_graph::source::Resolver;
use encoding_rs::*;
use futures::FutureExt;
use std::pin::Pin;
use std::{
    borrow::Cow,
    io::{Error, ErrorKind},
};

pub fn convert_to_utf8<'a>(bytes: &'a [u8], charset: &'_ str) -> Result<Cow<'a, str>, Error> {
    match Encoding::for_label(charset.as_bytes()) {
        Some(encoding) => encoding
            .decode_without_bom_handling_and_without_replacement(bytes)
            .ok_or_else(|| ErrorKind::InvalidData.into()),
        None => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Unsupported charset: {}", charset),
        )),
    }
}

// Given a vector of bytes and optionally a charset, decode the bytes to a
/// string.
pub fn get_source_from_bytes(
    bytes: Vec<u8>,
    maybe_charset: Option<String>,
) -> Result<String, AnyError> {
    let source = if let Some(charset) = maybe_charset {
        convert_to_utf8(&bytes, &charset)?.to_string()
    } else {
        String::from_utf8(bytes)?
    };

    Ok(source)
}

pub fn get_source_from_data_url(specifier: &ModuleSpecifier) -> Result<(String, String), AnyError> {
    let data_url =
        DataUrl::process(specifier.as_str()).map_err(|e| uri_error(format!("{:?}", e)))?;
    let mime = data_url.mime_type();
    let charset = mime.get_parameter("charset").map(|v| v.to_string());
    let (bytes, _) = data_url
        .decode_to_vec()
        .map_err(|e| uri_error(format!("{:?}", e)))?;
    Ok((get_source_from_bytes(bytes, charset)?, format!("{}", mime)))
}

pub struct EmbeddedModuleLoader {
    pub eszip: eszip::EszipV2,
    pub maybe_import_map_resolver: Option<crate::functions::resolver::ImportMapResolver>,
}

impl ModuleLoader for EmbeddedModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _is_main: bool,
    ) -> Result<ModuleSpecifier, AnyError> {
        // Try to follow redirects when resolving.
        let referrer = match self.eszip.get_module(referrer) {
            Some(eszip::Module { ref specifier, .. }) => deno_core::resolve_url_or_path(specifier)?,
            None => deno_core::resolve_url_or_path(referrer)?,
        };

        self.maybe_import_map_resolver.as_ref().map_or_else(
            || deno_core::resolve_import(specifier, referrer.as_str()).map_err(|err| err.into()),
            |r| r.resolve(specifier, &referrer).to_result(),
        )
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<ModuleSpecifier>,
        _is_dynamic: bool,
    ) -> Pin<Box<deno_core::ModuleSourceFuture>> {
        let module_specifier = module_specifier.clone();

        let is_data_uri = get_source_from_data_url(&module_specifier).ok();
        let module = self
            .eszip
            .get_module(module_specifier.as_str())
            .ok_or_else(|| type_error("Module not found"));

        async move {
            if let Some((source, _)) = is_data_uri {
                return Ok(deno_core::ModuleSource {
                    code: source.into_bytes().into_boxed_slice(),
                    module_type: deno_core::ModuleType::JavaScript,
                    module_url_specified: module_specifier.to_string(),
                    module_url_found: module_specifier.to_string(),
                });
            }

            let module = module?;
            let code = module.source().await;
            let code = std::str::from_utf8(&code)
                .map_err(|_| type_error("Module source is not utf-8"))?
                .to_owned();

            Ok(deno_core::ModuleSource {
                code: code.into_bytes().into_boxed_slice(),
                module_type: match module.kind {
                    eszip::ModuleKind::JavaScript => deno_core::ModuleType::JavaScript,
                    eszip::ModuleKind::Json => deno_core::ModuleType::Json,
                },
                module_url_specified: module_specifier.to_string(),
                module_url_found: module_specifier.to_string(),
            })
        }
        .boxed_local()
    }
}

*/
