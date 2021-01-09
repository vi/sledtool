use std::{io::Write, path::PathBuf};

/// CLI tool to work with Sled databases
#[derive(argh::FromArgs)]
struct Opts {
    #[argh(positional)]
    dbpath: PathBuf,

    #[argh(subcommand)]
    cmd: Cmd,
}

/// Outout entire content of the database to text file, stdout (suitable for migrations)
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "export")]
struct Export {}

/// Import entire content of the database from text file, stdin (suitable for migrations)
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "import")]
struct Import {}

#[derive(argh::FromArgs)]
#[argh(subcommand)]
enum Cmd {
    Export(Export),
    Import(Import),
}

fn main() -> anyhow::Result<()> {
    let opts: Opts = argh::from_env();

    let db: sled::Db = sled::open(opts.dbpath)?;

    match opts.cmd {
        Cmd::Export(Export {}) => {
            let so = std::io::stdout();
            let so = so.lock();
            let mut so = std::io::BufWriter::with_capacity(8192, so);
            writeln!(so, "{}", r#"{"version":"sledtool0", "data": ["#)?;
            for (a, b, c) in db.export() {
                writeln!(
                    so,
                    " {{\"a\":\"{}\",  \"b\":\"{}\", \"content\":[",
                    hex::encode(a),
                    hex::encode(b)
                )?;
                for (j, q) in c.enumerate() {
                    if j > 0 {
                        write!(so, "  ,[")?;
                    } else {
                        write!(so, "   [")?;
                    }
                    for (i, w) in q.iter().enumerate() {
                        if i > 0 {
                            write!(so, ",")?;
                        }
                        write!(so, "\"{}\"", hex::encode(w))?;
                    }
                    writeln!(so, "]")?;
                }
                writeln!(so, " ]}}")?;
            }
            writeln!(so, "{}", "]}")?;
        }
        Cmd::Import(Import {}) => {todo!()}
    }

    Ok(())
}
