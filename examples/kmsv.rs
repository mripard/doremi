extern crate clap;
extern crate doremi;
extern crate fixed;
extern crate image;

use std::cmp::min;
use std::convert::TryInto;
use std::thread;
use std::time;

use clap::App;
use clap::Arg;
use fixed::types::extra::U16;
use fixed::FixedU32;
use image::GenericImageView;

use doremi::Buffer;
use doremi::BufferType;
use doremi::ConnectorStatus;
use doremi::Device;
use doremi::Format;
use doremi::ModeType;
use doremi::PipelineInit;

struct Image<'a> {
    buffer:    Buffer<'a>,
    width:     usize,
    height:    usize,
    display_w: usize,
    display_h: usize,
    margin_w:  isize,
    margin_h:  isize,
}

fn main() {
    let matches = App::new("Kernel Mode Setting Image Viewer")
        .arg(
            Arg::with_name("device")
                .short("D")
                .help("DRM Device Path")
                .default_value("/dev/dri/card0"),
        )
        .arg(Arg::with_name("images").multiple(true).required(true))
        .get_matches();
    let dev_path = matches.value_of("device").unwrap();
    let dev = Device::new(dev_path).unwrap();
    let img_path = matches.values_of("images").unwrap();

    let connectors = dev.get_connectors().unwrap();
    let connector = connectors
        .into_iter()
        .filter(|con| con.get_status() == ConnectorStatus::Connected)
        .next()
        .unwrap();

    let modes = connector.get_modes().unwrap();
    let mode = modes
        .into_iter()
        .find(|mode| mode.has_type(ModeType::Preferred))
        .unwrap();

    let images: Vec<Image> = img_path
        .map(|path| {
            let img = image::open(path).unwrap();
            let rgb_data = img.to_bgra().into_vec();

            let img_h = img.height().try_into().unwrap();
            let img_w = img.width().try_into().unwrap();

            let buffer = dev
                .allocate_buffer(BufferType::Dumb, img_w, img_h, 32)
                .unwrap()
                .map()
                .unwrap()
                .into_framebuffer(Format::XRGB8888)
                .unwrap();

            let data = buffer.get_data().unwrap();
            data.copy_from_slice(&rgb_data);

            let mode_h_fixed = FixedU32::<U16>::from_num(mode.height());
            let mode_w_fixed = FixedU32::<U16>::from_num(mode.width());

            let scale_h = mode_h_fixed / img_h as u32;
            let scale_w = mode_w_fixed / img_w as u32;
            let scale = min(scale_h, scale_w);

            let display_h: usize = ((img_h as u32) * scale).ceil().to_num();
            let display_w: usize = ((img_w as u32) * scale).ceil().to_num();

            let margin_h = ((mode.height() - display_h) / 2) as isize;
            let margin_w = ((mode.width() - display_w) / 2) as isize;

            Image {
                buffer,
                height: img_h,
                width: img_w,
                display_h,
                display_w,
                margin_h,
                margin_w,
            }
        })
        .collect();

    let first = &images[0];
    let mut pipeline = PipelineInit::new(&dev)
        .discover(connector)
        .unwrap()
        .set_mode(mode)
        .set_buffer(&first.buffer)
        .set_plane_coordinates(first.margin_w, first.margin_h)
        .set_plane_dimensions(first.display_w, first.display_h)
        .build()
        .unwrap();

    let mut index = 1;
    loop {
        let sleep = time::Duration::from_millis(1000);
        thread::sleep(sleep);

        let image = &images[index % images.len()];

        pipeline = pipeline
            .update_buffer(&image.buffer)
            .unwrap()
            .update_plane_size(image.width, image.height)
            .unwrap()
            .update_plane_display_size(image.display_w, image.display_h)
            .unwrap()
            .update_plane_coordinates(image.margin_w, image.margin_h)
            .unwrap()
            .commit()
            .unwrap();

        index = index + 1;
    }
}
