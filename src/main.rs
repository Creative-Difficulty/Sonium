use sonium::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = r#"
name: CI
on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Run a one-line script
      run: echo Hello, world!
    "#;

    let workflow: parser::Workflow = serde_yml::from_str(yaml)?;
    println!("{:#?}", workflow);

    Ok(())
}
