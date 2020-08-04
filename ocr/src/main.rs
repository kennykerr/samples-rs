use bindings::*;
use windows::graphics::imaging::*;
use windows::media::ocr::*;
use windows::storage::*;

fn main() -> winrt::Result<()> {
    futures::executor::block_on(main_async())
}

async fn main_async() -> winrt::Result<()> {
    let mut message = std::env::current_dir().unwrap();
    message.push("message.png");

    let file = StorageFile::get_file_from_path_async(message.to_str().unwrap())?.await?;
    let stream = file.open_async(FileAccessMode::Read)?.await?;

    let decode = BitmapDecoder::create_async(stream)?.await?;
    let bitmap = decode.get_software_bitmap_async()?.await?;

    let engine = OcrEngine::try_create_from_user_profile_languages()?;
    let result = engine.recognize_async(bitmap)?.await?;

    println!("{}", result.text()?);
    Ok(())
}
