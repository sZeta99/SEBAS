use std::process::{Command, Stdio};

pub fn preseed(command: &str) -> Result<(), std::io::Error> {

    Command::new("bash")

        .arg("-c")

        .arg(format!("echo '{}' | perl -e 'ioctl STDOUT, 0x5412, $_ for split //, do{{ chomp($_ = <>); $_ }}'", command))

        .stdin(Stdio::inherit())

        .stdout(Stdio::inherit())

        .stderr(Stdio::inherit())

        .spawn()?

        .wait()?;

    Ok(())

}