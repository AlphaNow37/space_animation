from textwrap import dedent
import pathlib
import re
import itertools

MACRO_USE_RE = re.compile("[a-zA-Z]!")

class Stats:
    def __init__(self) -> None:
        self.file = 0
        self.line = 0
        self.char = 0
        self.macro_rule = 0
        self.macro_use = 0
        self.folder = 0
        self.mod = 0
        self.struct = 0
        self.enum = 0
        self.file_by_ext = {}
    
    def search(self, path: pathlib.Path):
        if path.is_file():
            self.file += 1
            ext = path.suffix
            self.file_by_ext[ext] = self.file_by_ext.setdefault(ext, 0) + 1
            if path.name == "mod.rs":
                self.mod += 1
            content = path.read_text()
            self.line += content.count("\n")
            self.char += len(content)
            self.macro_rule += content.count("macro_rules!")
            self.macro_use += len(re.findall(MACRO_USE_RE, content))
            self.struct += content.count("struct")
            self.enum += content.count("enum")
        else:
            self.folder += 1
            for sub in path.iterdir():
                self.search(sub)
    
    def pprint(self):
        print(dedent(f"""
        Stats:
        {self.folder} folders
        {self.file} files:
        - {", ".join(f"{count}{repr(ext)}" for (ext, count) in self.file_by_ext.items())}
        - {self.mod} mods
        {self.line} lines
        {self.char} chars
        {self.macro_use} macro used, {self.macro_rule} declared
        {self.struct} structs, {self.enum} enums
        """).strip())

stats = Stats()
stats.search(pathlib.Path(__file__).parent.parent / "src")
stats.pprint()
