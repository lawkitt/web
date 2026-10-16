// Records one window with ScreenCaptureKit at full Retina resolution.
// Usage: rec <window-id> <seconds> <out.mov>
// Writes ProRes 4444 with alpha, so the rounded window corners stay transparent
// (the film clips them anyway). The window may be partly covered; SCK captures it alone.
import AVFoundation
import AppKit
import ScreenCaptureKit

_ = NSApplication.shared  // initializes the window server connection SCK needs

let args = CommandLine.arguments
let windowID = CGWindowID(args[1])!
let seconds = Double(args[2])!
let url = URL(fileURLWithPath: args[3])
try? FileManager.default.removeItem(at: url)

final class Recorder: NSObject, SCStreamOutput {
  let writer: AVAssetWriter
  let input: AVAssetWriterInput
  var started = false
  init(url: URL, width: Int, height: Int) throws {
    writer = try AVAssetWriter(outputURL: url, fileType: .mov)
    input = AVAssetWriterInput(mediaType: .video, outputSettings: [
      AVVideoCodecKey: AVVideoCodecType.proRes4444,
      AVVideoWidthKey: width,
      AVVideoHeightKey: height,
    ])
    input.expectsMediaDataInRealTime = true
    writer.add(input)
  }
  func stream(_ stream: SCStream, didOutputSampleBuffer buffer: CMSampleBuffer, of type: SCStreamOutputType) {
    guard type == .screen, buffer.isValid,
      let attachments = CMSampleBufferGetSampleAttachmentsArray(buffer, createIfNecessary: false)
        as? [[SCStreamFrameInfo: Any]],
      let raw = attachments.first?[.status] as? Int,
      SCFrameStatus(rawValue: raw) == .complete
    else { return }
    if !started {
      writer.startWriting()
      writer.startSession(atSourceTime: buffer.presentationTimeStamp)
      started = true
    }
    if input.isReadyForMoreMediaData { input.append(buffer) }
  }
}

let content = try await SCShareableContent.excludingDesktopWindows(false, onScreenWindowsOnly: true)
guard let window = content.windows.first(where: { $0.windowID == windowID }) else {
  fatalError("window \(windowID) not found")
}
// Whole-display filter cropped to the window: any window-scoped filter makes macOS draw
// its capture indicator in place of the traffic lights.
guard let display = content.displays.first(where: { $0.frame.intersects(window.frame) }) else {
  fatalError("no display for window")
}
let scale = 2
let config = SCStreamConfiguration()
config.sourceRect = CGRect(
  x: window.frame.minX - display.frame.minX, y: window.frame.minY - display.frame.minY,
  width: window.frame.width, height: window.frame.height)
config.width = Int(window.frame.width) * scale
config.height = Int(window.frame.height) * scale
config.minimumFrameInterval = CMTime(value: 1, timescale: 60)
config.pixelFormat = kCVPixelFormatType_32BGRA
config.showsCursor = true
config.shouldBeOpaque = false
let recorder = try Recorder(url: url, width: config.width, height: config.height)
let stream = SCStream(filter: SCContentFilter(display: display, excludingWindows: []), configuration: config, delegate: nil)
try stream.addStreamOutput(recorder, type: .screen, sampleHandlerQueue: DispatchQueue(label: "rec"))
try await stream.startCapture()
print("recording", config.width, "x", config.height)
try await Task.sleep(nanoseconds: UInt64(seconds * 1_000_000_000))
try await stream.stopCapture()
recorder.input.markAsFinished()
await recorder.writer.finishWriting()
print("wrote", url.path)
