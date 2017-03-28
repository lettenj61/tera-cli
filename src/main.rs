extern crate clap;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde_json;
#[macro_use]
extern crate tera;

use std::fs;
use std::io;
use std::path::Path;

use clap::App;
use serde_json::Value;
use tera::Context;

const USAGE: &'static str = r#"
-d, --dump-stdout       'Print rendered HTML into STDOUT'
-o, --output=[DEST]     'Set output file to store rendered template'
-t, --template=[GLOB]   'Set globs to locate template files, defaults to templates/**/*'
-c, --context=[FILE]    'Valid JSON file contains context data for rendering'

[NAME]  'Name of the template to be rendered, defaults to templates/layout.html'
"#;

fn main() {

    pretty_env_logger::init().expect("FATAL: Couldn't start logger, shutting down.");

    let matches = App::new("tera-cli")
        .version(concat!(env!("CARGO_PKG_VERSION")))
        .about("Command line interface for Tera template engine")
        .args_from_usage(USAGE.trim())
        .get_matches();

    debug!("{:?}", matches);

    // Initialize contexts first
    let json: Value = matches.value_of("context")
        .and_then(|path| read_file(path).ok())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(Value::Null);

    debug!("context: {:?}",
           serde_json::to_string_pretty(&json).unwrap_or("Failed to read JSON".into()));

    let template_glob = matches.value_of("template").unwrap_or("templates/**/*");
    // App will go down when it encountered an error inside this macro call
    let tera = compile_templates!(template_glob);

    let template_name = matches.value_of("NAME").unwrap_or("template/layout.html");
    // If context json is null, use empty context instead;
    let result = match json {
        Value::Null => tera.render(&template_name, &Context::new()),
        _ => tera.render(&template_name, &json)
    };

    let use_stdout = matches.is_present("dump-stdout") || matches.value_of("output").is_none();
    if let Ok(body) = result {
        if use_stdout {
            println!("{}", body);
        } else {
            let out = matches.value_of("output").unwrap();
            write_file(out, &body).unwrap();
        }
    } else {
        println!("Failed to render template: {}", template_name);
    }
}

fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {

    let mut file = try!(fs::OpenOptions::new().read(true).open(path));
    let mut s = String::new();
    try!(io::Read::read_to_string(&mut file, &mut s));

    Ok(s)
}

fn write_file<P: AsRef<Path>>(path: P, contents: &str) -> io::Result<()> {
    let mut file = try!(fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path));

    try!(io::Write::write_all(&mut file, contents.as_bytes()));
    try!(file.sync_data());
    Ok(())
}
