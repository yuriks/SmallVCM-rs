static pi : f32 = 3.14159265358979;
static inv_pi : f32 = 1.0 / pi;

fn sqr<T: Mul<T, T>>(a: T) -> T {
    a * a
}

/// This trait is mis-named. It implements a vector of coordinates, not an abstract vector in the
/// mathematical sense.
pub trait Vector<T: Num> :
    Add<Self, Self> + Sub<Self, Self> +
    Mul<Self, Self> + Div<Self, Self> +
    Neg<Self> +
    Index<uint, T> + IndexMut<uint, T>
{
    fn dot(&self, &Self) -> T;

    fn length_sqr(&self) -> T {
        dot(self, self)
    }
}

pub trait VectorF<T: Float> : Vector<T> {
    fn length(&self) -> T {
        self.length_sqr().sqrt()
    }
}

#[inline]
fn dot<T: Num, V: Vector<T>>(a: &V, b: &V) -> T {
    a.dot(b)
}

/// Turns a list into a summantion:
/// ```
/// (a, b, c, d) => (a) + (b) + (c) + (d)
/// ```
///
/// Used to work around the fact that `+` isn't an allowed separator for macro expansions.
macro_rules! sum(
    ($e:expr) => (($e));
    ($e:expr, $($rest:expr),+) => (($e) + sum!($($rest),+));
)

macro_rules! impl_Vector(
    ($Self:ident { $($field:ident),+ }) => (
        struct $Self<T> {
            $($field: T),+
        }

        impl<T: Num + Clone> $Self<T> {
            #[inline]
            /// Creates a vector with the given value in all coordinates.
            fn spread(x: &T) -> $Self<T> {
                $Self {
                    $($field: x.clone()),+
                }
            }
        }

        impl<T: Num> Add<$Self<T>, $Self<T>> for $Self<T> {
            #[inline]
            fn add(&self, o: &$Self<T>) -> $Self<T> {
                $Self {
                    $($field: self.$field + o.$field),+
                }
            }
        }

        impl<T: Num> Sub<$Self<T>, $Self<T>> for $Self<T> {
            #[inline]
            fn sub(&self, o: &$Self<T>) -> $Self<T> {
                $Self {
                    $($field: self.$field - o.$field),+
                }
            }
        }

        impl<T: Num> Mul<$Self<T>, $Self<T>> for $Self<T> {
            #[inline]
            fn mul(&self, o: &$Self<T>) -> $Self<T> {
                $Self {
                    $($field: self.$field * o.$field),+
                }
            }
        }

        impl<T: Num> Div<$Self<T>, $Self<T>> for $Self<T> {
            #[inline]
            fn div(&self, o: &$Self<T>) -> $Self<T> {
                $Self {
                    $($field: self.$field / o.$field),+
                }
            }
        }

        impl<T: Num> Neg<$Self<T>> for $Self<T> {
            #[inline]
            fn neg(&self) -> $Self<T> {
                $Self {
                    $($field: -self.$field),+
                }
            }
        }

        impl<T: Num> Index<uint, T> for $Self<T> {
            #[inline]
            fn index(&self, index: &uint) -> &T {
                [$(&self.$field),+][*index]
            }
        }

        impl<T: Num> IndexMut<uint, T> for $Self<T> {
            #[inline]
            fn index_mut(&mut self, index: &uint) -> &mut T {
                [$(&mut self.$field),+][*index]
            }
        }

        impl<T: Num> Vector<T> for $Self<T> {
            #[inline]
            fn dot(&self, o: &$Self<T>) -> T {
                // + isn't a valid separator for a macro repetition, so instead of
                // `$((self.$field * o.$field))++`
                // we have to use a workaround:
                sum!($(self.$field * o.$field),+)
            }
        }
    )
)

impl_Vector!(Vector2 { x, y })
impl_Vector!(Vector3 { x, y, z })

type Vec2f = Vector2<f32>;
type Vec2i = Vector2<i32>;
type Vec3f = Vector3<f32>;
type Vec3i = Vector3<i32>;
