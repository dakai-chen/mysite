use std::collections::HashMap;

use bytesize::ByteSize;
use human_format::Formatter;
use tera::{Error, Value};

pub fn human_number(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let Some(value) = value.as_f64() else {
        return Err(Error::msg(format!("invalid value: {value}, expected f64")));
    };

    let decimals = args
        .get("decimals")
        .map(|v| v.as_u64().ok_or_else(|| format!("decimals: invalid value")))
        .transpose()?
        .unwrap_or(2);

    if value < 1000. {
        Ok(format!("{value:.0}").into())
    } else {
        Ok(Formatter::new()
            .with_decimals(usize::try_from(decimals).map_err(|e| format!("{e}"))?)
            .with_separator("")
            .format(value)
            .into())
    }
}

pub fn human_size(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let Some(value) = value.as_u64() else {
        return Err(Error::msg(format!("invalid value: {value}, expected u64")));
    };
    Ok(ByteSize::b(value).to_string().into())
}
