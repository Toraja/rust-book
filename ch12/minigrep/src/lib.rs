use std::env;
use std::error::Error;
use std::fs;
use std::io;

pub fn run(args: Vec<String>, out: &mut impl io::Write) -> Result<(), Box<dyn Error>> {
    let args = CmdArgs::new(&args)?;
    let contents = get_file_contents(&args.filename)?;
    let matches = if args.case_sensitive {
        search(&args.query, &contents)
    } else {
        search_case_insensitive(&args.query, &contents)
    };
    for mat in matches {
        writeln!(out, "{}", mat)?;
    }
    Ok(())
}

#[derive(Debug, PartialEq)]
struct CmdArgs {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl CmdArgs {
    const NUM_ARGS: usize = 2;
    fn new(args: &[String]) -> Result<Self, ArgumentError> {
        if args.len() != Self::NUM_ARGS {
            return Err(ArgumentError::InvalidNumberOfArguments);
        }

        Ok(Self {
            query: args[0].clone(),
            filename: args[1].clone(),
            case_sensitive: Self::is_case_sensitive(),
        })
    }

    fn is_case_sensitive() -> bool {
        let result = env::var("CASE_INSENSITIVE");
        if result.is_err() {
            return true;
        }
        let val = result.unwrap();
        if val == "0" {
            return true;
        }
        false
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
enum ArgumentError {
    #[error("Number of arguments must be {}", CmdArgs::NUM_ARGS)]
    InvalidNumberOfArguments,
}

fn get_file_contents(filepath: &str) -> io::Result<String> {
    fs::read_to_string(filepath)
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matched = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matched.push(line);
        }
    }
    matched
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matched = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matched.push(line);
        }
    }
    matched
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEXT: &str = "I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
";

    #[test]
    fn run_case_sensitive() {
        let args = vec![
            "to".to_string(),
            get_test_resource_path("poem.txt").to_string(),
        ];
        let mut output = Vec::new();
        run(args, &mut output).unwrap();
        assert_eq!(
            output,
            b"Are you nobody, too?\n\
How dreary to be somebody!\n",
            "actual: {}",
            String::from_utf8_lossy(&output)
        );
    }

    #[test]
    fn run_case_insensitive() {
        let args = vec![
            "to".to_string(),
            get_test_resource_path("poem.txt").to_string(),
        ];
        let mut output = Vec::new();
        env::set_var("CASE_INSENSITIVE", "1");
        run(args, &mut output).unwrap();
        env::remove_var("CASE_INSENSITIVE");
        assert_eq!(
            output,
            b"Are you nobody, too?\n\
How dreary to be somebody!\n\
To tell your name the livelong day\n\
To an admiring bog!\n",
            "actual: {}",
            String::from_utf8_lossy(&output)
        );
    }

    #[test]
    fn run_fails() {
        let args = vec![];
        let result = run(args, &mut io::stdout()).unwrap_err();
        let argument_error = result.downcast::<ArgumentError>().unwrap();
        assert_eq!(*argument_error, ArgumentError::InvalidNumberOfArguments);
        // The below works as well but does not check the value inside enum,
        // which, in this case, does not matter as the enum does not have value.
        // assert!(result.is::<ArgumentError>());
    }

    #[test]
    fn args_constructor_works() {
        let (arg1, arg2) = ("arg1".to_string(), "arg2".to_string());
        let result = CmdArgs::new(&vec![arg1.clone(), arg2.clone()]).unwrap();
        let expected = CmdArgs {
            query: arg1,
            filename: arg2,
            case_sensitive: true,
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn args_constructor_error_when_too_many_arguents() {
        let result = CmdArgs::new(&vec![
            "aaa".to_string(),
            "bbb".to_string(),
            "ccc".to_string(),
        ])
        .unwrap_err();
        let expected = ArgumentError::InvalidNumberOfArguments;
        assert_eq!(expected, result);
    }

    #[test]
    fn args_constructor_error_when_too_few_arguents() {
        let result = CmdArgs::new(&vec!["aaa".to_string()]).unwrap_err();
        let expected = ArgumentError::InvalidNumberOfArguments;
        assert_eq!(expected, result);
    }

    #[test]
    fn get_file_contents_works() {
        let filepath = get_test_resource_path("poem.txt");
        let contents = get_file_contents(&filepath).unwrap();
        let expected = TEXT;
        assert_eq!(expected, contents);
    }

    use std::path::PathBuf;
    fn get_test_resource_root_path_buf() -> PathBuf {
        let mut cargo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        cargo_root.push("resources/test");
        cargo_root
    }

    fn get_test_resource_path(subpath: &str) -> String {
        let mut test_resource_path = get_test_resource_root_path_buf();
        test_resource_path.push(subpath);
        test_resource_path
            .to_str()
            .unwrap_or_else(|| panic!("Test resource file {} was not found.", subpath))
            .to_string()
    }

    #[test]
    fn search_works() {
        let query = "ar";
        let contents = TEXT;

        assert_eq!(
            vec!["I'm nobody! Who are you?", "How dreary to be somebody!"],
            search(query, contents)
        );
    }

    #[test]
    fn search_case_insensitive_works() {
        let query = "ar";
        let contents = TEXT;

        assert_eq!(
            vec![
                "I'm nobody! Who are you?",
                "Are you nobody, too?",
                "How dreary to be somebody!"
            ],
            search_case_insensitive(query, contents)
        );
    }
}
