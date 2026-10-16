// Synthetic input for scripted captures, in global screen points.
// Usage (commands run in order):
//   input move X Y [ms]     eased pointer glide (default 600 ms)
//   input click [X Y]       click at the pointer or at X Y
//   input dclick [X Y]      double click
//   input drag X1 Y1 X2 Y2  press, drag, release (text selection)
//   input key CODE [mods]   virtual key code, mods: cmd,shift,alt,ctrl
//   input type TEXT         Unicode text
//   input wait MS
//   input front PID         activate PID; key, type and clicks then refuse to run
//                           unless PID is still the frontmost app (exit 3)
import AppKit
import CoreGraphics
import Foundation

var guardPid: pid_t?
func requireFront() {
  guard let pid = guardPid else { FileHandle.standardError.write("input: call front PID first\n".data(using: .utf8)!); exit(3) }
  for _ in 0..<60 {
    if NSWorkspace.shared.frontmostApplication?.processIdentifier == pid { return }
    usleep(50_000)
  }
  FileHandle.standardError.write("input: pid \(pid) is not frontmost, refusing input\n".data(using: .utf8)!)
  exit(3)
}

let src = CGEventSource(stateID: .hidSystemState)
func pos() -> CGPoint { CGEvent(source: nil)!.location }
func post(_ e: CGEvent?) { e?.post(tap: .cghidEventTap) }
func mouse(_ type: CGEventType, _ p: CGPoint, clicks: Int64 = 1) {
  let e = CGEvent(mouseEventSource: src, mouseType: type, mouseCursorPosition: p, mouseButton: .left)
  e?.setIntegerValueField(.mouseEventClickState, value: clicks)
  post(e)
}
func sleepMs(_ ms: Double) { usleep(useconds_t(ms * 1000)) }

var args = Array(CommandLine.arguments.dropFirst())
func next() -> String { args.removeFirst() }
while !args.isEmpty {
  switch next() {
  case "move":
    let to = CGPoint(x: Double(next())!, y: Double(next())!)
    let ms = args.first.flatMap(Double.init).map { _ in Double(next())! } ?? 600
    let from = pos()
    let steps = max(Int(ms / 8), 1)
    for i in 1...steps {
      let t = Double(i) / Double(steps)
      let k = t < 0.5 ? 4 * t * t * t : 1 - pow(-2 * t + 2, 3) / 2
      mouse(.mouseMoved, CGPoint(x: from.x + (to.x - from.x) * k, y: from.y + (to.y - from.y) * k))
      sleepMs(8)
    }
  case "front":
    let pid = pid_t(next())!
    guardPid = pid
    // NSRunningApplication.activate is unreliable with two instances of one bundle id;
    // System Events addresses the process by pid.
    let task = Process()
    task.executableURL = URL(fileURLWithPath: "/usr/bin/osascript")
    task.arguments = ["-e", "tell application \"System Events\" to set frontmost of (first process whose unix id is \(pid)) to true"]
    try? task.run()
    task.waitUntilExit()
    requireFront()
  case let cmd where cmd == "click" || cmd == "dclick":
    requireFront()
    let p = args.first.flatMap(Double.init) != nil ? CGPoint(x: Double(next())!, y: Double(next())!) : pos()
    mouse(.mouseMoved, p)
    for n in 1...(cmd == "dclick" ? 2 : 1) {
      mouse(.leftMouseDown, p, clicks: Int64(n)); sleepMs(50); mouse(.leftMouseUp, p, clicks: Int64(n)); sleepMs(80)
    }
  case "drag":
    requireFront()
    let a = CGPoint(x: Double(next())!, y: Double(next())!)
    let b = CGPoint(x: Double(next())!, y: Double(next())!)
    mouse(.mouseMoved, a); mouse(.leftMouseDown, a); sleepMs(80)
    for i in 1...60 {
      let t = Double(i) / 60
      mouse(.leftMouseDragged, CGPoint(x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t))
      sleepMs(12)
    }
    mouse(.leftMouseUp, b)
  case "key":
    requireFront()
    let code = CGKeyCode(next())!
    var flags: CGEventFlags = []
    if let m = args.first, Int(m) == nil, !["move","click","dclick","drag","key","type","wait","front"].contains(m) {
      for part in next().split(separator: ",") {
        switch part {
        case "cmd": flags.insert(.maskCommand)
        case "shift": flags.insert(.maskShift)
        case "alt": flags.insert(.maskAlternate)
        case "ctrl": flags.insert(.maskControl)
        default: break
        }
      }
    }
    for down in [true, false] {
      let e = CGEvent(keyboardEventSource: src, virtualKey: code, keyDown: down)
      e?.flags = flags
      post(e); sleepMs(30)
    }
  case "type":
    requireFront()
    for ch in next() {
      let units = Array(String(ch).utf16)
      for down in [true, false] {
        let e = CGEvent(keyboardEventSource: src, virtualKey: 0, keyDown: down)
        e?.keyboardSetUnicodeString(stringLength: units.count, unicodeString: units)
        post(e)
      }
      sleepMs(35)
    }
  case "wait":
    sleepMs(Double(next())!)
  default:
    fatalError("unknown command")
  }
}
