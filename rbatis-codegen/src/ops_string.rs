use rbs::Value;
use crate::ops::StrMethods;

impl StrMethods for Value {
    fn starts_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().starts_with(other)
    }

    fn ends_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().ends_with(other)
    }
}

impl StrMethods for &Value {
 
    fn starts_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().starts_with(other)
    }

    fn ends_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().ends_with(other)
    }
}

impl StrMethods for &&Value {
    fn starts_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().starts_with(other)
    }

    fn ends_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().ends_with(other)
    }
}