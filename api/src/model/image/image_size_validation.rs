#[derive(Debug, PartialEq)]
pub enum ImageSizeValidation {
    Size((u32, u32)),
    Vertical,
    None,
}
