use std::collections::HashMap;
use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::{fs::File, process::Command};

use config::Config;

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use printpdf::{image_crate::ImageDecoder, *};
use regex::Regex;
use resvg::tiny_skia::Color;
use resvg::usvg::fontdb;
use resvg::{self, tiny_skia, usvg};

mod common;
mod msd_config;
mod tests;

const MSD_COMPANION_SERVICE_NAME: &str = "msd-companion";

// const PDF_A4: (f32, f32) = (210.0, 297.0);
const SCORE_IMAGE_REGEX: &str = r".*(score_\d*)(.*)(\.png|\.svg)";

fn svg_to_png(path: &Path) -> PathBuf {
    let tree = {
        // Get file's absolute directory.
        let opt = resvg::usvg::Options {
            resources_dir: std::fs::canonicalize(path)
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf())),
            ..Default::default()
        };

        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();
        println!("PATH: {:?}", path);
        let svg_data = std::fs::read(path).unwrap();
        usvg::Tree::from_data(&svg_data, &opt, &fontdb).unwrap()
    };

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    pixmap.fill(Color::WHITE);
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    let output_path = path.with_extension("png");
    pixmap.save_png(&output_path).unwrap();
    output_path
}

fn add_image_page(doc: &PdfDocumentReference, idx: usize, mut path: PathBuf) {
    if path.extension().unwrap().to_str().unwrap().to_lowercase() == "svg" {
        println!("CONVERTING SVG TO PNG: {:?}", path);
        path = svg_to_png(&path);
    }
    let decoded_img = image_crate::codecs::png::PngDecoder::new(File::open(path).unwrap()).unwrap();
    let (img_w, img_h) = decoded_img.dimensions();

    let (page_idx, layer_idx) = doc.add_page(
        Px(img_w.try_into().unwrap()).into_pt(300.0).into(),
        Px(img_h.try_into().unwrap()).into_pt(300.0).into(),
        format!("Page {}", idx + 1),
    );
    let cur_layer = doc.get_page(page_idx).get_layer(layer_idx);
    let img = Image::try_from(decoded_img).unwrap();

    img.add_to_layer(cur_layer.clone(), ImageTransform::default());
}

fn pdf_from_images(output_path: &PathBuf, paths: &[PathBuf]) -> Result<(), String> {
    // Verify all files exist
    if !paths.iter().all(|e| e.exists()) {
        return Err("Could not verify all files downloaded!".to_owned());
    }
    let doc = PdfDocument::empty(output_path.file_stem().unwrap().to_str().unwrap());
    for (idx, path) in paths.iter().enumerate() {
        add_image_page(&doc, idx, path.to_owned());
    }
    doc.save(&mut BufWriter::new(File::create(output_path).unwrap()))
        .unwrap();
    Ok(())
}

fn get_score_image_paths(
    download_directory: &Path,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let score_image_regex = Regex::new(SCORE_IMAGE_REGEX).unwrap();
    let paths = std::fs::read_dir(download_directory)?
        // Filter out all those directory entries which couldn't be read OR that do not match the score image regex
        .filter_map(|res| {
            let res = res.ok().unwrap();
            println!(
                "PATH: {} IS MATCH: {}",
                res.path().to_str().unwrap(),
                score_image_regex.is_match(res.path().to_str().unwrap())
            );
            if score_image_regex.is_match(res.path().to_str().unwrap()) {
                Some(res)
            } else {
                None
            }
        })
        // Map the directory entries to paths
        .map(|dir_entry| dir_entry.path())
        .collect::<Vec<_>>();
    Ok(paths)
}

fn delete_all_score_image_downloads(directory: &Path) -> Result<Vec<PathBuf>, ()> {
    let paths = get_score_image_paths(directory).unwrap();
    paths.iter().for_each(|path| fs::remove_file(path).unwrap());
    Ok(paths)
}

#[derive(Debug, serde::Deserialize)]
struct QParamsMSD {
    paths: String,
    fname: String,
    output_dir: String,
}

#[get("/msd")]
async fn merge_files(req: HttpRequest) -> HttpResponse {
    let qstr = req.query_string();
    println!("qstr: {}", qstr);
    let params = web::Query::<QParamsMSD>::from_query(qstr).unwrap();
    println!("params: {:#?}", params);
    let (output_dir, fname) = (&params.output_dir, &params.fname);
    let paths: Vec<PathBuf> = params.paths.split(',').map(PathBuf::from).collect();
    let download_dir = paths.first().unwrap().parent().unwrap();
    let pdf_path = PathBuf::from(format!("{output_dir}/{fname}.pdf"));
    pdf_from_images(&pdf_path, &paths).unwrap();
    delete_all_score_image_downloads(download_dir).unwrap();
    let file = actix_files::NamedFile::open_async(pdf_path).await.unwrap();
    file.into_response(&req)
}

#[derive(Debug, serde::Deserialize)]
struct QParamsCleanScores {
    directory: String,
}
#[get("/clean-scores")]
async fn clean_scores(req: HttpRequest) -> HttpResponse {
    let qstr = req.query_string();
    let params = web::Query::<QParamsCleanScores>::from_query(qstr).unwrap();
    let directory = Path::new(&params.directory).canonicalize().unwrap();
    let delete_res = delete_all_score_image_downloads(&directory).unwrap();
    let removed_files_names: Vec<&str> = delete_res
        .iter()
        .map(|e| e.file_name().unwrap().to_str().unwrap())
        .collect();
    HttpResponse::Ok().body(if removed_files_names.is_empty() {
        "No score images found in the given directory! All clean!?".to_owned()
    } else {
        format!(
            "Successfully cleaned up score images in directory: {}!\nFiles cleaned: {:#?}",
            params.directory, removed_files_names
        )
    })
}

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    println!("Server starting...");
    HttpServer::new(|| App::new().service(merge_files).service(clean_scores))
        .bind(("127.0.0.1", 45542))?
        .run()
        .await
}

#[cfg(target_os = "windows")]
fn check_windows_service_exists() -> Option<bool> {
    if cfg!(target_os = "windows") {
        let output = Command::new(format!("sc interrogate \"{}\"", MSD_COMPANION_SERVICE_NAME))
            .output()
            .unwrap();
        println!("{:?}", output);
        Some(true)
    } else {
        None
    }
}

fn main() {
    let mut setup_errs: Vec<String> = vec![];
    let cur_dir = std::env::current_dir()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap()
        .to_owned();
    if cfg!(target_os = "windows") {
        // Add app as Service (windows)
        let shawl_add_cmd = format!(
            ".\\shawl.exe add --name {} -- {}\\msd-companion.exe",
            MSD_COMPANION_SERVICE_NAME, cur_dir
        );
        println!("{}", shawl_add_cmd);
        setup_errs = [
            Command::new(shawl_add_cmd).output(),
            Command::new(format!(
                "sc config {0} start=auto && sc start {0}",
                MSD_COMPANION_SERVICE_NAME
            ))
            .output(),
        ]
        .iter()
        .filter_map(|e| {
            if e.is_err() {
                Some(e.as_ref().unwrap_err().to_string())
            } else {
                None
            }
        })
        .collect();
    }
    if !setup_errs.is_empty() {
        println!("Failed to add and run msd-companion service:\n{setup_errs:?}\n\nBooting server directly...");
        start_server().expect("Failed to start MSD Companion");
    } else {
        println!("Successfully added server as a Windows service!");
    }
}
