use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CalculatorError {
    #[error("Division par zéro")]
    DivisionByZero,
    #[error("Expression invalide: {0}")]
    InvalidExpression(String),
    #[error("Paramètre manquant ou invalide: {0}")]
    InvalidInput(String),
}

#[derive(Debug, Clone, Default)]
pub struct Calculator;

#[derive(Debug, Deserialize)]
pub struct CalculatorInput {
    pub expression: String,
}

#[derive(Debug, Serialize)]
pub struct CalculatorOutput {
    pub status: String,
    pub expression: String,
    pub result: f64,
}

impl Calculator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, expr: &str) -> Result<f64, CalculatorError> {
        let cleaned: String = expr.chars().filter(|c| !c.is_whitespace()).collect();
        if cleaned.is_empty() {
            return Err(CalculatorError::InvalidExpression("Expression vide".into()));
        }

        let mut parser = Parser::new(&cleaned);
        let res = parser.parse_expression()?;
        if parser.pos < parser.chars.len() {
            return Err(CalculatorError::InvalidExpression(format!(
                "Caractère inattendu à la position {}",
                parser.pos
            )));
        }
        Ok(res)
    }

    pub fn execute_tool(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, CalculatorError> {
        let expr = payload
            .get("expression")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                CalculatorError::InvalidInput("Champ 'expression' obligatoire".into())
            })?;

        let result = self.evaluate(expr)?;
        Ok(serde_json::json!({
            "status": "success",
            "expression": expr,
            "result": result
        }))
    }
}

struct Parser<'a> {
    chars: Vec<char>,
    pos: usize,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            _marker: std::marker::PhantomData,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    fn parse_expression(&mut self) -> Result<f64, CalculatorError> {
        let mut left = self.parse_term()?;

        while let Some(op) = self.peek() {
            if op == '+' || op == '-' {
                self.next_char();
                let right = self.parse_term()?;
                if op == '+' {
                    left += right;
                } else {
                    left -= right;
                }
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<f64, CalculatorError> {
        let mut left = self.parse_factor()?;

        while let Some(op) = self.peek() {
            if op == '*' || op == '/' || op == '%' {
                self.next_char();
                let right = self.parse_factor()?;
                match op {
                    '*' => left *= right,
                    '/' => {
                        if right == 0.0 {
                            return Err(CalculatorError::DivisionByZero);
                        }
                        left /= right;
                    }
                    '%' => {
                        if right == 0.0 {
                            return Err(CalculatorError::DivisionByZero);
                        }
                        left %= right;
                    }
                    _ => unreachable!(),
                }
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<f64, CalculatorError> {
        let mut base = self.parse_primary()?;

        if let Some('^') = self.peek() {
            self.next_char();
            let exponent = self.parse_factor()?;
            base = base.powf(exponent);
        }
        Ok(base)
    }

    fn parse_primary(&mut self) -> Result<f64, CalculatorError> {
        match self.peek() {
            Some('+') => {
                self.next_char();
                self.parse_primary()
            }
            Some('-') => {
                self.next_char();
                let val = self.parse_primary()?;
                Ok(-val)
            }
            Some('(') => {
                self.next_char();
                let val = self.parse_expression()?;
                if self.next_char() != Some(')') {
                    return Err(CalculatorError::InvalidExpression(
                        "Parenthèse fermante ')' manquante".into(),
                    ));
                }
                Ok(val)
            }
            Some(c) if c.is_ascii_digit() || c == '.' => self.parse_number(),
            Some(c) if c.is_alphabetic() => self.parse_function_or_ident(),
            Some(other) => Err(CalculatorError::InvalidExpression(format!(
                "Symbole non reconnu: '{}'",
                other
            ))),
            None => Err(CalculatorError::InvalidExpression(
                "Fin inattendue de l'expression".into(),
            )),
        }
    }

    fn parse_number(&mut self) -> Result<f64, CalculatorError> {
        let start = self.pos;
        let mut has_dot = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.next_char();
            } else if c == '.' && !has_dot {
                has_dot = true;
                self.next_char();
            } else {
                break;
            }
        }

        let num_str: String = self.chars[start..self.pos].iter().collect();
        num_str
            .parse::<f64>()
            .map_err(|e| CalculatorError::InvalidExpression(e.to_string()))
    }

    fn parse_function_or_ident(&mut self) -> Result<f64, CalculatorError> {
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.next_char();
            } else {
                break;
            }
        }
        let ident: String = self.chars[start..self.pos].iter().collect();

        if self.peek() == Some('(') {
            self.next_char(); // consomme '('
            let arg = self.parse_expression()?;
            if self.next_char() != Some(')') {
                return Err(CalculatorError::InvalidExpression(format!(
                    "Parenthèse fermante manquante pour {}",
                    ident
                )));
            }

            match ident.to_lowercase().as_str() {
                "sqrt" => {
                    if arg < 0.0 {
                        return Err(CalculatorError::InvalidExpression(
                            "Racine carrée d'un nombre négatif".into(),
                        ));
                    }
                    Ok(arg.sqrt())
                }
                "abs" => Ok(arg.abs()),
                "round" => Ok(arg.round()),
                "floor" => Ok(arg.floor()),
                "ceil" => Ok(arg.ceil()),
                _ => Err(CalculatorError::InvalidExpression(format!(
                    "Fonction inconnue: {}",
                    ident
                ))),
            }
        } else {
            match ident.to_lowercase().as_str() {
                "pi" => Ok(std::f64::consts::PI),
                "e" => Ok(std::f64::consts::E),
                _ => Err(CalculatorError::InvalidExpression(format!(
                    "Identifiant inconnu: {}",
                    ident
                ))),
            }
        }
    }
}
