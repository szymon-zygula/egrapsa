use super::{Language, TextFormatter, Work};
use crate::config::FormatterConfig;
use crate::text::*;
use regex::Regex;

pub struct Latex {
    config: FormatterConfig,
    works: Vec<Work>,
}

impl Latex {
    pub fn new() -> Self {
        Self {
            config: FormatterConfig::default(),
            works: Vec::default(),
        }
    }

    fn get_language_packages(&self) -> &str {
        match self.config.language {
            Language::Latin => {
                r"
\usepackage[latin]{babel}
\usepackage[oldstyle, veryoldstyle]{kpfonts}"
            }
            Language::Greek => {
                r"
\usepackage[greek.polutoniko]{babel}
\usepackage{TheanoOldStyle}"
            }
        }
    }

    // Replace some characters not likely to be found in fonts
    fn normalize(mut text: String) -> String {
        text = text.replace("ↄ", r"\rotatebox[origin=c]{180}{c}"); // Roman numeral ↄ
        let marginpar_regex = Regex::new(r" \\refnumber\{(.*)\} ").unwrap();
        marginpar_regex
            .replace_all(&text, "\\refnumber{$1}")
            .to_string()
    }
}

impl TextFormatter for Latex {
    fn set_title(&mut self, title: Option<String>) {
        self.config.title = title.map(|x| x.format_for_latex(&self.config));
    }

    fn set_author(&mut self, author: Option<String>) {
        self.config.author = author.map(|x| x.format_for_latex(&self.config));
    }

    fn set_catchwords(&mut self, catchwords: bool) {
        self.config.catchwords = catchwords;
    }

    fn set_margin_notes(&mut self, margin_notes: bool) {
        self.config.ref_numbers = margin_notes;
    }

    fn set_footnotes(&mut self, footnotes: bool) {
        self.config.footnotes = footnotes;
    }

    fn add_work(&mut self, work: Work) {
        let work = Work {
            title: work.title.format_for_latex(&self.config),
            alt_title: work.alt_title.map(|x| x.format_for_latex(&self.config)),
            ..work
        };

        self.works.push(work);
    }

    fn set_language(&mut self, language: Language) {
        self.config.language = language;
    }

    fn format(&self) -> String {
        let mut text = String::from(
            r"
\documentclass[a5paper,12pt]{book}

\usepackage{csquotes, dirtytalk, marginnote, lipsum, scrextend, xcolor, graphicx, amssymb, amstext, amsmath, epstopdf, booktabs, verbatim, gensymb, geometry, appendix, natbib, lmodern}
\usepackage[pagestyles]{titlesec}
\usepackage{fancyhdr}
\usepackage{needspace}
\usepackage{etoolbox}
\usepackage{mparhack}
\usepackage{psvectorian}
\geometry{a5paper, bottom=2.5cm}

\usepackage[utf8]{inputenc}",
        );
        text.push_str(self.get_language_packages());
        if self.config.catchwords {
            text.push_str("\\usepackage{fwlw}");
        }

        text.push_str(
            r"
\usepackage{fontspec}
\usepackage{tocloft}
\usepackage[perpage]{footmisc}

% Show paragraphs in ToC (actually not used for paragraphs but for chapters)
\setcounter{tocdepth}{4}
\setcounter{secnumdepth}{4}

\usepackage{enumitem}
\makeatletter
\newcommand{\greekalpha}[1]{\c@greekalpha{#1}}
\newcommand{\c@greekalpha}[1]{%
  {%
    \ifcase\number\value{#1}%
    \or α´\or β´\or γ´\or δ´\or ε´\or ϛ´\or ζ´\or η´\or θ´\or ι´%
    \or ια´\or ιβ´\or ιγ´\or ιδ´\or ιε´\or ιϛ´\or ιζ´\or ιη´\or ιθ´%
    \or κα´\or κβ´\or κγ´\or κδ´\or κε´\or κϛ´\or κζ´\or κη´\or κθ´%
    \or λα´\or λβ´\or λγ´\or λδ´\or λε´\or λϛ´\or λζ´\or λη´\or λθ´%
    \or μα´\or μβ´\or μγ´\or μδ´\or με´\or μϛ´\or μζ´\or μη´\or μθ´%
    \or να´\or νβ´\or νγ´\or νδ´\or νε´\or νϛ´\or νζ´\or νη´\or νθ´%
    \fi
  }%
}

\AddEnumerateCounter*{\greekalpha}{\c@greekalpha}{5}
\makeatother

\usepackage{sectsty}
\allsectionsfont{\centering}

\newcommand{\refnumber}[1]{",
        );

        if self.config.ref_numbers {
            text.push_str(
                r" {\scriptsize\color{gray}(#1)} ",
            );
        }

        text.push_str(r"}

\date{}

\makeatletter
\renewcommand{\@seccntformat}[1]{%
  \ifcsname prefix@#1\endcsname
    \csname prefix@#1\endcsname
  \else
    \csname the#1\endcsname\quad
  \fi}
\newcommand\prefix@section{}
\makeatother

\titlespacing*{\chapter}{0pt}{0pt}{15pt}

\newcommand{\likechapter}[1]{{\center\huge #1 \\
\vspace{50pt}}}

\titleformat{\chapter}[display]{\normalfont\bfseries}{}{0pt}{\Huge\center}
\renewcommand{\chaptermark}[1]{\markboth{#1}{}}

% Start new sections on new pages
\AddToHook{cmd/section/before}{%
    \ifnum\value{section}=1%
    \else%
        % If current page is odd, it means that that the page left to the new section is going to be empty,
        % and so the title of the current work won't be visible anywhere. In that case it is added
        % by \thispagestyle{sectionpage}. Otherwise we can use plain style.
        \Ifthispageodd{%
            \cleardoublepage\thispagestyle{sectionpage}%
        }{%
            \cleardoublepage\thispagestyle{plain}%
        }%
    \fi%
}

\newcommand{\altchapter}{}
\newcommand{\orgchapter}{}
\newcommand{\orgsection}{}
\newcommand{\rectohead}{}
\newcommand{\versohead}{}
\fancyhf{}
\fancyhead[LE, RO]{\thepage}
\fancyhead[CE]{\versohead}
\fancyhead[CO]{\rectohead}
\setlength{\headheight}{14.5pt}
\setlength{\marginparpush}{-6pt}
",
        );

        if self.config.catchwords {
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

        if self.config.catchwords {
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

        // Use "versohead" on recto pages where new sections begin,
        // so that name of the work is visible
        text.push_str(
            r"
\fancypagestyle{sectionpage}{
\fancyhf{}
\fancyhead[CO]{\versohead}
\fancyhead[RO, LE]{\thepage}
",
        );

        if self.config.catchwords {
            text.push_str(
                r"
\fancyfoot[R]{\usebox\NextWordBox}
",
            );
        }
        text.push_str("}");

        if let Some(author) = self.config.author.as_ref() {
            text.push_str(r"\author{");
            text.push_str(&author);
            text.push_str(r"}");
        }

        if let Some(title) = self.config.title.as_ref() {
            text.push_str(r"\title{");
            text.push_str(&title);
            text.push_str(r"}");
        }

        text.push_str(
            r"

\begin{document}
",
        );

        if self.config.title.is_some() {
            text.push_str("\\maketitle\n");
            text.push_str(r"\clearpage\null\thispagestyle{empty}");
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

            text.push_str(&format!(r"\chapter*{{{}.}}", work.title));
            if let Some(alt_title) = &work.alt_title {
                text.push_str(&format!(
                    r"
\addtocontents{{toc}}{{\protect\vskip-10pt\needspace{{2\baselineskip}}}}
\addtocontents{{toc}}{{\protect\contentsline{{chapter}}{{{}}}{{}}{{}}}}
\addcontentsline{{toc}}{{paragraph}}{{\textbf{{({})}}}}
",
                    work.title, alt_title,
                ));
            } else {
                text.push_str(&format!(
                    r"
\addcontentsline{{toc}}{{paragraph}}{{\textbf{{{}}}}}",
                    work.title,
                ));
            }

            text.push_str("\\setcounter{section}{0}\n");
            text.push_str("\\renewcommand{\\rectohead}{}\n");
            text.push_str("\\renewcommand{\\orgsection}{}\n");
            text.push_str("\\thispagestyle{plain}\n");
            text.push_str(&format!(
                "\\renewcommand{{\\orgchapter}}{{{}}}\n",
                work.title
            ));
            text.push_str(&format!(
                "\\renewcommand{{\\altchapter}}{{{}}}\n",
                work.alt_title.as_ref().unwrap_or(&work.title)
            ));

            if work.alt_title.is_some() {
                text.push_str(
                    r"
\likechapter{\altchapter.}
\renewcommand{\versohead}{\orgchapter.}
",
                );
            } else {
                text.push_str(
                    r"
\renewcommand{\versohead}{\orgchapter.}
",
                );
            }

            text.push_str(&work.text.format_for_latex(&self.config));
        }

        text.push_str(
            r"
\vfill
\center
\begin{pspicture}(-1.5,-3.5)(1.5,1.5)%
\rput(0,0){\Large \textbf{FINIS.}}
\rput[t](0,-1.0){\psvectorian[width=5cm]{68}}
\end{pspicture}%
\renewcommand{\altchapter}{}
\clearpage\null\thispagestyle{empty}
\Ifthispageodd{%
    \clearpage\null\thispagestyle{empty}
    \clearpage\null\thispagestyle{empty}
}{%
    \clearpage\null\thispagestyle{empty}
}%
\renewcommand{\contentsname}{Index.}
\setlength{\cftparaindent}{0pt}
\renewcommand{\versohead}{Index.}
\tableofcontents
\vspace{1cm}
\textbf{FINIS TABULÆ.}
",
        );
        text.push_str(r"\end{document}");

        Self::normalize(text)
    }
}
