use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
struct HashJoinInner<LT, RT, LF, RF> {
    l_source: LT,
    r_source: RT,
    l_key_extractor: LF,
    r_key_extractor: RF,
}

impl<LT, RT, LF, RF, L, R, K, E> HashJoinInner<LT, RT, LF, RF>
where
    LT: IntoIterator<Item = L>,
    RT: IntoIterator<Item = R>,
    LF: for<'a> FnMut(&'a L) -> Result<Cow<'a, K>, E>,
    RF: for<'a> FnMut(&'a R) -> Result<Cow<'a, K>, E>,
    K: ?Sized + ToOwned + Hash + Eq,
    <K as ToOwned>::Owned: Hash + Eq,
{
    pub fn unique_join<F, O>(mut self, mut mapper: F) -> Result<Vec<O>, E>
    where
        F: FnMut(L, &mut Option<R>) -> Result<Option<O>, E>,
    {
        let mut map = HashMap::new();

        for r_item in self.r_source {
            let key = (self.r_key_extractor)(&r_item)?;
            let val = map.entry(key.into_owned()).or_insert_with(|| None);
            let _ = std::mem::replace(val, Some(r_item));
        }

        self.l_source
            .into_iter()
            .try_fold(vec![], |mut vec, l_item| {
                let r_item = map.get_mut((self.l_key_extractor)(&l_item)?.as_ref());
                if let Some(output) = mapper(l_item, r_item.unwrap_or(&mut None))? {
                    vec.push(output);
                }
                Ok::<_, E>(vec)
            })
    }

    pub fn join<F, O>(mut self, mut mapper: F) -> Result<Vec<O>, E>
    where
        F: FnMut(L, &mut Vec<R>) -> Result<Option<O>, E>,
    {
        let mut map = HashMap::new();

        for r_item in self.r_source {
            let key = (self.r_key_extractor)(&r_item)?;
            let vec = map.entry(key.into_owned()).or_insert_with(Vec::new);
            vec.push(r_item);
        }

        self.l_source
            .into_iter()
            .try_fold(vec![], |mut vec, l_item| {
                let r_items = map.get_mut((self.l_key_extractor)(&l_item)?.as_ref());
                if let Some(output) = mapper(l_item, r_items.unwrap_or(&mut vec![]))? {
                    vec.push(output);
                }
                Ok::<_, E>(vec)
            })
    }
}

#[derive(Debug)]
pub struct HashJoin<LT, RT, LF, RF> {
    l_source: Option<LT>,
    r_source: Option<RT>,
    l_key_extractor: Option<LF>,
    r_key_extractor: Option<RF>,
}

impl<LT, RT, LF, RF> HashJoin<LT, RT, LF, RF> {
    pub fn new() -> HashJoin<LT, RT, LF, RF> {
        HashJoin {
            l_source: None,
            r_source: None,
            l_key_extractor: None,
            r_key_extractor: None,
        }
    }
}

impl<LT, RT, LF, RF, L, R, K, E> HashJoin<LT, RT, LF, RF>
where
    LT: IntoIterator<Item = L>,
    RT: IntoIterator<Item = R>,
    LF: for<'a> FnMut(&'a L) -> Result<Cow<'a, K>, E>,
    RF: for<'a> FnMut(&'a R) -> Result<Cow<'a, K>, E>,
    K: ?Sized + ToOwned + Hash + Eq,
    <K as ToOwned>::Owned: Hash + Eq,
{
    pub fn l_source(mut self, t: LT) -> Self {
        self.l_source = Some(t);
        self
    }

    pub fn r_source(mut self, t: RT) -> Self {
        self.r_source = Some(t);
        self
    }

    pub fn l_key_extractor(mut self, f: LF) -> Self {
        self.l_key_extractor = Some(f);
        self
    }

    pub fn r_key_extractor(mut self, f: RF) -> Self {
        self.r_key_extractor = Some(f);
        self
    }

    fn try_build(self) -> Option<HashJoinInner<LT, RT, LF, RF>> {
        Some(HashJoinInner {
            l_source: self.l_source?,
            r_source: self.r_source?,
            l_key_extractor: self.l_key_extractor?,
            r_key_extractor: self.r_key_extractor?,
        })
    }

    fn build(self) -> HashJoinInner<LT, RT, LF, RF> {
        self.try_build().unwrap()
    }

    pub fn unique_join<F, O>(self, mapper: F) -> Result<Vec<O>, E>
    where
        F: FnMut(L, &mut Option<R>) -> Result<Option<O>, E>,
    {
        self.build().unique_join(mapper)
    }

    pub fn join<F, O>(self, mapper: F) -> Result<Vec<O>, E>
    where
        F: FnMut(L, &mut Vec<R>) -> Result<Option<O>, E>,
    {
        self.build().join(mapper)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::HashJoin;

    #[test]
    fn simple() {
        let v1 = vec![(0, "a"), (0, "b"), (1, "c"), (1, "d"), (2, "e"), (2, "f")];
        let v2 = vec![(0, "!"), (0, "@"), (1, "#"), (1, "$"), (2, "%"), (2, "^")];

        let vec = HashJoin::new()
            .l_source(v1)
            .r_source(v2)
            .l_key_extractor(|l| Ok::<_, ()>(Cow::Borrowed(&l.0)))
            .r_key_extractor(|r| Ok::<_, ()>(Cow::Borrowed(&r.0)))
            .join(|l, vals| Ok::<_, ()>(Some((l.0, l.1, vals.clone()))));

        assert_eq!(
            vec,
            Ok(vec![
                (0, "a", vec![(0, "!"), (0, "@")]),
                (0, "b", vec![(0, "!"), (0, "@")]),
                (1, "c", vec![(1, "#"), (1, "$")]),
                (1, "d", vec![(1, "#"), (1, "$")]),
                (2, "e", vec![(2, "%"), (2, "^")]),
                (2, "f", vec![(2, "%"), (2, "^")]),
            ])
        );
    }

    #[test]
    fn error() {
        let v1 = vec![(0, "a"), (0, "b"), (1, "c"), (1, "d"), (2, "e"), (2, "f")];
        let v2 = vec![(0, "!"), (0, "@"), (1, "#"), (1, "$"), (2, "%"), (2, "^")];

        let vec1 = HashJoin::new()
            .l_source(&v1)
            .r_source(&v2)
            .l_key_extractor(|_| Err::<_, ()>(()))
            .r_key_extractor(|r| Ok::<_, ()>(Cow::Borrowed(&r.0)))
            .join(|l, vals| Ok::<_, ()>(Some((l.0, l.1, vals.clone()))));

        let vec2 = HashJoin::new()
            .l_source(&v1)
            .r_source(&v2)
            .l_key_extractor(|l| Ok::<_, ()>(Cow::Borrowed(&l.0)))
            .r_key_extractor(|_| Err::<_, ()>(()))
            .join(|l, vals| Ok::<_, ()>(Some((l.0, l.1, vals.clone()))));

        let vec3 = HashJoin::new()
            .l_source(&v1)
            .r_source(&v2)
            .l_key_extractor(|l| Ok::<_, ()>(Cow::Borrowed(&l.0)))
            .r_key_extractor(|r| Ok::<_, ()>(Cow::Borrowed(&r.0)))
            .join(|_, _| Err::<Option<()>, _>(()));

        assert_eq!(vec1, Err(()));
        assert_eq!(vec2, Err(()));
        assert_eq!(vec3, Err(()));
    }

    #[test]
    fn filter() {
        let v1 = vec![(0, "a"), (0, "b"), (1, "c"), (1, "d"), (2, "e"), (2, "f")];
        let v2 = vec![(0, "!"), (0, "@"), (1, "#"), (1, "$"), (2, "%"), (2, "^")];

        let vec = HashJoin::new()
            .l_source(v1)
            .r_source(v2)
            .l_key_extractor(|l| Ok::<_, ()>(Cow::Borrowed(&l.0)))
            .r_key_extractor(|r| Ok::<_, ()>(Cow::Borrowed(&r.0)))
            .join(|l, vals| {
                if l.0 == 0 {
                    Ok::<_, ()>(Some((l.0, l.1, vals.clone())))
                } else {
                    Ok::<_, ()>(None)
                }
            });

        assert_eq!(
            vec,
            Ok(vec![
                (0, "a", vec![(0, "!"), (0, "@")]),
                (0, "b", vec![(0, "!"), (0, "@")]),
            ])
        );
    }

    #[test]
    fn replace() {
        let v1 = vec![(0, "a"), (0, "b"), (1, "c"), (1, "d"), (2, "e"), (2, "f")];
        let v2 = vec![(0, "!"), (0, "@"), (1, "#"), (1, "$"), (2, "%"), (2, "^")];

        let vec = HashJoin::new()
            .l_source(v1)
            .r_source(v2)
            .l_key_extractor(|l| Ok::<_, ()>(Cow::Borrowed(&l.0)))
            .r_key_extractor(|r| Ok::<_, ()>(Cow::Borrowed(&r.0)))
            .join(|l, vals| {
                let vals = std::mem::replace(vals, vec![]);
                Ok::<_, ()>(Some((l.0, l.1, vals)))
            });

        assert_eq!(
            vec,
            Ok(vec![
                (0, "a", vec![(0, "!"), (0, "@")]),
                (0, "b", vec![]),
                (1, "c", vec![(1, "#"), (1, "$")]),
                (1, "d", vec![]),
                (2, "e", vec![(2, "%"), (2, "^")]),
                (2, "f", vec![]),
            ])
        );
    }
}
