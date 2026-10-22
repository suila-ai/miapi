//! `cargo codegen` で実行する生成器。
//!
//! `specs/<name>/api.json` ごとに、ソースのツリーを 1 つ `src/generated/<name>/` に生成する。
//! あわせて、同じ名前の cargo feature でツリーを有効にする `src/generated/mod.rs` も生成する。
//! `specs/`・`src/generated/`・`Cargo.toml` は、このクレートの親であるワークスペースのルートから探す。

mod corrections;
mod emit;
mod model;
mod naming;
mod resolve;
mod spec;
#[cfg(test)]
mod tests;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("エラー: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("codegen/ には親のディレクトリがある");
    let specs_dir = root.join("specs");
    let out = root.join("src/generated");
    let mut specs: Vec<(String, PathBuf)> = fs::read_dir(&specs_dir)
        .map_err(|e| format!("{}: {e}", specs_dir.display()))?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|dir| dir.join("api.json").is_file())
        .map(|dir| {
            let name = dir.file_name().unwrap().to_string_lossy().into_owned();
            (name, dir)
        })
        .collect();
    specs.sort();
    if specs.is_empty() {
        return Err(format!("{} に */api.json が無い", specs_dir.display()));
    }
    let names: Vec<String> = specs.iter().map(|(name, _)| name.clone()).collect();
    check_features(&root.join("Cargo.toml"), &names)?;

    let mut files: Vec<(PathBuf, String)> = Vec::new();
    for (name, dir) in &specs {
        let path = dir.join("api.json");
        let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut spec: spec::Spec =
            serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        corrections::apply(&mut spec);
        let tree = resolve::build_tree(&spec).map_err(|e| format!("{name}: {e}"))?;
        for (relative, tokens) in emit::tree_files(&tree, name) {
            files.push((out.join(name).join(relative), tokens.to_string()));
        }
        println!(
            "{name}: Misskey {}（エンドポイント {} 個）",
            tree.version,
            spec.paths.len()
        );
    }
    let selector = out.join("mod.rs");
    files.push((selector.clone(), emit::selector_file(&names).to_string()));

    if out.exists() {
        fs::remove_dir_all(&out).map_err(|e| format!("{}: {e}", out.display()))?;
    }
    for (path, source) in &files {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        fs::write(path, source).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    // rustfmt は mod 宣言（`#[path]` 付きのものを含む）をたどって、子のファイルもすべて整形する。
    rustfmt(&selector)?;
    println!(
        "{} 個のファイルを {} に書き出した",
        files.len(),
        out.display()
    );
    Ok(())
}

fn check_features(manifest: &Path, names: &[String]) -> Result<(), String> {
    let text = fs::read_to_string(manifest).map_err(|e| format!("{}: {e}", manifest.display()))?;
    let table: toml::Table = text
        .parse()
        .map_err(|e| format!("{}: {e}", manifest.display()))?;
    let features = table.get("features").and_then(|f| f.as_table());
    let missing: Vec<&String> = names
        .iter()
        .filter(|name| !features.is_some_and(|f| f.contains_key(name.as_str())))
        .collect();
    if missing.is_empty() {
        return Ok(());
    }
    let lines: Vec<String> = missing
        .iter()
        .map(|name| format!("\"{name}\" = []"))
        .collect();
    Err(format!(
        "{} に、spec に対応する feature が無い。次の行を [features] に足すこと。\n{}",
        manifest.display(),
        lines.join("\n")
    ))
}

fn rustfmt(root: &Path) -> Result<(), String> {
    let status = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(root)
        .status()
        .map_err(|e| format!("生成したコードの整形には rustfmt が必要: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("rustfmt が失敗した: {status}"))
    }
}
