pub struct FloatParser {}

impl FloatParser {
    pub fn from_arg_price(price: &str) -> Result<f32, std::num::ParseFloatError> {
        let first = price.replace(".", "");
        let second = first.replace(",", ".");

        let parsed = second.parse::<f32>()?;
        Ok(parsed)
    }
}
