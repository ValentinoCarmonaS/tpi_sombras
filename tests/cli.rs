use std::process::{Command, ExitStatus};
use std::{fs::File};

fn run_main_with_input_file(input_path: &str) -> Result<(ExitStatus, String), &str> {
    let input_file = match File::open(input_path) {
        Ok(file) => file,
        Err(_) => return Err("Error in opening input file")
    };
    
    let output = match Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .stdin(input_file)
        .output() 
        {
            Ok(output) => output,
            Err(_) => return Err("Error in command execution")
        };

    let stdout = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(_) => return Err("Error in converting stdout to string")
    };

    Ok((output.status, stdout))
}

#[test]
fn test01() {
    let expected = "15.000000000000";
    match run_main_with_input_file("./tests/test01.txt"){
        Ok((status, stdout)) => {
            assert!(status.success(), "The program shuld have succeeded");
            assert!(
                stdout.contains(expected),
                "The stdout doesnt contains the expected value. \nStdout: {}Expected: {}", stdout, expected
            );
        }
        Err(e) => panic!("{}", e)
    };
}

#[test]
fn test02() {
    let expected = "446.4101615137755";
    match run_main_with_input_file("./tests/test02.txt"){
        Ok((status, stdout)) => {
            assert!(status.success(), "The program shuld have succeeded");
            assert!(
                stdout.contains(expected),
                "The stdout doesnt contains the expected value. \nStdout: {}Expected: {}", stdout, expected
            );
        }
        Err(e) => panic!("{}", e)
    };
}

#[test]
fn test03() {
    let expected = "300";
    match run_main_with_input_file("./tests/test03.txt"){
        Ok((status, stdout)) => {
            assert!(status.success(), "The program shuld have succeeded");
            assert!(
                stdout.contains(expected),
                "The stdout doesnt contains the expected value. \nStdout: {}Expected: {}", stdout, expected
            );
        }
        Err(e) => panic!("{}", e)
    };
}