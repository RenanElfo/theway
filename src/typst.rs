use chrono::{self, Datelike};
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::layout::PagedDocument;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World, compile};
use typst_pdf::{PdfOptions, pdf};

struct StringWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    main: Source,
    fonts: Vec<Font>,
}

impl StringWorld {
    fn new(content: String, font_data: Vec<u8>) -> Self {
        let font = Font::new(Bytes::new(font_data), 0).unwrap();

        let mut book = FontBook::new();
        book.push(font.info().clone());

        Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(book),
            main: Source::new(FileId::new(None, VirtualPath::new("main.typ")), content),
            fonts: vec![font],
        }
    }
}

impl World for StringWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main.id() {
            Ok(self.main.clone())
        } else {
            Err(FileError::NotFound(id.vpath().as_rooted_path().into()))
        }
    }

    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound("file".into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        let current_date: chrono::NaiveDate = chrono::Local::now().date_naive();
        let (year, month, day) = (
            current_date.year(),
            current_date.month(),
            current_date.day(),
        );
        Datetime::from_ymd(year, month as u8, day as u8)
    }
}

pub fn to_image() {
    let typst_code = include_str!("../typst/template.typ");
    let font_bytes = std::fs::read("Philosopher-Regular.ttf").unwrap();
    let world = StringWorld::new(typst_code.to_string(), font_bytes);
    let document = compile::<PagedDocument>(&world).output.unwrap();
    let pdf_bytes = pdf(&document, &PdfOptions::default()).unwrap();
    std::fs::write("output.pdf", pdf_bytes).unwrap();
}
