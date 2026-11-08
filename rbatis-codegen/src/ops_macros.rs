//! Implementation-matrix macros shared by the operator modules.

/// Implements every owned/reference combination for two `Value` operands.
macro_rules! impl_value_binary_op {
    ($trait:ident, $method:ident, $op:ident) => {
        impl $trait<Value> for Value {
            type Output = Value;

            fn $method(self, rhs: Value) -> Self::Output {
                $op(self, rhs)
            }
        }

        impl $trait<&Value> for Value {
            type Output = Value;

            fn $method(self, rhs: &Value) -> Self::Output {
                $op(self, rhs.to_owned())
            }
        }

        impl $trait<&&Value> for Value {
            type Output = Value;

            fn $method(self, rhs: &&Value) -> Self::Output {
                $op(self, (*rhs).to_owned())
            }
        }

        impl $trait<Value> for &Value {
            type Output = Value;

            fn $method(self, rhs: Value) -> Self::Output {
                $op(self.to_owned(), rhs)
            }
        }

        impl $trait<&Value> for &Value {
            type Output = Value;

            fn $method(self, rhs: &Value) -> Self::Output {
                $op(self.to_owned(), rhs.to_owned())
            }
        }

        impl $trait<&&Value> for &Value {
            type Output = Value;

            fn $method(self, rhs: &&Value) -> Self::Output {
                $op(self.to_owned(), (*rhs).to_owned())
            }
        }

        impl $trait<Value> for &&Value {
            type Output = Value;

            fn $method(self, rhs: Value) -> Self::Output {
                $op((*self).to_owned(), rhs)
            }
        }

        impl $trait<&Value> for &&Value {
            type Output = Value;

            fn $method(self, rhs: &Value) -> Self::Output {
                $op((*self).to_owned(), rhs.to_owned())
            }
        }

        impl $trait<&&Value> for &&Value {
            type Output = Value;

            fn $method(self, rhs: &&Value) -> Self::Output {
                $op((*self).to_owned(), (*rhs).to_owned())
            }
        }
    };
}

/// Implements a unary operator for every commonly generated `Value` receiver.
macro_rules! impl_value_unary_op {
    ($trait:ident, $method:ident, $op:ident) => {
        impl $trait for Value {
            type Output = Value;

            fn $method(self) -> Self::Output {
                $op(self)
            }
        }

        impl $trait for &Value {
            type Output = Value;

            fn $method(self) -> Self::Output {
                $op(self.to_owned())
            }
        }

        impl $trait for &&Value {
            type Output = Value;

            fn $method(self) -> Self::Output {
                $op((*self).to_owned())
            }
        }

        impl $trait for &mut Value {
            type Output = Value;

            fn $method(self) -> Self::Output {
                $op(self.to_owned())
            }
        }
    };
}

/// Implements the `Value`/primitive matrix for a numeric binary operator.
macro_rules! impl_numeric_binary_op {
    (
        $trait:ident,
        $method:ident,
        $op:ident,
        $reverse_op:ident,
        [$($ty:ty),* $(,)?],
        $output:ty
    ) => {
        $(
            impl $trait<$ty> for Value {
                type Output = $output;

                fn $method(self, rhs: $ty) -> Self::Output {
                    $op(&self, rhs as _)
                }
            }

            impl $trait<&$ty> for Value {
                type Output = $output;

                fn $method(self, rhs: &$ty) -> Self::Output {
                    $op(&self, *rhs as _)
                }
            }

            impl $trait<$ty> for &Value {
                type Output = $output;

                fn $method(self, rhs: $ty) -> Self::Output {
                    $op(self, rhs as _)
                }
            }

            impl $trait<&$ty> for &Value {
                type Output = $output;

                fn $method(self, rhs: &$ty) -> Self::Output {
                    $op(self, *rhs as _)
                }
            }

            impl $trait<$ty> for &&Value {
                type Output = $output;

                fn $method(self, rhs: $ty) -> Self::Output {
                    $op(self, rhs as _)
                }
            }

            impl $trait<&$ty> for &&Value {
                type Output = $output;

                fn $method(self, rhs: &$ty) -> Self::Output {
                    $op(self, *rhs as _)
                }
            }

            impl $trait<Value> for $ty {
                type Output = $output;

                fn $method(self, rhs: Value) -> Self::Output {
                    $reverse_op(&rhs, self as _)
                }
            }

            impl $trait<&Value> for $ty {
                type Output = $output;

                fn $method(self, rhs: &Value) -> Self::Output {
                    $reverse_op(rhs, self as _)
                }
            }

            impl $trait<&&Value> for $ty {
                type Output = $output;

                fn $method(self, rhs: &&Value) -> Self::Output {
                    $reverse_op(rhs, self as _)
                }
            }

            impl $trait<Value> for &$ty {
                type Output = $output;

                fn $method(self, rhs: Value) -> Self::Output {
                    $reverse_op(&rhs, *self as _)
                }
            }

            impl $trait<&Value> for &$ty {
                type Output = $output;

                fn $method(self, rhs: &Value) -> Self::Output {
                    $reverse_op(rhs, *self as _)
                }
            }

            impl $trait<&&Value> for &$ty {
                type Output = $output;

                fn $method(self, rhs: &&Value) -> Self::Output {
                    $reverse_op(rhs, *self as _)
                }
            }
        )*
    };
}

/// Implements a binary operator between identical primitive types.
macro_rules! impl_same_type_binary_op {
    ($trait:ident, $method:ident, $operator:tt, [$($ty:ty),* $(,)?]) => {
        $(
            impl $trait<$ty> for $ty {
                type Output = $ty;

                fn $method(self, rhs: $ty) -> Self::Output {
                    self $operator rhs
                }
            }

            impl $trait<&$ty> for $ty {
                type Output = $ty;

                fn $method(self, rhs: &$ty) -> Self::Output {
                    self $operator *rhs
                }
            }

            impl $trait<$ty> for &$ty {
                type Output = $ty;

                fn $method(self, rhs: $ty) -> Self::Output {
                    *self $operator rhs
                }
            }

            impl $trait<&$ty> for &$ty {
                type Output = $ty;

                fn $method(self, rhs: &$ty) -> Self::Output {
                    *self $operator *rhs
                }
            }
        )*
    };
}

/// Implements a checked shift between identical primitive types.
macro_rules! impl_same_type_shift_op {
    ($trait:ident, $method:ident, $checked:ident, [$($ty:ty),* $(,)?]) => {
        $(
            impl $trait<$ty> for $ty {
                type Output = $ty;

                fn $method(self, rhs: $ty) -> Self::Output {
                    self.$checked(rhs as u32).unwrap_or_default()
                }
            }

            impl $trait<&$ty> for $ty {
                type Output = $ty;

                fn $method(self, rhs: &$ty) -> Self::Output {
                    self.$checked(*rhs as u32).unwrap_or_default()
                }
            }

            impl $trait<$ty> for &$ty {
                type Output = $ty;

                fn $method(self, rhs: $ty) -> Self::Output {
                    (*self).$checked(rhs as u32).unwrap_or_default()
                }
            }

            impl $trait<&$ty> for &$ty {
                type Output = $ty;

                fn $method(self, rhs: &$ty) -> Self::Output {
                    (*self).$checked(*rhs as u32).unwrap_or_default()
                }
            }
        )*
    };
}

/// Implements the `Value`/primitive matrix for a comparison operator.
macro_rules! impl_value_scalar_compare_op {
    (
        $trait:ident,
        $method:ident,
        $compare:ident,
        $reverse_compare:ident,
        [$($ty:ty),* $(,)?],
        $output:ty
    ) => {
        $(
            impl $trait<$ty> for Value {
                fn $method(&self, rhs: &$ty) -> $output {
                    $compare(self, *rhs as _)
                }
            }

            impl $trait<&$ty> for Value {
                fn $method(&self, rhs: &&$ty) -> $output {
                    $compare(self, **rhs as _)
                }
            }

            impl $trait<$ty> for &Value {
                fn $method(&self, rhs: &$ty) -> $output {
                    $compare(self, *rhs as _)
                }
            }

            impl $trait<&$ty> for &Value {
                fn $method(&self, rhs: &&$ty) -> $output {
                    $compare(self, **rhs as _)
                }
            }

            impl $trait<$ty> for &&Value {
                fn $method(&self, rhs: &$ty) -> $output {
                    $compare(self, *rhs as _)
                }
            }

            impl $trait<&$ty> for &&Value {
                fn $method(&self, rhs: &&$ty) -> $output {
                    $compare(self, **rhs as _)
                }
            }

            impl $trait<Value> for $ty {
                fn $method(&self, rhs: &Value) -> $output {
                    $reverse_compare(rhs, *self as _)
                }
            }

            impl $trait<&Value> for $ty {
                fn $method(&self, rhs: &&Value) -> $output {
                    $reverse_compare(*rhs, *self as _)
                }
            }

            impl $trait<&&Value> for $ty {
                fn $method(&self, rhs: &&&Value) -> $output {
                    $reverse_compare(**rhs, *self as _)
                }
            }

            impl $trait<Value> for &$ty {
                fn $method(&self, rhs: &Value) -> $output {
                    $reverse_compare(rhs, **self as _)
                }
            }

            impl $trait<&Value> for &$ty {
                fn $method(&self, rhs: &&Value) -> $output {
                    $reverse_compare(*rhs, **self as _)
                }
            }

            impl $trait<&&Value> for &$ty {
                fn $method(&self, rhs: &&&Value) -> $output {
                    $reverse_compare(**rhs, **self as _)
                }
            }
        )*
    };
}

/// Expands every row in a comparison matrix into owned/reference impls.
macro_rules! impl_compare_matrix {
    (
        $trait:ident,
        $method:ident,
        $compare:ident,
        $cast:ty,
        $output:ty;
        $(
            $lhs:ty => [$($rhs:ty),* $(,)?]
        ),* $(,)?
    ) => {
        $(
            $(
                impl $trait<$lhs> for $rhs {
                    fn $method(&self, rhs: &$lhs) -> $output {
                        $compare(*self as $cast, *rhs as $cast)
                    }
                }

                impl $trait<&$lhs> for $rhs {
                    fn $method(&self, rhs: &&$lhs) -> $output {
                        $compare(*self as $cast, **rhs as $cast)
                    }
                }

                impl $trait<$lhs> for &$rhs {
                    fn $method(&self, rhs: &$lhs) -> $output {
                        $compare(**self as $cast, *rhs as $cast)
                    }
                }

                impl $trait<&$lhs> for &$rhs {
                    fn $method(&self, rhs: &&$lhs) -> $output {
                        $compare(**self as $cast, **rhs as $cast)
                    }
                }
            )*
        )*
    };
}

/// Implements boolean `&`/`|` for every `Value` reference combination.
macro_rules! impl_bool_value_binary_op {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl $trait<Value> for Value {
            type Output = bool;

            fn $method(self, rhs: Value) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<&Value> for Value {
            type Output = bool;

            fn $method(self, rhs: &Value) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<&&Value> for Value {
            type Output = bool;

            fn $method(self, rhs: &&Value) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<Value> for &Value {
            type Output = bool;

            fn $method(self, rhs: Value) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<&Value> for &Value {
            type Output = bool;

            fn $method(self, rhs: &Value) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<&&Value> for &Value {
            type Output = bool;

            fn $method(self, rhs: &&Value) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<Value> for &&Value {
            type Output = bool;

            fn $method(self, rhs: Value) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<&Value> for &&Value {
            type Output = bool;

            fn $method(self, rhs: &Value) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<&&Value> for &&Value {
            type Output = bool;

            fn $method(self, rhs: &&Value) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs.as_bool().unwrap_or(false)
            }
        }
    };
}

/// Implements boolean `&`/`|`/`^` between `Value` and scalar `bool` operands.
macro_rules! impl_bool_scalar_binary_op {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl $trait<bool> for Value {
            type Output = bool;

            fn $method(self, rhs: bool) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs
            }
        }

        impl $trait<&bool> for Value {
            type Output = bool;

            fn $method(self, rhs: &bool) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator *rhs
            }
        }

        impl $trait<bool> for &Value {
            type Output = bool;

            fn $method(self, rhs: bool) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs
            }
        }

        impl $trait<&bool> for &Value {
            type Output = bool;

            fn $method(self, rhs: &bool) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator *rhs
            }
        }

        impl $trait<bool> for &&Value {
            type Output = bool;

            fn $method(self, rhs: bool) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator rhs
            }
        }

        impl $trait<&bool> for &&Value {
            type Output = bool;

            fn $method(self, rhs: &bool) -> Self::Output {
                self.as_bool().unwrap_or(false) $operator *rhs
            }
        }

        impl $trait<Value> for bool {
            type Output = bool;

            fn $method(self, rhs: Value) -> Self::Output {
                self $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<&Value> for bool {
            type Output = bool;

            fn $method(self, rhs: &Value) -> Self::Output {
                self $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<&&Value> for bool {
            type Output = bool;

            fn $method(self, rhs: &&Value) -> Self::Output {
                self $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<Value> for &bool {
            type Output = bool;

            fn $method(self, rhs: Value) -> Self::Output {
                *self $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<&Value> for &bool {
            type Output = bool;

            fn $method(self, rhs: &Value) -> Self::Output {
                *self $operator rhs.as_bool().unwrap_or(false)
            }
        }

        impl $trait<&&Value> for &bool {
            type Output = bool;

            fn $method(self, rhs: &&Value) -> Self::Output {
                *self $operator rhs.as_bool().unwrap_or(false)
            }
        }
    };
}

pub(crate) use impl_bool_scalar_binary_op;
pub(crate) use impl_bool_value_binary_op;
pub(crate) use impl_compare_matrix;
pub(crate) use impl_numeric_binary_op;
pub(crate) use impl_same_type_binary_op;
pub(crate) use impl_same_type_shift_op;
pub(crate) use impl_value_binary_op;
pub(crate) use impl_value_scalar_compare_op;
pub(crate) use impl_value_unary_op;
