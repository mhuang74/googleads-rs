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

  for entry in WalkDir::new("proto")
    .into_iter()
    .map(|e| e.unwrap())
    .filter(|e| {
      (
        // pull in the 3 pakage we need for calling googleads api
        e.path().to_str().unwrap().contains(&"googleads/v14") ||
        e.path().to_str().unwrap().contains(&"google/rpc") ||
        e.path().to_str().unwrap().contains(&"google/longrunning")
      )
      &&
      e.path()
        .extension()
        .map_or(false, |ext| ext.to_str().unwrap() == "proto")
    })
  {
    let path = entry.path();
    protos.push(path.to_owned());

    let content = fs::read_to_string(&path).unwrap();
    let pkg = content
      .lines()
      .find(|line| line.starts_with("package "))
      .unwrap()
      // remove comment
      .split("//")
      .next()
      .unwrap()
      // extract package
      .trim()
      .trim_start_matches("package ")
      .trim_end_matches(";");

    pkgs.insert(pkg.to_string());
  }

  tonic_build::configure()
    .build_server(false)
    .compile(&protos, &[Path::new("proto")])?;

  write_protos_rs(pkgs)?;

  println!("cargo:rerun-if-changed=proto");

  Ok(())
}

fn write_protos_rs(pkgs: HashSet<String>) -> Res {
  let ref mut protos_rs = String::new();

  let mut packages: Vec<String> = pkgs.into_iter().collect();
  packages.sort();

  let mut path_stack: Vec<String> = vec![];

  for pkg in packages {
    // find common ancestor
    let pop_to = pkg
      .split(".")
      .map(|seg| map_keyword(seg))
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
    for seg in pkg.split(".").skip(pop_to).map(|seg| map_keyword(seg)) {
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
  while path_stack.len() > 0 {
    path_stack.pop();
    writeln!(protos_rs, "}}").unwrap();
  }

  fs::write(format!("{}/protos.rs", env::var("OUT_DIR")?), protos_rs)?;

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

