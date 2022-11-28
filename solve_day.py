#!/bin/env python

from argparse import ArgumentParser, Namespace
from datetime import date
from typing import Tuple
from pathlib import Path
from subprocess import check_call
from shutil import rmtree
from urllib.request import Request, urlopen
from urllib.error import HTTPError

TODAY = date.today()


def create_parser() -> ArgumentParser:
    parser = ArgumentParser(description="Create a default setup with everything necessary to write the solution")
    parser.add_argument("-s", "--session", help="aoc session cookie", required=True)
    parser.add_argument("-y", "--year", type=int, help="year [default: current year]", default=TODAY.year)
    parser.add_argument("-d", "--day", type=int, help="day [default: today if invoked during AOC]", default=0)

    solution_settings = parser.add_argument_group(
        "Solution settings", "Customise the solution template generated [default: rust]"
    )
    lang = solution_settings.add_mutually_exclusive_group()
    lang.add_argument(
        "--rust",
        action="store_const",
        const=create_rust_template,
        dest="lang_generator",
        help="Create a rust template for solution",
    )
    lang.add_argument(
        "--python",
        action="store_const",
        const=create_python_template,
        dest="lang_generator",
        help="Create a python template for solution",
    )
    solution_settings.add_argument(
        "--reset", help="delete any existing solution and replace with blank template", action="store_true"
    )
    parser.set_defaults(lang_generator=create_rust_template)
    return parser


def extract_puzzle_day(args: Namespace) -> Tuple[int, int]:
    if args.day == 0:
        if TODAY.month != 12 or TODAY.day > 25:
            raise ValueError("Today is not an AOC day. Please specify the exact date")
        args.day = TODAY.day
    if 2015 > args.year or args.year > TODAY.year:
        raise ValueError(f"AOC has only ran from 2015 till this year. Received {args.year}")
    if 1 > args.day or args.day > 25:
        raise ValueError(f"AOC only runs from 1st december to 25th december. Received {args.day}")
    return (args.year, args.day)


def main():
    parser = create_parser()
    args = parser.parse_args()
    year, day = extract_puzzle_day(args)
    lang_template_generator = args.lang_generator
    lang_template_generator(year, day, args.reset)
    download_input(year, day, args.session)


class RustTemplate:
    @staticmethod
    def init_workplace():
        already_init = Path("./Cargo.toml").exists()
        if already_init:
            # maybe parse the file and add the specific year explicitly here
            return
        with open("Cargo.toml", "w+") as f:
            f.write('[workspace]\nmembers = ["2022/rust/*"]')

    @staticmethod
    def init_solution(year: int, day: int, reset: bool = False):
        already_init = Path(f"{year}/rust/{day:02}").exists()
        if already_init:
            if reset:
                rmtree(f"{year}/rust/{day:02}")
            else:
                return
        Path(f"{year}/rust").mkdir(parents=True, exist_ok=True)
        check_call(["cargo", "new", f"{day:02}", "--name", f"aoc-{year}-{day:02}"], cwd=f"{year}/rust")


class PythonTemplate:
    @staticmethod
    def init_solution(year: int, day: int, reset: bool = False):
        already_init = Path(f"{year}/python/{day:02}").exists()
        if already_init:
            if reset:
                rmtree(f"{year}/python/{day:02}")
            else:
                return
        Path(f"{year}/python/{day:02}").mkdir(parents=True, exist_ok=True)
        Path(f"{year}/python/{day:02}/solution.py").touch()


def create_rust_template(year: int, day: int, reset: bool):
    RustTemplate.init_workplace()
    RustTemplate.init_solution(year, day, reset)


def create_python_template(year: int, day: int, reset: bool):
    PythonTemplate.init_solution(year, day, reset)


def download_input(year: int, day: int, session: str):
    if Path(f"{year}/inputs/{day:02}").exists():
        return
    Path(f"{year}/inputs").mkdir(parents=True, exist_ok=True)
    req = Request(f"https://adventofcode.com/{year}/day/{day}/input")
    req.add_header("Cookie", "session=" + session)
    try:
        res = urlopen(req)
        input = res.read()
        with open(f"{year}/inputs/{day:02}", "w+") as f:
            f.write(input.decode())
    except HTTPError as e:
        if e.code == 404:
            print("No input found. Skipping")
        else:
            raise e


if __name__ == "__main__":
    main()
