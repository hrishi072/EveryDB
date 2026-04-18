import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../theme"

Rectangle {
    color: Theme.surfaceHighlight
    
    ColumnLayout {
        anchors.fill: parent
        
        Label {
            text: "Schema Explorer"
            color: Theme.text
            font.family: Typography.fontFamily
            font.pixelSize: Typography.sizeNormal
            font.bold: true
            Layout.margins: Theme.spacingMedium
        }
        
        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            
            Label {
                text: "> users\n> products\n> orders"
                color: Theme.text
                font.family: Typography.monoFamily
                font.pixelSize: Typography.sizeNormal
                padding: Theme.spacingMedium
            }
        }
    }
}