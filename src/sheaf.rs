use std::io::{self, Write};

pub enum Sheaf {
    Project {
        front_matter: FrontMatter,
        register: Register,
        leaves: Vec<Leaf>,
    },
    Collection(Vec<Leaf>),
}

impl Sheaf {
    pub fn project(front_matter: FrontMatter, register: Register, leaves: Vec<Leaf>) -> Self {
        Self::Project {
            front_matter,
            register,
            leaves,
        }
    }

    pub fn collection(leaves: Vec<Leaf>) -> Self {
        Self::Collection(leaves)
    }
}

pub struct FrontMatter {
    title: String,
    // TODO(sv) add more metadata pulling from project files
}

impl FrontMatter {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

pub struct Leaf {
    path: String,
    ext: String,
    content: String,
}

pub type Register = Vec<String>;

impl Leaf {
    pub fn new(path: String, ext: String, content: String) -> Self {
        Self { path, ext, content }
    }
}

pub trait Bind {
    fn bind<W: Write>(&self, writer: &mut W) -> io::Result<()>;
}

impl Bind for FrontMatter {
    fn bind<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "# {}\n", self.title)?;
        Ok(())
    }
}

impl Bind for Leaf {
    fn bind<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "### `{}`\n", self.path)?;
        writeln!(writer, "```{}\n{}\n```", self.ext, self.content)?;
        Ok(())
    }
}

impl Bind for Sheaf {
    fn bind<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Sheaf::Project {
                front_matter,
                register,
                leaves,
            } => {
                front_matter.bind(writer)?;
                // Can't impl `Render` for `Index` as its an alias so we render here
                writeln!(writer, "## Index\n```text")?;
                for path in register {
                    writeln!(writer, "{}", path)?;
                }
                writeln!(writer, "```\n")?;
                writeln!(writer, "## Source Files\n")?;
                for src_file in leaves {
                    src_file.bind(writer)?;
                }
            }
            Sheaf::Collection(leaves) => {
                for src_file in leaves {
                    src_file.bind(writer)?;
                }
            }
        }
        Ok(())
    }
}
