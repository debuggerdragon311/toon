use crate::codec::{compact, tabular, text};
use crate::EncodeOptions;
use anyhow::{Context, Result};
use serde_json::Value;

pub fn encode(input: &Value, opt: &EncodeOptions) -> Result<Vec<u8>> {
    // Check if we should use tabular mode
    if opt.tabular_arrays {
        if let Some(result) = try_tabular_encode(input, opt)? {
            return Ok(result);
        }
        // Fall through to regular encoding if tabular doesn't apply
    }

    if opt.compact {
        compact::encode(input).context("Failed to encode in compact mode")
    } else {
        text::encode(input, opt.indent.unwrap_or(2))
            .context("Failed to encode in text mode")
    }
}

fn try_tabular_encode(input: &Value, opt: &EncodeOptions) -> Result<Option<Vec<u8>>> {
    match input {
        Value::Array(arr) => {
            if tabular::is_uniform_object_array(arr) {
                let result = if opt.compact {
                    tabular::encode_tabular_compact(arr)
                } else {
                    tabular::encode_tabular_text(arr, opt.indent.unwrap_or(2))
                }?;
                Ok(Some(result))
            } else if opt.strict {
                anyhow::bail!(
                    "Tabular mode requires uniform array of objects, but array has mixed types"
                );
            } else {
                Ok(None)
            }
        }
        Value::Object(obj) => {
            // Check if any field contains a uniform array
            for (_, v) in obj.iter() {
                if let Value::Array(arr) = v {
                    if !tabular::is_uniform_object_array(arr) && opt.strict {
                        anyhow::bail!(
                            "Tabular mode requires uniform arrays of objects"
                        );
                    }
                }
            }
            Ok(None)
        }
        _ => {
            if opt.strict {
                anyhow::bail!("Tabular mode only applies to arrays of objects");
            }
            Ok(None)
        }
    }
}
