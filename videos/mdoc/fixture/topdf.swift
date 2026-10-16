// Usage: swift topdf.swift out.pdf page1.jpg page2.jpg — one A4 image page per input.
import AppKit
import PDFKit
let args = CommandLine.arguments.dropFirst()
let doc = PDFDocument()
for (i, path) in args.dropFirst().enumerated() {
  guard let image = NSImage(contentsOfFile: path) else { fatalError("cannot read \(path)") }
  image.size = NSSize(width: 595, height: 842)
  doc.insert(PDFPage(image: image)!, at: i)
}
doc.write(toFile: args.first!)
