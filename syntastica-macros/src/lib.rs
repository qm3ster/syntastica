use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use schema::*;

mod schema;

static LANGUAGE_CONFIG: Lazy<LanguageConfig> = Lazy::new(|| {
    toml::from_str(include_str!("../languages.toml")).expect("invalid `languages.toml`")
});

#[proc_macro]
pub fn parsers_git(_: TokenStream) -> TokenStream {
    LANGUAGE_CONFIG
        .languages
        .iter()
        .map(|lang| {
            let feat = lang.group.to_string();
            let name = &lang.name;
            let url = &lang.parser.git.url;
            let rev = &lang.parser.git.rev;
            let external_c = lang.parser.external_scanner.c;
            let external_cpp = lang.parser.external_scanner.cpp;
            let path = match &lang.parser.git.path {
                Some(path) => quote! { Some(#path) },
                None => quote! { None },
            };
            quote! {
                #[cfg(feature = #feat)]
                compile_parser(#name, #url, #rev, #external_c, #external_cpp, #path)?;
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[proc_macro]
pub fn parsers_ffi(_: TokenStream) -> TokenStream {
    let extern_c = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let ffi_func = format_ident!("{}", lang.parser.ffi_func);
        quote! {
            #[cfg(feature = #feat)]
            fn #ffi_func() -> ::syntastica::providers::Language;
        }
    });
    let functions = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let name = format_ident!("{}", lang.name);
        let ffi_func = format_ident!("{}", lang.parser.ffi_func);
        let doc = format!(
            "Get the parser for [{}]({}/tree/{}).",
            lang.name, lang.parser.git.url, lang.parser.git.rev,
        );
        quote! {
            #[cfg(feature = #feat)]
            #[doc = #doc]
            pub fn #name() -> ::syntastica::providers::Language {
                #[cfg(not(feature = "docs"))]
                unsafe { #ffi_func() }
                #[cfg(feature = "docs")]
                ::std::unimplemented!()
            }
        }
    });
    parsers(
        functions,
        |_| true,
        Some(quote! {
            #[cfg(not(feature = "docs"))]
            extern "C" {
                #(#extern_c)*
            }
        }),
    )
}

#[proc_macro]
pub fn parsers_gitdep(_: TokenStream) -> TokenStream {
    let functions = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let feat = lang.group.to_string();
        let name = format_ident!("{}", lang.name);
        let (doc, body) = match &lang.parser.rust_func {
            Some(func) => {
                let func = format_ident!("{func}");
                let package = format_ident!("{}", lang.parser.package.replace('-', "_"));
                (
                    format!(
                        "Get the parser for [{}]({}/tree/{}).",
                        lang.name, lang.parser.git.url, lang.parser.git.rev,
                    ),
                    quote! { #package::#func() }
                )
            },
            None => (
                "**This parser is not supported by this parser collection and thus this function will panic!**"
                    .to_owned(),
                quote! { ::std::unimplemented!() }
            ),
        };
        quote! {
            #[cfg(feature = #feat)]
            #[doc = #doc]
            pub fn #name() -> ::syntastica::providers::Language {
                #body
            }
        }
    });
    parsers(functions, |lang| lang.parser.rust_func.is_some(), None)
}

fn parsers(
    functions: impl Iterator<Item = proc_macro2::TokenStream>,
    filter: impl Fn(&&Language) -> bool,
    extra: Option<proc_macro2::TokenStream>,
) -> TokenStream {
    let get_parsers = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .map(|lang| {
            let feat = lang.group.to_string();
            let name = format_ident!("{}", lang.name);
            let name_str = &lang.name;
            quote! {
                #[cfg(all(feature = #feat, not(feature = "docs")))]
                if self.0.map_or(true, |langs| langs.contains(&#name_str)) {
                    _map.insert(#name_str.to_owned(), #name());
                }
            }
        });
    let get_queries = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let name_str = &lang.name;
        let highlights = format_ident!("{}_HIGHLIGHTS", lang.name.to_uppercase());
        let injections = format_ident!("{}_INJECTIONS", lang.name.to_uppercase());
        let locals = format_ident!("{}_LOCALS", lang.name.to_uppercase());

        quote! {
            _map.insert(#name_str.to_owned(), [
                ::syntastica_queries::#highlights.into(),
                ::syntastica_queries::#injections.into(),
                ::syntastica_queries::#locals.into(),
            ]);
        }
    });
    let by_extension = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(&filter)
        .map(|lang| {
            let feat = lang.group.to_string();
            let extensions = &lang.file_extensions;
            let name_str = &lang.name;
            quote! {
                #[cfg(feature = #feat)]
                if [#(#extensions),*].contains(&file_extension) {
                    return ::std::option::Option::Some(#name_str.into());
                }
            }
        });
    let lang_count_some = LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(|lang| lang.group == Group::Some)
        .count();
    let lang_count_all = LANGUAGE_CONFIG.languages.len();
    let lang_count_most = lang_count_all - lang_count_some;
    let lang_count_parsers = quote! {
        if cfg!(feature = "all") {
            #lang_count_all
        } else if cfg!(feature = "most") {
            #lang_count_most
        } else if cfg!(feature = "some") {
            #lang_count_some
        } else {
            0
        }
    };
    quote! {
        #extra

        #(#functions)*

        // TODO: maybe create enum with all supported languages
        /// An implementation of [`LanguageProvider`](::syntastica::providers::LanguageProvider),
        /// providing all parsers in the enabled feature set (see [`all`](LanguageProviderImpl::all))
        /// or a subset of them (see [`with_languages`](LanguageProviderImpl::with_languages)).
        pub struct LanguageProviderImpl<'a>(::std::option::Option<&'a [&'a str]>);

        impl ::syntastica::providers::LanguageProvider for LanguageProviderImpl<'_> {
            fn get_parsers(&self) -> ::std::result::Result<::syntastica::providers::Parsers, ::syntastica::Error> {
                let mut _map: ::syntastica::providers::Parsers = ::std::collections::HashMap::with_capacity(#lang_count_parsers);
                #(#get_parsers)*
                ::std::result::Result::Ok(_map)
            }

            fn get_queries(&self) -> ::std::result::Result<::syntastica::providers::Queries, ::syntastica::Error> {
                let mut _map: ::syntastica::providers::Queries = ::std::collections::HashMap::with_capacity(#lang_count_all);
                #(#get_queries)*
                ::std::result::Result::Ok(_map)
            }

            fn for_extension<'a>(
                &self,
                file_extension: &'a str,
            ) -> ::std::option::Option<::std::borrow::Cow<'a, str>> {
                #(#by_extension)*
                ::std::option::Option::None
            }

            // TODO: injection regex
            // fn for_injection<'a>(&self, name: &'a str) -> ::std::option::Option<::std::borrow::Cow<'a, str>> {
            //     ::std::option::Option::None
            // }
        }
    }
    .into()
}

#[proc_macro]
pub fn parsers_gitdep_toml_deps(_: TokenStream) -> TokenStream {
    let mut added_packages = vec![];
    LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(|lang| lang.parser.rust_func.is_some())
        .filter_map(|lang| {
            let package = &lang.parser.package;
            let url = &lang.parser.git.url;
            let rev = &lang.parser.git.rev;
            if added_packages.contains(&package) {
                None
            } else {
                added_packages.push(package);
                let dep_str = format!(
                    r##"
[dependencies.{package}]
optional = true
git = "{url}"
rev = "{rev}"
"##
                );
                Some(quote! {
                    toml += #dep_str;
                })
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}

#[proc_macro]
pub fn parsers_gitdep_toml_feature_some(_: TokenStream) -> TokenStream {
    parsers_gitdep_toml_feature(Group::Some)
}

#[proc_macro]
pub fn parsers_gitdep_toml_feature_most(_: TokenStream) -> TokenStream {
    parsers_gitdep_toml_feature(Group::Most)
}

#[proc_macro]
pub fn parsers_gitdep_toml_feature_all(_: TokenStream) -> TokenStream {
    parsers_gitdep_toml_feature(Group::All)
}

fn parsers_gitdep_toml_feature(group: Group) -> TokenStream {
    let mut added_packages = vec![];
    let mut feature_str = format!("{group} = [\n");
    if let Some(group) = group.next_smaller() {
        feature_str += &format!("    \"{group}\",\n");
    }
    for lang in LANGUAGE_CONFIG
        .languages
        .iter()
        .filter(|lang| lang.parser.rust_func.is_some() && lang.group == group)
    {
        let package = &lang.parser.package;
        if added_packages.contains(&package) {
            continue;
        }
        added_packages.push(package);
        feature_str += &format!("    \"dep:{package}\",\n");
    }
    feature_str += "]\n";

    quote! { toml += #feature_str; }.into()
}

#[proc_macro]
pub fn queries(_: TokenStream) -> TokenStream {
    let langs = LANGUAGE_CONFIG.languages.iter().map(|lang| {
        let name_str = &lang.name;

        let highlights = match lang.queries.nvim_like {
            true => quote! { validate(lang, #name_str, "highlights.scm", process_highlights) },
            false => quote! { read_queries(#name_str, "highlights.scm") },
        };
        let injections = match (lang.queries.nvim_like, lang.queries.injections) {
            (true, true) => {
                quote! { validate(lang, #name_str, "injections.scm", process_injections) }
            }
            (false, true) => quote! { read_queries(#name_str, "injections.scm") },
            (_, false) => quote! { String::new() },
        };
        let locals = match (lang.queries.nvim_like, lang.queries.locals) {
            (true, true) => quote! { validate(lang, #name_str, "locals.scm", process_locals) },
            (false, true) => quote! { read_queries(#name_str, "locals.scm") },
            (_, false) => quote! { String::new() },
        };

        quote! {
            let lang = parsers.remove(#name_str).unwrap();
            let highlights = #highlights;
            let injections = #injections;
            let locals = #locals;
            _map.insert(#name_str, [highlights, injections, locals]);
        }
    });
    quote! {
        {
            let mut parsers = ::syntastica_parsers_git::LanguageProviderImpl::all().get_parsers()?;
            let mut _map: ::std::collections::BTreeMap<&'static str, [::std::string::String; 3]>
                = ::std::collections::BTreeMap::new();
            #(#langs)*
            ::std::result::Result::Ok(_map)
        }
    }
    .into()
}

#[proc_macro]
pub fn queries_test(_: TokenStream) -> TokenStream {
    LANGUAGE_CONFIG
        .languages
        .iter()
        .map(|lang| {
            let name = format_ident!("{}", lang.name);
            let name_str = &lang.name;
            let highlights = format_ident!("{}_HIGHLIGHTS", lang.name.to_uppercase());
            let injections = format_ident!("{}_INJECTIONS", lang.name.to_uppercase());
            let locals = format_ident!("{}_LOCALS", lang.name.to_uppercase());
            quote! {
                #[test]
                fn #name() {
                    let lang = PARSERS.get(#name_str).unwrap().clone();
                    validate_query(lang, ::syntastica_queries::#highlights, "highlights");
                    validate_query(lang, ::syntastica_queries::#injections, "injections");
                    validate_query(lang, ::syntastica_queries::#locals, "locals");
                }
            }
        })
        .collect::<proc_macro2::TokenStream>()
        .into()
}
