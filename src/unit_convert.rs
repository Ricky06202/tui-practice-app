use std::io::{Error, ErrorKind};

use crate::ui::{InputBox, MessageBox, UI};

struct UnitOption {
    name: String,
    factor: f64,
    offset: f64,
}

impl UnitOption {
    fn new(name: &str, factor: f64, offset: f64) -> Self {
        Self {
            name: name.to_string(),
            factor,
            offset,
        }
    }
}

pub struct UnitConvert {
    category: String,
    options: Vec<UnitOption>,
}

impl UnitConvert {
    pub fn new(category: &str, options: Vec<UnitOption>) -> Self {
        Self {
            category: category.to_string(),
            options,
        }
    }

    fn calculate(&self, from_id: usize, to_id: usize, value: f64) -> f64 {
        let from = &self.options[from_id];
        let to = &self.options[to_id];
        (value - from.offset) * (from.factor / to.factor) + to.offset
    }
}

pub fn run() {
    let categories = vec!["Longitud", "Temperatura", "Moneda"];
    let mut ui_cat = UI::new(
        "Seleccione una categoría para realizar una conversión",
        categories,
    );

    if let Some(cat_id) = ui_cat.show().unwrap() {
        let convert = match cat_id {
            0 => UnitConvert::new(
                "Longitud",
                vec![
                    UnitOption::new("Metros", 1.0, 0.0),
                    UnitOption::new("Kilómetros", 1000.0, 0.0),
                    UnitOption::new("Millas", 1609.34, 0.0),
                    UnitOption::new("Pies", 0.3048, 0.0),
                ],
            ),
            1 => UnitConvert::new(
                "Temperatura",
                vec![
                    UnitOption::new("Celsius", 1.0, 0.0),
                    UnitOption::new("Fahrenheit", 1.8, 32.0),
                    UnitOption::new("Kelvin", 1.0, 273.15),
                ],
            ),
            2 => UnitConvert::new(
                "Moneda",
                vec![
                    UnitOption::new("USD", 1.0, 0.0),
                    UnitOption::new("EUR", 1.1, 0.0),
                    UnitOption::new("MXN", 0.05, 0.0),
                ],
            ),
            _ => unreachable!(),
        };

        let unit_names: Vec<String> = convert.options.iter().map(|u| u.name.clone()).collect();

        let mut ui_from = UI::new(
            "Seleccione la unidad de origen",
            unit_names.iter().map(|s| s.as_str()).collect(),
        );

        if let Some(from_id) = ui_from.show().unwrap() {
            let mut ui_to = UI::new(
                "Seleccione la unidad de destino",
                unit_names.iter().map(|s| s.as_str()).collect(),
            );

            if let Some(to_id) = ui_to.show().unwrap() {
                match iniciar_conversion(convert, from_id, to_id) {
                    Ok(result) => {
                        let mut ui_result = MessageBox::new("Resultado", &format!("{}", result));
                        ui_result.show().unwrap();
                    }
                    Err(e) => {
                        let mut ui_error = MessageBox::new("Error", &e.to_string());
                        ui_error.show().unwrap();
                    }
                }
            }
        }
    }
}

fn iniciar_conversion(convert: UnitConvert, from_id: usize, to_id: usize) -> Result<f64, Error> {
    let mut ui_value = InputBox::new(&format!(
        "Convertir de {} a {}",
        convert.options[from_id].name, convert.options[to_id].name
    ));
    let value_str = ui_value.show()?;
    let value = value_str
        .parse::<f64>()
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Valor inválido"))?;
    Ok(convert.calculate(from_id, to_id, value))
}
