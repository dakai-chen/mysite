use std::fmt::Display;
use std::str::FromStr;

use boluo::http::HeaderMap;

pub fn typed_header<T>(headers: &HeaderMap, header_name: &str) -> Result<T, String>
where
    T: FromStr,
    T::Err: Display,
{
    optional_typed_header(headers, header_name)
        .transpose()
        .ok_or_else(|| format!("缺失请求头 `{header_name}`"))?
}

pub fn optional_typed_header<T>(headers: &HeaderMap, header_name: &str) -> Result<Option<T>, String>
where
    T: FromStr,
    T::Err: Display,
{
    let Some(header_value) = headers.get(header_name) else {
        return Ok(None);
    };
    str::from_utf8(header_value.as_bytes())
        .map_err(|_| format!("请求头 `{header_name}` 的值不是有效的 UTF-8 编码"))?
        .parse()
        .map(Some)
        .map_err(|e| {
            format!(
                "请求头 `{header_name}` 解析失败：{e}（预期类型：{}）",
                std::any::type_name::<T>()
            )
        })
}
