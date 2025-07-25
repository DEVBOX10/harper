use super::{LintGroup, MapPhraseSetLinter};

#[cfg(test)]
mod tests;

/// Produce a [`LintGroup`] that looks for errors in sets of common phrases.
pub fn lint_group() -> LintGroup {
    let mut group = LintGroup::default();

    // Each correction pair has a single bad form and a single correct form.
    macro_rules! add_1_to_1_mappings {
        ($group:expr, {
            $($name:expr => ($input_correction_pairs:expr, $message:expr, $description:expr)),+ $(,)?
        }) => {
            $(
                $group.add_expr_linter(
                    $name,
                    Box::new(
                        MapPhraseSetLinter::one_to_one(
                            $input_correction_pairs,
                            $message,
                            $description
                        ),
                    ),
                );
            )+
        };
    }

    // Each correction pair has multiple bad forms and multiple correct forms.
    macro_rules! add_many_to_many_mappings {
        ($group:expr, {
            $($name:expr => ($input_correction_multi_pairs:expr, $message:expr, $description:expr)),+ $(,)?
        }) => {
            $(
                // eprintln!("💗 {:?}", $name);
                $group.add_expr_linter(
                    $name,
                    Box::new(
                        MapPhraseSetLinter::many_to_many(
                            $input_correction_multi_pairs,
                            $message,
                            $description
                        ),
                    ),
                );
            )+
        };
    }

    add_1_to_1_mappings!(group, {
        "Ado" => (
            &[
                ("further adieu", "further ado"),
                ("much adieu", "much ado"),
            ],
            "Don't confuse the French/German `adieu`, meaning `farewell`, with the English `ado`, meaning `fuss`.",
            "Corrects `adieu` to `ado`."
        ),
        "ClientOrServerSide" => (
            &[
                ("client's side", "client-side"),
                ("server's side", "server-side"),
            ],
            "`Client-side` and `server-side` do not use an apostrophe.",
            "Corrects extraneous apostrophe in `client's side` and `server's side`."
        ),
        "DefiniteArticle" => (
            &[
                ("definitive article", "definite article"),
                ("definitive articles", "definite articles")
            ],
            "The correct term for `the` is `definite article`.",
            "The name of the word `the` is `definite article`."
        ),
        "Discuss" => (
            &[
                ("discuss about", "discuss"),
                ("discussed about", "discussed"),
                ("discusses about", "discusses"),
                ("discussing about", "discussing"),
            ],
            "`About` is redundant",
            "Removes unnecessary `about` after `discuss`."
        ),
        "ExpandArgument" => (
            &[
                ("arg", "argument"),
                ("args", "arguments"),
            ],
            "Use `argument` instead of `arg`",
            "Expands the abbreviation `arg` to the full word `argument` for clarity."
        ),
        "ExpandDependencies" => (
            &[
                ("deps", "dependencies"),
                ("dep", "dependency"),
            ],
            "Use `dependencies` instead of `deps`",
            "Expands the abbreviation `deps` to the full word `dependencies` for clarity."
        ),
        "ExpandStandardInputAndOutput" => (
            &[
                ("stdin", "standard input"),
                ("stdout", "standard output"),
                ("stderr", "standard error"),
            ],
            "Use `standard input`, `standard output`, and `standard error` instead of `stdin`, `stdout`, and `stderr`",
            "Expands the abbreviations `stdin`, `stdout`, and `stderr` to the full words `standard input`, etc. for clarity."
        ),
        "ExplanationMark" => (
            &[
                ("explanation mark", "exclamation mark"),
                ("explanation marks", "exclamation marks"),
                ("explanation point", "exclamation point"),
            ],
            "The correct names for the `!` punctuation are `exclamation mark` and `exclamation point`.",
            "Corrects the eggcorn `explanation mark/point` to `exclamation mark/point`."
        ),
        "HaveGone" => (
            &[
                ("had went", "had gone"),
                ("has went", "has gone"),
                ("have went", "have gone"),
                ("having went", "having gone"),
            ],
            "`Have gone` is the correct form.",
            "Corrects `have went` to `have gone`."
        ),
        "HavePassed" => (
            &[
                ("had past", "had passed"),
                ("has past", "has passed"),
                ("have past", "have passed"),
                ("having past", "having passed"),
            ],
            "Did you mean the verb `passed`?",
            "Suggests `past` for `passed` in case a verb was intended."
        ),
        "HomeInOn" => (
            &[
                ("hone in on", "home in on"),
                ("honed in on", "homed in on"),
                ("hones in on", "homes in on"),
                ("honing in on", "homing in on"),
            ],
            "Use `home in on` rather than `hone in on`",
            "Corrects `hone in on` to `home in on`."
        ),
        "InDetail" => (
            &[
                ("in details", "in detail"),
                ("in more details", "in more detail"),
            ],
            "Use singular `in detail` for referring to a detailed description.",
            "Corrects unidiomatic plural `in details` to `in detail`."
        ),
        "InvestIn" => (
            &[
                ("invest into", "invest in"),
                ("invested into", "invested in"),
                ("investing into", "investing in"),
                ("invests into", "invests in"),
            ],
            "Traditionally `invest` uses the preposition `in`.",
            "`Invest` is traditionally followed by 'in,' not `into.`"
        ),
        "MootPoint" => (
            &[
                ("mute point", "moot point"),
                ("point is mute", "point is moot"),
            ],
            "Use `moot` instead of `mute` when referring to a debatable or irrelevant point.",
            "Corrects `mute` to `moot` in the phrase `moot point`."
        ),
        "OperatingSystem" => (
            &[
                ("operative system", "operating system"),
                ("operative systems", "operating systems"),
            ],
            "Did you mean `operating system`?",
            "Ensures `operating system` is used correctly instead of `operative system`."
        ),
        "Piggyback" => (
            &[
                ("piggy bag", "piggyback"),
                ("piggy bagged", "piggybacked"),
                ("piggy bagging", "piggybacking"),
            ],
            "Did you mean `piggyback`?",
            "Corrects the eggcorn `piggy bag` to `piggyback`, which is the proper term for riding on someone’s back or using an existing system."
        ),
    });

    add_many_to_many_mappings!(group, {
        "ChangeTack" => (
            &[
                // verb
                (&["change tact", "change tacks", "change tacts"], &["change tack"]),
                (&["changed tact", "changed tacks", "changed tacts"], &["changed tack"]),
                (&["changes tact", "changes tacks", "changes tacts"], &["changes tack"]),
                (&["changing tact", "changing tacks", "changing tacts"], &["changing tack"]),
                // noun
                (&["change of tact", "change of tacks", "change of tacts"], &["change of tack"]),
                (&["changes of tact", "changes of tacks", "changes of tacts"], &["changes of tack"]),
                (&["changing of tact", "changing of tacks", "changing of tacts"], &["changing of tack"])
            ],
            "A change in direction or approach is a change of `tack`. Not `tact` (or `tacks` or `tacts`).",
            "Locates errors in the idioms `to change tack` and `change of tack` to convey the correct meaning of altering one's course or strategy."
        ),
        "GetRidOf" => (
            &[
                (&["get rid off", "get ride of", "get ride off"], &["get rid of"]),
                (&["gets rid off", "gets ride of", "gets ride off"], &["gets rid of"]),
                (&["getting rid off", "getting ride of", "getting ride off"], &["getting rid of"]),
                (&["got rid off", "got ride of", "got ride off"], &["got rid of"]),
                (&["gotten rid off", "gotten ride of", "gotten ride off"], &["gotten rid of"]),
            ],
            "The idiom is `to get rid of`, not `off` or `ride`.",
            "Corrects common misspellings of the idiom `get rid of`."
        ),
        "HowItLooksLike" => (
            &[
                (&["how he looks like"], &["how he looks", "what he looks like"]),
                (&["how it looks like", "how it look like", "how it look's like"], &["how it looks", "what it looks like"]),
                (&["how she looks like"], &["how she looks", "what she looks like"]),
                (&["how they look like", "how they looks like"], &["how they look", "what they look like"]),
            ],
            "Don't use both `how` and `like` together to express similarity.",
            "Corrects `how ... looks like` to `how ... looks` or `what ... looks like`."
        ),
        "RiseTheQuestion" => (
            &[
                (&["rise the question"], &["raise the question"]),
                (&["rises the question"], &["raises the question"]),

            ],
            "Use `raise` instead of `rise` when referring to the act of asking a question.",
            "Corrects `rise the question` to `raise the question`."
        ),
        "WholeEntire" => (
            &[
                (&["whole entire"], &["whole", "entire"]),
                // Avoid suggestions resulting in "a entire ...."
                (&["a whole entire"], &["a whole", "an entire"]),
            ],
            "Avoid redundancy. Use either `whole` or `entire` for referring to the complete amount or extent.",
            "Corrects the redundancy in `whole entire` to `whole` or `entire`."
        ),
        "WorseOrWorst" => (
            &[
                // worst -> worse
                (&["a lot worst", "alot worst"], &["a lot worse"]),
                (&["far worst"], &["far worse"]),
                (&["much worst"], &["much worse"]),
                (&["turn for the worst"], &["turn for the worse"]),
                (&["worst and worst", "worse and worst", "worst and worse"], &["worse and worse"]),
                (&["worst than"], &["worse than"]),
                // worse -> worst
                (&["worse case scenario", "worse-case scenario", "worse-case-scenario"], &["worst-case scenario"]),
                (&["worse ever"], &["worst ever"]),
            ],
            "`Worse` is for comparing and `worst` is for the extreme case.",
            "Corrects `worse` and `worst` used in contexts where the other belongs."
        )
    });

    group.set_all_rules_to(Some(true));

    group
}
