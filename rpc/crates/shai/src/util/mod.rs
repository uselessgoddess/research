mod extensions;

use std::pin::Pin;

pub use extensions::Extensions;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
