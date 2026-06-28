#!/usr/bin/env python3
import urllib.request
import xml.etree.ElementTree as ET

URL = "https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list-one.xml"


def main():
    req = urllib.request.Request(URL, headers={"User-Agent": "Mozilla/5.0"})
    with urllib.request.urlopen(req) as resp:
        data = resp.read()

    root = ET.fromstring(data)
    published = root.get("Pblshd")

    seen = {}  # code -> (name, minor_units, numeric_code)
    order = []

    for entry in root.iter("CcyNtry"):
        code = entry.findtext("Ccy")
        name = entry.findtext("CcyNm")
        minor = entry.findtext("CcyMnrUnts")
        numeric = entry.findtext("CcyNbr")

        # Skip rows with no alphabetic code (e.g. Antarctica)
        if not code or not code.isalpha():
            continue
        if numeric is None or not numeric.isdigit():
            continue

        minor_units = int(minor) if minor and minor.isdigit() else 0

        if code in seen:
            continue
        seen[code] = (name.strip() if name else code, minor_units, int(numeric))
        order.append(code)

    print(f"//! ISO 4217 Currency definitions, published on {published}")
    print("//!")
    print(f"//! source: <{URL}>")
    print()
    print("use crate::Currency;")
    print()

    for code in order:
        name, minor, numeric = seen[code]
        print(f"/// {name}")
        print("#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]")
        print(f"pub struct {code};")
        print(f"impl Currency for {code} {{")
        print("    fn code(&self) -> &'static str {")
        print(f'        "{code}"')
        print("    }")
        print()
        print("    fn minor_units(&self) -> u32 {")
        print(f"        {minor}")
        print("    }")
        print()
        print("    fn numeric_code(&self) -> u32 {")
        print(f"        {numeric}")
        print("    }")
        print("}")
        print()

    print("/// Returns a reference to the [Currency] with the provided `code`,")
    print("/// or `None` if the code did not match one of the ISO currencies.")
    print("pub fn find(code: &str) -> Option<&dyn Currency> {")
    print("    match code {")
    for code in order:
        print(f'        "{code}" => Some(&{code}),')
    print("        _ => None,")
    print("    }")
    print("}")


if __name__ == "__main__":
    main()