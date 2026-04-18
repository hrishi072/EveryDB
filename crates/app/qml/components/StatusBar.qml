import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../theme"

Rectangle {
    height: 24
    color: Theme.primary
    
    RowLayout {
        anchors.fill: parent
        anchors.margins: Theme.spacingSmall
        spacing: Theme.spacingLarge
        
        Label {
            text: "Connected: PostgreSQL"
            color: "#ffffff"
            font.family: Typography.fontFamily
            font.pixelSize: Typography.sizeSmall
        }
        
        Label {
            text: "Database: public"
            color: "#ffffff"
            font.family: Typography.fontFamily
            font.pixelSize: Typography.sizeSmall
        }
        
        Item { Layout.fillWidth: true } // Spacer
        
        Label {
            text: "Rows: 15"
            color: "#ffffff"
            font.family: Typography.fontFamily
            font.pixelSize: Typography.sizeSmall
        }
        
        Label {
            text: "Time: 42ms"
            color: "#ffffff"
            font.family: Typography.fontFamily
            font.pixelSize: Typography.sizeSmall
        }
    }
}