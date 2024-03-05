pub fn start() {
  let lines = std::io::stdin().lines();
  for line in lines {
    let line = line.unwrap();
    println!("here's a line: {line}");
  }
}
