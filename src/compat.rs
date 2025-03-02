
#[cfg(not(feature = "unicode"))]
pub mod symbols{
    pub const UP_ARROW:&str = "^";
    pub const DOWN_ARROW:&str = "v";
}
#[cfg(feature = "unicode")]
pub mod symbols{
    pub const UP_ARROW:&str = "↑";
    pub const DOWN_ARROW:&str = "↓";
}
