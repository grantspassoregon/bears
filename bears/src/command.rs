/// Variants of the `Action` enum encapsulate the different actions a user can select, exposing the
/// different capabilities of the library.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::FromStr)]
pub enum Action {
    Load,
    Download,
    NextError,
    Queue,
}
