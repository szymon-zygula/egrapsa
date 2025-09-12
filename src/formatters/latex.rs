use super::{Language, TextFormatter, Typography, Work};
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

    pub fn get_language_packages(&self) -> String {
        match self.config.language {
            Language::Latin => {
                let kpfonts_options = match self.config.typography {
                    Typography::Modern => "",
                    Typography::Old => "oldstyle",
                    Typography::VeryOld => "oldstyle, veryoldstyle",
                };
                
                if kpfonts_options.is_empty() {
                    format!(
                        r"
\usepackage[latin]{{babel}}
\usepackage{{kpfonts}}"
                    )
                } else {
                    format!(
                        r"
\usepackage[latin]{{babel}}
\usepackage[{}]{{kpfonts}}", 
                        kpfonts_options
                    )
                }
            }
            Language::Greek => {
                // For Greek, typography setting doesn't affect the package choice for now
                String::from(
                    r"
\usepackage[greek.polutoniko]{babel}
\usepackage{TheanoOldStyle}"
                )
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

    fn set_typography(&mut self, typography: Typography) {
        self.config.typography = typography;
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
        text.push_str(&self.get_language_packages());
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
            text.push_str(r" {\scriptsize\color{gray}(#1)} ");
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

#[cfg(test)]
mod tests {
    use crate::{
        formatters::{latex::Latex, Language, TextFormatter, Typography, Work},
        text::{Footnote, TextNode, TextNodeKind, TextParent},
    };

    fn make_paragraph(texts: Vec<Box<dyn TextNode>>) -> TextParent {
        TextParent {
            name: None,
            kind: TextNodeKind::Paragraph,
            subtexts: texts,
        }
    }

    fn make_section(subtexts: Vec<Box<dyn TextNode>>) -> TextParent {
        TextParent {
            name: None,
            kind: TextNodeKind::Section,
            subtexts,
        }
    }

    #[test]
    fn formats_basic_chapter_with_section_and_footnote() {
        // Build a small text tree: Section -> Paragraph -> (String + Footnote)
        let paragraph = make_paragraph(vec![
            Box::new(String::from("et homo ae")), // tests ligatures & start-of-line 'et' preservation
            Box::new(Footnote(String::from("lost"))), // tests footnote emission
        ]);
        let section = make_section(vec![Box::new(paragraph)]);

        let work = Work {
            title: "WORKTITLE".into(),
            alt_title: Some("ALT".into()),
            text: section,
        };

        let mut formatter = Latex::new();
        formatter.set_title(Some("My Title".into()));
        formatter.set_author(Some("Author".into()));
        formatter.set_catchwords(false);
        formatter.set_margin_notes(false);
        formatter.set_footnotes(true);
        formatter.set_language(Language::Latin);
        formatter.set_typography(Typography::VeryOld);
        formatter.add_work(work);

        let output = formatter.format();

        // Core document metadata
        assert!(
            output.contains(r"\\title{My Title}") || output.contains("\\title{My Title}"),
            "Missing title block:\n{output}"
        );
        assert!(
            output.contains(r"\\author{Author}") || output.contains("\\author{Author}"),
            "Missing author block:\n{output}"
        );

        // Chapter + alt title handling
        assert!(
            output.contains(r"\\chapter*{WORKTITLE.}") || output.contains("\\chapter*{WORKTITLE.}"),
            "Missing chapter heading:\n{output}"
        );
        assert!(
            output.contains(r"\\textbf{(ALT)}") || output.contains("\\textbf{(ALT)}"),
            "Missing alt title ToC entry:\n{output}"
        );
        assert!(
            output.contains(r"\\likechapter{\\altchapter.}")
                || output.contains("\\likechapter{\\altchapter.}"),
            "Missing likechapter rendering for alt title:\n{output}"
        );

        // Section heading (Latin variant)
        assert!(
            output.contains(r"\\section*{Liber \\Roman{section}.}")
                || output.contains("\\section*{Liber \\Roman{section}.}"),
            "Missing section heading (Latin):\n{output}"
        );

        // Text normalization & ligatures
        assert!(
            output.contains("et homo æ"),
            "Ligature or word replacement failed:\n{output}"
        );
        assert!(
            output.contains("et homo"),
            "Initial 'et' should not be replaced globally:\n{output}"
        );

        // Footnote emission
        assert!(
            output.contains(r"\\footnote{lost.") || output.contains("\\footnote{lost."),
            "Footnote not rendered or malformed:\n{output}"
        );

        // Language-specific package (Latin)
        assert!(
            output.contains(r"usepackage[latin]{babel}")
                || output.contains("usepackage[latin]{babel}"),
            "Missing Latin babel package (language packages section):\n{output}"
        );
    }

    #[test]
    fn typography_modern_uses_no_kpfonts_options() {
        let mut formatter = Latex::new();
        formatter.set_language(Language::Latin);
        formatter.set_typography(Typography::Modern);
        
        let packages = formatter.get_language_packages();
        
        // Should contain kpfonts without options
        assert!(
            packages.contains(r"\usepackage{kpfonts}") && !packages.contains("[oldstyle"),
            "Modern typography should use kpfonts without oldstyle options:\n{packages}"
        );
        assert!(
            packages.contains(r"\usepackage[latin]{babel}"),
            "Should include babel package:\n{packages}"
        );
    }

    #[test]
    fn typography_old_uses_oldstyle_option() {
        let mut formatter = Latex::new();
        formatter.set_language(Language::Latin);
        formatter.set_typography(Typography::Old);
        
        let packages = formatter.get_language_packages();
        
        // Should contain kpfonts with oldstyle option but not veryoldstyle
        assert!(
            packages.contains(r"\usepackage[oldstyle]{kpfonts}") && !packages.contains("veryoldstyle"),
            "Old typography should use kpfonts with oldstyle option only:\n{packages}"
        );
        assert!(
            packages.contains(r"\usepackage[latin]{babel}"),
            "Should include babel package:\n{packages}"
        );
    }

    #[test]
    fn typography_very_old_uses_both_options() {
        let mut formatter = Latex::new();
        formatter.set_language(Language::Latin);
        formatter.set_typography(Typography::VeryOld);
        
        let packages = formatter.get_language_packages();
        
        // Should contain kpfonts with both oldstyle and veryoldstyle options
        assert!(
            packages.contains(r"\usepackage[oldstyle, veryoldstyle]{kpfonts}"),
            "VeryOld typography should use kpfonts with both oldstyle and veryoldstyle options:\n{packages}"
        );
        assert!(
            packages.contains(r"\usepackage[latin]{babel}"),
            "Should include babel package:\n{packages}"
        );
    }

    #[test]
    fn typography_does_not_affect_greek_packages() {
        let mut formatter = Latex::new();
        formatter.set_language(Language::Greek);
        formatter.set_typography(Typography::Modern);
        
        let packages_modern = formatter.get_language_packages();
        
        formatter.set_typography(Typography::VeryOld);
        let packages_very_old = formatter.get_language_packages();
        
        // Greek packages should be identical regardless of typography setting
        assert_eq!(
            packages_modern, packages_very_old,
            "Typography setting should not affect Greek package selection"
        );
        assert!(
            packages_modern.contains(r"\usepackage{TheanoOldStyle}"),
            "Greek should use TheanoOldStyle package:\n{packages_modern}"
        );
    }
}
