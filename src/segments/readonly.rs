use std::{ffi::CString, marker::PhantomData};

use crate::{part::*, powerline::*, terminal::Color, R};

pub struct ReadOnly<S>(PhantomData<S>);

pub trait ReadOnlyScheme {
	const READONLY_FG: Color;
	const READONLY_BG: Color;
	const READONLY_SYMBOL: &'static str = "";
}
impl<S: ReadOnlyScheme> ReadOnly<S> {
	pub fn new() -> ReadOnly<S> {
		ReadOnly(PhantomData)
	}
}

impl<S: ReadOnlyScheme> Part for ReadOnly<S> {
	fn append_segments(&self, segments: &mut Vec<Segment>) -> R<()> {
		let readonly = unsafe {
			let path = CString::new("./")?;
			libc::access(path.as_ptr(), libc::W_OK) != 0
		};

		if readonly {
			segments.push(Segment::simple(&format!(" {} ", S::READONLY_SYMBOL), S::READONLY_FG, S::READONLY_BG));
		}
		Ok(())
	}
}
