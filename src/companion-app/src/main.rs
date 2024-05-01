use std::io::BufWriter;
use std::path::Path;
use std::process::Output;
use std::{fs::File, process::Command};

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use printpdf::{image_crate::ImageDecoder, *};

const PDF_A4: (f32, f32) = (210.0, 297.0);

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

fn get_pdf(fname: &String, paths: Vec<&Path>) -> Result<(), String> {
    // Verify all files exist
    if !paths.iter().all(|e| e.exists()) {
        return Err("Could not verify all files downloaded!".to_owned());
    }
    let doc = PdfDocument::empty(fname);
    for (idx, path) in paths.iter().enumerate() {
        add_image_page(&doc, idx, path);
    }
    doc.save(&mut BufWriter::new(
        File::create(format!("{}.pdf", fname)).unwrap(),
    ))
    .unwrap();
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct MSDQParams {
    paths: String,
    fname: String,
}

#[get("/msd")]
async fn merge_files(req: HttpRequest) -> HttpResponse {
    let qstr = req.query_string();
    let params = web::Query::<MSDQParams>::from_query(qstr).unwrap();
    println!("params: {:#?}", params);
    let fname = &params.fname;
    let paths: Vec<&Path> = params
        .paths
        .split(",")
        .map(|pstr| Path::new(pstr))
        .collect();
    get_pdf(fname, paths).unwrap();
    HttpResponse::Ok().body("PDF file successfully generated!")
}

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(merge_files))
        .bind(("127.0.0.1", 45542))?
        .run()
        .await
}

fn main() {
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
        let errs: Vec<String> = [
            Command::new(shawl_add_cmd).output(),
            Command::new("sc config msd-companion start=auto && sc start msd-companion").output()
        ].iter().filter(|e| e.is_err()).map(|e| e.as_ref().unwrap_err().to_string()).collect();
        if errs.len() > 0 {
                println!("Failed to add and run msd-companion service:\n{}\n\nBooting server directly...", {errs.join("\n")});
                start_server().unwrap();
        }
    }
}
