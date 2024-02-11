use std::io::Write;
use std::{env, io};

use env_logger::fmt::{Color, Formatter};
use lit_core::logging::kv::FieldCollectorKVVisitor;
use log::Record;
use serde_json::{Map, Value};

use lit_logging::{colored_level, fields_to_json, Padded};

pub const ENV_LOG_INIT_STAGE: &str = "LOG_INIT_STAGE";
pub const ENV_LOG_INIT_SUB_STAGE: &str = "LOG_INIT_SUB_STAGE";

pub struct LogFormatter;

impl LogFormatter {
    pub fn format(prefix: &str, buf: &mut Formatter, record: &Record) -> io::Result<()> {
        let metadata = record.metadata();
        let target = metadata.target();

        let mut style = buf.style();
        let level_style = colored_level(&mut style, record.level());
        let args_style = buf.style().set_color(Color::White).set_intense(true).clone();
        let target_style = buf.style().set_color(Color::Cyan).set_intense(true).clone();

        let prefix_style = buf.style().set_color(Color::Yellow).set_intense(true).clone();
        let stage_style = buf.style().set_color(Color::Magenta).set_intense(true).clone();
        let sub_stage_style = buf.style().set_color(Color::Blue).set_intense(false).clone();

        match env::var(ENV_LOG_INIT_STAGE) {
            Ok(stage) => {
                if !stage.is_empty() {
                    write!(
                        buf,
                        "[{} > {}]",
                        prefix_style.value(prefix),
                        stage_style.value(Padded { value: stage, width: 21 - (prefix.len() + 3) })
                    )?;
                } else {
                    write!(buf, "[{:<21}]", prefix_style.value(prefix))?;
                }
            }
            _ => write!(buf, "[{:<21}]", prefix_style.value(prefix))?,
        };

        write!(buf, " {} ({})", level_style, target_style.value(target))?;

        match env::var(ENV_LOG_INIT_SUB_STAGE) {
            Ok(sub) => {
                write!(
                    buf,
                    " [{}] {}",
                    sub_stage_style.value(sub),
                    args_style.value(record.args())
                )?;
            }
            _ => {
                write!(buf, " {}", args_style.value(record.args()))?;
            }
        };

        let mut fields: Map<String, Value> = Map::new();

        let kvs = record.key_values();
        if kvs.count() > 0 {
            kvs.visit(&mut FieldCollectorKVVisitor(&mut fields))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            if !fields.is_empty() {
                let mut fields_style = buf.style();
                fields_style.set_dimmed(true);

                write!(buf, "{}", fields_style.value(fields_to_json(&fields)))?;
            }
        }

        writeln!(buf)
    }
}
