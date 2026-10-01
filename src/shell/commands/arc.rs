use crate::explorer::human_readable_size;
use crate::navigator::Navigator;
use crate::output::{print_error, print_info, print_success};
use crate::t;
use anyhow::{anyhow, Result};
use colored::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn tokenize(args: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for ch in args.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ' ' | '\t' if !in_quotes => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(ch),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ArcKind {
    Zip,
    TarGz,
    Tar,
}

fn detect_kind(path: &str) -> Option<ArcKind> {
    let lower = path.to_lowercase();
    if lower.ends_with(".zip") {
        Some(ArcKind::Zip)
    } else if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") {
        Some(ArcKind::TarGz)
    } else if lower.ends_with(".tar") {
        Some(ArcKind::Tar)
    } else {
        None
    }
}

fn collect_inputs(inputs: &[String], cwd: &Path) -> Result<Vec<(PathBuf, String)>> {
    let mut files = Vec::new();
    for input in inputs {
        let abs = if Path::new(input).is_absolute() {
            PathBuf::from(input)
        } else {
            cwd.join(input)
        };
        if !abs.exists() {
            return Err(anyhow!("{}", t!("cmd.arc.not_found", input)));
        }
        if abs.is_file() {
            let name = abs
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| input.clone());
            files.push((abs, name));
        } else {
            for entry in walkdir::WalkDir::new(&abs).min_depth(1) {
                let entry = entry?;
                let p = entry.path().to_path_buf();
                let rel = p
                    .strip_prefix(cwd)
                    .unwrap_or(&p)
                    .to_string_lossy()
                    .replace('\\', "/");
                files.push((p, rel));
            }
        }
    }
    Ok(files)
}

fn create_zip(out: &Path, files: &[(PathBuf, String)], cwd: &Path) -> Result<()> {
    let file = File::create(out)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (abs, name) in files {
        if abs.is_dir() {
            let dir_name = if name.ends_with('/') {
                name.clone()
            } else {
                format!("{}/", name)
            };
            zip.add_directory(dir_name, options)?;
            continue;
        }
        zip.start_file(name, options)?;
        let mut f = File::open(abs)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        zip.write_all(&buf)?;
    }
    zip.finish()?;
    let _ = cwd;
    Ok(())
}

fn create_tar(out: &Path, files: &[(PathBuf, String)], cwd: &Path, gzip: bool) -> Result<()> {
    let file = File::create(out)?;
    if gzip {
        let enc = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        let mut builder = tar::Builder::new(enc);
        for (abs, name) in files {
            let arc_path = Path::new(name);
            if abs.is_dir() {
                builder.append_dir(arc_path, abs)?;
            } else {
                builder.append_path_with_name(abs, arc_path)?;
            }
        }
        let enc = builder.into_inner()?;
        enc.finish()?;
    } else {
        let mut builder = tar::Builder::new(file);
        for (abs, name) in files {
            let arc_path = Path::new(name);
            if abs.is_dir() {
                builder.append_dir(arc_path, abs)?;
            } else {
                builder.append_path_with_name(abs, arc_path)?;
            }
        }
        builder.finish()?;
    }
    let _ = cwd;
    Ok(())
}

fn cmd_create(args: &[String], nav: &mut Navigator) -> Result<bool> {
    if args.len() < 2 {
        print_error(&t!("cmd.arc.create_usage"));
        return Ok(false);
    }
    let cwd = nav.current_path().to_path_buf();
    let out_arg = &args[0];
    let kind = detect_kind(out_arg)
        .ok_or_else(|| anyhow!("{}", t!("cmd.arc.bad_ext")))?;
    let out = if Path::new(out_arg).is_absolute() {
        PathBuf::from(out_arg)
    } else {
        cwd.join(out_arg)
    };
    if let Some(parent) = out.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let files = collect_inputs(&args[1..], &cwd)?;
    if files.is_empty() {
        print_error(&t!("cmd.arc.no_inputs"));
        return Ok(false);
    }
    match kind {
        ArcKind::Zip => create_zip(&out, &files, &cwd)?,
        ArcKind::TarGz => create_tar(&out, &files, &cwd, true)?,
        ArcKind::Tar => create_tar(&out, &files, &cwd, false)?,
    }
    print_success(&t!("cmd.arc.created", out.display().to_string(), files.len()));
    Ok(false)
}

fn cmd_list(args: &[String], nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("cmd.arc.list_usage"));
        return Ok(false);
    }
    let cwd = nav.current_path().to_path_buf();
    let arg = &args[0];
    let path = if Path::new(arg).is_absolute() {
        PathBuf::from(arg)
    } else {
        cwd.join(arg)
    };
    let kind = detect_kind(arg).ok_or_else(|| anyhow!("{}", t!("cmd.arc.bad_ext")))?;
    match kind {
        ArcKind::Zip => {
            let file = File::open(&path)?;
            let mut archive = zip::ZipArchive::new(file)?;
            println!("{}", t!("cmd.arc.list_title", arg).cyan().bold());
            for i in 0..archive.len() {
                let f = archive.by_index(i)?;
                let size = f.size();
                let name = f.name().to_string();
                let dir_mark = if f.is_dir() { "/" } else { "" };
                println!("  {}{} ({})", name, dir_mark, human_readable_size(size));
            }
            print_info(&t!("cmd.arc.total", archive.len()));
        }
        ArcKind::TarGz | ArcKind::Tar => {
            println!("{}", t!("cmd.arc.list_title", arg).cyan().bold());
            let mut count = 0usize;
            if kind == ArcKind::TarGz {
                let file = File::open(&path)?;
                let dec = flate2::read::GzDecoder::new(file);
                let mut archive = tar::Archive::new(dec);
                for entry in archive.entries()? {
                    let entry = entry?;
                    let p = entry.path()?.to_string_lossy().into_owned();
                    let size = entry.header().size()?;
                    println!("  {} ({})", p, human_readable_size(size));
                    count += 1;
                }
            } else {
                let file = File::open(&path)?;
                let mut archive = tar::Archive::new(file);
                for entry in archive.entries()? {
                    let entry = entry?;
                    let p = entry.path()?.to_string_lossy().into_owned();
                    let size = entry.header().size()?;
                    println!("  {} ({})", p, human_readable_size(size));
                    count += 1;
                }
            }
            print_info(&t!("cmd.arc.total", count));
        }
    }
    Ok(false)
}

fn cmd_extract(args: &[String], nav: &mut Navigator) -> Result<bool> {
    if args.is_empty() {
        print_error(&t!("cmd.arc.extract_usage"));
        return Ok(false);
    }
    let cwd = nav.current_path().to_path_buf();
    let arg = &args[0];
    let src = if Path::new(arg).is_absolute() {
        PathBuf::from(arg)
    } else {
        cwd.join(arg)
    };
    if !src.exists() {
        return Err(anyhow!("{}", t!("cmd.arc.not_found", arg)));
    }
    let dest = if args.len() >= 2 {
        let d = &args[1];
        if Path::new(d).is_absolute() {
            PathBuf::from(d)
        } else {
            cwd.join(d)
        }
    } else {
        // Default: strip extension in the archive's own directory
        let parent = src.parent().unwrap_or(&cwd).to_path_buf();
        let stem = src
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "extracted".to_string());
        let lower = stem.to_lowercase();
        let base = if lower.ends_with(".tar.gz") {
            stem[..stem.len() - 7].to_string()
        } else if lower.ends_with(".tgz") {
            stem[..stem.len() - 4].to_string()
        } else if lower.ends_with(".zip") || lower.ends_with(".tar") {
            stem[..stem.rfind('.').unwrap_or(stem.len())].to_string()
        } else {
            stem
        };
        parent.join(base)
    };
    std::fs::create_dir_all(&dest)?;
    let kind = detect_kind(arg).ok_or_else(|| anyhow!("{}", t!("cmd.arc.bad_ext")))?;
    let mut count = 0usize;
    match kind {
        ArcKind::Zip => {
            let file = File::open(&src)?;
            let mut archive = zip::ZipArchive::new(file)?;
            for i in 0..archive.len() {
                let mut f = archive.by_index(i)?;
                let Some(safe) = f.enclosed_name() else {
                    continue;
                };
                let out = dest.join(safe);
                if f.is_dir() {
                    std::fs::create_dir_all(&out)?;
                } else {
                    if let Some(p) = out.parent() {
                        std::fs::create_dir_all(p)?;
                    }
                    let mut outf = File::create(&out)?;
                    std::io::copy(&mut f, &mut outf)?;
                    count += 1;
                }
            }
        }
        ArcKind::TarGz => {
            let file = File::open(&src)?;
            let dec = flate2::read::GzDecoder::new(file);
            let mut archive = tar::Archive::new(dec);
            for entry in archive.entries()? {
                let mut e = entry?;
                e.unpack_in(&dest)?;
                count += 1;
            }
        }
        ArcKind::Tar => {
            let file = File::open(&src)?;
            let mut archive = tar::Archive::new(file);
            for entry in archive.entries()? {
                let mut e = entry?;
                e.unpack_in(&dest)?;
                count += 1;
            }
        }
    }
    print_success(&t!("cmd.arc.extracted", count, dest.display().to_string()));
    Ok(false)
}

fn print_usage() {
    println!("{}", t!("cmd.arc.usage_title").cyan().bold());
    println!("  {}", t!("cmd.arc.help_create"));
    println!("  {}", t!("cmd.arc.help_extract"));
    println!("  {}", t!("cmd.arc.help_list"));
    println!();
    println!("{}", t!("cmd.arc.examples_title").green());
    println!("    arc create backup.zip src Cargo.toml");
    println!("    arc list backup.zip");
    println!("    arc extract backup.zip restored");
}

pub fn cmd_arc(args: &str, nav: &mut Navigator) -> Result<bool> {
    let tokens = tokenize(args);
    if tokens.is_empty() {
        print_usage();
        return Ok(false);
    }
    let sub = tokens[0].to_lowercase();
    let rest = &tokens[1..];
    match sub.as_str() {
        "create" | "c" | "mk" => cmd_create(rest, nav),
        "extract" | "x" | "unzip" => cmd_extract(rest, nav),
        "list" | "ls" | "l" => cmd_list(rest, nav),
        "help" | "-h" | "--help" => {
            print_usage();
            Ok(false)
        }
        _ => {
            print_error(&t!("cmd.arc.unknown_sub", tokens[0]));
            println!();
            print_usage();
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_kind_by_extension() {
        assert_eq!(detect_kind("a.zip"), Some(ArcKind::Zip));
        assert_eq!(detect_kind("a.TAR.GZ"), Some(ArcKind::TarGz));
        assert_eq!(detect_kind("a.tgz"), Some(ArcKind::TarGz));
        assert_eq!(detect_kind("a.tar"), Some(ArcKind::Tar));
        assert_eq!(detect_kind("a.7z"), None);
    }

    #[test]
    fn tokenize_respects_quotes() {
        let t = tokenize("create out.zip \"my file.txt\" src");
        assert_eq!(t, vec!["create", "out.zip", "my file.txt", "src"]);
    }

    #[test]
    fn zip_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("hello.txt");
        std::fs::write(&src, "hello arc").unwrap();
        let out = dir.path().join("out.zip");
        let files = vec![(src.clone(), "hello.txt".to_string())];
        create_zip(&out, &files, dir.path()).unwrap();
        assert!(out.exists());
        let f = File::open(&out).unwrap();
        let mut archive = zip::ZipArchive::new(f).unwrap();
        assert_eq!(archive.len(), 1);
        let mut content = String::new();
        archive.by_index(0).unwrap().read_to_string(&mut content).unwrap();
        assert_eq!(content, "hello arc");
    }

    #[test]
    fn targz_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("data.txt");
        std::fs::write(&src, "tar gz content").unwrap();
        let out = dir.path().join("out.tar.gz");
        let files = vec![(src, "data.txt".to_string())];
        create_tar(&out, &files, dir.path(), true).unwrap();
        assert!(out.exists());
        let f = File::open(&out).unwrap();
        let dec = flate2::read::GzDecoder::new(f);
        let mut archive = tar::Archive::new(dec);
        let entries: Vec<_> = archive.entries().unwrap().collect();
        assert_eq!(entries.len(), 1);
    }
}
