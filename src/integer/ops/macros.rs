macro_rules! impl_single_binop {
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self, ref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(mut self, rhs: $rhs) -> Self::Output {
                $fn(&mut self, &rhs);

                self
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(mut self, rhs: $rhs) -> Self::Output {
                $fn(&mut self, rhs);

                self
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, &rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, deref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, *rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self, deref rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(mut self, rhs: $rhs) -> Self::Output {
                $fn(&mut self, *rhs);

                self
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, into rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(self, &Integer::from(rhs))
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self, into rhs) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(mut self, rhs: $rhs) -> Self::Output {
                $fn(&mut self, &Integer::from(rhs));

                self
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self, ref rhs, no reuse) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, &rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self, no reuse) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self, deref rhs, no reuse) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, *rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, $ret:ty, ref self, into rhs, no reuse) => {
        impl $op_path<$rhs> for $celf {
            type Output = $ret;

            fn $op_fn(self, rhs: $rhs) -> Self::Output {
                $fn(&self, &Integer::from(rhs))
            }
        }
    };
}

macro_rules! impl_single_op_assign {
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path) => {
        impl $op_path<$rhs> for $celf {
            fn $op_fn(&mut self, rhs: $rhs) {
                $fn(self, rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, ref rhs) => {
        impl $op_path<$rhs> for $celf {
            fn $op_fn(&mut self, rhs: $rhs) {
                $fn(self, &rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, deref rhs) => {
        impl $op_path<$rhs> for $celf {
            fn $op_fn(&mut self, rhs: $rhs) {
                $fn(self, *rhs)
            }
        }
    };
    ($op_path:ident, $op_fn:ident, $celf:ty, $rhs:ty, $fn:path, into rhs) => {
        impl $op_path<$rhs> for $celf {
            fn $op_fn(&mut self, rhs: $rhs) {
                $fn(self, &Integer::from(rhs))
            }
        }
    };
}
