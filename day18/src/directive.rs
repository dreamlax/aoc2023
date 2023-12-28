#[derive(Debug)]
pub struct DirectiveError {
    msg: &'static str
}

impl DirectiveError {
    pub fn new(msg: &'static str) -> Self {
        Self {
            msg
        }
    }
}

impl std::fmt::Display for DirectiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Directive error: {}", self.msg)
    }
}

impl std::error::Error for DirectiveError {}

pub struct Directive {
    pub dx: isize,
    pub dy: isize,
}

impl TryFrom<String> for Directive {
    type Error = DirectiveError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut fields = value.split_ascii_whitespace();

        let direction = fields.next().ok_or_else(|| DirectiveError::new("No direction"))?;
        let amount: isize = isize::from_str_radix(
                fields.next().ok_or_else(|| DirectiveError::new("No amount"))?,
                10
            )
            .map_err(|_| DirectiveError::new("Amount is unparseable"))?;
        let color = fields.next().ok_or_else(|| DirectiveError::new("No color"))?;

        if fields.next().is_some_and(|x| x.trim().len() > 0) {
            return Err(DirectiveError::new("Too much input"));
        }

        let (dx, dy) = if cfg!(feature = "part2") {
            let amount = isize::from_str_radix(&color[2..=6], 16).map_err(|_| DirectiveError::new("Unparsable color"))?;

            match &color[7..=7] {
                "0" => (amount, 0),
                "1" => (0, amount),
                "2" => (-amount, 0),
                "3" => (0, -amount),
                _ => return Err(DirectiveError::new("Unknown direction"))
            }
        }
        else {
            match direction {
                "L" => (-amount, 0),
                "R" => (amount, 0),
                "U" => (0, -amount),
                "D" => (0, amount),
                _ => return Err(DirectiveError::new("Unknown direction"))
            }
        };

        Ok(Self {
            dx,
            dy
        })
    }
}
