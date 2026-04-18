import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../theme"

Rectangle {
    color: Theme.surface
    
    ColumnLayout {
        anchors.fill: parent
        
        Label {
            text: "Connections"
            color: Theme.text
            font.family: Typography.fontFamily
            font.pixelSize: Typography.sizeNormal
            font.bold: true
            Layout.margins: Theme.spacingMedium
        }
        
        ListView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            model: ["PostgreSQL (Local)", "MongoDB (Prod)", "Redis (Cache)"]
            delegate: ItemDelegate {
                width: ListView.view.width
                text: modelData
                contentItem: Label {
                    text: parent.text
                    color: Theme.text
                    font.family: Typography.fontFamily
                    font.pixelSize: Typography.sizeNormal
                }
                background: Rectangle {
                    color: parent.hovered ? Theme.surfaceHighlight : "transparent"
                }
            }
        }
    }
}