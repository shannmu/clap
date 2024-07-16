#![cfg(feature = "unstable-dynamic")]

use std::path::Path;

use clap::{builder::PossibleValue, Command};
use snapbox::assert_data_eq;

macro_rules! complete {
    ($cmd:expr, $input:expr$(, current_dir = $current_dir:expr)? $(,)?) => {
        {
            #[allow(unused)]
            let current_dir = None;
            $(let current_dir = $current_dir;)?
            complete(&mut $cmd, $input, current_dir)
        }
    }
}

#[test]
fn suggest_subcommand_subset() {
    let mut cmd = Command::new("exhaustive")
        .subcommand(Command::new("hello-world"))
        .subcommand(Command::new("hello-moon"))
        .subcommand(Command::new("goodbye-world"));

    assert_data_eq!(
        complete!(cmd, "he"),
        snapbox::str![
            "hello-moon
hello-world
help\tPrint this message or the help of the given subcommand(s)"
        ],
    );
}

#[test]
fn suggest_hidden_long_flags() {
    let mut cmd = Command::new("exhaustive")
        .arg(clap::Arg::new("hello-world").long("hello-world"))
        .arg(clap::Arg::new("hello-moon").long("hello-moon").hide(true))
        .arg(clap::Arg::new("goodbye-world").long("goodbye-world"));

    assert_data_eq!(complete!(cmd, "--hello"), snapbox::str!["--hello-world"]);

    assert_data_eq!(complete!(cmd, "--hello-m"), snapbox::str!["--hello-moon"]);
}

#[test]
fn suggest_hidden_subcommand_and_aliases() {
    let mut cmd = Command::new("exhaustive")
        .subcommand(
            Command::new("hello-world")
                .visible_alias("hello-world-foo")
                .alias("hidden-world"),
        )
        .subcommand(
            Command::new("hello-moon")
                .visible_alias("hello-moon-foo")
                .alias("hidden-moon")
                .hide(true),
        )
        .subcommand(
            Command::new("goodbye-world")
                .visible_alias("goodbye-world-foo")
                .alias("hidden-goodbye"),
        );

    assert_data_eq!(
        complete!(cmd, "hello-"),
        snapbox::str![
            "hello-world
hello-world-foo"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "hello-m"),
        snapbox::str![
            "hello-moon
hello-moon-foo"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "hidden-"),
        snapbox::str![
            "hidden-goodbye
hidden-moon
hidden-world"
        ]
    );

    assert_data_eq!(complete!(cmd, "hidden-w"), snapbox::str!["hidden-world"]);
}

#[test]
fn suggest_subcommand_aliases() {
    let mut cmd = Command::new("exhaustive")
        .subcommand(
            Command::new("hello-world")
                .visible_alias("hello-world-foo")
                .alias("hidden-world"),
        )
        .subcommand(
            Command::new("hello-moon")
                .visible_alias("hello-moon-foo")
                .alias("hidden-moon"),
        )
        .subcommand(
            Command::new("goodbye-world")
                .visible_alias("goodbye-world-foo")
                .alias("hidden-goodbye"),
        );

    assert_data_eq!(
        complete!(cmd, "hello"),
        snapbox::str![
            "hello-moon
hello-moon-foo
hello-world
hello-world-foo"
        ],
    );
}

#[test]
fn suggest_hidden_possible_value() {
    let mut cmd = Command::new("exhaustive").arg(
        clap::Arg::new("hello-world")
            .long("hello-world")
            .value_parser([
                PossibleValue::new("hello-world").help("Say hello to the world"),
                PossibleValue::new("hello-moon")
                    .help("Say hello to the moon")
                    .hide(true),
                PossibleValue::new("goodbye-world").help("Say goodbye to the world"),
            ]),
    );

    assert_data_eq!(
        complete!(cmd, "--hello-world=hel"),
        snapbox::str![
            "--hello-world=hello-world\tSay hello to the world
--hello-world=hello-moon\tSay hello to the moon"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "--hello-world=hello-m"),
        snapbox::str!["--hello-world=hello-moon\tSay hello to the moon"]
    )
}

#[test]
fn suggest_hidden_long_flag_aliases() {
    let mut cmd = Command::new("exhaustive")
        .arg(
            clap::Arg::new("hello-world")
                .long("hello-world")
                .visible_alias("hello-world-foo")
                .alias("hidden-world"),
        )
        .arg(
            clap::Arg::new("hello-moon")
                .long("hello-moon")
                .visible_alias("hello-moon-foo")
                .alias("hidden-moon")
                .hide(true),
        )
        .arg(
            clap::Arg::new("goodbye-world")
                .long("goodbye-world")
                .alias("goodbye-world-foo"),
        );

    assert_data_eq!(
        complete!(cmd, "--hello"),
        snapbox::str![
            "--hello-world
--hello-world-foo"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "--hello-m"),
        snapbox::str![
            "--hello-moon
--hello-moon-foo"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "--hidden-"),
        snapbox::str![
            "--hidden-world
--hidden-moon"
        ]
    );

    assert_data_eq!(
        complete!(cmd, "--hidden-w"),
        snapbox::str!["--hidden-world"]
    );
}

#[test]
fn suggest_long_flag_subset() {
    let mut cmd = Command::new("exhaustive")
        .arg(
            clap::Arg::new("hello-world")
                .long("hello-world")
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("hello-moon")
                .long("hello-moon")
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("goodbye-world")
                .long("goodbye-world")
                .action(clap::ArgAction::Count),
        );

    assert_data_eq!(
        complete!(cmd, "--he"),
        snapbox::str![
            "--hello-world
--hello-moon
--help\tPrint help"
        ],
    );
}

#[test]
fn suggest_possible_value_subset() {
    let name = "exhaustive";
    let mut cmd = Command::new(name).arg(clap::Arg::new("hello-world").value_parser([
        PossibleValue::new("hello-world").help("Say hello to the world"),
        "hello-moon".into(),
        "goodbye-world".into(),
    ]));

    assert_data_eq!(
        complete!(cmd, "hello"),
        snapbox::str![
            "hello-world\tSay hello to the world
hello-moon"
        ],
    );
}

#[test]
fn suggest_additional_short_flags() {
    let mut cmd = Command::new("exhaustive")
        .arg(
            clap::Arg::new("a")
                .short('a')
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("b")
                .short('b')
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("c")
                .short('c')
                .action(clap::ArgAction::Count),
        );

    assert_data_eq!(
        complete!(cmd, "-a"),
        snapbox::str![
            "-aa
-ab
-ac
-ah\tPrint help"
        ],
    );
}

#[test]
fn suggest_subcommand_positional() {
    let mut cmd = Command::new("exhaustive").subcommand(Command::new("hello-world").arg(
        clap::Arg::new("hello-world").value_parser([
            PossibleValue::new("hello-world").help("Say hello to the world"),
            "hello-moon".into(),
            "goodbye-world".into(),
        ]),
    ));

    assert_data_eq!(
        complete!(cmd, "hello-world [TAB]"),
        snapbox::str![
            "--help\tPrint help (see more with '--help')
-h\tPrint help (see more with '--help')
hello-world\tSay hello to the world
hello-moon
goodbye-world"
        ],
    );
}

fn complete(cmd: &mut Command, args: impl AsRef<str>, current_dir: Option<&Path>) -> String {
    let input = args.as_ref();
    let mut args = vec![std::ffi::OsString::from(cmd.get_name())];
    let arg_index;

    if let Some((prior, after)) = input.split_once("[TAB]") {
        args.extend(prior.split_whitespace().map(From::from));
        if prior.ends_with(char::is_whitespace) {
            args.push(std::ffi::OsString::default());
        }
        arg_index = args.len() - 1;
        // HACK: this cannot handle in-word '[TAB]'
        args.extend(after.split_whitespace().map(From::from));
    } else {
        args.extend(input.split_whitespace().map(From::from));
        if input.ends_with(char::is_whitespace) {
            args.push(std::ffi::OsString::default());
        }
        arg_index = args.len() - 1;
    }

    clap_complete::dynamic::complete(cmd, args, arg_index, current_dir)
        .unwrap()
        .into_iter()
        .map(|candidate| {
            let compl = candidate.get_content().to_str().unwrap();
            if let Some(help) = candidate.get_help() {
                format!("{compl}\t{help}")
            } else {
                compl.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
