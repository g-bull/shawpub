// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use ego_tree::iter::Edge;
use scraper::{Html, Node, Selector};

use crate::transliterator::Transliterator;


pub fn html2shaw(xhtml_content : &String, trans: Transliterator)  -> String {
    let mut document = Html::parse_document(xhtml_content);
    let skip_tags = ["script", "style", "meta", "head", "title"];

    let mut text_node_ids = Vec::new();
    let mut skip_depth = 0;

    // 1. Traverse the tree immutably to find target text node IDs
    for edge in document.tree.root().traverse() {
        match edge {
            Edge::Open(node_ref) => match node_ref.value() {
                Node::Element(element) => {
                    let tag_name = element.name();
                    if skip_tags.iter().any(|&t| t.eq_ignore_ascii_case(tag_name)) {
                        skip_depth += 1;
                    }
                }
                Node::Text(_) => {
                    if skip_depth == 0 {
                        text_node_ids.push(node_ref.id());
                    }
                }
                _ => {}
            },
            Edge::Close(node_ref) => {
                if let Node::Element(element) = node_ref.value() {
                    let tag_name = element.name();
                    if skip_tags.iter().any(|&t| t.eq_ignore_ascii_case(tag_name)) {
                        skip_depth -= 1;
                    }
                }
            }
        }
    }

    // 2. Mutate the collected text nodes in place
    for id in text_node_ids {
        if let Some(mut node) = document.tree.get_mut(id) {
            if let Node::Text(text) = node.value() {
                // `text.text` is a `tendril::StrTendril`, which converts from `String`
                text.text = trans.transliterate(text.text.to_string()).into();
            }
        }
    }

    // 3. Serialize the modified DOM back to an HTML string
    document.html()

}

/* 
fn main() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Hidden Title</title>
            <meta name="description" content="Hidden Meta">
            <style>.hidden { display: none; }</style>
        </head>
        <body>
            <h1>Hello, World!</h1>
            <script>console.log("Hidden Script");</script>
            <p>This is <strong>visible</strong> text.</p>
        </body>
        </html>
    "#;

    let clean_text = extract_visible_text(html);
    println!("{}", clean_text);
    // Output: Hello, World! This is visible text.
}
*/