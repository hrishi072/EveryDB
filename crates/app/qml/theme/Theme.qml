pragma Singleton
import QtQuick

QtObject {
    property bool isDark: true
    
    // Colors
    property color background: isDark ? "#1e1e1e" : "#ffffff"
    property color surface: isDark ? "#252526" : "#f5f5f5"
    property color surfaceHighlight: isDark ? "#333333" : "#e0e0e0"
    property color text: isDark ? "#d4d4d4" : "#333333"
    property color textMuted: isDark ? "#858585" : "#666666"
    property color primary: "#007acc"
    property color accent: "#0098ff"
    property color error: "#f48771"
    property color success: "#89d185"
    
    // Spacing
    property int spacingSmall: 4
    property int spacingMedium: 8
    property int spacingLarge: 16
}