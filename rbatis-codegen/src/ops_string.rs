use rbs::Value;
use crate::ops::StringContain;

impl StringContain for Value {
    fn contains(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().contains(other)
    }

    fn starts_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().starts_with(other)
    }

    fn ends_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().ends_with(other)
    }
}

impl StringContain for &Value {
    fn contains(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().contains(other)
    }

    fn starts_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().starts_with(other)
    }

    fn ends_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().ends_with(other)
    }
}

impl StringContain for &&Value {
    fn contains(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().contains(other)
    }

    fn starts_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().starts_with(other)
    }

    fn ends_with(self, other: &str) -> bool {
        self.as_str().unwrap_or_default().ends_with(other)
    }
}