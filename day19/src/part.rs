pub struct PartError{}

pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

fn parse_component(s: &str) -> Result<(&str, u32),PartError> {
    let equals = s.chars().position(|c| c == '=').ok_or_else(|| PartError {})?;
    Ok((&s[0..equals], u32::from_str_radix(&s[equals+1..], 10).map_err(|_| PartError{})?))
}

impl TryFrom<&str> for Part {
    type Error = PartError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.strip_prefix('{').ok_or_else(|| PartError {})?;
        let value = value.strip_suffix('}').ok_or_else(|| PartError {})?;

        let components: Vec<(&str,u32)> = value
            .split(',')
            .map(parse_component)
            .collect::<Result<Vec<_>,_>>()?;

        if components.len() != 4 {
            return Err(PartError {})
        }

        Ok(Self {
            x: components[0].1,
            m: components[1].1,
            a: components[2].1,
            s: components[3].1
        })
    }    
}

impl Part {
    pub fn value(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}