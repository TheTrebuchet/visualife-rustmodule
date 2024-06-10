pub trait ToSvg: Send + Sync{
    fn to_svg(&self) -> String;
}