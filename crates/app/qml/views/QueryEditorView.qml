import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../theme"

Rectangle {
    color: Theme.background
    
    ColumnLayout {
        anchors.fill: parent
        spacing: 0
        
        // Editor controls
        ToolBar {
            Layout.fillWidth: true
            background: Rectangle { color: Theme.surface }
            RowLayout {
                anchors.fill: parent
                spacing: Theme.spacingMedium
                
                Button {
                    text: "Run Query"
                    highlighted: true
                }
                
                Button {
                    text: "Explain"
                }
                
                Button {
                    text: "Format"
                }
                
                Item { Layout.fillWidth: true } // Spacer
            }
        }
        
        // The actual editor (simplified)
        TextArea {
            Layout.fillWidth: true
            Layout.preferredHeight: 150
            text: "SELECT * FROM users LIMIT 10;"
            color: Theme.text
            font.family: Typography.monoFamily
            font.pixelSize: Typography.sizeNormal
            background: Rectangle { color: Theme.background }
        }
        
        // Result Grid (simplified)
        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: Theme.surface
            border.color: Theme.surfaceHighlight
            
            Label {
                anchors.centerIn: parent
                text: "Query Results Here"
                color: Theme.textMuted
                font.family: Typography.fontFamily
                font.pixelSize: Typography.sizeLarge
            }
        }
    }
}