// Traits
trait HasUp {
    type Up;
}
trait Container {
    type Output;

    fn create() -> Result<(), ()>;
}
trait HasContainerType {
    type ContainerType;
}

// Structs
struct Up<V>(V)
where
    V: HasUp,
    V::Up: HasContainerType<ContainerType: Container>;

// Impls
impl<V> HasContainerType for V
where
    V: HasUp,
    V::Up: HasContainerType<ContainerType: Container>,
{
    type ContainerType = Up<V>;
}

impl<V> Container for Up<V>
where
    V: HasUp,
    V::Up: HasContainerType<ContainerType: Container>,
{
    type Output = Self;
    fn create() -> Result<(), ()> {
        Ok(())
    }
}
