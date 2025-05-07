use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use proc_macro::TokenStream as ProcTokenStream;

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        // Basic Kannada keywords
        "ನಿಜ" => "true",
        "ಸುಳ್ಳು" => "false",
        "ಕಾರ್ಯ" => "fn",
        "ಮಾಡು" => "let",
        "ಯದಿ" => "if",
        "ಅಥವಾ" => "else",
        "ರಚನೆ" => "struct",
        "ವರ್ಗ" => "enum",
        "ಸಾರ್ವಜನಿಕ" => "pub",
        "ನಿರ್ದೇಶಕ" => "mod",
        "ಬಳಕೆ" => "use",
        "ಮುರಿ" => "break",
        "ಮುಂದುವರಿ" => "continue",
        "ಹಿಂತಿರುಗಿಸು" => "return",
        "ಸ್ವಯಂ" => "self",
        "ಸೂಪರ್" => "super",
        "ಸ್ಥಿರ" => "static",
        "ಸ್ಥಿರಾಂಕ" => "const",
        "ಮುಖ್ಯ" => "main",
        "ಯಾವಾಗ" => "while",
        "ಬದಲಾಯಿಸಬಹುದಾದ" => "mut",
        "ಬದಲು" => "mut",
        "ಮುದ್ರಿಸು" => "println",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn kannada(input: ProcTokenStream) -> ProcTokenStream {
    let input: TokenStream = input.into();
    let mut returned = Vec::new();
    replace_stream(input, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out.into()
}