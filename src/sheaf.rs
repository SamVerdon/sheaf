use std::io::{self, Write};

pub struct Document {
    meta: MetaData,
    index: Index,
    files: Vec<SrcFile>,
}

impl Document {
    pub fn new(meta: MetaData, index: Index, files: Vec<SrcFile>) -> Self {
        Self { meta, index, files }
    }
}

pub struct MetaData {
    title: String,
    // TODO(sv) add more metadata pulling from project files
}

impl MetaData {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

pub struct SrcFile {
    path: String,
    ext: String,
    content: String,
}

pub type Index = Vec<String>;

impl SrcFile {
    pub fn new(path: String, ext: String, content: String) -> Self {
        Self { path, ext, content }
    }
}

pub trait Render {
    fn render<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}

impl Render for MetaData {
    fn render<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "# {}\n", self.title)?;
        Ok(())
    }
}

impl Render for SrcFile {
    fn render<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "### `{}`\n", self.path)?;
        writeln!(writer, "```{}\n{}\n```", self.ext, self.content)?;
        Ok(())
    }
}

impl Render for Document {
    fn render<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.meta.render(writer)?;
        // Can't impl `Render` for `Index` as its an alias so we render here
        writeln!(writer, "## Index\n```text")?;
        for path in &self.index {
            writeln!(writer, "{}", path)?;
        }
        writeln!(writer, "```\n")?;
        writeln!(writer, "## Source Files\n")?;
        for src_file in &self.files {
            src_file.render(writer)?;
        }
        Ok(())
    }
}
