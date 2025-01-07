// Mevcut string birleştirme fonksiyonu
fn concatenate_strings(str1: &str, str2: &str) -> String {
  let mut result = String::new();
  result.push_str(str1);
  result.push_str(str2);
  result
}

// Operation enum'u
enum Operation {
  Add(f64, f64),
  Subtract(f64, f64),
  Multiply(f64, f64),
  Divide(f64, f64),
}

// Hesaplama fonksiyonu
fn calculate(operation: Operation) -> f64 {
  match operation {
      Operation::Add(a, b) => a + b,
      Operation::Subtract(a, b) => a - b,
      Operation::Multiply(a, b) => a * b,
      Operation::Divide(a, b) => {
          if b != 0.0 {
              a / b
          } else {
              panic!("Division by zero is not allowed!")
          }
      }
  }
}

fn main() {
  // 1. Mevcut string birleştirme işlevi
  let string1 = String::from("Hello, ");
  let string2 = String::from("world!");
  let concatenated_string = concatenate_strings(&string1, &string2);
  println!("{}", concatenated_string);

  // 2. Yeni hesap makinesi özelliği
  println!("Enter the first number:");
  let mut input1 = String::new();
  std::io::stdin().read_line(&mut input1).unwrap();
  let num1: f64 = input1.trim().parse().expect("Invalid input!");

  println!("Enter the operator (+, -, *, /):");
  let mut operator = String::new();
  std::io::stdin().read_line(&mut operator).unwrap();
  let operator = operator.trim();

  println!("Enter the second number:");
  let mut input2 = String::new();
  std::io::stdin().read_line(&mut input2).unwrap();
  let num2: f64 = input2.trim().parse().expect("Invalid input!");

  let operation = match operator {
      "+" => Operation::Add(num1, num2),
      "-" => Operation::Subtract(num1, num2),
      "*" => Operation::Multiply(num1, num2),
      "/" => Operation::Divide(num1, num2),
      _ => panic!("Unsupported operator!"),
  };

  let result = calculate(operation);
  println!("Result: {}", result);
}
