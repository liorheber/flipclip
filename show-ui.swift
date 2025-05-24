#!/usr/bin/swift

import Cocoa

// Get command-line arguments
let args = CommandLine.arguments
let title = args.count > 1 ? args[1] : "FlipClip"
let message = args.count > 2 ? args[2] : ""
let duration = args.count > 3 ? Double(args[3]) ?? 2.0 : 2.0

// Create a floating window
let screen = NSScreen.main!
let screenRect = screen.visibleFrame

// Calculate window position (top-right corner)
let windowWidth: CGFloat = 300
let windowHeight: CGFloat = 80
let windowX = screenRect.maxX - windowWidth - 20
let windowY = screenRect.maxY - windowHeight - 20

let window = NSWindow(
    contentRect: NSRect(x: windowX, y: windowY, width: windowWidth, height: windowHeight),
    styleMask: [.borderless],
    backing: .buffered,
    defer: false
)

window.isOpaque = false
window.backgroundColor = NSColor.clear
window.level = .floating
window.hasShadow = true

// Create background view
let backgroundView = NSVisualEffectView(frame: window.contentView!.bounds)
backgroundView.blendingMode = .behindWindow
backgroundView.material = .hudWindow
backgroundView.state = .active
backgroundView.wantsLayer = true
backgroundView.layer?.cornerRadius = 10
window.contentView = backgroundView

// Create title label
let titleLabel = NSTextField(frame: NSRect(x: 20, y: 40, width: windowWidth - 40, height: 25))
titleLabel.stringValue = title
titleLabel.isEditable = false
titleLabel.isBordered = false
titleLabel.backgroundColor = .clear
titleLabel.font = NSFont.boldSystemFont(ofSize: 16)
titleLabel.textColor = .white
backgroundView.addSubview(titleLabel)

// Create message label
if !message.isEmpty {
    let messageLabel = NSTextField(frame: NSRect(x: 20, y: 15, width: windowWidth - 40, height: 20))
    messageLabel.stringValue = message
    messageLabel.isEditable = false
    messageLabel.isBordered = false
    messageLabel.backgroundColor = .clear
    messageLabel.font = NSFont.systemFont(ofSize: 13)
    messageLabel.textColor = .lightGray
    backgroundView.addSubview(messageLabel)
}

// Show window
window.orderFront(nil)

// Auto-hide after duration
DispatchQueue.main.asyncAfter(deadline: .now() + duration) {
    NSAnimationContext.runAnimationGroup({ context in
        context.duration = 0.5
        window.animator().alphaValue = 0
    }) {
        exit(0)
    }
}

// Keep the app running
NSApplication.shared.run()
