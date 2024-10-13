use std::collections::VecDeque;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

// Основна функція для обчислення виразу
#[wasm_bindgen]
pub fn calculate(expression: &str) -> String {
    match evaluate_expression(expression) {
        Ok(result) => result.to_string(), // Повертаємо результат у вигляді рядка
        Err(e) => e, // Повертаємо помилку
    }
}

// Перелічення токенів, які можуть бути числом, оператором або дужкою
#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Operator(char),
    LeftParen,
    RightParen,
}

// Функція для оцінки математичного виразу
fn evaluate_expression(expr: &str) -> Result<f64, String> {
    let tokens = tokenize(expr)?; // Токенізуємо вираз
    let rpn = to_rpn(tokens)?; // Перетворюємо токени в постфіксну нотацію
    evaluate_rpn(&rpn) // Оцінюємо постфіксний вираз
}

// Перетворення виразу у список токенів
fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new(); // Вектор для зберігання токенів
    let mut chars = expr.chars().peekable(); // Ітератор по символах виразу

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' => { // Обробка чисел і десяткових крапок
                let mut number_str = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_digit(10) || digit == '.' {
                        number_str.push(digit);
                        chars.next(); // Продовжуємо читати число
                    } else {
                        break; // Вихід з циклу, якщо символ не число
                    }
                }
                // Перетворення рядка в число
                let number = f64::from_str(&number_str).map_err(|_| format!("Невірне число: {}", number_str))?;
                tokens.push(Token::Number(number)); // Додаємо токен числа
            }
            '+' | '-' | '*' | '/' => { // Обробка операторів
                tokens.push(Token::Operator(c)); // Додаємо токен оператора
                chars.next(); // Переходимо до наступного символу
            }
            '(' => {
                tokens.push(Token::LeftParen); // Додаємо ліву дужку
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen); // Додаємо праву дужку
                chars.next();
            }
            ' ' => {
                chars.next(); // Пропускаємо пробіли
            }
            _ => return Err(format!("Невідомий символ: {}", c)), // Обробка невідомих символів
        }
    }
    Ok(tokens) // Повертаємо токени
}

// Пріоритет операцій
fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1, // Низький пріоритет
        '*' | '/' => 2, // Високий пріоритет
        _ => 0, // Невідомий оператор
    }
}

// Перетворення інфіксного виразу у постфіксну нотацію (Reverse Polish Notation - RPN)
fn to_rpn(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output = Vec::new(); // Вектор для виходу
    let mut operators = VecDeque::new(); // Дек для операторів

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token), // Додаємо числа до виходу
            Token::Operator(op) => {
                // Обробка операторів за пріоритетом
                while let Some(Token::Operator(top_op)) = operators.back() {
                    if precedence(*top_op) >= precedence(op) {
                        output.push(operators.pop_back().unwrap()); // Додаємо верхній оператор
                    } else {
                        break; // Вихід з циклу, якщо пріоритет нижчий
                    }
                }
                operators.push_back(Token::Operator(op)); // Додаємо новий оператор
            }
            Token::LeftParen => operators.push_back(Token::LeftParen), // Додаємо ліву дужку
            Token::RightParen => {
                // Обробка правої дужки
                while let Some(op) = operators.pop_back() {
                    if let Token::LeftParen = op {
                        break; // Вихід, якщо досягли лівої дужки
                    } else {
                        output.push(op); // Додаємо оператор до виходу
                    }
                }
            }
        }
    }

    // Додаємо залишилися оператори до виходу
    while let Some(op) = operators.pop_back() {
        if let Token::LeftParen = op {
            return Err("Незакриті дужки.".to_string()); // Помилка для незакритих дужок
        }
        output.push(op);
    }

    Ok(output) // Повертаємо постфіксну нотацію
}

// Оцінка постфіксного виразу (RPN)
fn evaluate_rpn(rpn: &[Token]) -> Result<f64, String> {
    let mut stack = Vec::new(); // Стек для обчислень

    for token in rpn {
        match token {
            Token::Number(num) => stack.push(*num), // Додаємо число до стека
            Token::Operator(op) => {
                // Обробка операторів
                if stack.len() < 2 {
                    return Err("Недостатньо операндів для виконання операції.".to_string());
                }
                let b = stack.pop().unwrap(); // Другий операнд
                let a = stack.pop().unwrap(); // Перший операнд
                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => {
                        if b == 0.0 {
                            return Err("Помилка: Ділення на нуль.".to_string()); // Помилка для ділення на нуль
                        }
                        a / b
                    }
                    _ => return Err(format!("Невідома операція: {}", op)), // Обробка невідомих операторів
                };
                stack.push(result); // Додаємо результат обчислення до стека
            }
            _ => return Err("Невірний токен.".to_string()), // Помилка для невірних токенів
        }
    }

    if stack.len() != 1 {
        return Err("Невірний вираз.".to_string()); // Помилка для невірного виразу
    }

    Ok(stack[0]) // Повертаємо результат
}
