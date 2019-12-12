pub type ParseResult<'a, R> = Option<(&'a [u8], R)>;

pub fn token<P, R>(predicate: P) -> impl Fn(&[u8]) -> ParseResult<R>
where
    P: Fn(u8) -> Option<R>,
{
    move |s| {
        let (first, rest) = s.split_first()?;
        let result = predicate(*first)?;
        Some((rest, result))
    }
}

const EMPTY_SLICE: &[u8] = &[];
pub fn chunk<'a>(chunk: &'a [u8]) -> impl Fn(&'a [u8]) -> ParseResult<()> {
    move |s| {
        if chunk == s.get(0..chunk.len())? {
            let rest = s.get(chunk.len()..).unwrap_or(EMPTY_SLICE);
            Some((rest, ()))
        } else {
            None
        }
    }
}

pub fn satisfy<P>(predicate: P) -> impl Fn(&[u8]) -> ParseResult<u8>
where
    P: Fn(u8) -> bool,
{
    token(move |c| if predicate(c) { Some(c) } else { None })
}

pub fn byte(byte: u8) -> impl Fn(&[u8]) -> ParseResult<u8> {
    satisfy(move |c| c == byte)
}

pub fn take_while_p<P>(predicate: P) -> impl Fn(&[u8]) -> ParseResult<&[u8]>
where
    P: Fn(u8) -> bool,
{
    move |s| {
        let mut ix = 0;
        for c in s {
            if !predicate(*c) {
                break;
            }

            ix += 1;
        }
        Some((&s[ix..], &s[0..ix]))
    }
}

pub fn take_while1_p<P>(predicate: P) -> impl Fn(&[u8]) -> ParseResult<&[u8]>
where
    P: Fn(u8) -> bool,
{
    move |s| {
        s.get(0).map(|c| predicate(*c))?;

        let mut ix = 1;
        for c in s.get(1..)? {
            if !predicate(*c) {
                break;
            }

            ix += 1;
        }
        Some((&s[ix..], &s[0..ix]))
    }
}

pub fn take_while<P, R>(parser: P) -> impl Fn(&[u8]) -> ParseResult<Vec<R>>
where
    P: Fn(&[u8]) -> ParseResult<R>,
{
    move |mut s| {
        let mut results = Vec::new();
        while let Some((new_s, result)) = parser(s) {
            s = new_s;
            results.push(result)
        }
        Some((s, results))
    }
}

pub fn map<'a, P, F, Rin, Rout>(parser: P, func: F) -> impl Fn(&'a [u8]) -> ParseResult<Rout>
where
    P: Fn(&'a [u8]) -> ParseResult<Rin>,
    F: Fn(Rin) -> Rout,
    Rin: 'a,
{
    move |s| parser(s).map(|(s, result_in)| (s, func(result_in)))
}

pub fn void<P, Rin, Rout>(parser: P) -> impl Fn(&[u8]) -> ParseResult<()>
where
    P: Fn(&[u8]) -> ParseResult<Rin>,
{
    move |s| parser(s).map(|(s, _result)| (s, ()))
}

pub fn sep_by<'a, P, S, RP, RS>(
    parser: P,
    separator: S,
) -> impl Fn(&'a [u8]) -> ParseResult<Vec<RP>>
where
    P: Fn(&'a [u8]) -> ParseResult<RP>,
    S: Fn(&'a [u8]) -> ParseResult<RS>,
    RP: 'a,
{
    move |s| {
        let mut results = Vec::new();
        let mut s_before_p = s;
        let mut s_after_p = s;

        while let Some((new_s, result)) = parser(s_before_p) {
            s_after_p = new_s;
            results.push(result);

            if let Some((new_s, _)) = separator(s_after_p) {
                s_before_p = new_s;
            } else {
                break;
            }
        }
        Some((s_after_p, results))
    }
}
