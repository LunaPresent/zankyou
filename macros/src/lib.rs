use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(CommandCategory)]
pub fn derive_command_category(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let Data::Enum(data) = input.data else {
		panic!("can only derive `CommandCategory` for enum type.");
	};

	let ident = format_ident!("{}", input.ident);

	let variants = data.variants.iter().map(|variant| {
		let ident = format_ident!("{}", variant.ident);
		let name = variant.ident.to_string().to_case(Case::Kebab);

		quote! { #name => Ok(Self::#ident(rest.parse()?)), }
	});

	let expanded = quote! {
		impl std::str::FromStr for #ident {
			type Err = ::parser::ParseCommandError;

			fn from_str(string: &str) -> Result<Self, Self::Err> {
				use ::parser::*;

				let (category, rest) = match string.trim().split_once(' ') {
					Some((category, rest)) => (category, rest),
					None => (string.trim(), ""),
				};

				match category {
					"" => Err(ParseCommandError::EmptyString),
					#(#variants)*
					_ => Err(ParseCommandError::UnknownCategory(category.into())),
				}
			}
		}
	};

	expanded.into()
}

#[proc_macro_derive(Subcommand)]
pub fn derive_subcommand(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let Data::Enum(data) = input.data else {
		panic!("can only derive `Subcommand` for enum type.");
	};

	let ident = format_ident!("{}", input.ident);

	let variants = data.variants.iter().map(|variant| {
		let ident = format_ident!("{}", variant.ident);
		let name = variant.ident.to_string().to_case(Case::Kebab);

		let fields = variant
			.fields
			.iter()
			.filter_map(|field| field.ident.as_ref().map(|ident| format_ident!("{}", ident)));

		let field_strings = fields
			.clone()
			.map(|field| field.to_string().to_case(Case::Kebab));

		// Borrow checker strikes again
		let strings2 = field_strings.clone();

		let constructor = if variant.fields.is_empty() {
			quote! { Self::#ident }
		} else {
			quote! {
				Self::#ident {
					#(#fields: args.get(#field_strings)?),*
				}
			}
		};

		quote! {
			#name => {
				let args = ArgumentParser::new().required(&[#(#strings2),*]).parse(rest)?;
				Ok(#constructor)
			}
		}
	});

	let expanded = quote! {
		impl std::str::FromStr for #ident {
			type Err = ::parser::ParseCommandError;

			fn from_str(string: &str) -> Result<Self, Self::Err> {
				use ::parser::*;

				let (subcommand, rest) = match string.split_once(' ') {
					Some((subcommand, rest)) => (subcommand, rest),
					None => (string, ""),
				};

				match subcommand {
					"" => Err(ParseCommandError::NoSubcommand),
					#(#variants)*
					_ => Err(ParseCommandError::UnknownSubcommand(subcommand.to_string())),
				}
			}
		}
	};

	expanded.into()
}
