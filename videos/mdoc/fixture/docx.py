"""Writes the fictional claim letter as a minimal, well-formed DOCX.

Real Heading 1/2 styles and a real table, so mdoc's conversion shows structure.
Usage: python3 docx.py "Letter before claim.docx"
"""

import sys
import zipfile
from xml.sax.saxutils import escape

LETTER = [
    ("h1", "Marsh & Cole LLP"),
    ("p", "Solicitors, 14 Harbour Row, Bristol BS1 4QA"),
    ("p", "Our ref: MC/2026/0417 · Your ref: NL-CR-883 · 3 November 2026"),
    ("p", "Daniel Okafor, Head of Legal, Northgate Logistics Ltd, Unit 9, Ferry Lane "
          "Industrial Estate, Southampton SO14 3TL. By email: d.okafor@northgate-logistics.example"),
    ("h1", "Letter before claim"),
    ("p", "We act for Brightwater Foods Ltd (company no. 08812345) in connection with the "
          "haulage agreement dated 12 March 2026 between our client and Northgate Logistics "
          "Ltd (the “Agreement”)."),
    ("p", "On 22 September 2026 a refrigerated consignment collected from our client’s depot "
          "was delivered two days late and at an unsafe temperature. Our client’s quality "
          "manager, Priya Raman, rejected the load on arrival and recorded the reasons the same "
          "day. Northgate Logistics Ltd has since invoiced our client in full and has not "
          "responded to her complaint of 25 September 2026."),
    ("h2", "Sums claimed"),
    ("table", [
        ["Item", "Reference", "Amount"],
        ["Rejected consignment", "PO-55120", "£18,420.00"],
        ["Disposal and re-delivery", "INV-7731", "£2,915.50"],
        ["Overpaid freight charges", "INV-7698", "£1,240.00"],
    ]),
    ("p", "Our client therefore claims £22,575.50, together with interest under the Late "
          "Payment of Commercial Debts (Interest) Act 1998."),
    ("h2", "What we ask you to do"),
    ("p", "Please pay £22,575.50 by 17 November 2026 to our client account at Harbour Bank "
          "plc, IBAN GB33 BUKB 2020 1555 5555 55, quoting reference MC/2026/0417."),
    ("p", "If you dispute the claim, please send a full written response by the same date. "
          "Your driver’s delivery log for vehicle WX71 KLM and the temperature records for "
          "22 September 2026 should be preserved."),
    ("h2", "Next steps"),
    ("p", "If we do not receive payment or a substantive response by 17 November 2026, our "
          "client intends to issue proceedings in the Business and Property Courts in Bristol. "
          "Our client remains open to mediation."),
    ("p", "Please direct all correspondence to Helen Marsh on 0117 496 0832 or at "
          "h.marsh@marshcole.example."),
    ("p", "Yours sincerely,"),
    ("p", "Helen Marsh, Partner, Marsh & Cole LLP"),
    ("p", "cc: Priya Raman, Brightwater Foods Ltd, 41 Mill Street, Bath BA1 2JN"),
]

W = 'xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"'


def run(text, bold=False):
    props = "<w:rPr><w:b/></w:rPr>" if bold else ""
    return f'<w:r>{props}<w:t xml:space="preserve">{escape(text)}</w:t></w:r>'


def para(text, style=None, bold=False):
    props = f'<w:pPr><w:pStyle w:val="{style}"/></w:pPr>' if style else ""
    return f"<w:p>{props}{run(text, bold)}</w:p>"


def table(rows):
    border = "".join(
        f'<w:{side} w:val="single" w:sz="4" w:space="0" w:color="444444"/>'
        for side in ("top", "left", "bottom", "right", "insideH", "insideV")
    )
    out = f"<w:tbl><w:tblPr><w:tblW w:w=\"0\" w:type=\"auto\"/><w:tblBorders>{border}</w:tblBorders></w:tblPr>"
    out += "<w:tblGrid>" + '<w:gridCol w:w="3000"/>' * len(rows[0]) + "</w:tblGrid>"
    for i, row in enumerate(rows):
        out += "<w:tr>" + "".join(
            f'<w:tc><w:tcPr><w:tcW w:w="3000" w:type="dxa"/></w:tcPr>{para(cell, bold=i == 0)}</w:tc>'
            for cell in row
        ) + "</w:tr>"
    return out + "</w:tbl>"


def body():
    parts = []
    for kind, value in LETTER:
        if kind == "table":
            parts.append(table(value))
        else:
            parts.append(para(value, {"h1": "Heading1", "h2": "Heading2"}.get(kind)))
    return "".join(parts)


STYLES = f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles {W}>
<w:docDefaults><w:rPrDefault><w:rPr><w:rFonts w:ascii="Georgia" w:hAnsi="Georgia"/><w:sz w:val="22"/></w:rPr></w:rPrDefault>
<w:pPrDefault><w:pPr><w:spacing w:after="160" w:line="276" w:lineRule="auto"/></w:pPr></w:pPrDefault></w:docDefaults>
<w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/></w:style>
<w:style w:type="paragraph" w:styleId="Heading1"><w:name w:val="heading 1"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/>
<w:pPr><w:keepNext/><w:spacing w:before="360" w:after="160"/><w:outlineLvl w:val="0"/></w:pPr><w:rPr><w:b/><w:sz w:val="32"/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Heading2"><w:name w:val="heading 2"/><w:basedOn w:val="Normal"/><w:next w:val="Normal"/>
<w:pPr><w:keepNext/><w:spacing w:before="280" w:after="120"/><w:outlineLvl w:val="1"/></w:pPr><w:rPr><w:b/><w:sz w:val="26"/></w:rPr></w:style>
</w:styles>"""

DOCUMENT = f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document {W}><w:body>{{body}}<w:sectPr><w:pgSz w:w="11906" w:h="16838"/>
<w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440" w:header="708" w:footer="708" w:gutter="0"/></w:sectPr></w:body></w:document>"""

CONTENT_TYPES = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
<Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>
</Types>"""

RELS = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"""

DOC_RELS = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>
</Relationships>"""

with zipfile.ZipFile(sys.argv[1], "w", zipfile.ZIP_DEFLATED) as z:
    z.writestr("[Content_Types].xml", CONTENT_TYPES)
    z.writestr("_rels/.rels", RELS)
    z.writestr("word/_rels/document.xml.rels", DOC_RELS)
    z.writestr("word/styles.xml", STYLES)
    z.writestr("word/document.xml", DOCUMENT.replace("{body}", body()))

# The Markdown mdoc makes from this DOCX, so a capture session can restore the tab
# directly (session.json needs a .md path beside its source attachment).
md = []
for kind, value in LETTER:
    if kind == "table":
        md.append("| " + " | ".join(value[0]) + " |")
        md.append("|" + " --- |" * len(value[0]))
        md.extend("| " + " | ".join(row) + " |" for row in value[1:])
        md.append("")
    else:
        md.append({"h1": "# ", "h2": "## "}.get(kind, "") + value + "\n")
with open(sys.argv[1].rsplit(".", 1)[0] + ".md", "w") as f:
    f.write("\n".join(md))
