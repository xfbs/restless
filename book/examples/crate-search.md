# Crate Search

Consider [crates.io][], the Rust programming language package index. It has an API that
you can use to query the crates it knows about.

Imagine you want to write some code which needs to make some requests to it's
API, in order to search for crates by a string.

The very first thing you can do is define what the response should look like. For this,
typically you can define a Rust type and derive `Serialize` and `Deserialize` from the
`serde` crate. This allows Rust to automatically parse it.

```rust
{{#rustdoc_include crates_io.rs:7:10}}
```

Next, you define your request type. This contains all of the information needed to make
the request. In our case, the only thing we need to make the request is the query string
that we want to search for.

```rust
{{#rustdoc_include crates_io.rs:12:15}}
```

Now that we have both our request data structure, we can implement some traits from
`restless` that tell it how to actually perform this request. Since we want to make a
`GET` request, we will implement the `GetRequest` trait. We also need to implement the
`RequestMethod` trait, because that will implement `Request` using our `GetRequest`
implementation for us.

```rust
{{#rustdoc_include crates_io.rs:17:32}}
```

Now, our `CrateSearchRequest` type can be used by any of the `restless` clients to
perform the request.

[crates.io]: https://crates.io/
