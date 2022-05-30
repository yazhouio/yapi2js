use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Result};
use tera::{Context, Tera, try_get_value};

use crate::YapiObj;

fn first_lower(s: &tera::Value, _: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let c = try_get_value!("data.name", "value", String, s);
    if !regex::Regex::new(r"^\w+$").unwrap().is_match(&c) {
        return Ok(tera::Value::String("unknown_error".to_string()));
    }

    let mut c = c.chars();
    match c.next() {
        None => Ok(tera::Value::String(String::new())),
        Some(f) => Ok(tera::Value::String(f.to_lowercase().collect::<String>() + c.as_str())),
    }
}

fn lower_case(s: &tera::Value, _: &HashMap<String, tera::Value>) -> Result<tera::Value, tera::Error> {
    let c = try_get_value!("data.name", "value", String, s);
    if !regex::Regex::new(r"^\w+$").unwrap().is_match(&c) {
        return Ok(tera::Value::String("unknown_error".to_string()));
    }
    Ok(tera::Value::String(c.to_lowercase()))
}


pub fn generate(out_file: &Path, data: &Vec<YapiObj>) -> Result<()> {
    let temp = r#"
// generated by https://github.com/spike2044/yapi2js

export default {
{% for data in list %}
  {{ data.name | first_lower }}: {
{% for item in data.list %}
  {{ item.title | first_lower }}: [
    '{{item.method }}',
    '{{item.path}}'
  ],
{% endfor %}
},
{% endfor %}
}
"#;
    let mut tera = Tera::default();
    tera.register_filter("first_lower", first_lower);
    tera.register_filter("lower", lower_case);
    tera.add_raw_template("api", temp)?;
    let mut context = Context::new();
    context.insert("list", &data);

    match OpenOptions::new().read(true).write(true).truncate(true).create(true).open(out_file) {
        Ok(mut file) => {
            let code = tera.render("api", &context)?;
            file.write_all(code.as_bytes())?;
            Ok(())
        }
        Err(e) => {
            println!("{}", e);
            Err(anyhow!(e.to_string()))
        }
    }
}
