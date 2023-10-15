mod body;
mod children;
mod head;
mod navbar;
mod page;

pub mod Layout {
    pub use super::body::Body;
    pub use super::children::Children;
    pub use super::head::Head;
    pub use super::navbar::Navbar;
    pub use super::page::Page;
}
