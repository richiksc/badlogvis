#[derive(Debug, PartialEq, Clone)]
pub enum Attribute {
    Hide,
    Area,
    Xaxis,
    Differentiate,
    Integrate,
    Delta,
    Zero,
    Log,
    Join(String),
}

impl Attribute {
    pub fn from(attribute_text: &str) -> Result<Attribute, ()> {
        if attribute_text.eq("hide") {
            return Result::Ok(Attribute::Hide);
        }
        if attribute_text.eq("area") {
            return Result::Ok(Attribute::Area);
        }
        if attribute_text.eq("xaxis") {
            return Result::Ok(Attribute::Xaxis);
        }
        if attribute_text.eq("differentiate") {
            return Result::Ok(Attribute::Differentiate);
        }
        if attribute_text.eq("zero") {
            return Result::Ok(Attribute::Zero);
        }
        if attribute_text.eq("integrate") {
            return Result::Ok(Attribute::Integrate);
        }
        if attribute_text.eq("delta") {
            return Result::Ok(Attribute::Delta);
        }
        if attribute_text.eq("log") {
            return Result::Ok(Attribute::Log);
        }
        if attribute_text.starts_with("join:") {
            let (_, right) = attribute_text.split_at(5);
            if right.is_empty() {
                error!("Failed to join topic: {}", attribute_text);
            }
            return Result::Ok(Attribute::Join(right.to_string()));
        }

        Result::Err(())
    }
}
