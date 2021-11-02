use definition::{Block, Inline};

pub trait RemoveBy<V> {
    fn remove_by<P>(&mut self, predicate: P) -> Option<V> where P: Fn(&V) -> bool;
}

pub trait RemoveByKey<K, V> {
    fn remove_by_key<KV: Into<K>>(&mut self, element: KV) -> Option<V>;
}

impl<K, V> RemoveByKey<K, V> for Vec<(K, V)> where K: PartialEq {
    fn remove_by_key<KV: Into<K>>(&mut self, element: KV) -> Option<V> {
        let selection_key = &element.into();
        self.iter().position(|(k, _)| k == selection_key)
            .map(|idx| self.swap_remove(idx).1)
    }
}

impl<V> RemoveBy<V> for Vec<V> {
    fn remove_by<P>(&mut self, predicate: P) -> Option<V> where P: Fn(&V) -> bool {
        self.iter().position(predicate).map(|pos| self.swap_remove(pos))
    }
}

pub fn latex_block<S: Into<String>>(content: S) -> Block {
    Block::RawBlock(String::from("latex"), content.into())
}

pub fn latex_inline<S: Into<String>>(content: S) -> Inline {
    Inline::RawInline(String::from("latex"), content.into())
}

pub fn concat_map<F, T>(f: &mut F) -> impl FnMut(Vec<T>) -> Vec<T> + '_ where F: FnMut(T) -> Vec<T> {
    let res = move |mut arr: Vec<T>| {
        let mut i = 0;
        while i < arr.len() {
            let res = f(arr.remove(i));
            let offset = if res.is_empty() { 0 } else { 1 };
            for (idx, element) in res.into_iter().enumerate() {
                arr.insert(i + idx, element);
            }
            i += offset;
        }
        arr
    };

    res
}

pub trait None<T> {
    fn none<F>(&mut self, f: F) -> bool
        where
            Self: Sized,
            F: FnMut(T) -> bool;
}

impl<Iter, T> None<T> for Iter where Iter: Iterator<Item=T> {
    fn none<F>(&mut self, f: F) -> bool where Self: Sized, F: FnMut(T) -> bool {
        !self.any(f)
    }
}