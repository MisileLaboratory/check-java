use std::process::Command;

enum JavaErrors {
    NoJava
}

fn check_java<T>(path: T) -> Result<String, JavaErrors>
where T: ToString {
    let result = match Command::new("java").arg("version").spawn() {
        Ok(a) => a,
        Err(_) => return Err(JavaErrors::NoJava)
    };
}