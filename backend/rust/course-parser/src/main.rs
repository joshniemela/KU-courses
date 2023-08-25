use storage_manager::{self, LocalStorageConfig, LocalStorage, Storage};
use clap::Parser;
pub mod parser;
use unicase::UniCase;

const DATA_DIR: &str = "../../../data";

const TEST_DIR: &str ="./test_data";

static WHITE_LIST: [&str; 2] = [
    "NMAA",
    "NDAB"
    ];

#[derive(Parser)]
struct CliArgs {
    dir: Option<String>,
}

fn main() {
    println!("Starting course parser...");

    let mut white_list: Vec<unicase::UniCase<&str>> = Vec::new();
    for x in WHITE_LIST.iter() {
        white_list.push(UniCase::new(x));
    }

    // Collecting commandline args to enable switching between the different directories
    // Right now we just treat the first variable as the indication to what dir (i.e. DATA_DIR or
    // TEST_DIR).
    // I.e. if you wish to use the TEST_DIR (defined above), you pass TEST_DIR as the first cli
    // argument. if nothing gets passed in we use the DATA_DIR (to maintain default functionality).
    let args = CliArgs::parse();

    // Configuration variables
    let root = args.dir.map_or_else(|| String::from(DATA_DIR), |dir| {
        match dir.as_str() {
            "TEST_DIR" => String::from(TEST_DIR),
            _ => String::from(DATA_DIR)
        }
    });
        
    let conf = LocalStorageConfig{ root };
    let search_depth = 0;

    // We create the storage, and if succesful we start parsing
    LocalStorage::new(conf).map_or_else(|error| {
        eprintln!("Failed while creating storage, got err: {error}");
        println!("exiting ...");
    }, |storage| {
        // time how long it takes to run this
        let start = std::time::Instant::now();
        match storage.list("pages", &search_depth) {

            // If we get back any filenames we can continue.
            Ok(filenames) => {
                let mut fails = 0;
                let mut passes = 0;

                // count the number of errors in a dictionary
                let mut errors: std::collections::HashMap<String, u32> =
                    std::collections::HashMap::new();

                for filename in filenames {
                    // For each filename, we try to read it, if we succeed we then try to parse it.
                    if let Some(course_code) = filename.get(6..10) {
                        let course_code_uni = UniCase::new(course_code);
                        if white_list.iter().any(|&x| x == course_code_uni) {
                            storage.read(filename.as_str()).map_or_else(|_| {
                                println!("Couldn't read file associated with {filename}");
                            }, |f| {
                                let result = parser::parse_course(&f);
                                match result {
                                    Ok(_) => passes += 1,
                                    Err(err) => {
                                        fails += 1;
                                        let err_string = format!("{err}");
                                        let count = errors.entry(err_string).or_insert(0);
                                        *count += 1;
                                        // Print out information on the file we failed to parse
                                        println!("Failed on course: {filename}");
                                        println!("Error: {err}");
                                    }
                                }
                            });
                        }
                    }
                }
                
                // Summary information
                println!(
                    "{} passes, {} fails\nparsed: {:.0}%",
                    passes,
                    fails,
                    f64::from(passes) / f64::from(passes + fails)
                );
                for (err, count) in &errors {
                    // print raw (without the newlines)
                    println!("{}: {}\n", err.replace('\n', "\\n"), count);
                }
            }
            Err(err) => eprintln!("Error: {err}"),
        }
        println!("Time elapsed: {:.2?}", start.elapsed());
    });
    
}
