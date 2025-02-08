use std::{collections::HashSet, env, fmt::Write, fs, path::Path};
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
    println!("Proto path: {:?}", &proto_path);

    for entry in WalkDir::new(&proto_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            let path_str = e.path().to_str().unwrap();
            (
                // pull in the 3 pakage we need for calling googleads api
                path_str.contains(&format!("googleads{}v18", std::path::MAIN_SEPARATOR))
                    || path_str.contains(&format!("google{}rpc", std::path::MAIN_SEPARATOR))
                    || path_str.contains(&format!("google{}longrunning", std::path::MAIN_SEPARATOR))
                ) && e
                .path()
                .extension()
                .map_or(false, |ext| ext == "proto")
        })
    {
        let path = entry.path();
        println!("Found proto file: {:?}", path);
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
        println!("Number of proto files: {}", protos.len());
    }

    // split proto into 2 batches; easiest is to compile services last
    let (protos_without_services, protos_with_services): (Vec<_>, Vec<_>) = protos
        .iter()
        .partition(|path| !path.to_str().unwrap().contains("services"));

    println!("{} proto files without services; {} proto files for services", protos_without_services.len(), protos_with_services.len());

    tonic_build::configure()
        .build_server(false)
        .compile(&protos_without_services, &[proto_path.clone()])?;

    tonic_build::configure()
        .build_server(true)
        .compile(&protos_with_services, &[proto_path.clone()])?;

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
            .position(|(idx, pkg_seg)| {
                path_stack
                    .get(idx)
                    .map_or(true, |stack_seg| stack_seg != &pkg_seg)
            })
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

        // write include_proto! inside module
        writeln!(
            protos_rs,
            "tonic::include_proto!(\"{}\");",
            path_stack.join(".")
        )?;
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
