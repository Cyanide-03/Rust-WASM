use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate(num1 : f64, num2:f64, op:&str) -> String {
    let result=match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2==0.0 {
                return "Error: Division by zero".to_string();
            }
            num1 / num2
        },
        _ => return "Error: Invalid operator".to_string(),
    };
    format!("Result: {:.2}", result)
}
