// Traits
trait Debug {
    fn fmt() -> Result<(), ()>;
}
trait HasUp {
    type Up;
}
trait Container {
    type Output;
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
}
impl<V> Debug for Up<V>
where
    V: HasUp,
    V::Up: HasContainerType<ContainerType: Container>,
{
    fn fmt() -> Result<(), ()> {
        Ok(())
    }
}
