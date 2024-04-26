use std::path::Path;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};

const PDF_A4: (f32, f32) = (210.0, 297.0);

fn get_pdf(fname: &String, paths: Vec<&Path>) -> Result<(), String> {
    // Verify all files exist
    if !paths.iter().all(|e| e.exists()) { return Err("Could not verify all files downloaded!".to_owned()) }
    let (doc, page1, layer1) = PdfDocument::new("", Mm(210.0), Mm(297.0), "Page 0");
    for (idx, path) in paths.iter().enumerate() {
        doc.add_page(Mm(PDF_A4.0), Mm(PDF_A4.1), format!("Page {}", idx+1));
    }
    doc.save(&mut BufWriter::new(File::create(format!("{}.pdf", fname)).unwrap())).unwrap();
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct MSDQParams {
    paths: String,
    fname: String 
}

#[get("/msd")]
async fn merge_files(req: HttpRequest) -> HttpResponse {
    let qstr = req.query_string();
    println!("{:#?}", qstr);
    let params = web::Query::<MSDQParams>::from_query(qstr).unwrap();
    let fname = &params.fname;
    let paths: Vec<&Path> = params.paths.split(",").map(|pstr| Path::new(pstr)).collect();
    get_pdf(fname, paths);
    HttpResponse::Ok().body("Received file paths")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(merge_files))
        .bind(("127.0.0.1", 45542))?
        .run()
        .await
}
