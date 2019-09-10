use crate::layout::RgbaImage;
use crate::{draw_line, draw_line_at, Line};
use image::GenericImage;
use image::{DynamicImage, FilterType, ImageBuffer};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub struct PreviewService {
    original_image: RgbaImage,
    preview_image: RgbaImage,
    rx: Receiver<UpdateRequest>,
    tx: Sender<RgbaImage>,
    lines: Vec<PositionedLine>,
}

pub struct PositionedLine {
    pub line: Line,
    pub position: Option<(i32, i32)>,
    pub line_id: usize,
}

pub enum UpdateRequest {
    InitialUpdate {
        positioned_line: PositionedLine,
    },
    TextUpdate {
        line_id: usize,
        new_text: String,
    },
    PositionUpdate {
        line_id: usize,
        position: (i32, i32),
    },
    SaveUpdate,
}

impl PreviewService {
    pub fn new(image: RgbaImage) -> (Sender<UpdateRequest>, Receiver<RgbaImage>, PreviewService) {
        let preview = PreviewService::generate_preview(&image);
        let (tx_update, rx_update) = mpsc::channel();
        let (tx_image, rx_image) = mpsc::channel();
        let lines = Vec::new();
        let service = PreviewService {
            original_image: image,
            preview_image: preview,
            rx: rx_update,
            tx: tx_image,
            lines,
        };
        (tx_update, rx_image, service)
    }
    pub fn start(self) {
        thread::spawn(move || {
            // extract the inner fields here so we do not need inner mutability on the `self` struct
            let rx = self.rx;
            let mut lines = self.lines;
            let preview_image = self.preview_image;
            let tx = self.tx;
            let original_image = self.original_image;

            // iterate over all incoming requests
            rx.iter().for_each(|req| {
                match req {
                    UpdateRequest::InitialUpdate { positioned_line } => {
                        lines.push(positioned_line);
                    }
                    UpdateRequest::PositionUpdate { line_id, position } => {
                        match lines.get_mut(line_id) {
                            Some(line) => {
                                line.position = Some((position.0, position.1));
                            }
                            _ => {}
                        }
                    }
                    UpdateRequest::TextUpdate { line_id, new_text } => {
                        match lines.get_mut(line_id) {
                            Some(mut line) => {
                                line.line.text = new_text;
                            }
                            _ => {}
                        }
                    }
                    UpdateRequest::SaveUpdate =>{
                        println!("Would save OI with: {:?}",original_image.dimensions());
                    }
                };

                let (x, y) = preview_image.dimensions();
                let mut result = ImageBuffer::new(x, y);
                result.clone_from(&preview_image);

                //draw all lines (if any)
                lines.iter_mut().for_each(|line| match line.position {
                    Some(pos) => {
                        draw_line_at(&mut line.line, &mut result, pos.0 as f32, pos.1 as f32);
                    }
                    None => {
                        draw_line(&mut line.line, &mut result);
                    }
                });
                let send_res = tx.send(result);
                if send_res.is_err() {
                    println!("Error sending: {}", send_res.unwrap_err())
                }
            });
        });
    }

    pub fn generate_preview(image: &RgbaImage) -> RgbaImage {
        let (x, y) = image.dimensions();
        let cropfactor = match x > y {
            true => 1024.0 / (x as f32),
            _ => 1024.0 / (y as f32),
        };
        let new_x = (x as f32 * cropfactor) as u32;
        let new_y = (y as f32 * cropfactor) as u32;
        let mut preview = DynamicImage::new_rgb8(x, y);
        preview.copy_from(image, 0, 0);
        let preview = preview.resize(new_x, new_y, FilterType::Gaussian);
        preview.to_rgba()
    }
}

#[cfg(test)]
mod tests {
    use crate::service::{PositionedLine, PreviewService, UpdateRequest};
    use crate::Line;
    use image::DynamicImage;
    use std::time::Duration;

    #[test]
    fn test_preview_image_creation() {
        let image = DynamicImage::new_rgb8(1920, 1080).to_rgba();
        let (_, _, preview) = PreviewService::new(image);
        //maxwidth
        assert_eq!(512, preview.preview_image.width());
        //9:16*512=288
        assert_eq!(288, preview.preview_image.height());

        let portrait_image = DynamicImage::new_rgb8(1080, 1920).to_rgba();
        let (_, _, portrait_preview) = PreviewService::new(portrait_image);
        //maxheight
        assert_eq!(512, portrait_preview.preview_image.height());
        //9:16*512=288
        assert_eq!(288, portrait_preview.preview_image.width());
    }

    #[test]
    fn test_service_start_send_receive() {
        let image = DynamicImage::new_rgb8(1920, 1080).to_rgba();
        let (tx, rx, mut preview) = PreviewService::new(image);

        preview.start();

        tx.send(UpdateRequest::InitialUpdate {
            positioned_line: PositionedLine {
                line: Line {
                    text: "Test".to_string(),
                    ..Line::default()
                },
                position: None,
                line_id: 0,
            },
        });

        tx.send(UpdateRequest::PositionUpdate {
            line_id: 0,
            position: (100, 100),
        });

        match rx.recv_timeout(Duration::from_secs(10)) {
            Ok(res) => {
                assert_eq!(512, res.width());
                res.save("test_output/test_service_start_send_receive_1.jpg")
                    .unwrap();
            }
            Err(err) => panic!("Errored on receiving"),
        }

        match rx.recv_timeout(Duration::from_secs(10)) {
            Ok(res) => {
                assert_eq!(512, res.width());
                res.save("test_output/test_service_start_send_receive_2.jpg")
                    .unwrap();
            }
            Err(err) => panic!("Errored on receiving"),
        }
    }
}
