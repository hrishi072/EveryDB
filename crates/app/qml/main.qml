import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "theme"
import "components"
import "views"

ApplicationWindow {
    id: window
    visible: true
    width: 1280
    height: 800
    title: "EveryDB"
    color: Theme.background

    // Toolbar
    header: ToolBar {
        background: Rectangle { color: Theme.surface }
        RowLayout {
            anchors.fill: parent
            spacing: Theme.spacingMedium
            
            Button {
                text: "+ New Connection"
                highlighted: true
            }
            
            ComboBox {
                model: ["PostgreSQL (Local)", "MongoDB (Prod)", "Redis (Cache)"]
                Layout.fillWidth: true
            }
            
            Item { Layout.fillWidth: true } // Spacer
            
            Button {
                text: "Theme"
                onClicked: Theme.isDark = !Theme.isDark
            }
            
            Button {
                text: "Settings"
            }
        }
    }
    
    // Three-panel layout
    SplitView {
        anchors.fill: parent
        orientation: Qt.Horizontal
        
        // Left Panel - Connections
        ConnectionsView {
            SplitView.preferredWidth: 250
            SplitView.minimumWidth: 200
        }
        
        // Center Panel - Schema + Editor + Results
        SplitView {
            orientation: Qt.Vertical
            SplitView.fillWidth: true
            
            SchemaExplorerView {
                SplitView.preferredHeight: 200
            }
            
            QueryEditorView {
                SplitView.fillHeight: true
            }
        }
        
        // Right Panel - Inspector
        Rectangle {
            color: Theme.surface
            SplitView.preferredWidth: 300
            SplitView.minimumWidth: 250
            
            Label {
                text: "Inspector"
                color: Theme.text
                font.family: Typography.fontFamily
                font.pixelSize: Typography.sizeLarge
                anchors.centerIn: parent
            }
        }
    }
    
    // Status Bar
    footer: StatusBar {}
}