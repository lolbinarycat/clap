//! Dummy implementations that we emit along with an error.

use proc_macro2::Ident;
use proc_macro_error::append_dummy;
use quote::quote;

pub fn clap_struct(name: &Ident) {
    into_app(name);
    args(name);
    append_dummy(quote!( impl clap::Clap for #name {} ));
}

pub fn clap_enum(name: &Ident) {
    into_app(name);
    subcommand(name);
    append_dummy(quote!( impl clap::Clap for #name {} ));
}

pub fn into_app(name: &Ident) {
    append_dummy(quote! {
        impl clap::IntoApp for #name {
            fn into_app<'b>() -> clap::App<'b> {
                unimplemented!()
            }
            fn into_app_for_update<'b>() -> clap::App<'b> {
                unimplemented!()
            }
        }
    });
}

pub fn from_arg_matches(name: &Ident) {
    append_dummy(quote! {
        impl clap::FromArgMatches for #name {
            fn from_arg_matches(_m: &clap::ArgMatches) -> Option<Self> {
                unimplemented!()
            }
            fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) {
                unimplemented!()
            }
        }
    });
}

pub fn subcommand(name: &Ident) {
    from_arg_matches(name);
    append_dummy(quote! {
        impl clap::Subcommand for #name {
            fn augment_subcommands(_app: clap::App<'_>) -> clap::App<'_> {
                unimplemented!()
            }
            fn augment_subcommands_for_update(_app: clap::App<'_>) -> clap::App<'_> {
                unimplemented!()
            }
            fn has_subcommand(name: &str) -> bool {
                unimplemented!()
            }
        }
    });
}

pub fn args(name: &Ident) {
    from_arg_matches(name);
    append_dummy(quote! {
        impl clap::Args for #name {
            fn augment_args(_app: clap::App<'_>) -> clap::App<'_> {
                unimplemented!()
            }
            fn augment_args_for_update(_app: clap::App<'_>) -> clap::App<'_> {
                unimplemented!()
            }
        }
    });
}

pub fn arg_enum(name: &Ident) {
    append_dummy(quote! {
        impl clap::ArgEnum for #name {
            fn value_variants<'a>() -> &'a [Self]{
                unimplemented!()
            }
            fn from_str(_input: &str, _case_insensitive: bool) -> Result<Self, String> {
                unimplemented!()
            }
            fn arg_value<'a>(&self) -> Option<clap::ArgValue<'a>>{
                unimplemented!()
            }
        }
    })
}
