use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::{fs::File, process::Command};

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use printpdf::{image_crate::ImageDecoder, *};
use regex::Regex;

const PDF_A4: (f32, f32) = (210.0, 297.0);
const SCORE_IMAGE_REGEX: &str = r"/score_\d*.png/";

fn add_image_page(doc: &PdfDocumentReference, idx: usize, path: &Path) {
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

fn pdf_from_images(output_path: &PathBuf, paths: &Vec<PathBuf>) -> Result<(), String> {
    // Verify all files exist
    if !paths.iter().all(|e| e.exists()) {
        return Err("Could not verify all files downloaded!".to_owned());
    }
    let doc = PdfDocument::empty(output_path.file_stem().unwrap().to_str().unwrap());
    for (idx, path) in paths.iter().enumerate() {
        add_image_page(&doc, idx, path);
    }
    doc.save(&mut BufWriter::new(
        File::create(output_path.to_owned()).unwrap(),
    ))
    .unwrap();
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct MSDQParams {
    paths: String,
    fname: String,
    output_dir: String,
}

#[get("/msd")]
async fn merge_files(req: HttpRequest) -> HttpResponse {
    let qstr = req.query_string();
    let params = web::Query::<MSDQParams>::from_query(qstr).unwrap();
    println!("params: {:#?}", params);
    let output_dir = &params.output_dir;
    let fname = &params.fname;
    let paths: Vec<PathBuf> = params
        .paths
        .split(",")
        .map(|pstr| PathBuf::from(pstr))
        .collect();
    let download_dir = paths.get(0).unwrap().parent().unwrap();

    let pdf_path = PathBuf::from(format!("{output_dir}/{fname}.pdf"));
    pdf_from_images(&pdf_path, &paths).unwrap();
    delete_all_score_image_downloads(download_dir).unwrap();
    let file = actix_files::NamedFile::open_async(pdf_path).await.unwrap();
    file.into_response(&req)
}

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    println!("Server starting...");
    HttpServer::new(|| App::new().service(merge_files))
        .bind(("127.0.0.1", 45542))?
        .run()
        .await
}

fn get_score_image_paths(
    download_directory: &Path,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let score_image_regex = Regex::new(SCORE_IMAGE_REGEX).unwrap();
    let paths = std::fs::read_dir(download_directory)?
        // Filter out all those directory entries which couldn't be read OR that do not match the score image regex
        .filter_map(|res| {
            let res = res.ok().unwrap();
            if score_image_regex.is_match(res.file_name().to_str().unwrap()) {
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

fn delete_all_score_image_downloads(download_directory: &Path) -> Result<(), ()> {
    let paths = get_score_image_paths(download_directory).unwrap();
    paths.iter().for_each(|path| fs::remove_file(path).unwrap());
    Ok(())
}

fn main() {
    let mut setup_errs: Vec<String> = vec![];
    if cfg!(target_os = "windows") {
        // Add app as Service (windows)
        let shawl_add_cmd = format!(
            ".\\shawl.exe add --name msd-companion -- {}\\msd-companion.exe",
            std::env::current_dir()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
        );
        println!("{}", shawl_add_cmd);
        setup_errs = [
            Command::new(shawl_add_cmd).output(),
            Command::new("sc config msd-companion start=auto && sc start msd-companion").output(),
        ]
        .iter()
        .filter(|e| e.is_err())
        .map(|e| e.as_ref().unwrap_err().to_string())
        .collect();
    }
    if setup_errs.len() > 0 {
        println!("Failed to add and run msd-companion service:\n{setup_errs:?}\n\nBooting server directly...");
        start_server().unwrap();
    }
}
