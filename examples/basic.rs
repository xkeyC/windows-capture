use std::{
    io::{self, Write}
};

use windows_capture::{
    capture::GraphicsCaptureApiHandler,
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};
use windows_capture::encoder::ImageEncoder;
use windows_capture::frame::ImageFormat;

// This struct will be used to handle the capture events.
struct Capture {
    // The video encoder that will be used to encode the frames.
    encoder: Option<ImageEncoder>,
}

impl GraphicsCaptureApiHandler for Capture {
    // The type of flags used to get the values from the settings.
    type Flags = String;

    // The type of error that can occur during capture, the error will be returned from `CaptureControl` and `start` functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create the struct. The flags can be passed from settings.
    fn new(message: Self::Flags) -> Result<Self, Self::Error> {
        println!("Got The Flag: {message}");
        let encoder = ImageEncoder::new(
            ImageFormat::JpegXr,
            ColorFormat::Rgba16F,
        );
        Ok(Self {
            encoder: Some(encoder)
        })
    }

    // Called every time a new frame is available.
    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        io::stdout().flush()?;
        capture_control.stop();
        // Send the frame to the video encoder
        let mut buffer = frame.buffer().unwrap();
        let frame_width = buffer.width();
        let frame_height = buffer.height();
        let raw_buffer = buffer.as_raw_buffer();
        // write buffer to file
        let encoder = self.encoder.as_mut().unwrap();
        let jxr_data = encoder.encode(raw_buffer, frame_width, frame_height)?;
        // write to file
        let mut file = std::fs::File::create("frame.jxr")?;
        file.write_all(&jxr_data)?;
        // TODO convert to AVIF image ...
        Ok(())
    }

    // Optional handler called when the capture item (usually a window) closes.
    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture Session Closed");

        Ok(())
    }
}

fn main() {
    // Gets The Foreground Window, Checkout The Docs For Other Capture Items
    let primary_monitor = Monitor::primary().expect("There is no primary monitor");

    let settings = Settings::new(
        // Item To Captue
        primary_monitor,
        // Capture Cursor Settings
        CursorCaptureSettings::WithCursor,
        // Draw Borders Settings
        DrawBorderSettings::WithBorder,
        // The desired color format for the captured frame.
        ColorFormat::Rgba16F,
        // Additional flags for the capture settings that will be passed to user defined `new` function.
        "".to_string(),
    );

    // Starts the capture and takes control of the current thread.
    // The errors from handler trait will end up here
    Capture::start(settings).expect("Screen Capture Failed");
}
