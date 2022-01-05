# `List` filtering

Playing with Druid `List`s, and applying a filter.

The current version demonstrates what I want and approximately how it can be achieved, but I think it's probably much more cleanly achievable.

Druid already provides an impelementation of `ListIter` for `(S, Arc<Vec<T>>)`. Rather than writing an implementation of `ListIter` for `AppData`, is it possible to `lens` into `AppData` such that `List` is looking at `(String, usize)`?
