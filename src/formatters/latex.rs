use super::TextFormatter;
use crate::text::*;

pub struct Latex {}

impl TextFormatter for Latex {
    fn format(text_tree: &TextTree) -> String {
        let mut text = String::from(
r"
\documentclass[a4paper,12pt]{book}

\usepackage{marginnote, lipsum, scrextend, xcolor, graphicx, amssymb, amstext, amsmath, epstopdf, booktabs, verbatim, gensymb, geometry, appendix, natbib, lmodern}
\geometry{a4paper}

\usepackage[utf8]{inputenc}
\usepackage[greek.polutoniko]{babel}
\usepackage{fontspec}
\usepackage{TheanoOldStyle}

\newcommand{\alignedmarginpar}[1]{%
    \ifthispageodd{%
        \marginpar{\raggedright\small #1}
    }{%
        \marginpar{\raggedleft\small #1}
    }%
}

\begin{document}
",
        );

        text.push_str(&text_tree.format_for_latex());
        text.push_str(r"\end{document}");

        text
    }
}
