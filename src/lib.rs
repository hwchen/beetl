pub use beetl_macros::*;

pub trait MeltRecord<I, T>
    where I: Iterator<Item=T>
{
    fn melt(self) -> I;
}
