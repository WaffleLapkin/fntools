# fntools

[![Build Status dev](https://img.shields.io/travis/com/WaffleLapkin/fntools/dev?logo=GitHub)](https://travis-ci.com/WaffleLapkin/fntools)
[![Build Status master](https://travis-ci.com/WaffleLapkin/fntools.svg?branch=master)](https://travis-ci.com/WaffleLapkin/fntools)
[![Telegram](https://img.shields.io/badge/tg-WaffleLapkin-9cf?logo=telegram)](https://vee.gg/t/WaffleLapkin)
[![docs.rs](https://img.shields.io/badge/docs.rs-link-blue.svg)](https://docs.rs/fntools)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/fntools)


<img height="256" width="256" align="left" src="./logo.svg" alt="logo"> Weird tools for working with functions in rust <pre lang="rust">let fun = (|a, b| a + b)
&nbsp;   .chain(|x| (x % 2, x % 4))
&nbsp;   .chain(|t, f| (t, f)
&nbsp;        .also(|(t, f)| println!("{}, {}", t, f))
&nbsp;   );
&nbsp;
assert_eq!(fun(13, 10), (1, 3));
</pre>
<br><br>
## DISCLAIMER
This library more an fun experiment with rust, than really useful library.

However, in some cases it can make code a bit cleaner.
