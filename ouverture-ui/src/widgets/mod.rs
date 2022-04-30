use iced_wgpu::Renderer;

mod renderer;
pub mod style;
mod widget;

pub use widget::header;
pub use widget::table_row;

pub type Header<'a, Message> = widget::header::Header<'a, Message, Renderer>;
pub type TableRow<'a, Message> = widget::table_row::TableRow<'a, Message, Renderer>;
