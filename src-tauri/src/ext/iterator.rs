use anyhow::{Result, bail};

pub trait IteratorExt {
    fn advance(&mut self, n: usize) -> Result<()>;
}

impl<T, I> IteratorExt for I
where
    I: Iterator<Item = T>
{

    fn advance(&mut self, n: usize) -> Result<()> {
        let mut i = 0;

        while i < n && self.next().is_some() {
            i += 1;
        }

        if i < n {
            bail!("Iterator out of bounds");
        }

        Ok(())
    }
}