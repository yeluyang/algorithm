import argparse
import markdownify as md
import logging as log
import re


class Convertor(md.MarkdownConverter):
    def __init__(self, **options):
        super().__init__(**options)

    def convert_pre(self, el, text: str, convert_as_inline):
        if not text:
            return ''
        if not text.startswith('\n'):
            text = '\n' + text
        if not text.endswith('\n'):
            text = text+'\n'
        return '\n```%s%s```\n' % (self.options['code_language'], text)


def format(fpath: str):
    log.debug("formatting %s", fpath)
    convertor = Convertor(strip=[
        'strong', 'p', ], bullets='-', code_language='text')
    with open(fpath) as fp:
        lines = fp.readlines()
    if len(lines) == 0:
        return
    lines[0] = re.sub(r"^\[([0-9]+)\]\s(.*)$", "# \\1.\\2", lines[0])
    log.debug("fix title: %s", lines[0])
    rm = [
        re.compile(r"Tags: "),
        re.compile(r"Langs: "),
        re.compile(r"\* algorithms"),
        re.compile(r"\* (Easy|Medium|Hard) \([.0-9]+%\)"),
        re.compile(r"\* Likes: "),
        re.compile(r"\* Dislikes: "),
        re.compile(r"\* Total Accepted: "),
        re.compile(r"\* Total Submissions: "),
        re.compile(r"\* Testcase Example: "),
        re.compile(r"\* Source Code: "),
    ]
    for i, _ in enumerate(lines):
        for r in rm:
            if r.match(lines[i]) is not None:
                log.debug("removing: %s", lines[i])
                lines[i] = ""
        lines[i] = lines[i].replace(u'\xa0', u' ')
    lines = ''.join(lines)
    lines = convertor.convert(lines)
    with open(fpath, 'w') as fp:
        fp.write(lines)


if __name__ == "__main__":
    cli_parser = argparse.ArgumentParser("fmt_lc_qd")
    cli_parser.add_argument("--verbose", '-v', action='count')
    cli_parser.add_argument("PATHS", type=str, nargs='+',
                            help="/path/to/leetcode/questions/descriptions")
    args = cli_parser.parse_args()
    match args.verbose:
        case 0:
            log.basicConfig(level=log.CRITICAL)
        case 1:
            log.basicConfig(level=log.ERROR)
        case 2:
            log.basicConfig(level=log.WARN)
        case 3:
            log.basicConfig(level=log.INFO)
        case 4:
            log.basicConfig(level=log.DEBUG)
    log.info("starting")
    for fpath in args.PATHS:
        format(fpath)
