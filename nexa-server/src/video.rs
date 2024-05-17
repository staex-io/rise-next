use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::info;

use crate::Error;

pub(crate) async fn video_post(mut multipart: Multipart) -> Result<impl IntoResponse, Error> {
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().ok_or_else(|| Error::internal("no name"))?.to_string();
        if name == "video" {
            let filename = field.file_name().ok_or_else(|| Error::internal("no file name"))?;
            let filename = Path::new(get_file_name_portable(filename));
            let filename = filename.extension().unwrap_or_else(|| OsStr::new("mp4"));
            let path = format!("/tmp/video.{}", filename.to_str().unwrap_or("mp4"));
            let data = field.bytes().await?;
            info!("video file size: {}", data.len());
            if data.is_empty() {
                return Err(Error::internal("the file is empty"));
            }
            std::fs::write(path.as_str(), data)?;
            #[rustfmt::skip]
            let ffmpeg_output = Command::new("ffmpeg")
                .args([
                    "-loglevel", "warning",
                    "-y",
                    "-i", path.as_str(),
                    "-vcodec", "libx264",
                    "-crf", "18",
                    //"-threads", "1",
                    "-pix_fmt", "yuv420p",
                    "-movflags", "+faststart",
                    "-profile:v", "main",
                    "/tmp/video-web.mp4",
                ])
                .output()
                .map_err(|e| Error::internal(format!("failed to run ffmpeg: {}", e)))?;
            match ffmpeg_output.status.code() {
                Some(code) if code != 0 => {
                    return Err(Error::internal(format!(
                        "ffmpeg exited with status code: {}\nstdout:\n{}\nstderr:\n{}",
                        code,
                        String::from_utf8_lossy(ffmpeg_output.stdout.as_slice()),
                        String::from_utf8_lossy(ffmpeg_output.stderr.as_slice()),
                    )))
                }
                Some(_code) => {
                    std::fs::rename("/tmp/video-web.mp4", "/srv/web3.staex.io/video.mp4")
                        .map_err(|e| Error::internal(format!("failed to rename file: {}", e)))?;
                    return Ok((StatusCode::OK, "ffmpeg: ok".to_string()));
                }
                None => return Err(Error::internal("ffmpeg process terminated by signal")),
            }
        }
    }
    Err(Error::internal("no video field"))
}

// https://commons.apache.org/proper/commons-fileupload/faq.html#whole-path-from-IE
pub(crate) fn get_file_name_portable(path: &str) -> &str {
    let from = match (path.rfind('/'), path.rfind('\\')) {
        (Some(i), Some(j)) => i.max(j),
        (Some(i), None) => i,
        (None, Some(j)) => j,
        (None, None) => return path,
    };
    &path[(from + 1)..]
}
