/*!
 * Video Export to MP4
 *
 * Captures rendered frames and encodes them to MP4 video using ffmpeg.
 */

use std::process::{Command, Stdio, ChildStdin};
use std::io::Write;

pub struct VideoExporter {
    ffmpeg_stdin: Option<ChildStdin>,
    frame_count: u64,
    width: u32,
    height: u32,
    fps: u32,
}

impl VideoExporter {
    pub fn new(output_path: &str, width: u32, height: u32, fps: u32) -> Result<Self, std::io::Error> {
        // Start ffmpeg process
        let mut ffmpeg = Command::new("ffmpeg")
            .args(&[
                "-y", // Overwrite output file
                "-f", "rawvideo",
                "-pixel_format", "rgba",
                "-video_size", &format!("{}x{}", width, height),
                "-framerate", &fps.to_string(),
                "-i", "-", // Read from stdin
                "-c:v", "libx264",
                "-preset", "fast",
                "-crf", "18", // High quality (0-51, lower is better)
                "-pix_fmt", "yuv420p",
                "-movflags", "+faststart",
                output_path,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let stdin = ffmpeg.stdin.take().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to open ffmpeg stdin")
        })?;

        println!("🎥 Video export started: {}", output_path);
        println!("   Resolution: {}x{} @ {} FPS", width, height, fps);

        Ok(Self {
            ffmpeg_stdin: Some(stdin),
            frame_count: 0,
            width,
            height,
            fps,
        })
    }

    pub fn write_frame(&mut self, rgba_data: &[u8]) -> Result<(), std::io::Error> {
        if let Some(ref mut stdin) = self.ffmpeg_stdin {
            stdin.write_all(rgba_data)?;
            self.frame_count += 1;

            if self.frame_count % (self.fps as u64) == 0 {
                println!("📹 Recorded {} seconds", self.frame_count / self.fps as u64);
            }
        }
        Ok(())
    }

    pub fn finish(mut self) -> Result<(), std::io::Error> {
        if let Some(mut stdin) = self.ffmpeg_stdin.take() {
            stdin.flush()?;
            drop(stdin);
        }

        let duration = self.frame_count as f64 / self.fps as f64;
        println!("✅ Video export complete: {:.2} seconds, {} frames", duration, self.frame_count);

        Ok(())
    }
}

impl Drop for VideoExporter {
    fn drop(&mut self) {
        if let Some(mut stdin) = self.ffmpeg_stdin.take() {
            let _ = stdin.flush();
        }
    }
}
