winrt::build!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    types
        windows::storage::streams::*
        windows::graphics::imaging::*
        windows::media::ocr::*
);

fn main() {
    build();
}
