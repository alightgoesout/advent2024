use std::io::{BufRead, BufReader, Read};
use std::iter::{Filter, MapWhile};
use std::marker::PhantomData;
use std::str::FromStr;

use crate::Error;

pub trait FilterNotEmpty: Iterator + Sized {
    fn filter_not_empty(self) -> Filter<Self, fn(&String) -> bool>;
}

impl<I> FilterNotEmpty for I
where
    I: Iterator<Item = String>,
{
    fn filter_not_empty(self) -> Filter<Self, fn(&String) -> bool> {
        self.filter(|s| !s.is_empty())
    }
}

pub struct Parse<I, T>(I, PhantomData<T>);

impl<I, U, T> Iterator for Parse<I, T>
where
    I: Iterator<Item = U>,
    U: ToString,
    T: FromStr,
    T::Err: Into<Error>,
{
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|item| item.to_string().parse().map_err(Into::into))
    }
}

pub trait ParseExt<I> {
    fn parse<T>(self) -> Parse<I, T>;
}

impl<I: Iterator> ParseExt<I> for I {
    fn parse<T>(self) -> Parse<I, T> {
        Parse(self, PhantomData::default())
    }
}

pub trait ReadLines {
    type Iterator: Iterator<Item = String>;

    fn read_lines(self) -> Self::Iterator;
}

impl<R: Read> ReadLines for R {
    type Iterator = MapWhile<
        std::io::Lines<BufReader<R>>,
        fn(Result<String, std::io::Error>) -> Option<String>,
    >;

    fn read_lines(self) -> Self::Iterator {
        BufReader::new(self).lines().map_while(Result::ok)
    }
}
