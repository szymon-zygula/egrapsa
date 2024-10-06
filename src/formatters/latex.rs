use super::{TextFormatter, Work};
use crate::text::*;

pub struct Latex {
    title: Option<String>,
    author: Option<String>,
    works: Vec<Work>,
    catchwords: bool,
}

impl Latex {
    pub fn new() -> Self {
        Self {
            title: None,
            author: None,
            works: Vec::default(),
            catchwords: false,
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

    fn add_work(mut self, work: Work) -> Self {
        self.works.push(work);
        self
    }

    fn catchwords(self, catchwords: bool) -> Self {
        Self { catchwords, ..self }
    }

    fn format(&self) -> String {
        let mut text = String::from(
            r"
\documentclass[a5paper,12pt]{book}

\usepackage{csquotes, dirtytalk, marginnote, lipsum, scrextend, xcolor, graphicx, amssymb, amstext, amsmath, epstopdf, booktabs, verbatim, gensymb, geometry, appendix, natbib, lmodern}
\usepackage[pagestyles]{titlesec}
\usepackage{fancyhdr}
\usepackage{etoolbox}
\geometry{a5paper}

\usepackage[utf8]{inputenc}
\usepackage[greek.polutoniko]{babel}
\usepackage{fontspec}
\usepackage{TheanoOldStyle}
\usepackage{fwlw}
\usepackage{tocloft}

\newcommand{\alignedmarginpar}[1]{%
    \Ifthispageodd{%
        \marginpar{\raggedright\small #1}
    }{%
        \marginpar{\raggedleft\small #1}
    }%
}

\date{}

\titlespacing*{\chapter}{0pt}{0pt}{15pt}

\newcommand{\likechapter}[1]{{\center\huge #1 \\
\vspace{50pt}}}

\titleformat{\chapter}[display]{\normalfont\bfseries}{}{0pt}{\Huge\center}
\renewcommand{\chaptermark}[1]{\markboth{#1}{}}

\newcommand{\altchapter}{}
\newcommand{\orgchapter}{}
\fancyhf{}
\fancyhead[LE, RO]{\thepage}
\fancyhead[CE]{\orgchapter}
\fancyhead[CO]{\altchapter}

",
        );

        if self.catchwords {
            text.push_str(
                r"
\fancyfoot[R]{\usebox\NextWordBox}
                      ",
            );
        }

        text.push_str(
            r"
\fancypagestyle{plain}{
\fancyhf{}
\fancyhead[RO, LE]{\thepage}
                      ",
        );

        if self.catchwords {
            text.push_str(
                r"
\fancyfoot[R]{\usebox\NextWordBox}
                      ",
            );
        }

        text.push_str(
            r"
}
\renewcommand\headrulewidth{0pt}
\pagestyle{fancy}
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

        for (i, work) in self.works.iter().enumerate() {
            if i != 0 {
                text.push_str(
                    r"
\Ifthispageodd{%
    \clearpage\null\thispagestyle{empty}
}{%
    \clearpage\null
    \clearpage\null\thispagestyle{empty}
}%
                          ",
                );
            }

            text.push_str(r"\chapter");
            if let Some(alt_title) = &work.alt_title {
                text.push_str(&format!("[{} ({})]", work.title, alt_title));
            }
            text.push_str("{");
            text.push_str(&work.title);
            text.push_str(".}\n");
            text.push_str(r"\renewcommand{\orgchapter}{");
            text.push_str(&work.title);
            text.push_str(".}\n");
            text.push_str(r"\renewcommand{\altchapter}{");
            text.push_str(work.alt_title.as_ref().unwrap_or(&work.title));
            text.push_str(
                r".}
                \likechapter{\altchapter}

                ",
            );

            text.push_str(&work.text.format_for_latex());
        }

        text.push_str(r"\renewcommand{\altchapter}{}");

        text.push_str(
            r"
\clearpage\null\thispagestyle{empty}
\Ifthispageodd{%
    \clearpage\null\thispagestyle{empty}
    \clearpage\null\thispagestyle{empty}
}{%
    \clearpage\null\thispagestyle{empty}
}%
\renewcommand{\contentsname}{Index}
\renewcommand{\cftchapleader}{\cftdotfill{\cftdotsep}}
\center
\tableofcontents",
        );
        text.push_str(r"\end{document}");

        text
    }
}
