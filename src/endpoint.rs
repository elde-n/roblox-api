#[macro_export]
macro_rules! endpoint {
    // -----------------------------------------------------------------------
    // Internal: method ident → reqwest::Method
    // -----------------------------------------------------------------------
    (@method GET)    => { ::reqwest::Method::GET    };
    (@method POST)   => { ::reqwest::Method::POST   };
    (@method PUT)    => { ::reqwest::Method::PUT    };
    (@method PATCH)  => { ::reqwest::Method::PATCH  };
    (@method DELETE) => { ::reqwest::Method::DELETE };

    // -----------------------------------------------------------------------
    // Internal: emit Some(&body) or None::<&()>
    // -----------------------------------------------------------------------
    (@body $body:expr) => { Some(&$body) };
    (@body)            => { None::<&()> };

    // -----------------------------------------------------------------------
    // Internal: decode dispatch
    // -----------------------------------------------------------------------
    (@decode $resp:ident, $ret:ty, json)              => { $resp.json::<$ret>().await };
    (@decode $resp:ident, $ret:ty, bytes)             => { $resp.bytes().await        };
    (@decode $resp:ident, $ret:ty, void)              => { Ok(())                     };
    (@decode $resp:ident, $ret:ty, [map $r:ident : $wrap:ty => $proj:expr]) => {
        $resp.json::<$wrap>().await.map(|$r| $proj)
    };

    // -----------------------------------------------------------------------
    // Internal: type definition dispatcher (TT muncher)
    //
    // Direct struct emission — fields matched inline with optional rename:
    //   name("literal"): Type    →   #[serde(rename = "literal")] pub name: Type,
    //   name: Type               →   pub name: Type,
    //
    // Derive selection by type name:
    //   Request  →  Serialize
    //   Response →  Deserialize
    //   other    →  Serialize, Deserialize
    // -----------------------------------------------------------------------
    // Request<'a>
    (@types_entry Request<$lt:lifetime> { $($field:ident $( ( $rename:literal ) )? : $ftype:ty),* $(,)? } $($rest:tt)*) => {
        #[derive(Serialize)]
        struct Request<$lt> {
            $(
                $(#[serde(rename = $rename)])?
                pub $field: $ftype,
            )*
        }
        endpoint!(@types_entry $($rest)*)
    };
    // Request
    (@types_entry Request { $($field:ident $( ( $rename:literal ) )? : $ftype:ty),* $(,)? } $($rest:tt)*) => {
        #[derive(Serialize)]
        struct Request {
            $(
                $(#[serde(rename = $rename)])?
                pub $field: $ftype,
            )*
        }
        endpoint!(@types_entry $($rest)*)
    };

    // Response<'a>
    (@types_entry Response<$lt:lifetime> { $($field:ident $( ( $rename:literal ) )? : $ftype:ty),* $(,)? } $($rest:tt)*) => {
        #[derive(Deserialize)]
        struct Response<$lt> {
            $(
                $(#[serde(rename = $rename)])?
                pub $field: $ftype,
            )*
        }
        endpoint!(@types_entry $($rest)*)
    };
    // Response
    (@types_entry Response { $($field:ident $( ( $rename:literal ) )? : $ftype:ty),* $(,)? } $($rest:tt)*) => {
        #[derive(Deserialize)]
        struct Response {
            $(
                $(#[serde(rename = $rename)])?
                pub $field: $ftype,
            )*
        }
        endpoint!(@types_entry $($rest)*)
    };

    // Named type with lifetime <'a>
    (@types_entry $name:ident<$lt:lifetime> { $($field:ident $( ( $rename:literal ) )? : $ftype:ty),* $(,)? } $($rest:tt)*) => {
        #[derive(Serialize, Deserialize)]
        struct $name<$lt> {
            $(
                $(#[serde(rename = $rename)])?
                pub $field: $ftype,
            )*
        }
        endpoint!(@types_entry $($rest)*)
    };
    // Named type (no lifetime)
    (@types_entry $name:ident { $($field:ident $( ( $rename:literal ) )? : $ftype:ty),* $(,)? } $($rest:tt)*) => {
        #[derive(Serialize, Deserialize)]
        struct $name {
            $(
                $(#[serde(rename = $rename)])?
                pub $field: $ftype,
            )*
        }
        endpoint!(@types_entry $($rest)*)
    };

    // Unit structs (bare name, no braces)
    (@types_entry Request $($rest:tt)*) => {
        #[derive(Serialize)] struct Request;
        endpoint!(@types_entry $($rest)*)
    };
    (@types_entry Response $($rest:tt)*) => {
        #[derive(Deserialize)] struct Response;
        endpoint!(@types_entry $($rest)*)
    };
    (@types_entry $name:ident $($rest:tt)*) => {
        #[derive(Serialize, Deserialize)] struct $name;
        endpoint!(@types_entry $($rest)*)
    };

    // Base case — no more types
    (@types_entry) => {};

    // -----------------------------------------------------------------------
    // Internal: generate the function body
    // -----------------------------------------------------------------------
    (@fn
        attrs     = [$(#[$attr:meta])*],
        client    = $client:ident,
        name      = $name:ident,
        params    = [$($param:ident : $param_ty:ty),*],
        ret       = $ret:ty,
        method    = $method:ident,
        url       = $url:expr,
        paging    = [$($p_var:ident, $p_limit:literal)?],
        types     = [$($types:tt)*],
        pre       = [$($pre:tt)*],
        query     = [$($qk:literal => $qv:expr),*],
        body      = [$($body_expr:expr)?],
        decode    = $decode:tt,
    ) => {
        $(#[$attr])*
        #[allow(unused_variables)]
        pub async fn $name(
            $client: &mut $crate::client::Client,
            $($param: $param_ty,)*
        ) -> Result<$ret, $crate::Error> {
            endpoint!(@types_entry $($types)*);
            $($pre)*

            // Paging query — generated when paging_query { ... } is present
            let mut __query_pairs: ::std::vec::Vec<(&str, &str)> = ::std::vec::Vec::new();
            $(
                let __pq_limit = $p_var.limit.unwrap_or($p_limit).to_string();
                let __pq_order = $p_var.order.unwrap_or_default().to_string();
                let __pq_cursor = match $p_var.cursor {
                    Some(cursor) => cursor.to_string(),
                    None => ::std::string::String::new(),
                };
                __query_pairs.push(("limit", &__pq_limit));
                __query_pairs.push(("sortOrder", &__pq_order));
                __query_pairs.push(("cursor", &__pq_cursor));
            )?

            let url: ::std::string::String = ::std::format!($url);

            $( __query_pairs.push(($qk, $qv)); )*
            let query = if __query_pairs.is_empty() { None } else { Some(__query_pairs.as_slice()) };

            let __response = $client
                .requestor
                .request(
                    endpoint!(@method $method),
                    &url,
                    endpoint!(@body $($body_expr)?),
                    query,
                    None,
                )
                .await?;

            endpoint!(@decode __response, $ret, $decode)
        }
    };

    // -----------------------------------------------------------------------
    // Internal: parse body — map variant
    // -----------------------------------------------------------------------
    (@body_inner
        attrs     = [$(#[$attr:meta])*],
        client    = $client:ident,
        name      = $name:ident,
        params    = [$($param:ident : $param_ty:ty),*],
        ret       = $ret:ty,
        body      = [
            $method:ident $url:expr ;
            $(paging_query { $p_var:ident, limit = $p_limit:literal })?
            $(types    { $($types:tt)* })?
            $(prelude  { $($pre:tt)* })?
            $(query    { $($qk:literal => $qv:expr),* $(,)? })?
            $(body_serialize { $body_expr:expr })?
            map |$r:ident : $wrap:ty| $proj:expr
        ],
    ) => {
        endpoint!(@fn
            attrs     = [$(#[$attr])*],
            client    = $client,
            name      = $name,
            params    = [$($param : $param_ty),*],
            ret       = $ret,
            method    = $method,
            url       = $url,
            paging    = [$($p_var, $p_limit)?],
            types     = [$($($types)*)?],
            pre       = [$($($pre)*)?],
            query     = [$($($qk => $qv),*)?],
            body      = [$($body_expr)?],
            decode    = [map $r : $wrap => $proj],
        );
    };

    // -----------------------------------------------------------------------
    // Internal: parse body — raw_bytes variant
    // -----------------------------------------------------------------------
    (@body_inner
        attrs     = [$(#[$attr:meta])*],
        client    = $client:ident,
        name      = $name:ident,
        params    = [$($param:ident : $param_ty:ty),*],
        ret       = $ret:ty,
        body      = [
            $method:ident $url:expr ;
            $(paging_query { $p_var:ident, limit = $p_limit:literal })?
            $(types    { $($types:tt)* })?
            $(prelude  { $($pre:tt)* })?
            $(query    { $($qk:literal => $qv:expr),* $(,)? })?
            $(body_serialize { $body_expr:expr })?
            raw_bytes
        ],
    ) => {
        endpoint!(@fn
            attrs     = [$(#[$attr])*],
            client    = $client,
            name      = $name,
            params    = [$($param : $param_ty),*],
            ret       = $ret,
            method    = $method,
            url       = $url,
            paging    = [$($p_var, $p_limit)?],
            types     = [$($($types)*)?],
            pre       = [$($($pre)*)?],
            query     = [$($($qk => $qv),*)?],
            body      = [$($body_expr)?],
            decode    = bytes,
        );
    };

    // -----------------------------------------------------------------------
    // Internal: parse body — void variant
    // -----------------------------------------------------------------------
    (@body_inner
        attrs     = [$(#[$attr:meta])*],
        client    = $client:ident,
        name      = $name:ident,
        params    = [$($param:ident : $param_ty:ty),*],
        ret       = $ret:ty,
        body      = [
            $method:ident $url:expr ;
            $(paging_query { $p_var:ident, limit = $p_limit:literal })?
            $(types    { $($types:tt)* })?
            $(prelude  { $($pre:tt)* })?
            $(query    { $($qk:literal => $qv:expr),* $(,)? })?
            $(body_serialize { $body_expr:expr })?
            void
        ],
    ) => {
        endpoint!(@fn
            attrs     = [$(#[$attr])*],
            client    = $client,
            name      = $name,
            params    = [$($param : $param_ty),*],
            ret       = (),
            method    = $method,
            url       = $url,
            paging    = [$($p_var, $p_limit)?],
            types     = [$($($types)*)?],
            pre       = [$($($pre)*)?],
            query     = [$($($qk => $qv),*)?],
            body      = [$($body_expr)?],
            decode    = void,
        );
    };

    // -----------------------------------------------------------------------
    // Internal: parse body — plain JSON variant
    // -----------------------------------------------------------------------
    (@body_inner
        attrs     = [$(#[$attr:meta])*],
        client    = $client:ident,
        name      = $name:ident,
        params    = [$($param:ident : $param_ty:ty),*],
        ret       = $ret:ty,
        body      = [
            $method:ident $url:expr ;
            $(paging_query { $p_var:ident, limit = $p_limit:literal })?
            $(types    { $($types:tt)* })?
            $(prelude  { $($pre:tt)* })?
            $(query    { $($qk:literal => $qv:expr),* $(,)? })?
            $(body_serialize { $body_expr:expr })?
        ],
    ) => {
        endpoint!(@fn
            attrs     = [$(#[$attr])*],
            client    = $client,
            name      = $name,
            params    = [$($param : $param_ty),*],
            ret       = $ret,
            method    = $method,
            url       = $url,
            paging    = [$($p_var, $p_limit)?],
            types     = [$($($types)*)?],
            pre       = [$($($pre)*)?],
            query     = [$($($qk => $qv),*)?],
            body      = [$($body_expr)?],
            decode    = json,
        );
    };

    // -----------------------------------------------------------------------
    // Entry: top-level types block
    // -----------------------------------------------------------------------
    (
        types { $($types:tt)* }
        $($rest:tt)*
    ) => {
        endpoint!(@types_entry $($types)*);
        endpoint!($($rest)*);
    };

    // -----------------------------------------------------------------------
    // Entry: with explicit `client` parameter
    // -----------------------------------------------------------------------
    (
        $(#[$attr:meta])*
        $name:ident($cli:ident $(, $param:ident : $param_ty:ty)*) -> $ret:ty {
            $($body:tt)*
        }
        $($rest:tt)*
    ) => {
        endpoint!(@body_inner
            attrs     = [$(#[$attr])*],
            client    = $cli,
            name      = $name,
            params    = [$($param : $param_ty),*],
            ret       = $ret,
            body      = [$($body)*],
        );
        endpoint!($($rest)*);
    };

    // -----------------------------------------------------------------------
    // Entry: without explicit `client` parameter
    // -----------------------------------------------------------------------
    (
        $(#[$attr:meta])*
        $name:ident($($param:ident : $param_ty:ty),*) -> $ret:ty {
            $($body:tt)*
        }
        $($rest:tt)*
    ) => {
        endpoint!(@body_inner
            attrs     = [$(#[$attr])*],
            client    = client,
            name      = $name,
            params    = [$($param : $param_ty),*],
            ret       = $ret,
            body      = [$($body)*],
        );
        endpoint!($($rest)*);
    };

    // -----------------------------------------------------------------------
    // Base case: nothing left to process
    // -----------------------------------------------------------------------
    () => {};
}
