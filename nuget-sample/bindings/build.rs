winrt::build!(
    dependencies
        nuget: Microsoft.Windows.SDK.Contracts
    types
        windows::data::xml::dom::*
);

fn main() {
    build();
}
