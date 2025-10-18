use build_print::info;
use std::{
    collections::HashMap, collections::HashSet, env, fmt::Write, fs, path::Path,
    path::MAIN_SEPARATOR,
};
use walkdir::WalkDir;

type Res = Result<(), Box<dyn std::error::Error>>;

// replace:
//   "ROOT DIR": root dir of proto files to generate
//   "INCLUDE DIR": where all "package" specifier based on
//   "REPO DIR": where to monitor change for build.rs rerun

fn main() -> Res {
    let mut protos = vec![];
    let mut pkgs = HashSet::new();

    let proto_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("proto");
    info!("Proto path: {:?}", &proto_path);

    for entry in WalkDir::new(&proto_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            let path_str = e.path().to_str().unwrap();
            (
                // pull in the 3 pakage we need for calling googleads api
                path_str.contains(&format!("googleads{}v22", std::path::MAIN_SEPARATOR))
                    || path_str.contains(&format!("google{}rpc", std::path::MAIN_SEPARATOR))
                    || path_str.contains(&format!("google{}longrunning", std::path::MAIN_SEPARATOR))
            ) && e.path().extension().is_some_and(|ext| ext == "proto")
        })
    {
        let path = entry.path();
        // info!("Found proto file: {:?}", path);
        protos.push(path.to_owned());

        let content = fs::read_to_string(path).expect("Failed to read proto file");
        let pkg = content
            .lines()
            .find(|line| line.starts_with("package "))
            .expect("Package declaration not found")
            // remove comment
            .split("//")
            .next()
            .unwrap()
            // extract package
            .trim()
            .trim_start_matches("package ")
            .trim_end_matches(';');

        pkgs.insert(pkg.to_string());
    }

    if protos.is_empty() {
        return Err("No .proto files found".into());
    } else {
        info!("Number of proto files: {}", protos.len());
    }

    let package_names = ["common", "enums", "errors", "resources", "services"];
    let mut protos_by_package: HashMap<&str, Vec<_>> = HashMap::new();
    let mut misc_protos: Vec<_> = Vec::new();

    for proto in &protos {
        let path_str = proto.to_str().unwrap();
        let mut matched = false;

        for &package in &package_names {
            let pkg_str = format!("{MAIN_SEPARATOR}{package}{MAIN_SEPARATOR}");
            if path_str.contains(&pkg_str) {
                protos_by_package
                    .entry(package)
                    .or_insert_with(Vec::new)
                    .push(proto.clone());
                matched = true;
                // info!("added to {token}: {path_str}");
                break;
            }
        }

        if !matched {
            // info!("Misc proto: {}", path_str);
            misc_protos.push(proto.clone());
        }
    }

    if !misc_protos.is_empty() {
        info!("> Compiling {} misc proto files", misc_protos.len());
        tonic_prost_build::configure()
            .build_server(false)
            .type_attribute(".", "#[allow(clippy::all)]")
            .compile_protos(&misc_protos, std::slice::from_ref(&proto_path))?;
    }

    for &package in &package_names {
        if let Some(protos) = protos_by_package.get(package) {
            info!(
                "> Compiling {} proto files from package '{}'",
                protos.len(),
                package
            );
            for chunk in protos.chunks(185) {
                info!(
                    "  Compiling batch of {} proto files from package '{}'",
                    chunk.len(),
                    package
                );
                tonic_prost_build::configure()
                    .build_server(false)
                    .type_attribute(".", "#[allow(clippy::all)]")
                    .compile_protos(chunk, std::slice::from_ref(&proto_path))?;
            }
        }
    }

    write_protos_rs(pkgs)?;

    Ok(())
}

fn write_protos_rs(pkgs: HashSet<String>) -> Res {
    let protos_rs = &mut String::new();

    let mut packages: Vec<String> = pkgs.into_iter().collect();
    packages.sort();

    let mut path_stack: Vec<String> = vec![];

    for pkg in packages {
        // find common ancestor
        let pop_to = pkg
            .split('.')
            .map(map_keyword)
            .enumerate()
            .position(|(idx, pkg_seg)| path_stack.get(idx) != Some(&pkg_seg))
            .unwrap_or(0);

        // pop stack
        while path_stack.len() > pop_to {
            path_stack.pop();
            writeln!(protos_rs, "}}")?;
        }

        // now push stack
        for seg in pkg.split('.').skip(pop_to).map(map_keyword) {
            writeln!(protos_rs, "pub mod {} {{", &seg)?;
            path_stack.push(seg);
        }

        // write include! inside module for prost_build
        writeln!(protos_rs, "include!(\"{}.rs\");", path_stack.join("."))?;
    }

    // pop all stack
    while !path_stack.is_empty() {
        path_stack.pop();
        writeln!(protos_rs, "}}").unwrap();
    }

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    fs::write(Path::new(&out_dir).join("protos.rs"), protos_rs)?;

    Ok(())
}

// This is copied from prost-build/src/ident.rs
fn map_keyword(kw: &str) -> String {
    let mut ident = kw.to_string();
    match ident.as_str() {
      // 2015 strict keywords.
      "as" | "break" | "const" | "continue" | "else" | "enum" | "false"
      | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut"
      | "pub" | "ref" | "return" | "static" | "struct" | "trait" | "true"
      | "type" | "unsafe" | "use" | "where" | "while"
      // 2018 strict keywords.
      | "dyn"
      // 2015 reserved keywords.
      | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv" | "typeof"
      | "unsized" | "virtual" | "yield"
      // 2018 reserved keywords.
      | "async" | "await" | "try" => ident.insert_str(0, "r#"),
      // the following keywords are not supported as raw identifiers and are therefore suffixed with an underscore.
      "self" | "super" | "extern" | "crate" => ident += "_",
      _ => (),
  }
    ident
}
