use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::Error;
use crate::get_file_name_portable;

pub(crate) async fn photo_post(mut multipart: Multipart) -> Result<impl IntoResponse, Error> {
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().ok_or_else(|| Error::internal("no name"))?.to_string();
        if name == "photo" {
            let filename = field.file_name().ok_or_else(|| Error::internal("no file name"))?;
            let filename = Path::new(get_file_name_portable(filename));
            let filename = filename.extension().unwrap_or_else(|| OsStr::new("png"));
            let path = format!("/tmp/photo.{}", filename.to_str().unwrap_or("png"));
            let data = field.bytes().await?;
            std::fs::write(path.as_str(), data)?;
            #[rustfmt::skip]
            let convert_status = Command::new("convert")
                .args([path.as_str(), "/tmp/photo-web.jpg"])
                .status()
                .map_err(|e| Error::internal(format!("failed to run convert: {}", e)))?;
            match convert_status.code() {
                Some(code) if code != 0 => {
                    return Err(Error::internal(format!(
                        "convert exited with status code: {}",
                        code
                    )))
                }
                Some(_code) => {
                    std::fs::rename("/tmp/photo-web.jpg", "/srv/web3.staex.io/photo.jpg")
                        .map_err(|e| Error::internal(format!("failed to rename file: {}", e)))?;
                    return Ok((StatusCode::OK, "convert: ok".to_string()));
                }
                None => return Err(Error::internal("convert process terminated by signal")),
            }
        }
    }
    Err(Error::internal("no photo field"))
}
