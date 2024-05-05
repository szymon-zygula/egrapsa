use super::TextFormatter;
use crate::text::*;

pub struct Latex {
    title: Option<String>,
    author: Option<String>,
}

impl Latex {
    pub fn new() -> Self {
        Self {
            title: None,
            author: None,
        }
    }
}

impl TextFormatter for Latex {
    fn title(self, title: Option<String>) -> Self {
        Self { title, ..self }
    }

    fn author(self, author: Option<String>) -> Self {
        Self { author, ..self }
    }

    fn format(&self, text_tree: &TextParent) -> String {
        let mut text = String::from(
            r"
\documentclass[a5paper,12pt]{book}

\usepackage{csquotes, dirtytalk, marginnote, lipsum, scrextend, xcolor, graphicx, amssymb, amstext, amsmath, epstopdf, booktabs, verbatim, gensymb, geometry, appendix, natbib, lmodern}
\geometry{a5paper}

\usepackage[utf8]{inputenc}
\usepackage[greek.polutoniko]{babel}
\usepackage{fontspec}
\usepackage{TheanoOldStyle}

\newcommand{\alignedmarginpar}[1]{%
    \Ifthispageodd{%
        \marginpar{\raggedright\small #1}
    }{%
        \marginpar{\raggedleft\small #1}
    }%
}

\date{}

",
        );

        if let Some(author) = self.author.as_ref() {
            text.push_str(r"\author{");
            text.push_str(&author);
            text.push_str(r"}");
        }

        if let Some(title) = self.title.as_ref() {
            text.push_str(r"\title{");
            text.push_str(&title);
            text.push_str(r"}");
        }

        text.push_str(
            r"
\setcounter{secnumdepth}{0}

\begin{document}
            ",
        );

        if self.title.is_some() {
            text.push_str("\\maketitle\n");
        }

        text.push_str(&text_tree.format_for_latex());
        text.push_str(r"\end{document}");

        text
    }
}
