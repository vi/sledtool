use std::{io::Write, path::PathBuf};

/// CLI tool to work with Sled databases
#[derive(argh::FromArgs)]
struct Opts {
    #[argh(positional)]
    dbpath: PathBuf,

    /// set `use_compression` in `sled::Config` to true
    #[argh(switch,short='c')]
    compress: bool,

    /// set `compression_factor` in `sled::Config` to specified value
    #[argh(option,short='C')]
    compression_factor: Option<i32>,

    /// set `cache_capacity` in `sled::Config` to the specified value
    #[argh(option,short='P')]
    cache_capacity: Option<u64>,

    /// set `create_new` in `sled::Config` to true, making it a failure to open existing database
    #[argh(switch,short='N')]
    create_new: bool,

    /// set `mode` in `sled::Config` to HighThroughput
    #[argh(switch,short='F')]
    throughput_mode: bool,

    /// set `mode` in `sled::Config` to LowSpace
    #[argh(switch,short='L')]
    low_space: bool,

    #[argh(subcommand)]
    cmd: Cmd,
}

/// Outout entire content of the database to JSON with hex-encoded buffers
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "export")]
struct Export {}

/// Import entire content of the database from JSON with hex-encoded buffers
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "import")]
struct Import {}

/// Get value of specific key (or first/last) from the database
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "get")]
struct Get {
    #[argh(positional)]
    key: String,

    /// tree to use
    #[argh(option, short = 't')]
    tree: Option<String>,

    /// inhibit hex-encoding the value
    #[argh(switch, short = 'r')]
    raw_value: bool,

    /// inhibit hex-decoding or hex-encoding the key
    #[argh(switch, short = 'R')]
    raw_key: bool,

    /// inhibit hex-decoding the tree name
    #[argh(switch, short = 'T')]
    raw_tree_name: bool,

    /// use `get_gt` instead of `get`
    #[argh(switch)]
    gt: bool,

    /// use `get_lt` instead of `get`
    #[argh(switch)]
    lt: bool,

    /// print key in addition to the value, with `=` sign in between
    #[argh(switch, short = 'K')]
    print_key: bool,

    /// do not print `Not found` to console, just set exit code 1
    #[argh(switch, short = 'q')]
    quiet: bool,

    /// ignore key, get first record instead
    #[argh(switch, short = 'f')]
    first: bool,

    /// ignore key, get last record instead
    #[argh(switch, short = 'l')]
    last: bool,
}
/// Set value of specific key in the database
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "set")]
struct Set {
    #[argh(positional)]
    key: String,

    #[argh(positional)]
    value: String,

    /// tree to use
    #[argh(option, short = 't')]
    tree: Option<String>,

    /// inhibit hex-decoding or hex-encoding the value
    #[argh(switch, short = 'r')]
    raw_value: bool,

    /// inhibit hex-decoding the key
    #[argh(switch, short = 'R')]
    raw_key: bool,

    /// inhibit hex-decoding the tree name
    #[argh(switch, short = 'T')]
    raw_tree_name: bool,

    /// do not the old value
    #[argh(switch, short = 'q')]
    quiet: bool,
}

/// Remove specific key or range of keys
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "rm")]
struct Remove {
    #[argh(positional)]
    key: String,

    /// remove range of keys (ignoring missing) up to this one
    #[argh(option, short = 'U')]
    end_key: Option<String>,

    /// tree to use
    #[argh(option, short = 't')]
    tree: Option<String>,

    /// inhibit hex-decoding the key
    #[argh(switch, short = 'R')]
    raw_key: bool,

    /// inhibit hex-decoding the tree name
    #[argh(switch, short = 'T')]
    raw_tree_name: bool,

    /// do not print `Not found` to console, just set exit code 1
    #[argh(switch, short = 'q')]
    quiet: bool,

   /// do not remove `end_key` entry itself, only up to it. 
   #[argh(switch, short = 'r')]
   right_exclusive: bool,

}

/// Open Sled database, then wait indefinitely
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "idle")]
struct Idle {}

/// List tree names
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "treenames")]
struct TreeNames {
    /// inhibit hex-decoding the tree names
    #[argh(switch, short = 'T')]
    raw_tree_names: bool,
}
/// Generate monotonic ID
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "genid")]
struct GenerateId {
}
/// No operation, just open and close the database
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "nop")]
struct Noop {
    /// wait to some stdin input before exiting from the program
    #[argh(switch, short = 'w')]
    wait: bool,
}

/// Call `checksum` and output the result
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "checksum")]
struct Checksum {
}

/// Call `size_on_disk` and output the result
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "sizeondisk")]
struct SizeOnDisk {
}

#[derive(argh::FromArgs)]
#[argh(subcommand)]
enum Cmd {
    Export(Export),
    Import(Import),
    Get(Get),
    Set(Set),
    Remove(Remove),
    Noop(Noop),
    Idle(Idle),
    TreeNames(TreeNames),
    GenerateId(GenerateId),
    Checksum(Checksum),
    SizeOnDisk(SizeOnDisk),
}

pub mod sledimporter;

fn main() -> anyhow::Result<()> {
    let opts: Opts = argh::from_env();

    let mut config = sled::Config::default().path(opts.dbpath);

    if opts.compress {
        config = config.use_compression(true);
    }
    if let Some(x) = opts.compression_factor {
        config = config.compression_factor(x);
    }
    if opts.create_new {
        config = config.create_new(true);
    }
    if opts.low_space {
        config = config.mode(sled::Mode::LowSpace);
    }
    if opts.throughput_mode {
        config = config.mode(sled::Mode::HighThroughput);
    }
    if let Some(x) = opts.cache_capacity {
        config = config.cache_capacity(x);
    }

    let db: sled::Db = config.open()?;

    match opts.cmd {
        Cmd::Export(Export {}) => {
            let so = std::io::stdout();
            let so = so.lock();
            let mut so = std::io::BufWriter::with_capacity(8192, so);

            writeln!(so, "{{")?;
            for (tn, tree_name) in db.tree_names().into_iter().enumerate() {
                if tn > 0 {
                    write!(so, ",")?;
                } else {
                    write!(so, " ")?;
                }
                writeln!(so, r#""{}":{{"#, hex::encode(&tree_name))?;
                let tree = db.open_tree(&tree_name)?;
                for (vn, x) in tree.into_iter().enumerate() {
                    if vn > 0 {
                        write!(so, " ,")?;
                    } else {
                        write!(so, "  ")?;
                    }
                    let (k, v) = x?;
                    writeln!(so, r#""{}": "{}""#, hex::encode(k), hex::encode(v))?;
                }
                writeln!(so, " }}")?;
            }
            writeln!(so, "}}")?;
        }
        Cmd::Import(Import {}) => {
            let si = std::io::stdin();
            let si = si.lock();
            let si = std::io::BufReader::with_capacity(8192, si);
            use serde::de::DeserializeSeed;
            let () = sledimporter::DbDeserializer(&db)
                .deserialize(&mut serde_json::Deserializer::from_reader(si))?;
        }
        Cmd::Get(Get {
            key,
            tree,
            raw_value,
            raw_key,
            raw_tree_name,
            gt,
            lt,
            print_key,
            quiet,
            first,
            last,
        }) => {
            if lt && gt {
                anyhow::bail!("--gt and --lt options are specified simultaneously");
            }
            if first && last && !quiet {
                eprintln!("Warning: with both --first and --last options active it would only succeed with exactly one record in the tree.");
            }
            if (first || last) && ! key.is_empty() && !quiet {
                eprintln!("Specifying non-empty (`\"\"`) key with --first or --last asserts it is indeed that key and fails otherwise.");
            }
            let mut t: &sled::Tree = &db;
            let tree_buf;
            if let Some(tree_name) = tree {
                let tn = if raw_tree_name {
                    tree_name.as_bytes().to_vec()
                } else {
                    hex::decode(tree_name)?
                };
                tree_buf = db.open_tree(tn)?;
                t = &tree_buf;
            }

            let mut k = if raw_key {
                key.as_bytes().to_vec()
            } else {
                hex::decode(key)?
            };

            let v: Option<_>;
            match (first, last) {
                (true, true) => {
                    let x1 = t.first()?;
                    let x2 = t.last()?;
                    v = if let (Some(x1), Some(x2)) = (x1,x2) {
                        let kd = x1.0.to_vec();
                        if k.is_empty() {
                            if x1.0 == x2.0 {
                                k = kd;
                                Some(x1.1)
                            } else {
                                None
                            }
                        } else if kd == k && x1.0 == x2.0  {
                            Some(x1.1)
                        } else {
                            None
                        }
                    } else {
                        // empty tree
                        None
                    };
                }
                (true, false) | (false, true) => {
                    let x = if first {
                        t.first()?
                    } else {
                        t.last()?
                    };
                    v = if let Some(x) = x {  
                        let kd = x.0.to_vec();
                        if k.is_empty() {
                            k = kd;
                            Some(x.1)
                        } else if k == kd {
                            Some(x.1)
                        } else {
                            // found, but key not matches the one specified by user
                            None
                        }
                    } else {
                        // not found, empty tree
                        None
                    };
                }
                (false, false) => {
                    match (lt, gt) {
                        (false, false) => v = t.get(&k)?,
                        (true, false) => {
                            v = t.get_lt(&k)?.map(|(ke, va)| {
                                k = ke.to_vec();
                                va
                            })
                        }
                        (false, true) => {
                            v = t.get_gt(&k)?.map(|(ke, va)| {
                                k = ke.to_vec();
                                va
                            })
                        }
                        (true, true) => unreachable!(),
                    }
                }
            };

            if let Some(v) = v {
                if print_key {
                    if raw_key {
                        print!("{}=", String::from_utf8_lossy(&k));
                    } else {
                        print!("{}=", hex::encode(k));
                    }
                }
                if raw_value {
                    println!("{}", String::from_utf8_lossy(&v));
                } else {
                    println!("{}", hex::encode(v));
                }
            } else {
                if !quiet {
                    eprintln!("Not found");
                }
                std::process::exit(1);
            }
        }
        Cmd::Set(Set {
            key,
            value,
            tree,
            raw_value,
            raw_key,
            raw_tree_name,
            quiet,
        }) => {
            let mut t: &sled::Tree = &db;
            let tree_buf;
            if let Some(tree_name) = tree {
                let tn = if raw_tree_name {
                    tree_name.as_bytes().to_vec()
                } else {
                    hex::decode(tree_name)?
                };
                tree_buf = db.open_tree(tn)?;
                t = &tree_buf;
            }

            let k = if raw_key {
                key.as_bytes().to_vec()
            } else {
                hex::decode(key)?
            };

            let v = if raw_value {
                value.as_bytes().to_vec()
            } else {
                hex::decode(value)?
            };

            let ov = t.insert(k, v)?;

            if let Some(ov) = ov {
                if !quiet {
                    if raw_value {
                        println!("{}", String::from_utf8_lossy(&ov));
                    } else {
                        println!("{}", hex::encode(ov));
                    }
                }
            }
        }
        Cmd::Remove(Remove { key, end_key, tree, raw_key, raw_tree_name , quiet, right_exclusive}) => {
            let mut t: &sled::Tree = &db;
            let tree_buf;
            if let Some(tree_name) = tree {
                let tn = if raw_tree_name {
                    tree_name.as_bytes().to_vec()
                } else {
                    hex::decode(tree_name)?
                };
                tree_buf = db.open_tree(tn)?;
                t = &tree_buf;
            }

            let k = if raw_key {
                key.as_bytes().to_vec()
            } else {
                hex::decode(key)?
            };

            let endkey = match end_key {
                None => None,
                Some(ek) => {
                    Some(if raw_key {
                        ek.as_bytes().to_vec()
                    } else {
                        hex::decode(ek)?
                    })
                }
            };

            if let Some(ek) = endkey {
                let iter = if right_exclusive {
                    t.range(k..ek)
                } else {
                    t.range(k..=ek)
                };
                let mut ctr = 0usize;
                for x in iter {
                    let x = x?;
                    if t.remove(x.0)?.is_some() {
                        ctr += 1;
                    }
                }
                if !quiet {
                    println!("{} entries removed", ctr);
                }
            } else if t.remove(k)?.is_some() {
                // OK
            } else {
                if !quiet {
                    eprintln!("Not found");
                }
                std::process::exit(1);
            }

        }
        Cmd::Idle(Idle {}) => loop {
            std::thread::sleep(std::time::Duration::from_secs(3600));
        },
        Cmd::Noop(Noop {wait}) => {
            if wait {
                let _ = std::io::stdin().read_line(&mut String::new());
            }
        }
        Cmd::TreeNames(TreeNames { raw_tree_names }) => {
            for tree_name in db.tree_names() {
                if raw_tree_names {
                    println!("{}", String::from_utf8_lossy(&tree_name));
                } else {
                    println!("{}", hex::encode(tree_name));
                }
            }
        }
        Cmd::GenerateId(GenerateId{}) => {
            println!("{}", db.generate_id()?);
        }
        Cmd::Checksum(Checksum{}) => {
            println!("{}", db.checksum()?);
        }
        Cmd::SizeOnDisk(SizeOnDisk{}) => {
            println!("{}", db.size_on_disk()?);
        }
    }

    Ok(())
}
